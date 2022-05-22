use chrono::NaiveDate;

use super::config::Config;
use crate::model::person::Person;
use crate::model::shift::Shift;

#[derive(Debug, Default, Clone)]
pub struct Dispatch<'a> {
    pub allocations: Vec<(NaiveDate, Vec<(&'a Shift, &'a Person)>)>,
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
                .all(|daily| daily.1.len() == num_shifts)
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
        let last_date = self.allocations.last();
        match last_date {
            Some(date) => {
                todo!("{date:?}")
            }
            None => {
                self.allocations.push((
                    config.planning.date_start,
                    vec![(&config.planning.shifts[0], person)],
                ));
            }
        }
    }
}
