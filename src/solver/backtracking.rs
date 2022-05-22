use anyhow::anyhow;
use anyhow::Ok;
use anyhow::Result;

use crate::model::config::Config;
use crate::model::dispatch::Dispatch;

pub struct Solver;

impl Solver {
    pub fn solve(config: &Config) -> Result<Dispatch> {
        let mut stack = Vec::new();
        let start = Dispatch::default();
        stack.push(start);

        while !stack.is_empty() {
            let candidate = stack.pop();
            match candidate {
                Some(candidate) => {
                    dbg!(&candidate);

                    if candidate.is_complete(config) {
                        return Ok(candidate);
                    }

                    let successors = candidate.successors(config);
                    stack.extend(successors);
                }
                None => break,
            }
        }

        Err(anyhow!("Could not find solution"))
    }
}
