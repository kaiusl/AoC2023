use std::collections::HashMap;

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
    todo!()
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
        assert_eq!(answer, todo!());
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
        assert_eq!(answer, todo!());
    }
}
