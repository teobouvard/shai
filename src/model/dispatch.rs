use chrono::NaiveDate;

use crate::model::config::Config;
use crate::model::constraints::Constraints;
use crate::model::person::Person;
use crate::model::shift::Shift;

#[derive(Debug, Clone)]
pub struct Allocation<'a> {
    date: NaiveDate,
    shift: &'a Shift,
    person: Option<&'a Person>,
}

#[derive(Debug, Clone)]
pub struct Dispatch<'a> {
    config: &'a Config,
    pub allocations: Vec<Allocation<'a>>,
}

impl<'a> Dispatch<'a> {
    pub fn new(config: &'a Config) -> Self {
        Dispatch {
            config,
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
        }
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
        let previous_person_shifts = self.previous_person_shifts(person);

        // Vacations
        if constraints.vacations.contains(date) {
            return false;
        }

        // TODO: Rest duration
        match previous_person_shifts.last() {
            Some(alloc) => {
                if alloc.shift.rest_needed {
                    let next_shift_start = shift.datetime_start(date);
                    let last_shift_end = alloc.shift.datetime_end(&alloc.date);
                    let time_between_shifts = (next_shift_start - last_shift_end)
                        .to_std()
                        .expect("Invalid duration");
                    if time_between_shifts < alloc.shift.rest_duration {
                        return false;
                    }
                }
            }
            None => (),
        }

        // TODO: Shift balancing
        // TODO: Shift exclusions

        true
    }

    fn previous_person_shifts(&self, person: &Person) -> Vec<&Allocation> {
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
