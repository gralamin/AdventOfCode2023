extern crate filelib;

pub use filelib::load_no_blanks;
pub use std::collections::HashMap;
pub use std::collections::VecDeque;

// pulses, high or lose

// flip flop %, start off, can go on
// Low turns it on / off, off -> on, sends off a high pulse
// on -> off -> low pulse. High coming in does nothing.

// Conjuctions & remember the last pulse, start as low on high and low.
// when it gets a pulse, updates the pulse to the value. if both are high, send a high.
// else send a low.

// broadcast (named broadcaster), when it recieves a pulse, it sends the same pulse to a bunch
// Low pulses come into broadcaster when you want.

// pulses handled in a queue.

type Pulse = bool;
type PulseCounter = HashMap<Pulse, u64>;
type PulseMemory = HashMap<usize, Pulse>;
const LOW_PULSE: Pulse = false;
const HIGH_PULSE: Pulse = true;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Operation {
    Broadcast,
    Conjuction,
    FlipFlop,
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Module {
    memory: PulseMemory,
    state: bool,
    operation: Operation,
    name: String,
    targets: Vec<String>,
    id_num: usize,
}

impl Module {
    pub fn new(name: String, operation: Operation, targets: Vec<String>, id_num: usize) -> Module {
        let memory = PulseMemory::new();
        let state = LOW_PULSE;
        return Module {
            memory: memory,
            state: state,
            operation: operation,
            name: name,
            targets: targets,
            id_num: id_num,
        };
    }

    fn add_input(&mut self, input_num: usize) {
        self.memory.insert(input_num, LOW_PULSE);
    }

    fn process(&mut self, input: Pulse, from_id: usize) -> Option<Pulse> {
        return match self.operation {
            Operation::Broadcast => Some(input),
            Operation::Conjuction => {
                self.memory.get_mut(&from_id).map(|val| {
                    *val = input;
                });
                if self.memory.values().all(|a| *a == HIGH_PULSE) {
                    return Some(LOW_PULSE);
                }
                return Some(HIGH_PULSE);
            }
            Operation::FlipFlop => {
                if input == HIGH_PULSE {
                    return None;
                }
                let old_state = self.state;
                self.state = !old_state;
                if old_state {
                    return Some(LOW_PULSE);
                }
                return Some(HIGH_PULSE);
            }
        };
    }
}

fn parse_modules(string_list: &Vec<String>) -> Vec<Module> {
    let mut result = vec![];

    for l in string_list {
        let id_num = result.len();
        let (module_descriptor, target_s) = l.split_once(" -> ").unwrap();
        let operator: Operation;
        let mut targets = vec![];
        let name: String;
        if module_descriptor == "broadcaster" {
            operator = Operation::Broadcast;
            name = module_descriptor.to_string();
            for value in target_s.split(", ") {
                targets.push(value.to_string());
            }
        } else if module_descriptor.starts_with("%") {
            operator = Operation::FlipFlop;
            name = module_descriptor[1..].to_string();
            for value in target_s.split(", ") {
                targets.push(value.to_string());
            }
        } else if module_descriptor.starts_with("&") {
            operator = Operation::Conjuction;
            name = module_descriptor[1..].to_string();
            for value in target_s.split(", ") {
                targets.push(value.to_string());
            }
        } else {
            panic!("Cannot parse");
        }
        //println!("Adding {}", name);
        result.push(Module::new(name, operator, targets, id_num))
    }
    return result;
}

fn scan_add_inputs(modules: &mut Vec<Module>) {
    for m in modules.clone().iter() {
        for t in m.targets.iter() {
            for z in modules.iter_mut() {
                if z.name == t.clone() {
                    z.add_input(m.id_num);
                    break;
                }
            }
        }
    }
}

fn process_inputs(
    input: Pulse,
    modules: &mut Vec<Module>,
    module_name: &String,
    last_id: Option<usize>,
) -> (Vec<String>, Option<Pulse>, usize) {
    //println!("Trying to find {}", module_name);
    let found = modules
        .iter_mut()
        .filter(|m| m.name == module_name.clone())
        .nth(0);
    if let Some(cur_module) = found {
        let result = cur_module.process(input, last_id.unwrap_or(0));

        return (cur_module.targets.clone(), result, cur_module.id_num);
    }
    return (vec![], None, 0);
}

fn process_button_presess(num_preses: usize, modules: &mut Vec<Module>) -> PulseCounter {
    let mut pulse_count = PulseCounter::new();
    pulse_count.insert(LOW_PULSE, 0);
    pulse_count.insert(HIGH_PULSE, 0);
    let start = "broadcaster".to_string();
    let mut process_queue: VecDeque<(String, Pulse, Option<usize>)> = VecDeque::new();
    for _ in 0..num_preses {
        process_queue.push_back((start.clone(), LOW_PULSE, None));
        while let Some((module, pulse, last_id)) = process_queue.pop_front() {
            *pulse_count.entry(pulse).or_default() += 1;
            let (targets, signal, process_id) = process_inputs(pulse, modules, &module, last_id);
            if let Some(s) = signal {
                for t in targets {
                    process_queue.push_back((t, s, Some(process_id)));
                }
            }
        }
    }
    return pulse_count;
}

/// Count pulses sent in total, when pressing button 1000 times.
/// Multiply high and low together.
/// ```
/// let vec1: Vec<String> = vec![
///     "broadcaster -> a, b, c",
///     "%a -> b",
///     "%b -> c",
///     "%c -> inv",
///     "&inv -> a",
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day20::puzzle_a(&vec1), 32000000);
/// ```
pub fn puzzle_a(string_list: &Vec<String>) -> u64 {
    let mut modules = parse_modules(string_list);
    scan_add_inputs(&mut modules);
    let counter = process_button_presess(1000, &mut modules);
    return counter.values().product();
}

/// Foo
/// ```
/// let vec1: Vec<String> = vec![
///     "foo"
/// ].iter().map(|s| s.to_string()).collect();
/// assert_eq!(day20::puzzle_b(&vec1), 0);
/// ```
pub fn puzzle_b(string_list: &Vec<String>) -> u64 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_puzzle_a() {
        let vec1: Vec<String> = vec![
            "broadcaster -> a",
            "%a -> inv, con",
            "&inv -> b",
            "%b -> con",
            "&con -> output",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();
        assert_eq!(puzzle_a(&vec1), 11687500);
    }
}
