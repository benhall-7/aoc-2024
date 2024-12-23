use std::collections::HashMap;

use nalgebra::Vector2;

/*
789
456
123
 0A
*/

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum NumPad {
    ButtonA,
    Button0,
    Button1,
    Button2,
    Button3,
    Button4,
    Button5,
    Button6,
    Button7,
    Button8,
    Button9,
}

impl NumPad {
    pub const fn position(self) -> Vector2<isize> {
        match self {
            Self::ButtonA => Vector2::new(2, 0),
            Self::Button0 => Vector2::new(1, 0),
            Self::Button1 => Vector2::new(0, 1),
            Self::Button2 => Vector2::new(1, 1),
            Self::Button3 => Vector2::new(2, 1),
            Self::Button4 => Vector2::new(0, 2),
            Self::Button5 => Vector2::new(1, 2),
            Self::Button6 => Vector2::new(2, 2),
            Self::Button7 => Vector2::new(0, 3),
            Self::Button8 => Vector2::new(1, 3),
            Self::Button9 => Vector2::new(2, 3),
        }
    }

    pub fn operations(self, to: Self) -> Vec<DirPad> {
        let diff = to.position() - self.position();
        let num_x = diff.x.abs() as usize;
        let num_y = diff.y.abs() as usize;
        let dir_x = match diff.x >= 0 {
            true => DirPad::ButtonRight,
            false => DirPad::ButtonLeft,
        };
        let dir_y = match diff.y >= 0 {
            true => DirPad::ButtonUp,
            false => DirPad::ButtonDown,
        };
        if self.position().x == 0 && to.position().y == 0 {
            [vec![dir_x; num_x], vec![dir_y; num_y]].concat()
        } else if self.position().y == 0 && to.position().x == 0 {
            [vec![dir_y; num_y], vec![dir_x; num_x]].concat()
        } else {
            let mut ops = [vec![dir_x; num_x], vec![dir_y; num_y]].concat();
            ops.sort_by(|a, b| a.get_sort_order().cmp(&b.get_sort_order()));
            ops
        }
    }

    pub fn operations_commit(self, to: Self) -> Vec<DirPad> {
        let mut ops = self.operations(to);
        ops.push(DirPad::ButtonA);
        ops
    }

    pub fn translate(seq: Vec<Self>) -> Vec<DirPad> {
        Self::translate_chunks(seq).into_iter().flatten().collect()
    }

    pub fn translate_chunks(seq: Vec<Self>) -> Vec<Vec<DirPad>> {
        let mut pad = Self::default();
        seq.iter()
            .map(move |input| {
                let ret = pad.operations_commit(*input);
                pad = *input;
                ret
            })
            .collect()
    }
}

impl Default for NumPad {
    fn default() -> Self {
        NumPad::ButtonA
    }
}

impl TryFrom<char> for NumPad {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Self::ButtonA),
            '0' => Ok(Self::Button0),
            '1' => Ok(Self::Button1),
            '2' => Ok(Self::Button2),
            '3' => Ok(Self::Button3),
            '4' => Ok(Self::Button4),
            '5' => Ok(Self::Button5),
            '6' => Ok(Self::Button6),
            '7' => Ok(Self::Button7),
            '8' => Ok(Self::Button8),
            '9' => Ok(Self::Button9),
            _ => Err(value),
        }
    }
}

/*
  ^ A
< v >
*/
impl DirPad {
    pub const fn position(self) -> Vector2<isize> {
        match self {
            Self::ButtonA => Vector2::new(2, 1),
            Self::ButtonUp => Vector2::new(1, 1),
            Self::ButtonRight => Vector2::new(2, 0),
            Self::ButtonDown => Vector2::new(1, 0),
            Self::ButtonLeft => Vector2::new(0, 0),
        }
    }

