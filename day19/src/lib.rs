extern crate filelib;

pub use filelib::load;
pub use filelib::split_lines_by_blanks;

type WorkflowValue = u64;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Category {
    ExtremelyCoolLooking,
    Musical,
    Aerodynamic,
    Shiny,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Part {
    x: WorkflowValue,
    m: WorkflowValue,
    a: WorkflowValue,
    s: WorkflowValue,
}

impl Part {
    fn get_value_by_category(&self, category: Category) -> WorkflowValue {
        return match category {
            Category::ExtremelyCoolLooking => self.x,
            Category::Musical => self.m,
            Category::Aerodynamic => self.a,
            Category::Shiny => self.s,
        };
    }

    fn get_value_sum(&self) -> WorkflowValue {
        return self.x + self.m + self.a + self.s;
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Comparison {
    LessThan,
    GreaterThan,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Flow {
    Accept,
    Reject,
    SendTo(String),
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Operation {
    category: Option<Category>,
    compare: Option<Comparison>,
    value: Option<WorkflowValue>,
    flow: Flow,
}

impl Operation {
    fn should_do_flow(&self, part: &Part) -> bool {
        if let Some(cat) = self.category {
            if let Some(comp) = self.compare {
                if let Some(value) = self.value {
                    let v = part.get_value_by_category(cat);
                    return match comp {
                        Comparison::LessThan => v < value,
                        Comparison::GreaterThan => v > value,
                    };
                }
            }
        }
        return true;
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Workflow {
    name: String,
    rules: Vec<Operation>,
}

impl Workflow {
    fn check_part(&self, part: &Part) -> Flow {
        for r in self.rules.iter() {
            if r.should_do_flow(part) {
                return r.flow.clone();
            }
        }
        panic!("Workflow with no default flow");
    }
}

fn parse_input(input: &Vec<Vec<String>>) -> (Vec<Part>, Vec<Workflow>) {
    let workflows_list = &input[0];
    let part_list = &input[1];

    let mut parts = vec![];
    let mut workflows = vec![];

    for line in part_list {
        let (x_equal, rest) = line
            .split_once("{")
            .unwrap()
            .1
            .split_once("}")
            .unwrap()
            .0
            .split_once(",")
            .unwrap();
        let (m_equal, rest) = rest.split_once(",").unwrap();
        let (a_equal, s_equal) = rest.split_once(",").unwrap();
        let (_, x_vs) = x_equal.split_once("=").unwrap();
        let (_, m_vs) = m_equal.split_once("=").unwrap();
        let (_, a_vs) = a_equal.split_once("=").unwrap();
        let (_, s_vs) = s_equal.split_once("=").unwrap();

        let x: WorkflowValue = x_vs.parse().unwrap();
        let m: WorkflowValue = m_vs.parse().unwrap();
        let a: WorkflowValue = a_vs.parse().unwrap();
        let s: WorkflowValue = s_vs.parse().unwrap();
        let part = Part {
            x: x,
            m: m,
            a: a,
            s: s,
        };
        parts.push(part);
    }

    for line in workflows_list {
        let (name, rest) = line.split_once("{").unwrap();
        let (no_end, _) = rest.split_once("}").unwrap();
        let mut rules = vec![];
        for rule_str in no_end.split(",") {
            if rule_str == "R" {
                rules.push(Operation {
                    category: None,
                    compare: None,
                    value: None,
                    flow: Flow::Reject,
                });
                continue;
            }
            if rule_str == "A" {
                rules.push(Operation {
                    category: None,
                    compare: None,
                    value: None,
                    flow: Flow::Accept,
                });
                continue;
            }
            if let Some((comp_s, result)) = rule_str.split_once(":") {
                let flow;
                if result == "R" {
                    flow = Flow::Reject
                } else if result == "A" {
                    flow = Flow::Accept
                } else {
                    flow = Flow::SendTo(result.to_string());
                }

                let comp_v = comp_s.chars().nth(0).unwrap();
                let comp_op = comp_s.chars().nth(1).unwrap();
                let comp_value: WorkflowValue = comp_s[2..].parse().unwrap();
                let op = match comp_op {
                    '<' => Comparison::LessThan,
                    '>' => Comparison::GreaterThan,
                    _ => panic!("Unknown operator {}", comp_op),
                };
                let variable = match comp_v {
                    'x' => Category::ExtremelyCoolLooking,
                    'm' => Category::Musical,
                    'a' => Category::Aerodynamic,
                    's' => Category::Shiny,
                    _ => panic!("Unknown variable {}", comp_v),
                };
                rules.push(Operation {
                    category: Some(variable),
                    compare: Some(op),
                    value: Some(comp_value),
                    flow: flow,
                });
            } else {
                let flow = Flow::SendTo(rule_str.to_string());
                rules.push(Operation {
                    category: None,
                    compare: None,
                    value: None,
                    flow: flow,
                });
            }
        }

        let workflow = Workflow {
            name: name.to_string(),
            rules: rules,
        };
        workflows.push(workflow);
    }

    return (parts, workflows);
}

fn eval_part(part: &Part, initial_name: &str, workflows: &Vec<Workflow>) -> bool {
    let first: Vec<&Workflow> = workflows
        .iter()
        .filter(|f| f.name == initial_name)
        .collect();
    let flow = first[0].check_part(part);
    return match flow {
        Flow::Accept => true,
        Flow::Reject => false,
        Flow::SendTo(new_name) => eval_part(part, &new_name, workflows),
    };
}

/// Add ratings of all accepted parts.
/// ```
/// let vec1: Vec<Vec<String>> = vec![
///    vec!["px{a<2006:qkq,m>2090:A,rfg}",
///    "pv{a>1716:R,A}",
///    "lnx{m>1548:A,A}",
///    "rfg{s<537:gd,x>2440:R,A}",
///    "qs{s>3448:A,lnx}",
///    "qkq{x<1416:A,crn}",
///    "crn{x>2662:A,R}",
///    "in{s<1351:px,qqz}",
///    "qqz{s>2770:qs,m<1801:hdj,R}",
///    "gd{a>3333:R,R}",
///    "hdj{m>838:A,pv}"].iter().map(|s| s.to_string()).collect(),
///    vec!["{x=787,m=2655,a=1222,s=2876}",
///    "{x=1679,m=44,a=2067,s=496}",
///    "{x=2036,m=264,a=79,s=2244}",
///    "{x=2461,m=1339,a=466,s=291}",
///    "{x=2127,m=1623,a=2188,s=1013}"].iter().map(|s| s.to_string()).collect()
/// ];
/// assert_eq!(day19::puzzle_a(&vec1), 19114);
/// ```
pub fn puzzle_a(input: &Vec<Vec<String>>) -> WorkflowValue {
    let first_endpoint_name = "in";
    let (parts, workflows) = parse_input(input);
    return parts
        .iter()
        .filter(|f| eval_part(f, first_endpoint_name, &workflows))
        .map(|p| p.get_value_sum())
        .sum();
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct PartRange {
    x_mins: WorkflowValue,
    x_maxs: WorkflowValue,
    m_mins: WorkflowValue,
    m_maxs: WorkflowValue,
    a_mins: WorkflowValue,
    a_maxs: WorkflowValue,
    s_mins: WorkflowValue,
    s_maxs: WorkflowValue,
}

fn recurse_derive_num_acceptable_parts(
    flows: &Vec<Workflow>,
    first_endpoint_name: &str,
    in_range: PartRange,
) -> Vec<PartRange> {
    // Split mins / maxs by Flow
    // Reject: Eliminate, don't add
    // Accept: Take out, and add on to the recurse value at the end.
    // SendTo: Recurse down the flow for each distinct send to.
    let first: Vec<&Workflow> = flows
        .iter()
        .filter(|f| f.name == first_endpoint_name)
        .collect();
    let workflow = first[0];

    let mut accept_ranges: Vec<PartRange> = vec![];
    let mut range = in_range.clone();

    for rule in workflow.rules.clone() {
        // Check if range is impossible now, if it is, no point in continuing
        if range.x_mins > range.x_maxs
            || range.m_mins > range.m_maxs
            || range.a_mins > range.a_maxs
            || range.s_mins > range.s_maxs
        {
            break;
        }

        if let Some(cat) = rule.category {
            if let Some(comp) = rule.compare {
                if let Some(value) = rule.value {
                    match rule.flow {
                        Flow::Accept => {
                            let mut to_accept = range.clone();
                            match comp {
                                Comparison::LessThan => {
                                    // Add an accept that looks like range.
                                    // Figure out value to alter by category
                                    // we want to decrease max to be value - 1.
                                    // and we want to change range to min value + 1
                                    match cat {
                                        Category::ExtremelyCoolLooking => {
                                            to_accept.x_maxs = value - 1;
                                            range.x_mins = value;
                                        }
                                        Category::Musical => {
                                            to_accept.m_maxs = value - 1;
                                            range.m_mins = value;
                                        }
                                        Category::Aerodynamic => {
                                            to_accept.a_maxs = value - 1;
                                            range.a_mins = value;
                                        }
                                        Category::Shiny => {
                                            to_accept.s_maxs = value - 1;
                                            range.s_mins = value;
                                        }
                                    }
                                }
                                Comparison::GreaterThan => {
                                    // Add an accept that looks like range.
                                    // Figure out value to alter by category
                                    // we want to increase min to be value + 1.
                                    // and we want to change range to max value - 1
                                    match cat {
                                        Category::ExtremelyCoolLooking => {
                                            range.x_maxs = value;
                                            to_accept.x_mins = value + 1;
                                        }
                                        Category::Musical => {
                                            range.m_maxs = value;
                                            to_accept.m_mins = value + 1;
                                        }
                                        Category::Aerodynamic => {
                                            range.a_maxs = value;
                                            to_accept.a_mins = value + 1;
                                        }
                                        Category::Shiny => {
                                            range.s_maxs = value;
                                            to_accept.s_mins = value + 1;
                                        }
                                    }
                                }
                            }
                            accept_ranges.push(to_accept);
                        }
                        Flow::Reject => {
                            match comp {
                                Comparison::LessThan => {
                                    // Alter self, make sure we aren't less than the value
                                    // Set min to Value
                                    match cat {
                                        Category::ExtremelyCoolLooking => {
                                            range.x_mins = value;
                                        }
                                        Category::Musical => {
                                            range.m_mins = value;
                                        }
                                        Category::Aerodynamic => {
                                            range.a_mins = value;
                                        }
                                        Category::Shiny => {
                                            range.s_mins = value;
                                        }
                                    }
                                }
                                Comparison::GreaterThan => {
                                    // Alter self, make sure we aren't more than the value
                                    // Set max to Value
                                    match cat {
                                        Category::ExtremelyCoolLooking => {
                                            range.x_maxs = value;
                                        }
                                        Category::Musical => {
                                            range.m_maxs = value;
                                        }
                                        Category::Aerodynamic => {
                                            range.a_maxs = value;
                                        }
                                        Category::Shiny => {
                                            range.s_maxs = value;
                                        }
                                    }
                                }
                            }
                        }
                        Flow::SendTo(c) => {
                            let mut copy = range.clone();
                            match comp {
                                Comparison::LessThan => {
                                    // This copy should get max = value - 1
                                    // While range should get min = value
                                    match cat {
                                        Category::ExtremelyCoolLooking => {
                                            copy.x_maxs = value - 1;
                                            range.x_mins = value;
                                        }
                                        Category::Musical => {
                                            copy.m_maxs = value - 1;
                                            range.m_mins = value;
                                        }
                                        Category::Aerodynamic => {
                                            copy.a_maxs = value - 1;
                                            range.a_mins = value;
                                        }
                                        Category::Shiny => {
                                            copy.s_maxs = value - 1;
                                            range.s_mins = value;
                                        }
                                    }
                                }
                                Comparison::GreaterThan => {
                                    // This copy should get min = value + 1
                                    // While range should get max = value
                                    match cat {
                                        Category::ExtremelyCoolLooking => {
                                            range.x_maxs = value;
                                            copy.x_mins = value + 1;
                                        }
                                        Category::Musical => {
                                            range.m_maxs = value;
                                            copy.m_mins = value + 1;
                                        }
                                        Category::Aerodynamic => {
                                            range.a_maxs = value;
                                            copy.a_mins = value + 1;
                                        }
                                        Category::Shiny => {
                                            range.s_maxs = value;
                                            copy.s_mins = value + 1;
                                        }
                                    }
                                }
                            }
                            let recurse = recurse_derive_num_acceptable_parts(flows, &c, copy);
                            accept_ranges.extend(recurse);
                        }
                    }
                }
            }
        } else {
            match rule.flow {
                Flow::Accept => {
                    accept_ranges.push(range);
                }
                Flow::Reject => {
                    // Throw away this entire range
                }
                Flow::SendTo(c) => {
                    let recurse = recurse_derive_num_acceptable_parts(flows, &c, range);
                    accept_ranges.extend(recurse);
                }
            }
            break;
        }
    }

    return accept_ranges;
}

fn derive_num_acceptable_parts(flows: &Vec<Workflow>, first_endpoint_name: &str) -> WorkflowValue {
    let x_mins = 1;
    let x_maxs = 4000;
    let m_mins = 1;
    let m_maxs = 4000;
    let a_mins = 1;
    let a_maxs = 4000;
    let s_mins = 1;
    let s_maxs = 4000;

    let part_ranges = PartRange {
        x_mins: x_mins,
        x_maxs: x_maxs,
        m_mins: m_mins,
        m_maxs: m_maxs,
        a_mins: a_mins,
        a_maxs: a_maxs,
        s_mins: s_mins,
        s_maxs: s_maxs,
    };
    let ranges = recurse_derive_num_acceptable_parts(flows, first_endpoint_name, part_ranges);
    return ranges
        .iter()
        .map(|s| {
            (s.x_maxs - s.x_mins + 1)
                * (s.m_maxs - s.m_mins + 1)
                * (s.a_maxs - s.a_mins + 1)
                * (s.s_maxs - s.s_mins + 1)
        })
        .sum();
}

/// Ignore parts, figure out how many combinations would be accepted.
/// ```
/// let vec1: Vec<Vec<String>> = vec![
///    vec!["px{a<2006:qkq,m>2090:A,rfg}",
///    "pv{a>1716:R,A}",
///    "lnx{m>1548:A,A}",
///    "rfg{s<537:gd,x>2440:R,A}",
///    "qs{s>3448:A,lnx}",
///    "qkq{x<1416:A,crn}",
///    "crn{x>2662:A,R}",
///    "in{s<1351:px,qqz}",
///    "qqz{s>2770:qs,m<1801:hdj,R}",
///    "gd{a>3333:R,R}",
///    "hdj{m>838:A,pv}"].iter().map(|s| s.to_string()).collect(),
///    vec!["{x=787,m=2655,a=1222,s=2876}",
///    "{x=1679,m=44,a=2067,s=496}",
///    "{x=2036,m=264,a=79,s=2244}",
///    "{x=2461,m=1339,a=466,s=291}",
///    "{x=2127,m=1623,a=2188,s=1013}"].iter().map(|s| s.to_string()).collect()
/// ];
/// assert_eq!(day19::puzzle_b(&vec1), 167409079868000);
/// ```
pub fn puzzle_b(input: &Vec<Vec<String>>) -> WorkflowValue {
    let first_endpoint_name = "in";
    let (_, workflows) = parse_input(input);
    return derive_num_acceptable_parts(&workflows, first_endpoint_name);
}
