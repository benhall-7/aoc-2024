pub fn get_input() -> Vec<u8> {
    let input = include_str!("input.txt");
    // skip newlines
    let nums: Vec<u8> = input
        .lines()
        .flat_map(|line| {
            line.chars().map(|chr| {
                u8::from_str_radix(&chr.to_string(), 10)
                    .expect("expected a valid numeric character")
            })
        })
        .collect();
    assert_eq!((nums.len() - 1) % 2, 0);
    nums
}

#[derive(Debug, Clone)]
struct Day9Iterator {
    sizes: Vec<u8>,
    left: usize,
    left_read: u8,
    right: usize,
    right_read: u8,
}

impl Day9Iterator {
    pub fn new(input: Vec<u8>) -> Self {
        let len = input.len();
        Self {
            sizes: input,
            left: 0,
            left_read: 0,
            right: len - 1,
            right_read: 0,
        }
    }
}

impl Iterator for Day9Iterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // skip empty data on left and right pointers
        while self.left <= self.right && self.sizes[self.left] == 0 {
            self.left += 1;
        }
        while self.left <= self.right && self.sizes[self.right] == 0 {
            self.right -= 2;
        }

        if self.left < self.right {
            // which side of the data should we read?
            // depends if the left pointer is looking at a file or buffer space
            if self.left % 2 == 0 {
                let file_id = self.left / 2;
                self.left_read += 1;
                if self.left_read >= self.sizes[self.left] {
                    self.left += 1;
                    self.left_read = 0;
                }

                Some(file_id)
            } else {
                let file_id = self.right / 2;
                self.left_read += 1;
                self.right_read += 1;

                if self.left_read >= self.sizes[self.left] {
                    self.left += 1;
                    self.left_read = 0;
                }
                if self.right_read >= self.sizes[self.right] {
                    self.right -= 2;
                    self.right_read = 0;
                }

                Some(file_id)
            }
        } else if self.left == self.right {
            let file_id = self.left / 2;
            self.left_read += 1;
            // reduce the amount left to read by the amount already read from the right
            if self.left_read >= self.sizes[self.left] - self.right_read {
                self.left += 1;
                self.left_read = 0;
            }

            Some(file_id)
        } else {
            None
        }
    }
}

fn part_1() {
    let input = get_input();
    // let input = vec![2, 3, 3, 3, 1, 3, 3, 1, 2, 1, 4, 1, 4, 1, 3, 1, 4, 0, 2];
    let sum: usize = Day9Iterator::new(input)
        .enumerate()
        .map(|(index, file_id)| index * file_id)
        .sum();
    println!("{sum}");
}

#[derive(Debug, Clone, Copy)]
struct Block {
    pub file_id: Option<usize>,
    pub size: u8,
}

fn part_2() {
    let input = get_input();
    // let input = vec![2, 3, 3, 3, 1, 3, 3, 1, 2, 1, 4, 1, 4, 1, 3, 1, 4, 0, 2];
    let mut blocks = input
        .into_iter()
        .enumerate()
        .map(|(index, size)| Block {
            file_id: if index % 2 == 0 {
                Some(index / 2)
            } else {
                None
            },
            size,
        })
        .collect::<Vec<_>>();

    let mut right = blocks.len() - 1;
    while right > 0 {
        let block = blocks[right];

        if block.file_id.is_some() {
            for left in 0..right {
                let space = blocks[left];
                if space.file_id.is_none() && space.size >= block.size {
                    // the moved block is filled with empty space
                    blocks[right].file_id = None;
                    // the old empty space is reduced in size
                    blocks[left].size -= block.size;
                    // the moved block moves into the index where the empty space was
                    blocks.insert(left, block);
                    // the indexes shift, so the right index has to shift too
                    right += 1;
                    break;
                }
            }
        }
        right -= 1;
    }

    let sum: usize = blocks
        .iter()
        .filter(|block| block.size > 0)
        .flat_map(|block| (0..block.size).map(|_| block.file_id))
        .enumerate()
        .map(|(index, file_id)| file_id.unwrap_or(0) * index)
        .sum();

    println!("{sum}");
}

pub fn compute() {
    part_1();
    part_2();
}
