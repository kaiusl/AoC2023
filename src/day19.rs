use std::collections::HashMap;

use std::ops::Range;

const DAY: u8 = 19;
pub const INPUT: &str = include_str!("../inputs/day19.txt");

pub fn run() {
    solve(INPUT);
}

fn solve(input: &str) {
    let answer_part1 = solve_part1(input);
    println!("day{DAY}::part1 answer: {}", answer_part1);

    let answer_part2 = solve_part2(input);
    println!("day{DAY}::part2 answer: {}", answer_part2);
}

fn solve_part1(input: &str) -> u64 {
    let (workflows, parts) = parse(input);

    parts
        .filter(|p| is_part_accepted(&workflows, p))
        .map(|p| p.sum_categories())
        .sum()
}

fn is_part_accepted(workflows: &HashMap<&str, Workflow>, part: &Part) -> bool {
    let mut workflow = String::from("in");

    loop {
        let result = workflows[workflow.as_str()].eval(part);
        match result {
            RuleResult::Accept => return true,
            RuleResult::Reject => return false,
            RuleResult::NextWorkflow(name) => workflow = name,
            RuleResult::NextRule => unreachable!(),
        }
    }
}

fn solve_part2(input: &str) -> u64 {
    let (workflows, _) = parse2(input);
    let part = RangePart::new(1..4001);
    part2_core(&workflows, "in", part)
}

fn part2_core(workflows: &HashMap<&str, Vec<RuleDef>>, workflow: &str, mut part: RangePart) -> u64 {
    let mut out = 0;
    let rules = &workflows[workflow];

    for rule in rules {
        let result = split_part_by_rule(rule, part.clone());

        if let Some(accepted) = result.accepted {
            match accepted.1 {
                RuleResult::Accept => {
                    out += accepted.0.num_combinations();
                }
                RuleResult::Reject => {
                    // nothing to do
                }
                RuleResult::NextWorkflow(wf) => {
                    out += part2_core(workflows, wf.as_str(), accepted.0);
                }
                RuleResult::NextRule => {
                    // this is the rejected part
                    unreachable!()
                }
            }
        }

        if let Some(rejected) = result.rejected {
            part = rejected;
        } else {
            break;
        }
    }

    out
}

#[derive(Debug, Clone)]
struct RuleSplitResult {
    accepted: Option<(RangePart, RuleResult)>,
    rejected: Option<RangePart>,
}

fn split_part_by_rule(rule: &RuleDef, part: RangePart) -> RuleSplitResult {
    let Some(condition) = &rule.condition else {
        return RuleSplitResult {
            accepted: Some((part, rule.result.clone())),
            rejected: None,
        };
    };

    macro_rules! split_rule {
        (@createResult $cat:ident, $accepted:ident, $rejected:ident) => {
            RuleSplitResult {
                accepted: $accepted.map(|accepted| {
                    (
                        RangePart {
                            $cat: accepted,
                            ..part.clone()
                        },
                        rule.result.clone(),
                    )
                }),
                rejected: $rejected.map(|rejected| RangePart {
                    $cat: rejected,
                    ..part
                }),
            }
        };
        ($cat:ident, Op::Gt) => {{
            let (rejected, accepted) = split_range(&part.$cat, condition.value + 1);
            split_rule!(@createResult $cat, accepted, rejected)
        }};
        ($cat:ident, Op::Lt) => {{
            let (accepted, rejected) = split_range(&part.$cat, condition.value);
            split_rule!(@createResult $cat, accepted, rejected)
        }};
    }

    match (&condition.category, &condition.op) {
        (Category::X, Op::Gt) => split_rule!(x, Op::Gt),
        (Category::X, Op::Lt) => split_rule!(x, Op::Lt),
        (Category::M, Op::Gt) => split_rule!(m, Op::Gt),
        (Category::M, Op::Lt) => split_rule!(m, Op::Lt),
        (Category::A, Op::Gt) => split_rule!(a, Op::Gt),
        (Category::A, Op::Lt) => split_rule!(a, Op::Lt),
        (Category::S, Op::Gt) => split_rule!(s, Op::Gt),
        (Category::S, Op::Lt) => split_rule!(s, Op::Lt),
    }
}

