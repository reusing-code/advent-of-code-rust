use std::collections::{HashMap, HashSet};

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<i64> {
    let mut conns: HashMap<&str, HashSet<&str>> = HashMap::new();
    input.lines().for_each(|l| {
        conns
            .entry(&l[..2])
            .or_insert(HashSet::new())
            .insert(&l[3..]);
        conns
            .entry(&l[3..])
            .or_insert(HashSet::new())
            .insert(&l[..2]);
    });

    let mut result = 0;
    for (&pc, neighbors) in conns.iter() {
        let neigh = neighbors.iter().collect::<Vec<_>>();
        for i in 0..neigh.len() - 1 {
            for j in i + 1..neigh.len() {
                let &a = neigh[i];
                let &b = neigh[j];
                if conns[a].contains(b) {
                    if &pc[..1] == "t" || &a[..1] == "t" || &b[..1] == "t" {
                        result += 1;
                    }
                }
            }
        }
    }

    Some(result / 3)
}

pub fn part_two(input: &str) -> Option<String> {
    let mut conns: HashMap<&str, HashSet<&str>> = HashMap::new();
    input.lines().for_each(|l| {
        conns
            .entry(&l[..2])
            .or_insert(HashSet::new())
            .insert(&l[3..]);
        conns
            .entry(&l[3..])
            .or_insert(HashSet::new())
            .insert(&l[..2]);
    });

    let mut largest_len = 3; // we know it is larger than 3
    let mut largest_elems = vec![];

    for (&pc, neighbors) in &conns {
        let neigh = neighbors.iter().collect::<Vec<_>>();
        let mut subsets: Vec<Vec<&str>> = vec![];
        let mut tmp: Vec<&str> = vec![];
        calc_subsets(&neigh, &mut subsets, &mut tmp, 0, largest_len);
        for subset in subsets {
            if subset.len() + 1 <= largest_len {
                continue;
            }

            if all_connected(&conns, &subset) {
                largest_len = subset.len() + 1;
                largest_elems = subset.clone();
                largest_elems.push(pc);
            }
        }
    }

    largest_elems.sort();
    Some(largest_elems.join(","))
}

fn calc_subsets<'a>(
    all: &Vec<&&'a str>,
    res: &mut Vec<Vec<&'a str>>,
    subset: &mut Vec<&'a str>,
    index: usize,
    minsize: usize,
) {
    if subset.len() >= minsize {
        res.push(subset.clone());
    }

    for i in index..all.len() {
        subset.push(all[i]);

        calc_subsets(all, res, subset, i + 1, minsize);

        subset.pop();
    }
}

fn all_connected(conns: &HashMap<&str, HashSet<&str>>, pcs: &Vec<&str>) -> bool {
    for i in 0..pcs.len() - 1 {
        for j in i + 1..pcs.len() {
            let a = pcs[i];
            let b = pcs[j];
            if !conns[a].contains(b) {
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(String::from("co,de,ka,ta")));
    }
}
