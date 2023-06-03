use num::integer::lcm;
use std::io::{self, BufRead};
use std::vec;

struct Monkey {
    items: Vec<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    test: Box<dyn Fn(u64) -> bool>,
    throw_to_true: usize,
    throw_to_false: usize,
}

impl Monkey {
    fn new(
        items: Vec<u64>,
        operation: Box<dyn Fn(u64) -> u64>,
        test: Box<dyn Fn(u64) -> bool>,
        throw_to_true: usize,
        throw_to_false: usize,
    ) -> Monkey {
        Monkey {
            items,
            operation,
            test,
            throw_to_true,
            throw_to_false,
        }
    }

    fn print(&self) {
        println!("Starting items: {:?}", self.items);
    }

    fn throw(&self, item: u64) -> (u64, usize) {
        let mut new_worry_lvl: u64 = (self.operation)(item);
        new_worry_lvl = new_worry_lvl / 3;
        if (self.test)(new_worry_lvl) {
            (new_worry_lvl, self.throw_to_true)
        } else {
            (new_worry_lvl, self.throw_to_false)
        }
    }

    fn throw_chilled(&self, item: u64) -> (u64, usize) {
        let new_worry_lvl: u64 = (self.operation)(item);
        //new_worry_lvl = new_worry_lvl / 3;
        if (self.test)(new_worry_lvl) {
            (new_worry_lvl, self.throw_to_true)
        } else {
            (new_worry_lvl, self.throw_to_false)
        }
    }

    fn get_divisor(&self) -> u64 {
        let mut num = 1;
        while !(self.test)(num) {
            num += 1;
        }
        num
    }
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    let mut current_monkey: Option<Monkey> = None;

    for line in input.lines() {
        if line.starts_with("Monkey") {
            // println!("line: {}", line);
            if let Some(monkey) = current_monkey.take() {
                monkeys.push(monkey);
            }
            current_monkey = Some(Monkey {
                items: Vec::new(),
                operation: Box::new(|_| 0),
                test: Box::new(|_| false),
                throw_to_true: 0,
                throw_to_false: 0,
            });
        } else if line.starts_with("  Starting items:") {
            if let Some(monkey) = current_monkey.as_mut() {
                let items: Vec<u64> = line
                    .trim_start_matches("  Starting items: ")
                    .split(", ")
                    .map(|item| item.parse().unwrap())
                    .collect();
                monkey.items = items;
            }
        } else if line.starts_with("  Operation:") {
            if let Some(monkey) = current_monkey.as_mut() {
                // println!("line: {}", line);
                let operation_str = line.trim_start_matches("  Operation: new = ");
                // writing this almost blew my brain
                let operation = match operation_str {
                    s if s.starts_with("old * old") => {
                        Box::new(|old| old * old) as Box<dyn Fn(u64) -> u64>
                    }
                    s if s.starts_with("old * ") => {
                        let factor: u64 = s.trim_start_matches("old * ").parse().unwrap();
                        Box::new(move |old| old * factor) as Box<dyn Fn(u64) -> u64>
                    }
                    s if s.starts_with("old + old") => {
                        Box::new(|old| old + old) as Box<dyn Fn(u64) -> u64>
                    }
                    s if s.starts_with("old + ") => {
                        let value: u64 = s.trim_start_matches("old + ").parse().unwrap();
                        Box::new(move |old| old + value) as Box<dyn Fn(u64) -> u64>
                    }
                    _s => unreachable!(),
                };

                monkey.operation = operation;
            }
        } else if line.starts_with("  Test:") {
            if let Some(monkey) = current_monkey.as_mut() {
                let test_str = line.trim_start_matches("  Test: ");
                let test = match test_str {
                    s if s.starts_with("divisible by ") => {
                        let divisor: u64 = s.trim_start_matches("divisible by ").parse().unwrap();
                        let divisor_clone = divisor.clone();
                        Box::new(move |num| num % divisor_clone == 0 as u64)
                            as Box<dyn Fn(u64) -> bool>
                    }
                    _s => unreachable!(),
                };
                monkey.test = Box::new(test);
            }
        } else if line.starts_with("    If true: throw to monkey") {
            if let Some(monkey) = current_monkey.as_mut() {
                let throw_to = line
                    .trim_start_matches("    If true: throw to monkey ")
                    .parse()
                    .unwrap();
                monkey.throw_to_true = throw_to;
            }
        } else if line.starts_with("    If false: throw to monkey") {
            if let Some(monkey) = current_monkey.as_mut() {
                let throw_to = line
                    .trim_start_matches("    If false: throw to monkey ")
                    .parse()
                    .unwrap();
                monkey.throw_to_false = throw_to;
            }
        }
    }
    if let Some(monkey) = current_monkey.take() {
        monkeys.push(monkey);
    }
    monkeys
}