fn split_range(range: &Range<u16>, index: u16) -> (Option<Range<u16>>, Option<Range<u16>>) {
    if index < range.start {
        (None, Some(range.clone()))
    } else if index >= range.end {
        (Some(range.clone()), None)
    } else {
        (Some(range.start..index), Some(index..range.end))
    }
}

struct Workflow {
    rules: Vec<Box<Rule>>,
}

impl Workflow {
    fn eval(&self, part: &Part) -> RuleResult {
        for rule in &self.rules {
            match rule(part) {
                RuleResult::NextRule => {}
                result => return result,
            }
        }

        unreachable!()
    }
}

type Rule = dyn Fn(&Part) -> RuleResult;

#[derive(Debug, Clone, PartialEq, Eq)]
enum RuleResult {
    Accept,
    Reject,
    NextWorkflow(String),
    NextRule,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Part {
    x: u16,
    m: u16,
    a: u16,
    s: u16,
}

impl Part {
    fn sum_categories(&self) -> u64 {
        self.x as u64 + self.m as u64 + self.a as u64 + self.s as u64
    }
}

#[derive(Debug, Clone)]
struct RangePart {
    x: Range<u16>,
    m: Range<u16>,
    a: Range<u16>,
    s: Range<u16>,
}

impl RangePart {
    fn new(range: Range<u16>) -> Self {
        Self {
            x: range.clone(),
            m: range.clone(),
            a: range.clone(),
            s: range.clone(),
        }
    }

    fn num_combinations(&self) -> u64 {
        fn range_len(range: &Range<u16>) -> u64 {
            (range.end - range.start) as u64
        }

        range_len(&self.x) * range_len(&self.m) * range_len(&self.a) * range_len(&self.s)
    }
}

fn parse(input: &str) -> (HashMap<&str, Workflow>, impl Iterator<Item = Part> + '_) {
    let mut workflows = HashMap::new();

    let (workflows_input, parts_input) = input.split_once("\n\n").unwrap();

    for line in workflows_input.lines() {
        let (name, workflow) = line.split_once('{').unwrap();
        let workflow = workflow.trim_end_matches('}');
        let rules = workflow.split(',').map(parse_rule).collect::<Vec<_>>();
        workflows.insert(name, Workflow { rules });
    }

    (workflows, parse_parts(parts_input))
}

fn parse2(input: &str) -> (HashMap<&str, Vec<RuleDef>>, impl Iterator<Item = Part> + '_) {
    let mut workflows = HashMap::new();

    let (workflows_input, parts_input) = input.split_once("\n\n").unwrap();

    for line in workflows_input.lines() {
        let (name, workflow) = line.split_once('{').unwrap();
        let workflow = workflow.trim_end_matches('}');
        let rules = workflow.split(',').map(parse_rule2).collect::<Vec<_>>();
        workflows.insert(name, rules);
    }

    (workflows, parse_parts(parts_input))
}
fn parse_parts(input: &str) -> impl Iterator<Item = Part> + '_ {
    input.lines().map(|line| {
        let line = line.trim_start_matches('{').trim_end_matches('}');

        let mut parts_line = line.split(',');
        let x = parts_line
            .next()
            .unwrap()
            .trim_start_matches("x=")
            .parse()
            .unwrap();
        let m = parts_line
            .next()
            .unwrap()
            .trim_start_matches("m=")
            .parse()
            .unwrap();
        let a = parts_line
            .next()
            .unwrap()
            .trim_start_matches("a=")
            .parse()
            .unwrap();
        let s = parts_line
            .next()
            .unwrap()
            .trim_start_matches("s=")
            .parse()
            .unwrap();

        Part { x, m, a, s }
    })
}

