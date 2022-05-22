use chrono::NaiveDate;

use super::config::Config;
use crate::model::person::Person;
use crate::model::shift::Shift;

#[derive(Debug, Clone)]
pub struct Allocation<'a> {
    date: NaiveDate,
    shifts: Vec<(&'a Shift, &'a Person)>,
}

impl<'a> Allocation<'a> {
    pub fn new(date: NaiveDate, shift: &'a Shift, person: &'a Person) -> Self {
        Allocation {
            date,
            shifts: vec![(shift, person)],
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Dispatch<'a> {
    pub allocations: Vec<Allocation<'a>>,
}

impl<'a> Dispatch<'a> {
    pub fn is_complete(&self, config: &Config) -> bool {
        let planning_duration = config
            .planning
            .date_end
            .signed_duration_since(config.planning.date_start)
            .num_days();
        let num_shifts = config.planning.shifts.len();
        self.allocations.len() as i64 == planning_duration
            && self
                .allocations
                .iter()
                .all(|day| day.shifts.len() == num_shifts)
    }

    pub fn successors(&self, config: &'a Config) -> Vec<Self> {
        config
            .members
            .iter()
            .map(|(p, _constraints)| {
                let mut copy = self.clone();
                copy.add_to_next_shift(p, config);
                copy
            })
            .collect()
    }

    fn add_to_next_shift(&mut self, person: &'a Person, config: &'a Config) {
        match self.allocations.last() {
            Some(alloc) => {
                if config.planning.shifts.len() == alloc.shifts.len() {
                    self.allocations.push(Allocation::new(
                        alloc.date.succ(),
                        &config.planning.shifts[0],
                        person,
                    ))
                } else {
                    let next_shift = &config.planning.shifts[alloc.shifts.len()];
                    if let Some(v) = self.allocations.last_mut() {
                        v.shifts.push((next_shift, person))
                    }
                }
            }
            None => {
                self.allocations.push(Allocation::new(
                    config.planning.date_start,
                    &config.planning.shifts[0],
                    person,
                ));
            }
        }
    }
}
