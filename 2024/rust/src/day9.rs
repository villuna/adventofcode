use aoc::AOContext;

#[derive(Clone)]
enum Block {
    File {
        id: usize,
        size: u8,
    },
    Space { size: u8 },
}

fn parse(input: &str) -> Vec<Block> {
    let mut res = Vec::new();
    let mut chars = input.trim_end().chars();
    let mut id = 0;

    loop {
        let Some(block) = chars.next() else { break; };
        let size = block as u8 - b'0';
        res.push(Block::File { id, size });
        id += 1;

        let Some(free) = chars.next() else { break; };
        let size = free as u8 - b'0';
        res.push(Block::Space { size });
    }

    res
}

fn checksum(blocks: &[Block]) -> u64 {
    let mut index = 0;
    let mut checksum = 0;

    for block in blocks {
        match block {
            Block::File { id, size } => { 
                checksum += (index..index + *size as u64).sum::<u64>() * *id as u64;
                index += *size as u64;
            },
            Block::Space { size } => {
                index += *size as u64;
            }
        }
    }

    checksum
}

fn part1(mut blocks: Vec<Block>) -> u64 {
    loop {
        let Some((i, space)) = blocks.iter().cloned().enumerate().find(|(_, s)| matches!(s, Block::Space { .. })) else { break; };
        let Block::Space { size: space_size } = space else { unreachable!() };
        let f@Block::File { id, size: file_size } = blocks.last().unwrap().clone() else { unreachable!() };

        if space_size >= file_size {
            blocks.pop();
            while matches!(blocks.last().unwrap(), Block::Space { .. }) {
                blocks.pop();
            }
            blocks.insert(i, f);

            if space_size > file_size {
                let Some(Block::Space { size }) = blocks.get_mut(i + 1) else { break };
                *size -= file_size;
            } else {
                blocks.remove(i + 1);
            }
        } else {
            blocks.insert(i, Block::File { id, size: space_size });
            let Block::File { size, .. } = blocks.last_mut().unwrap() else { unreachable!() };
            *size -= space_size;
            blocks.remove(i + 1);
        }
    }

    checksum(&blocks) 
}

fn part2(mut blocks: Vec<Block>) -> u64 {
    let Some(Block::File { id: last_id, .. }) = blocks.last() else { panic!() };

    for id in (0..=*last_id).rev() {
        let Some((i, f@Block::File { size: file_size, .. })) = blocks.iter().cloned().enumerate().find(|(_, b)| matches!(b, Block::File { id: bid, .. } if *bid == id)) else { 
            continue;
        };

        let Some((si, Block::Space { size: space_size })) = blocks.iter().cloned().enumerate().find(|(si, s)| *si < i && match s {
            Block::Space { size: space_size } if *space_size >= file_size => true,
            _ => false
        }) else {
            continue;
        };

        blocks[i] = Block::Space { size: file_size };
        blocks.insert(si, f);

        if file_size == space_size {
            blocks.remove(si + 1);
        } else {
            let Some(Block::Space { size }) = blocks.get_mut(si + 1) else { break; };
            *size -= file_size;
        }

        for i in (0..blocks.len()).rev() {
            if let Some(Block::Space { size: s1 }) = blocks.get(i).cloned() {
                if let Some(Block::Space { size: s2 }) = blocks.get(i - 1).cloned() {
                    blocks.remove(i);
                    blocks.remove(i - 1);
                    blocks.insert(i - 1, Block::Space { size: s1 + s2 });
                }
            }
        }
    }

    checksum(&blocks)
}

pub fn day9(input: String, ctx: &mut AOContext) {
    let blocks: Vec<Block> = parse(&input);
    ctx.submit_part1(part1(blocks.clone()));
    ctx.submit_part2(part2(blocks));
}
