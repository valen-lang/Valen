use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::marker::PhantomData;

// mig: struct SimpleSolverState
pub struct SimpleSolverState<Rule, Rune, Conclusion>
where
    Rune: Eq + Hash,
{
    rule_to_puzzles: Box<dyn Fn(&Rule) -> Vec<Vec<Rune>>>,
    steps: Vec<super::Step<Rule, Rune, Conclusion>>,
    rules: Vec<Rule>,
    all_runes: HashSet<Rune>,
    open_rule_to_puzzle_to_runes: HashMap<i32, Vec<Vec<Rune>>>,
    rune_to_conclusion: HashMap<Rune, Conclusion>,
}

// mig: impl SimpleSolverState
impl<Rule, Rune, Conclusion> SimpleSolverState<Rule, Rune, Conclusion>
where
    Rule: Clone,
    Rune: Clone + Hash + Eq,
    Conclusion: Clone + PartialEq,
{

// mig: fn sanity_check
    pub fn sanity_check(&self) {
        // vassert(rules == rules.distinct)
    }

// mig: fn get_rule
    pub fn get_rule(&self, rule_index: i32) -> &Rule {
        &self.rules[rule_index as usize]
    }

// mig: fn get_conclusion
    pub fn get_conclusion(&self, rune: &Rune) -> Option<Conclusion> {
        self.rune_to_conclusion.get(rune).cloned()
    }

// mig: fn get_conclusions
    pub fn get_conclusions(&self) -> Vec<(Rune, Conclusion)> {
        self.rune_to_conclusion.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }

// mig: fn userify_conclusions
    pub fn userify_conclusions(&self) -> Vec<(Rune, Conclusion)> {
        self.rune_to_conclusion.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }

    // mig: fn get_all_runes (matches Scala's getAllRunes() -> Set[Rune])
    pub fn get_all_runes(&self) -> HashSet<Rune> {
        self.all_runes.clone()
    }

    // mig: fn is_complete
    pub fn is_complete(&self) -> bool {
        self.rune_to_conclusion.len() == self.all_runes.len()
    }

    // mig: fn commit_step (matches Scala's commitStep)
    pub fn commit_step<ErrType>(
        &mut self,
        complex: bool,
        solved_rule_indices: Vec<i32>,
        conclusions: HashMap<Rune, Conclusion>,
        new_rules: Vec<Rule>,
        new_runes: HashSet<Rune>,
    ) -> Result<(), super::ISolverError<Rune, Conclusion, ErrType>> {
        self.all_runes.extend(new_runes);
        let solved_rules: Vec<(i32, Rule)> = solved_rule_indices
            .iter()
            .map(|&idx| (idx, self.rules[idx as usize].clone()))
            .collect();
        let step = super::Step {
            complex,
            solved_rules,
            added_rules: new_rules.clone(),
            conclusions: conclusions.clone(),
        };
        // Append step before checking for conflicts (audit trail captures conflicting step)
        self.steps.push(step);
        // Check and apply conclusions
        for (rune, new_conclusion) in &conclusions {
            if let Some(existing) = self.rune_to_conclusion.get(rune) {
                if existing != new_conclusion {
                    return Err(super::ISolverError::SolverConflict(
                        super::SolverConflict {
                            rune: rune.clone(),
                            previous_conclusion: existing.clone(),
                            new_conclusion: new_conclusion.clone(),
                            _phantom: PhantomData,
                        },
                    ));
                }
            }
            self.rune_to_conclusion.insert(rune.clone(), new_conclusion.clone());
        }
        // Remove solved rules
        for &rule_index in &solved_rule_indices {
            self.open_rule_to_puzzle_to_runes.remove(&rule_index);
        }
        // Add new rules with puzzles
        for rule in new_rules {
            let rule_index = self.rules.len() as i32;
            self.rules.push(rule.clone());
            let puzzles = (self.rule_to_puzzles)(&rule);
            for puzzle in &puzzles {
                for rune in puzzle {
                    assert!(self.all_runes.contains(rune), "vassert: rune in puzzle must be in allRunes");
                }
            }
            self.sanity_check();
            for puzzle in puzzles {
                let entry = self.open_rule_to_puzzle_to_runes
                    .entry(rule_index)
                    .or_insert_with(Vec::new);
                entry.push(puzzle);
            }
            self.sanity_check();
        }
        Ok(())
    }

// mig: fn get_next_solvable
    pub fn get_next_solvable(&self) -> Option<i32> {
        // Get rule with lowest ID, keep it deterministic (matches Scala)
        self.open_rule_to_puzzle_to_runes
            .iter()
            .filter(|(_, puzzle_to_runes)| {
                puzzle_to_runes.iter().any(|runes| {
                    runes
                        .iter()
                        .all(|r| self.rune_to_conclusion.contains_key(r))
                })
            })
            .map(|(rule_index, _)| *rule_index)
            .min()
    }

// mig: fn get_unsolved_rules
    pub fn get_unsolved_rules(&self) -> Vec<Rule> {
        self.open_rule_to_puzzle_to_runes
            .keys()
            .map(|&idx| self.rules[idx as usize].clone())
            .collect()
    }

    // mig: fn get_unsolved_runes (matches Scala's getUnsolvedRunes)
    pub fn get_unsolved_runes(&self) -> Vec<Rune> {
        self.all_runes.difference(
            &self.rune_to_conclusion.keys().cloned().collect()
        ).cloned().collect()
    }

    // mig: fn get_steps
    pub fn get_steps(&self) -> Vec<super::Step<Rule, Rune, Conclusion>> {
        self.steps.clone()
    }

    // mig: fn rule_is_solved (matches Scala's ruleIsSolved)
    pub fn rule_is_solved(&self, rule_index: i32) -> bool {
        !self.open_rule_to_puzzle_to_runes.contains_key(&rule_index)
    }

pub fn new(
    rule_to_puzzles: Box<dyn Fn(&Rule) -> Vec<Vec<Rune>>>,
    all_runes: Vec<Rune>,
) -> Self {
    SimpleSolverState {
        rule_to_puzzles,
        steps: vec![],
        rules: vec![],
        all_runes: all_runes.into_iter().collect(),
        open_rule_to_puzzle_to_runes: HashMap::new(),
        rune_to_conclusion: HashMap::new(),
    }
}

}
