use std::cell::RefCell;

advent_of_code::solution!(9);

#[derive(Clone, Debug)]
struct Block {
    length: i64,
    empty: bool,
    id: i64,
}

struct Segment {
    blocks: Vec<Block>,
    start_idx: i64,
}

fn parse(input: &str) -> Vec<RefCell<Segment>> {
    let mut segments = vec![];
    let mut file = true;
    let mut file_id = 0;
    let mut idx = 0;

    for v in input
        .chars()
        .filter(|x| x.is_digit(10))
        .map(|x| x.to_string().parse::<i64>().unwrap())
    {
        let block: Block;
        if file {
            block = Block {
                length: v,
                empty: false,
                id: file_id,
            };
            file_id += 1;
        } else {
            block = Block {
                length: v,
                empty: true,
                id: -1,
            };
        }
        segments.push(RefCell::new(Segment {
            blocks: vec![block],
            start_idx: idx,
        }));
        file = !file;
        idx += v;
    }
    segments
}

fn relocate_block(
    segments: &Vec<RefCell<Segment>>,
    segment_idx: i64,
    empty_idx: &mut i64,
    block: &Block,
) -> Option<Block> {
    let mut bl = block.clone();
    while *empty_idx < segment_idx && bl.length > 0 {
        let mut seg = segments[*empty_idx as usize].borrow_mut();
        if seg.blocks.iter().any(|x| x.empty) {
            let mut new_blocks = seg.blocks.clone();
            for (i, curr_block) in seg.blocks.iter().enumerate() {
                if curr_block.empty {
                    if curr_block.length == bl.length {
                        new_blocks[i].empty = false;
                        new_blocks[i].id = bl.id;
                        bl.length = 0;
                        break;
                    } else if curr_block.length > bl.length {
                        new_blocks[i].length -= bl.length;
                        new_blocks.insert(i, bl.clone());
                        bl.length = 0;
                        break;
                    } else {
                        // curr_block.length < bl.length
                        new_blocks[i].empty = false;
                        new_blocks[i].id = bl.id;
                        bl.length -= curr_block.length;
                    }
                }
            }
            seg.blocks = new_blocks;
        } else {
            *empty_idx += 1;
        }
    }
    if bl.length > 0 {
        return Some(bl);
    }
    None
}

fn relocate_block2(segments: &Vec<RefCell<Segment>>, segment_idx: i64, block: &Block) -> bool {
    let mut bl = block.clone();
    let mut empty_idx = 0;
    while empty_idx < segment_idx && bl.length > 0 {
        let mut seg = segments[empty_idx as usize].borrow_mut();
        if seg.blocks.iter().any(|x| x.empty) {
            let mut new_blocks = seg.blocks.clone();
            for (i, curr_block) in seg.blocks.iter().enumerate() {
                if curr_block.empty {
                    if curr_block.length == bl.length {
                        new_blocks[i].empty = false;
                        new_blocks[i].id = bl.id;
                        bl.length = 0;
                        break;
                    } else if curr_block.length > bl.length {
                        new_blocks[i].length -= bl.length;
                        new_blocks.insert(i, bl.clone());
                        bl.length = 0;
                        break;
                    } else {
                    }
                }
            }
            seg.blocks = new_blocks;
        }

        empty_idx += 1;
    }
    if bl.length > 0 {
        return false;
    }
    true
}

#[allow(dead_code)]
fn debug_blocks(segments: &Vec<RefCell<Segment>>) {
    for seg in segments {
        for block in &seg.borrow().blocks {
            for _i in 0..block.length {
                if block.empty {
                    print!(".");
                } else {
                    print!("{:?}", block.id);
                }
            }
        }
    }
    println!("");
}

pub fn part_one(input: &str) -> Option<i64> {
    let segments = parse(input);

    let mut empty_idx = 1;
    for (i, seg) in segments.iter().enumerate().rev() {
        let mut new_blocks = vec![];
        for block in &seg.borrow().blocks {
            if !block.empty {
                let res = relocate_block(&segments, i as i64, &mut empty_idx, block);
                if res.is_some() {
                    new_blocks.push(res.clone().unwrap());
                    new_blocks.push(Block {
                        length: block.length - res.unwrap().length,
                        empty: true,
                        id: -1,
                    });
                } else {
                    new_blocks.push(Block {
                        length: block.length,
                        empty: true,
                        id: -1,
                    });
                }
            } else {
                new_blocks.push(block.clone());
            }
        }
        seg.borrow_mut().blocks = new_blocks;
    }
    Some(calc_checksum(&segments))
}

pub fn part_two(input: &str) -> Option<i64> {
    let segments = parse(input);
    for (i, seg) in segments.iter().enumerate().rev() {
        let mut new_blocks = vec![];
        for block in &seg.borrow().blocks {
            if !block.empty {
                let res = relocate_block2(&segments, i as i64, block);
                if !res {
                    new_blocks.push(block.clone());
                } else {
                    new_blocks.push(Block {
                        length: block.length,
                        empty: true,
                        id: -1,
                    });
                }
            } else {
                new_blocks.push(block.clone());
            }
        }
        seg.borrow_mut().blocks = new_blocks;
    }
    Some(calc_checksum(&segments))
}

fn calc_checksum(segments: &Vec<RefCell<Segment>>) -> i64 {
    let mut result = 0;
    for seg in segments {
        let mut idx = seg.borrow().start_idx;
        for block in &seg.borrow().blocks {
            if !block.empty {
                for _i in 0..block.length {
                    result += idx * block.id;
                    idx += 1;
                }
            } else {
                idx += block.length
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