    fn operations(self, to: Self) -> Vec<Self> {
        let diff = to.position() - self.position();
        let num_x = diff.x.abs() as usize;
        let num_y = diff.y.abs() as usize;
        let dir_x = match diff.x >= 0 {
            true => Self::ButtonRight,
            false => Self::ButtonLeft,
        };
        let dir_y = match diff.y >= 0 {
            true => Self::ButtonUp,
            false => Self::ButtonDown,
        };
        if self.position().x == 0 && to.position().y == 1 {
            [vec![dir_x; num_x], vec![dir_y; num_y]].concat()
        } else if self.position().y == 1 && to.position().x == 0 {
            [vec![dir_y; num_y], vec![dir_x; num_x]].concat()
        } else {
            let mut ops = [vec![dir_x; num_x], vec![dir_y; num_y]].concat();
            ops.sort_by(|a, b| a.get_sort_order().cmp(&b.get_sort_order()));
            ops
        }
    }

    pub fn operations_commit(self, to: Self) -> Vec<Self> {
        let mut ops = self.operations(to);
        ops.push(Self::ButtonA);
        ops
    }

    pub fn translate(seq: Vec<Self>) -> Vec<Self> {
        Self::translate_chunks(seq).into_iter().flatten().collect()
    }

    pub fn translate_chunks(seq: Vec<Self>) -> Vec<Vec<Self>> {
        let mut pad = Self::default();
        seq.iter()
            .map(move |input| {
                let ret = pad.operations_commit(*input);
                pad = *input;
                ret
            })
            .collect()
    }

    pub const fn get_sort_order(self) -> usize {
        match self {
            Self::ButtonA => 0,
            Self::ButtonLeft => 1,
            Self::ButtonUp => 2,
            Self::ButtonDown => 3,
            Self::ButtonRight => 4,
        }
    }
}

impl Default for DirPad {
    fn default() -> Self {
        DirPad::ButtonA
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DirPad {
    ButtonA,
    ButtonUp,
    ButtonDown,
    ButtonLeft,
    ButtonRight,
}

// const INPUT: [&str; 5] = ["029A", "980A", "179A", "456A", "379A"];
const INPUT: [&str; 5] = ["319A", "670A", "349A", "964A", "586A"];

fn part_1() {
    let key_pad_buttons = INPUT
        .iter()
        .map(|pass| {
            let num = usize::from_str_radix(&pass[0..3], 10).unwrap();
            pass.chars()
                .map(|chr| NumPad::try_from(chr))
                .collect::<Result<Vec<_>, _>>()
                .map(|res| (res, num))
        })
        .collect::<Result<Vec<_>, _>>()
        .expect("couldn't parse input");

    let scores: usize = key_pad_buttons
        .into_iter()
        .map(|(seq, digits)| {
            let dir_inputs_1 = NumPad::translate(seq);
            let dir_inputs_2 = DirPad::translate(dir_inputs_1);
            let dir_inputs_3 = DirPad::translate(dir_inputs_2);

            let min_sequence_len = dir_inputs_3.len();
            min_sequence_len * digits
        })
        .sum();

    println!("total scores: {scores}");
}

fn count_seq(
    remaining: usize,
    chunk: Vec<DirPad>,
    seen: &mut HashMap<(usize, Vec<DirPad>), usize>,
) -> usize {
    let key = (remaining, chunk.clone());
    if let Some(val) = seen.get(&key) {
        return *val;
    }

    let sum = if remaining > 0 {
        DirPad::translate_chunks(chunk)
            .into_iter()
            .map(|subchunk| count_seq(remaining - 1, subchunk, seen))
            .sum()
    } else {
        chunk.len()
    };

    seen.insert(key, sum);

    sum
}

fn part_2() {
    let depth = 25;
    let key_pad_buttons = INPUT
        .iter()
        .map(|pass| {
            let num = usize::from_str_radix(&pass[0..3], 10).unwrap();
            pass.chars()
                .map(|chr| NumPad::try_from(chr))
                .collect::<Result<Vec<_>, _>>()
                .map(|res| (res, num))
        })
        .collect::<Result<Vec<_>, _>>()
        .expect("couldn't parse input");

    let scores: usize = key_pad_buttons
        .into_iter()
        .map(|(seq, digits)| {
            let chunks_1 = NumPad::translate_chunks(seq);
            let mut seen = HashMap::new();

            let min_sequence_len: usize = chunks_1
                .into_iter()
                .map(|chunk| count_seq(depth, chunk, &mut seen))
                .sum();

            min_sequence_len * digits
        })
        .sum();

    println!("extended scores: {scores}");
}

pub fn compute() {
    part_1();
    part_2();
}