fn parse_rule(input: &str) -> Box<Rule> {
    if let Some((condition, result)) = input.split_once(':') {
        let category = &condition[..1];
        let op = &condition[1..2];
        let value = condition[2..].parse::<u16>().unwrap();

        let result = match result {
            "A" => RuleResult::Accept,
            "R" => RuleResult::Reject,
            s => RuleResult::NextWorkflow(s.into()),
        };

        macro_rules! rule {
            ($cat:ident, $op:tt) => {
                Box::new(move |p: &Part| -> RuleResult {
                    if p.$cat $op value {
                        result.clone()
                    } else {
                        RuleResult::NextRule
                    }
                })
            };
        }

        match (category, op) {
            ("x", ">") => rule!(x, >),
            ("x", "<") => rule!(x, <),
            ("m", ">") => rule!(m, >),
            ("m", "<") => rule!(m, <),
            ("a", ">") => rule!(a, >),
            ("a", "<") => rule!(a, <),
            ("s", ">") => rule!(s, >),
            ("s", "<") => rule!(s, <),
            _ => todo!(),
        }
    } else {
        let result = match input {
            "A" => RuleResult::Accept,
            "R" => RuleResult::Reject,
            s => RuleResult::NextWorkflow(s.into()),
        };

        Box::new(move |_: &Part| -> RuleResult { result.clone() })
    }
}

fn parse_rule2(input: &str) -> RuleDef {
    if let Some((condition, result)) = input.split_once(':') {
        let category = &condition[..1];
        let op = &condition[1..2];
        let value = condition[2..].parse::<u16>().unwrap();

        let result = match result {
            "A" => RuleResult::Accept,
            "R" => RuleResult::Reject,
            s => RuleResult::NextWorkflow(s.into()),
        };

        RuleDef {
            condition: Some(Condition {
                category: Category::from_str(category),
                op: Op::from_str(op),
                value,
            }),
            result,
        }
    } else {
        let result = match input {
            "A" => RuleResult::Accept,
            "R" => RuleResult::Reject,
            s => RuleResult::NextWorkflow(s.into()),
        };

        RuleDef {
            condition: None,
            result,
        }
    }
}

struct RuleDef {
    condition: Option<Condition>,
    result: RuleResult,
}

struct Condition {
    category: Category,
    op: Op,
    value: u16,
}

enum Category {
    X,
    M,
    A,
    S,
}

impl Category {
    fn from_str(s: &str) -> Self {
        match s {
            "x" => Category::X,
            "m" => Category::M,
            "a" => Category::A,
            "s" => Category::S,
            _ => unreachable!(),
        }
    }
}

enum Op {
    Gt,
    Lt,
}

impl Op {
    fn from_str(s: &str) -> Self {
        match s {
            ">" => Op::Gt,
            "<" => Op::Lt,
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = indoc::indoc! {"
    px{a<2006:qkq,m>2090:A,rfg}
    pv{a>1716:R,A}
    lnx{m>1548:A,A}
    rfg{s<537:gd,x>2440:R,A}
    qs{s>3448:A,lnx}
    qkq{x<1416:A,crn}
    crn{x>2662:A,R}
    in{s<1351:px,qqz}
    qqz{s>2770:qs,m<1801:hdj,R}
    gd{a>3333:R,R}
    hdj{m>838:A,pv}
    
    {x=787,m=2655,a=1222,s=2876}
    {x=1679,m=44,a=2067,s=496}
    {x=2036,m=264,a=79,s=2244}
    {x=2461,m=1339,a=466,s=291}
    {x=2127,m=1623,a=2188,s=1013}
    "};

    #[test]
    fn test_part1() {
        let answer = solve_part1(TEST_INPUT1);
        assert_eq!(answer, 19114);
    }

    #[test]
    fn test_part2() {
        let answer = solve_part2(TEST_INPUT1);
        assert_eq!(answer, 167_409_079_868_000);
    }

    #[test]
    fn test_split_part_by_rule() {
        let part = RangePart::new(2001..2005);
        let rule = RuleDef {
            condition: Some(Condition {
                category: Category::X,
                op: Op::Gt,
                value: 2006,
            }),
            result: RuleResult::Accept,
        };

        let result = split_part_by_rule(&rule, part);
        println!("{:#?}", result);
    }
}

#[cfg(feature = "divan")]
mod benches {
    use super::*;
    use divan::black_box;

    #[divan::bench]
    fn part1() {
        let answer = solve_part1(black_box(INPUT));
        assert_eq!(answer, 480738);
    }

    #[divan::bench]
    fn part2() {
        let answer = solve_part2(black_box(INPUT));
        assert_eq!(answer, 131_550_418_841_958);
    }
}