pub fn task11_part1_v1() {
    let stdin = io::stdin();
    let input: String = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>()
        .join("\n");

    let mut monkeys = parse_monkeys(&input);
    // println!("Monkeys len: {}", monkeys.len());
    let mut mokey_inspected_cnt = vec![0; monkeys.len()];

    for rounds in 0..20 {
        for m_i in 0..monkeys.len() {
            let mut items_to_process = vec![];
            {
                let monkey = &monkeys[m_i];
                let items = monkey.items.clone();
                for item in items {
                    let (new_worry_lvl, throw_to) = monkey.throw(item);
                    // println!("Monkey {} threw old ite {} now new {} to monkey {}", m_i, item, new_worry_lvl, throw_to);
                    items_to_process.push((new_worry_lvl, throw_to));
                }
            }

            monkeys[m_i].items.clear();

            for (new_worry_lvl, throw_to) in items_to_process {
                // print!("> Monkey {} threw new {} to monkey {}", m_i, new_worry_lvl, throw_to);
                let monkey_target = &mut monkeys[throw_to];
                monkey_target.items.push(new_worry_lvl);
                mokey_inspected_cnt[m_i] += 1;
            }

            // for m_i in 0..monkeys.len() {
            //     let monkey = &monkeys[m_i];
            //     println!("Monkey {}", m_i);
            //     monkey.print();
            // }
        }
    }

    // for m_i in 0..monkeys.len() {
    //     println!("Monkey {} inspected cnt {}", m_i, mokey_inspected_cnt[m_i]);
    // }

    // sort mokey_inspected_cnt
    let mut mokey_inspected_cnt_sorted = mokey_inspected_cnt.clone();
    mokey_inspected_cnt_sorted.sort();
    mokey_inspected_cnt_sorted.reverse();
    // println!("Top 2: {}, {}", mokey_inspected_cnt_sorted[0], mokey_inspected_cnt_sorted[1]);
    println!(
        "{}",
        mokey_inspected_cnt_sorted[0] * mokey_inspected_cnt_sorted[1]
    )
}

pub fn task11_part2_v1() {
    let stdin = io::stdin();
    let input: String = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>()
        .join("\n");

    let mut monkeys = parse_monkeys(&input);
    // println!("Monkeys len: {}", monkeys.len());
    let mut mokey_inspected_cnt = vec![0; monkeys.len()];
    let mut monkey_divisors = vec![];
    for m_i in 0..monkeys.len() {
        let monkey = &monkeys[m_i];
        let num = monkey.get_divisor();
        monkey_divisors.push(num);
    }
    // get smallest common multiple
    let mut lcm_value = monkey_divisors[0];
    for m_i in 1..monkey_divisors.len() {
        lcm_value = lcm(monkey_divisors[m_i], lcm_value);
    }
    // println!("LCM: {}", lcm_value);
    for rounds in 0..10000 {
        for m_i in 0..monkeys.len() {
            let mut items_to_process = vec![];
            {
                let monkey = &monkeys[m_i];
                let items = monkey.items.clone();
                for item in items {
                    let (mut new_worry_lvl, throw_to) = monkey.throw_chilled(item);
                    // println!("Monkey {} threw old ite {} now new {} to monkey {}", m_i, item, new_worry_lvl, throw_to);
                    new_worry_lvl = new_worry_lvl % lcm_value;
                    items_to_process.push((new_worry_lvl, throw_to));
                }
            }

            monkeys[m_i].items.clear();

            for (new_worry_lvl, throw_to) in items_to_process {
                // print!("> Monkey {} threw new {} to monkey {}", m_i, new_worry_lvl, throw_to);
                let monkey_target = &mut monkeys[throw_to];
                monkey_target.items.push(new_worry_lvl);
                mokey_inspected_cnt[m_i] += 1;
            }

            // for m_i in 0..monkeys.len() {
            //     let monkey = &monkeys[m_i];
            //     println!("Monkey {}", m_i);
            //     monkey.print();
            // }
        }
    }

    // for m_i in 0..monkeys.len() {
    //     println!("Monkey {} inspected cnt {}", m_i, mokey_inspected_cnt[m_i]);
    // }

    // sort mokey_inspected_cnt
    let mut mokey_inspected_cnt_sorted: Vec<u64> = mokey_inspected_cnt.clone();
    mokey_inspected_cnt_sorted.sort();
    mokey_inspected_cnt_sorted.reverse();
    // println!("Top 2: {}, {}", mokey_inspected_cnt_sorted[0], mokey_inspected_cnt_sorted[1]);
    println!(
        "{}",
        mokey_inspected_cnt_sorted[0] * mokey_inspected_cnt_sorted[1]
    )
}
