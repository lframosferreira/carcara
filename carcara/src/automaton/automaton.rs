use pest::Parser;
use pest_derive::Parser;

pub struct Automaton {
    states: Vec<usize>,
    // naive implementation
    graph: Vec<Vec<(usize, usize)>>,
}

impl Automaton {
    pub fn new() -> Self {
        Automaton {
            states: Vec::new(),
            graph: Vec::new(),
        }
    }

    fn number_of_states(&self) -> usize {
        self.states.len()
    }

    fn trim(&self) -> () {
        unimplemented!()
    }

    fn is_lang_empty(&self) -> bool {
        true
    }

    fn intersect_in_place(&mut self, other: &Automaton) -> () {
        unimplemented!()
    }
}

pub fn parse_from_goal_format(description: &str) -> Automaton {
    let nfa: Automaton = Automaton::new();

    nfa
}
