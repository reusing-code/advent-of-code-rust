use core::fmt;
use std::collections::HashMap;

advent_of_code::solution!(24);

#[derive(Clone, Debug, PartialEq, Eq)]
enum Operation {
    OR,
    XOR,
    AND,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Gate<'a> {
    a: &'a str,
    b: &'a str,
    c: &'a str,
    op: Operation,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Gateo {
    a: String,
    b: String,
    c: String,
    op: Operation,
}

fn parse_input<'a>(input: &'a str) -> (HashMap<&str, bool>, Vec<Gate<'a>>) {
    let mut wires = HashMap::<&str, bool>::new();
    let mut split: Vec<Vec<&str>> = vec![vec![], vec![]];
    let mut first = true;
    for line in input.lines() {
        if line.is_empty() {
            first = false;
            continue;
        }
        if first {
            split[0].push(line);
        } else {
            split[1].push(line);
        }
    }
    for l in &split[0] {
        let mut s2 = l.split(": ");
        let n = s2.next().unwrap();
        wires.insert(n, s2.next().unwrap().parse::<i32>().unwrap() > 0);
    }

    let mut gates = vec![];

    for l in &split[1] {
        if l.contains("AND") {
            gates.push(Gate {
                a: &l[0..3],
                b: &l[8..11],
                c: &l[15..18],
                op: Operation::AND,
            });
        } else if l.contains("XOR") {
            gates.push(Gate {
                a: &l[0..3],
                b: &l[8..11],
                c: &l[15..18],
                op: Operation::XOR,
            });
        } else {
            gates.push(Gate {
                a: &l[0..3],
                b: &l[7..10],
                c: &l[14..17],
                op: Operation::OR,
            });
        }
    }

    (wires, gates)
}

pub fn part_one(input: &str) -> Option<i64> {
    let (mut wires, mut gates) = parse_input(input);

    while !gates.is_empty() {
        for i in 0..gates.len() {
            let g = &gates[i];
            if wires.contains_key(g.a) && wires.contains_key(g.b) {
                *wires.entry(g.c).or_default() = match g.op {
                    Operation::AND => wires[g.a] && wires[g.b],
                    Operation::XOR => wires[g.a] ^ wires[g.b],
                    Operation::OR => wires[g.a] || wires[g.b],
                };
                gates.swap_remove(i);
                break;
            }
        }
    }

    let mut wires_vec = wires
        .iter()
        .filter(|(&k, &_v)| k.starts_with("z"))
        .collect::<Vec<_>>();

    wires_vec.sort_by(|a, b| b.0.cmp(a.0));

    Some(
        wires_vec
            .iter()
            .fold(0_i64, |acc, &x| acc * 2 + if *x.1 { 1 } else { 0 }),
    )
}

fn print_rec(gates: &HashMap<&str, Gate>, wire: &str) -> String {
    if wire.starts_with("x") || wire.starts_with("y") {
        return String::from(wire);
    }
    let g = gates.get(wire).unwrap();
    format!(
        "({} {:?} {})[{}]",
        print_rec(gates, g.a),
        g.op,
        print_rec(gates, g.b),
        g.c
    )
}

fn get_gates_rec<'a>(gates: &'a HashMap<&str, Gate>, wire: &str) -> Vec<&'a Gate<'a>> {
    if wire.starts_with("x") || wire.starts_with("y") {
        return vec![];
    }
    let g = gates.get(wire).unwrap();
    let mut res = get_gates_rec(gates, g.a);
    res.extend(get_gates_rec(gates, g.b));
    res.push(g);
    res
}

fn gate_name(c: char, n: i32) -> String {
    format!("{c}{n:02}")
}

