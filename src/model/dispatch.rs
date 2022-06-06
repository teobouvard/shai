use std::collections::HashMap;
use std::ops::Range;

use chrono::Datelike;
use chrono::Duration;
use chrono::NaiveDate;
use chrono::NaiveDateTime;
use chrono::Weekday;
use log::debug;

use crate::model::config::Config;
use crate::model::constraints::Constraints;
use crate::model::person::Person;
use crate::model::shift::Shift;

#[derive(Debug, Clone)]
pub struct Allocation<'a> {
    pub date: NaiveDate,
    pub shift: &'a Shift,
    pub person: Option<&'a Person>,
}

impl Allocation<'_> {
    pub fn datetime_start(&self) -> NaiveDateTime {
        self.shift.datetime_start(&self.date)
    }

    pub fn datetime_end(&self) -> NaiveDateTime {
        self.shift.datetime_end(&self.date)
    }

    pub fn datetime_range(&self) -> Range<NaiveDateTime> {
        self.datetime_start()..self.datetime_end()
    }
}

#[derive(Debug, Clone)]
pub struct Dispatch<'a> {
    pub config: &'a Config,
    pub allocations: Vec<Allocation<'a>>,
    shift_load: HashMap<&'a Shift, usize>,
    weekly_shift_load: usize,
}

impl<'a> Dispatch<'a> {
    pub fn new(config: &'a Config) -> Self {
        let mut obj = Dispatch {
            config,
            shift_load: HashMap::new(),
            weekly_shift_load: 0,
            allocations: config
                .planning
                .date_start
                .iter_days()
                .take_while(|date| date < &config.planning.date_end)
                .flat_map(|date| {
                    config
                        .planning
                        .shifts
                        .iter()
                        .filter(move |shift| shift.occurs_on_date(&date))
                        .map(move |shift| Allocation {
                            date,
                            shift,
                            person: None,
                        })
                })
                .collect(),
        };

        obj.shift_load = config
            .planning
            .shifts
            .iter()
            .map(|shift| {
                (
                    shift,
                    obj.allocations
                        .iter()
                        .filter(|alloc| alloc.shift == shift)
                        .count()
                        / obj
                            .config
                            .members
                            .iter()
                            .filter(|(_, constraints)| obj.can_assign_to_shift(shift, constraints))
                            .count(),
                )
            })
            .collect();

        let n_weeks = (config.planning.date_end - config.planning.date_start).num_weeks();
        obj.weekly_shift_load = (config.planning.shifts.len() / n_weeks as usize) + 1;
        // Prevent zero shift load -> no allocation possible
        if obj.weekly_shift_load == 0 {
            obj.weekly_shift_load = 1;
        }

        obj
    }

    pub fn is_complete(&self) -> bool {
        self.allocations.iter().all(|alloc| alloc.person.is_some())
    }

    pub fn successors(&self) -> Vec<Self> {
        let next_alloc = self.allocations.iter().find(|alloc| alloc.person.is_none());
        match next_alloc {
            Some(alloc) => self
                .config
                .members
                .iter()
                .filter(|(person, constraints)| {
                    self.is_available(&alloc.date, alloc.shift, person, constraints)
                })
                .map(|(p, _)| {
                    let mut copy = self.clone();
                    let refer = copy
                        .allocations
                        .iter_mut()
                        .find(|alloc| alloc.person.is_none())
                        .expect("Already checked, should not fail");
                    refer.person = Some(p);
                    copy
                })
                .collect(),
            None => vec![],
        }
    }

    fn is_available(
        &self,
        date: &NaiveDate,
        shift: &Shift,
        person: &Person,
        constraints: &Constraints,
    ) -> bool {
        debug!(
            "Checking availability for '{}' '{}' '{}'",
            date.to_string(),
            &shift.name,
            person.name
        );

        // Shift exclusions
        if !self.can_assign_to_shift(shift, constraints) {
            debug!("Shift not assignable");
            return false;
        }

        // Vacations
        if constraints.vacations.contains(date) {
            debug!("On vacation");
            return false;
        }

        let previous_person_allocations = self.previous_person_allocations(person);
        let shift_start = shift.datetime_start(date);
        let shift_end = shift.datetime_end(date);

        // Shift overlap + rest duration
        for alloc in &previous_person_allocations {
            let range = alloc.datetime_range();
            if range.contains(&shift_start) || range.contains(&shift_end) {
                debug!("Overlap with '{}'", alloc.shift.name);
                return false;
            }
            if alloc.shift.rest_needed {
                let last_shift_end = alloc.datetime_end();
                let time_between_shifts = (shift_start - last_shift_end)
                    .to_std()
                    .expect("Invalid duration");
                debug!(
                    "Time since {} ({} -> {}) : {}",
                    alloc.shift.name,
                    last_shift_end,
                    shift_start,
                    time_between_shifts.as_secs()
                );
                if time_between_shifts < alloc.shift.rest_duration {
                    debug!("Not enough rest '{}'", alloc.shift.name);
                    return false;
                }
            }
            debug!(
                "Overlap and rest period compatible with {} {}",
                alloc.date, alloc.shift.name
            );
        }

        // Shift balancing
        // TODO[feature] balance on different time periods (week, month)
        let num_same_shifts = previous_person_allocations
            .iter()
            .filter(|alloc| alloc.shift.name == shift.name)
            .count();

        let num_expected_shifts = *self.shift_load.get(shift).unwrap_or(&0);
        if num_same_shifts > num_expected_shifts {
            debug!("Shift imbalance");
            return false;
        }

        // Shift load
        let num_shifts_in_week = previous_person_allocations
            .iter()
            .filter(|alloc| (shift_start - alloc.datetime_start()) < Duration::weeks(1))
            .count();
        if num_shifts_in_week > self.weekly_shift_load {
            debug!("Too many shifts in last week");
            return false;
        }

        // Joined dates
        // TODO[feature] Join on configurable weekdays
        let day_join_first = Weekday::Fri;
        let day_join_second = Weekday::Sun;
        if shift_start.weekday() == day_join_second
            && !previous_person_allocations.iter().any(|alloc| {
                alloc.date.weekday() == day_join_first
                    && shift_start - alloc.datetime_start() < Duration::weeks(1)
            })
        {
            debug!("No joined shifts");
            return false;
        }

        // TODO[feature]: Shift exclusions

        debug!("Availability OK");
        true
    }

    fn can_assign_to_shift(&self, shift: &Shift, constraints: &Constraints) -> bool {
        !constraints.shifts_exclude.contains(&shift.name)
    }

    fn previous_person_allocations(&self, person: &Person) -> Vec<&Allocation> {
        self.allocations
            .iter()
            .filter(|alloc| alloc.person == Some(person))
            .collect()
    }

    pub fn info(&self) {
        let n_total = self.allocations.len();
        let n_filled = self
            .allocations
            .iter()
            .filter(|alloc| alloc.person.is_some())
            .count();
        println!("{n_filled} / {n_total}");
    }
}
