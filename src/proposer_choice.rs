use std::collections::HashMap;

// trait ProposerSelection {
//     fn select_next_proposer(height: u64, round: u64) -> Validator;
// }

/// Represents a validator in the validator set
#[derive(Debug, Clone)]
pub struct Validator {
    /// The validator's ID
    pub id: usize,
    /// The validator's voting power
    pub voting_power: u64,
}

/// Represents the validator set with their priorities
#[derive(Debug, Clone)]
pub struct ValidatorSet {
    /// The validators in the set
    pub validators: Vec<Validator>,
    /// The accumulated priorities for each validator
    pub priorities: HashMap<usize, i64>,
}

impl ValidatorSet {
    /// Creates a new validator set with the given validators
    pub fn new(validators: Vec<Validator>) -> Self {
        let priorities = validators
            .iter()
            .map(|v| (v.id, 0i64))
            .collect();
        Self {
            validators,
            priorities,
        }
    }

    /// Gets the total voting power of the validator set
    pub fn total_voting_power(&self) -> u64 {
        self.validators.iter().map(|v| v.voting_power).sum()
    }

    /// Gets the average priority of all validators
    pub fn average_priority(&self) -> f64 {
        let sum: i64 = self.priorities.values().sum();
        sum as f64 / self.validators.len() as f64
    }

    /// Selects the proposer for the next round based on voting power and priorities
    pub fn select_proposer(&mut self) -> usize {
        let total_power = self.total_voting_power();
        let avg_priority = self.average_priority();

        // Scale priorities if the difference is too large
        let max_priority = self.priorities.values().max().unwrap_or(&0);
        let min_priority = self.priorities.values().min().unwrap_or(&0);
        let diff = max_priority - min_priority;
        let threshold = 2 * total_power as i64;

        if diff > threshold {
            let scale = diff as f64 / threshold as f64;
            for priority in self.priorities.values_mut() {
                *priority = (*priority as f64 / scale) as i64;
            }
        }

        // Center priorities around zero
        let avg_priority = self.average_priority();
        for priority in self.priorities.values_mut() {
            *priority = (*priority as f64 - avg_priority) as i64;
        }

        // Add voting power to priorities
        for validator in &self.validators {
            let priority = self.priorities.get_mut(&validator.id).unwrap();
            *priority += validator.voting_power as i64;
        }

        // Select proposer with highest priority
        let proposer = self
            .priorities
            .iter()
            .max_by_key(|&(_, priority)| priority)
            .map(|(id, _)| *id)
            .unwrap_or(0);

        // Update priority of selected proposer
        if let Some(priority) = self.priorities.get_mut(&proposer) {
            *priority -= total_power as i64;
        }

        proposer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validator_set_creation() {
        let validators = vec![
            Validator {
                id: 1,
                voting_power: 10,
            },
            Validator {
                id: 2,
                voting_power: 20,
            },
        ];
        let vset = ValidatorSet::new(validators);
        assert_eq!(vset.total_voting_power(), 30);
        assert_eq!(vset.priorities.len(), 2);
    }

    #[test]
    fn test_proposer_selection() {
        let validators = vec![
            Validator {
                id: 1,
                voting_power: 10,
            },
            Validator {
                id: 2,
                voting_power: 20,
            },
            Validator {
                id: 3,
                voting_power: 15,
            },
        ];
        let mut vset = ValidatorSet::new(validators);

        // First round - should select validator with highest voting power
        let proposer1 = vset.select_proposer();
        assert_eq!(proposer1, 2); // Validator 2 has highest voting power

        // Second round - should consider updated priorities
        let proposer2 = vset.select_proposer();
        assert_ne!(proposer2, 2); // Should not select same validator twice in a row
    }

    #[test]
    fn test_priority_scaling() {
        let validators = vec![
            Validator {
                id: 1,
                voting_power: 10,
            },
            Validator {
                id: 2,
                voting_power: 10,
            },
        ];
        let mut vset = ValidatorSet::new(validators);

        // Set very high priority for one validator
        *vset.priorities.get_mut(&1).unwrap() = 1000;

        // Should scale down priorities
        let proposer = vset.select_proposer();
        assert!(vset.priorities.values().max().unwrap() < &1000);
    }

    #[test]
    fn test_priority_centering() {
        let validators = vec![
            Validator {
                id: 1,
                voting_power: 10,
            },
            Validator {
                id: 2,
                voting_power: 10,
            },
        ];
        let mut vset = ValidatorSet::new(validators);

        // Set different priorities
        *vset.priorities.get_mut(&1).unwrap() = 100;
        *vset.priorities.get_mut(&2).unwrap() = 50;

        // Should center priorities around zero
        let proposer = vset.select_proposer();
        let avg = vset.average_priority();
        assert!(avg.abs() < 1.0); // Average should be close to zero
    }
} 