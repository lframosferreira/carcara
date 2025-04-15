use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "automaton.pest"]
struct AutomatonParser;

#[derive(Debug)]
struct Transition {
    from: String,
    to: String,
    range: (u32, u32),
}

#[derive(Debug)]
struct AutomatonFragment {
    name: String,
    init_state: String,
    transitions: Vec<Transition>,
    accepting_states: Vec<String>,
}

fn parse_automaton(input: &str) -> Result<AutomatonFragment, String> {
    let pairs = AutomatonParser::parse(Rule::automaton, input)
        .map_err(|e| format!("Parse error: {}", e))?;

    let mut name = String::new();
    let mut init_state = String::new();
    let mut transitions = vec![];
    let mut accepting_states = vec![];

    for pair in pairs {
        match pair.as_rule() {
            Rule::automaton => {
                let mut inner = pair.into_inner();
                name = inner.next().unwrap().as_str().to_string();

                for item in inner {
                    match item.as_rule() {
                        Rule::init => {
                            init_state = item.into_inner().next().unwrap().as_str().to_string();
                        }
                        Rule::transition => {
                            let mut inner = item.into_inner();
                            let from = inner.next().unwrap().as_str().to_string();
                            let to = inner.next().unwrap().as_str().to_string();
                            let mut range = inner.next().unwrap().into_inner();
                            let start = range.next().unwrap().as_str().parse::<u32>().unwrap();
                            let end = range.next().unwrap().as_str().parse::<u32>().unwrap();
                            transitions.push(Transition { from, to, range: (start, end) });
                        }
                        Rule::accepting => {
                            accepting_states =
                                item.into_inner().map(|p| p.as_str().to_string()).collect();
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

    Ok(AutomatonFragment {
        name,
        init_state,
        transitions,
        accepting_states,
    })
}
