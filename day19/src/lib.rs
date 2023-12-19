extern crate filelib;

pub use filelib::load;
pub use filelib::split_lines_by_blanks;

type WorkflowValue = u32;

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

/// Foo
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
/// assert_eq!(day19::puzzle_b(&vec1), 0);
/// ```
pub fn puzzle_b(input: &Vec<Vec<String>>) -> WorkflowValue {
    return 0;
}