fn find_gate<'a>(gates: &'a Vec<Gate>, find: &Gateo) -> Option<&'a Gate<'a>> {
    for g in gates {
        if g.a == &find.a && g.b == &find.b && g.op == find.op {
            return Some(g);
        }
        if g.b == &find.a && g.a == &find.b && g.op == find.op {
            return Some(g);
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<String> {
    let (_, gates) = parse_input(input);
    let mut replacement = HashMap::<String, String>::new();
    let mut check_gate = |c: String, mut g: Gateo| {
        g.a = replacement.get(&g.a).unwrap_or(&g.a).to_string();
        g.b = replacement.get(&g.b).unwrap_or(&g.b).to_string();
        g.c = replacement.get(&g.c).unwrap_or(&g.c).to_string();
        let r = find_gate(&gates, &g);
        if r.is_none() {
            // requires manual fixing the diff
            println!("NOT FOUND: {}", g);
            let mut rc = replacement.iter().collect::<Vec<_>>();
            rc.sort();
            for r in &rc {
                println!("{:?}", r);
            }
            panic!("");
        }
        if r.unwrap().c != c {
            replacement.insert(c, r.unwrap().c.to_string());
        }
    };
    check_gate(
        String::from("z00"),
        Gateo {
            a: String::from("x00"),
            b: String::from("y00"),
            op: Operation::XOR,
            c: String::from("z00"),
        },
    );
    check_gate(
        String::from("c00"),
        Gateo {
            a: String::from("x00"),
            b: String::from("y00"),
            op: Operation::AND,
            c: String::from("c00"),
        },
    );
    for i in 1..45 {
        check_gate(
            gate_name('d', i),
            Gateo {
                a: gate_name('x', i),
                b: gate_name('y', i),
                op: Operation::XOR,
                c: gate_name('d', i),
            },
        );
        check_gate(
            gate_name('e', i),
            Gateo {
                a: gate_name('x', i),
                b: gate_name('y', i),
                op: Operation::AND,
                c: gate_name('e', i),
            },
        );
        check_gate(
            gate_name('z', i),
            Gateo {
                a: gate_name('c', i - 1),
                b: gate_name('d', i),
                op: Operation::XOR,
                c: gate_name('z', i),
            },
        );
        check_gate(
            gate_name('f', i),
            Gateo {
                a: gate_name('c', i - 1),
                b: gate_name('d', i),
                op: Operation::AND,
                c: gate_name('f', i),
            },
        );
        check_gate(
            gate_name('c', i),
            Gateo {
                a: gate_name('e', i),
                b: gate_name('f', i),
                op: Operation::OR,
                c: gate_name('c', i),
            },
        );
    }
    /*
        let mut gate_vec = vec![];
        gate_vec.extend(gates.into_values());
        gate_vec.sort_by(|a, b| a.c[1..].cmp(&b.c[1..]));

        for g in gate_vec {
            println!("{}", g);
        }

        // END c44 == z45
    */
    None
}

impl fmt::Display for Gateo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {:?} {} -> {}", self.a, self.op, self.b, self.c)
    }
}

pub fn part_two_internal(input: &str, _swaps: usize) -> Option<String> {
    let (_, mut gates) = parse_input(input);
    let mut gate_map = HashMap::new();
    for gate in &gates {
        gate_map.insert(gate.c, gate.clone());
    }
    gates.sort_by(|a, b| a.c.cmp(b.c));
    let mut previous_xy_gates: Vec<&Gate> = vec![];
    let mut save_gates: Vec<&Gate> = vec![];
    gates.iter().filter(|g| g.c.starts_with("z")).for_each(|g| {
        println!("{}: {}\n", g.c, print_rec(&gate_map, g.c));
        let mut check = || {
            let curr_gates = get_gates_rec(&gate_map, g.c);
            if g.c != "z00" && g.c != "z01" {
                for pg in &previous_xy_gates {
                    if !curr_gates.iter().any(|g| **g == **pg) {
                        return false;
                    }
                }
                let mut find_xor = false;
                let mut find_and1 = false;

                let num = g.c[1..].parse::<u32>().unwrap();
                for &cg in &curr_gates {
                    if cg.a.starts_with("x") || cg.a.starts_with("y") {
                        let num2 = cg.a[1..].parse::<u32>().unwrap();
                        if num2 == num && cg.op == Operation::XOR {
                            find_xor = true;
                        }
                        if num2 == num - 1 && cg.op == Operation::AND {
                            find_and1 = true;
                        }
                    }
                }
                if !find_and1 || !find_xor {
                    return false;
                }
            }

            save_gates = curr_gates.clone();
            previous_xy_gates = curr_gates
                .iter()
                .filter(|g| g.a.starts_with("x") || g.a.starts_with("y"))
                .map(|g| *g)
                .collect::<Vec<&Gate>>();
            true
        };

        if !check() {
            let mut gats = gate_map.clone();
            for g in &save_gates {
                gats.remove(g.c);
            }
            for g in gats {
                println!("{}", g.0);
            }
        }
    });
    /*   gates.iter().filter(|g| g.c.starts_with("z")).for_each(|g| {
        println!("{}", print_rec(&gate_map, g.c));
        println!("out: {}: {:?}", g.c, find_rec(&gate_map, g.c, &g.c[1..], 0))
    });*/
    /*
        for i in 0..gates.len() {
            for i2 in i + 1..gates.len() {
                for j in i + 1..gates.len() {
                    if j == i2 {
                        continue;
                    }
                    for j2 in j + 1..gates.len() {
                        if j2 == i2 {
                            continue;
                        }
                        let mut new_gates = gates.clone();
                        (new_gates[i].c, new_gates[i2].c) = (new_gates[i2].c, new_gates[i].c);
                        (new_gates[j].c, new_gates[j2].c) = (new_gates[j2].c, new_gates[j].c);
                        let mut gate_map = HashMap::new();
                        for gate in &new_gates {
                            gate_map.insert(gate.c, gate.clone());
                        }
                        if plausible(&gate_map) {
                            let mut result = vec![
                                new_gates[i].c,
                                new_gates[i2].c,
                                new_gates[j].c,
                                new_gates[j2].c,
                            ];
                            result.sort();
                            println!("Plausible: {:?}", result.join(","));
                        }
                    }
                }
            }
        }
    */
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
    }
}
