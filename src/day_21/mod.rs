mod tests;

use itertools::Itertools;
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

    pub fn operations(self, to: Self) -> Vec<Vec<DirPad>> {
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
        let base_directions = [vec![dir_x; num_x], vec![dir_y; num_y]].concat();
        let mut all_paths: Vec<Vec<DirPad>> = base_directions
            .clone()
            .into_iter()
            .permutations(num_x + num_y)
            .unique()
            .collect::<Vec<_>>();

        if self.position().x == 0 && to.position().y == 0 {
            all_paths = all_paths
                .into_iter()
                .filter(|operations| operations[0..num_y] != vec![dir_y; num_y])
                .collect();
        } else if self.position().y == 0 && to.position().x == 0 {
            all_paths = all_paths
                .into_iter()
                .filter(|operations| operations[0..num_x] != vec![dir_x; num_x])
                .collect();
        }

        all_paths
    }

    pub fn operations_commit(self, to: Self) -> Vec<Vec<DirPad>> {
        let mut ops = self.operations(to);
        ops.iter_mut().for_each(|seq| seq.push(DirPad::ButtonA));
        ops
    }

    pub fn all_possible_operations(seq: Vec<Self>) -> Vec<Vec<Vec<DirPad>>> {
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

    fn operations(self, to: Self) -> Vec<Vec<Self>> {
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
        let base_directions = [vec![dir_x; num_x], vec![dir_y; num_y]].concat();
        let mut all_paths = base_directions
            .clone()
            .into_iter()
            .permutations(num_x + num_y)
            .unique()
            .collect::<Vec<_>>();

        if self.position().x == 0 && to.position().y == 1 {
            all_paths = all_paths
                .into_iter()
                // doesn't start with upward motion
                .filter(|operations| operations[0] != dir_y)
                .collect();
        } else if self.position().y == 1 && to.position().x == 0 {
            all_paths = all_paths
                .into_iter()
                // doesn't end with downward motion
                .filter(|operations| operations[num_x] != dir_y)
                .collect();
        }

        all_paths
    }

    pub fn operations_commit(self, to: Self) -> Vec<Vec<Self>> {
        let mut ops = self.operations(to);
        ops.iter_mut().for_each(|seq| seq.push(Self::ButtonA));
        ops
    }

    pub fn all_possible_operations(seq: Vec<Self>) -> Vec<Vec<Vec<Self>>> {
        let mut pad = Self::default();
        seq.iter()
            .map(move |input| {
                let ret = pad.operations_commit(*input);
                pad = *input;
                ret
            })
            .collect()
    }

    pub fn get_char(self) -> char {
        match self {
            Self::ButtonA => 'A',
            Self::ButtonUp => '^',
            Self::ButtonRight => '>',
            Self::ButtonDown => 'v',
            Self::ButtonLeft => '<',
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

const INPUT: [&str; 5] = ["319A", "670A", "349A", "964A", "586A"];

fn part_1() {
    let key_pad_buttons = ["029A"]
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
            type Inputs = Vec<DirPad>;
            type Sequences = Vec<Vec<Inputs>>;
            println!("DIGITS = {digits}");
            let dir_inputs_1: Sequences = NumPad::all_possible_operations(seq);
            println!("possible inputs for first remote: {:#?}", dir_inputs_1);
            let dir_inputs_2: Vec<_> = dir_inputs_1
                .into_iter()
                .map(|possible| {
                    possible
                        .into_iter()
                        .map(|seq| DirPad::all_possible_operations(seq))
                        .collect::<Vec<_>>()
                })
                .collect();
            println!("possible inputs for second remote: {:#?}", dir_inputs_2);
            // let dir_inputs_3: Sequences = dir_inputs_2
            //     .into_iter()
            //     .map(|seq| DirPad::all_possible_operations(seq))
            //     .collect();
            // let dir_inputs_4: Sequences = dir_inputs_3
            //     .into_iter()
            //     .flat_map(|seq| DirPad::all_possible_operations(seq))
            //     .collect();

            let min_sequence_len = dir_inputs_2.into_iter().map(|seq| seq.len()).min().unwrap();
            min_sequence_len * digits
        })
        .sum();

    println!("total scores: {scores}");
    // > 2888
}

pub fn compute() {
    part_1();
}
