use std::{
    fmt::Display,
    io::Write,
    ops::{Index, IndexMut},
};

const WIDTH: usize = 240;
const HEIGHT: usize = 160;

fn pad(val: usize, bits: usize) -> u32 {
    let rem = val % bits;
    if rem > 0 {
        return (val + bits - rem) as u32;
    }
    val as u32
}

#[derive(Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

const OFFSETS: [(i8, i8); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

enum Rotation {
    Straight,
    Left,
    Right,
    Reverse,
}

#[derive(Copy, Clone)]
enum State {
    Black,
    White,
}

impl State {
    fn to_val(&self) -> u32 {
        match self {
            State::Black => 0,
            State::White => 1,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Pos(usize, usize);

impl Direction {
    fn to_offset(&self) -> (i8, i8) {
        match self {
            Direction::North => (0, 1),
            Direction::East => (-1, 0),
            Direction::South => (0, -1),
            Direction::West => (1, 0),
        }
    }

    // fn rotate(&self, rot: Rotation) -> Direction {}
}

#[derive(Copy, Clone)]
struct StateTransition {
    new_color: u8,
    new_state: State,
    rotation: i8,
}

#[derive(Copy, Clone)]
struct Turmite {
    state: u8,
    dir: i8,
    pos: Pos,
}

impl Turmite {
    fn new(pos: Pos) -> Turmite {
        Turmite {
            state: 0,
            dir: 0,
            pos,
        }
    }
}

struct Grid {
    states: Vec<State>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        let mut grid = Grid {
            states: vec![State::Black; width * height],
            width,
            height,
        };
        // for x in (0..WIDTH).step_by(16) {
        //     for y in (0..HEIGHT).step_by(8) {
        //         grid.states[y][24] = State::White;
        //     }
        // }
        grid
    }

    fn step(
        &mut self,
        turmites: &mut Vec<Turmite>,
        state_table: &Vec<(StateTransition, StateTransition)>,
    ) {
        for turmite in turmites.iter_mut() {
            let state = self.index(turmite.pos);
            let color = turmite.state;
            let transition = match state {
                State::Black => state_table[color as usize].0,
                State::White => state_table[color as usize].1,
            };
            let new_color = transition.new_color;
            let new_dir = (turmite.dir + transition.rotation).rem_euclid(4);
            let offset = OFFSETS[turmite.dir as usize];
            self[turmite.pos] = transition.new_state;
            turmite.dir = new_dir;
            turmite.state = new_color;
            turmite.pos = Pos(
                (turmite.pos.0 as isize + offset.0 as isize).rem_euclid(WIDTH as isize) as usize,
                (turmite.pos.1 as isize + offset.1 as isize).rem_euclid(HEIGHT as isize) as usize,
            );
        }
    }

    fn write_frame(&self, encoder: &mut gif::Encoder<&mut std::fs::File>) {
        encoder
            .write_frame(&gif::Frame::from_palette_pixels(
                WIDTH as u16,
                HEIGHT as u16,
                &self
                    .states
                    .iter()
                    .map(|v| v.to_val() as u8)
                    .collect::<Vec<u8>>(),
                &[0xff, 0xff, 0xff, 0, 0, 0],
                None,
            ))
            .unwrap();
    }

    fn print_blocks(&self) {
        for row in self.states.chunks_exact(self.width) {
            for col in row.iter() {
                match col {
                    State::Black => print!(" "),
                    State::White => print!("*"),
                }
            }
            println!();
        }
    }
    fn print_braille(&self) {
        // for chunk in self.states.chunks_exact(4 * self.width) {
        //     for row in chunk.chunks_exact(self.width) {
        //         for x in (0..WIDTH).step_by(2) {
        //             let mut code = 0x2800;
        //             code |= rows[0][x + 0].to_val() << 0;
        //             code |= rows[1][x + 0].to_val() << 1;
        //             code |= rows[2][x + 0].to_val() << 2;
        //             code |= rows[3][x + 0].to_val() << 6;
        //             code |= rows[0][x + 1].to_val() << 3;
        //             code |= rows[1][x + 1].to_val() << 4;
        //             code |= rows[2][x + 1].to_val() << 5;
        //             code |= rows[3][x + 1].to_val() << 7;
        //             print!("{}", char::from_u32(code).unwrap());
        //         }
        //         println!();
        //     }
        // }
        for x in (0..self.width).step_by(2) {
            for y in (0..self.height).step_by(4) {
                for row in 0..2 {
                    for col in 0..4 {
                        let i = (y + row) * self.width + x + col;
                        let mut code = 0x2800;
                        code |= self.states[i].to_val() << 0;
                        code |= self.states[i].to_val() << 1;
                        code |= self.states[i].to_val() << 2;
                        code |= self.states[i].to_val() << 6;
                        code |= self.states[i].to_val() << 3;
                        code |= self.states[i].to_val() << 4;
                        code |= self.states[i].to_val() << 5;
                        code |= self.states[i].to_val() << 7;
                        print!("{}", char::from_u32(code).unwrap());
                    }
                }
            }
            println!();
        }
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.print_braille();
        Ok(())
    }
}

impl Index<Pos> for Grid {
    type Output = State;

    fn index(&self, index: Pos) -> &Self::Output {
        let i = index.0 + self.width * index.1;
        &self.states[i]
    }
}
impl IndexMut<Pos> for Grid {
    fn index_mut(&mut self, index: Pos) -> &mut Self::Output {
        let i = index.0 + self.width * index.1;
        &mut self.states[i]
    }
}

fn main() {
    let mut grid = Grid::new(WIDTH, HEIGHT);
    let mut state_table = Vec::new();
    let mut turmites = Vec::new();

    // new state, new direction, new color
    state_table.push((
        StateTransition {
            new_color: 0,
            new_state: State::White,
            rotation: -1,
        },
        StateTransition {
            new_color: 1,
            new_state: State::White,
            rotation: 1,
        },
    ));
    state_table.push((
        StateTransition {
            new_color: 0,
            new_state: State::Black,
            rotation: 1,
        },
        StateTransition {
            new_color: 1,
            new_state: State::Black,
            rotation: -1,
        },
    ));
    // state_table.push((StateTransition(1, -1, 0), StateTransition(0, 0, 1)));
    // state_table.push((StateTransition(0, -1, 0), StateTransition(0, -1, 1)));
    // [[[1, -1, 0], [0, 0, 1]], [[0, -1, 0], [0, -1, 1]]],
    // [[[1, -1, 0], [1, 1, 1]], [[0, 1, 0], [0, -1, 1]]],

    // println!("{}", char::from_u32(0x2832).unwrap());
    turmites.push(Turmite::new(Pos(WIDTH / 2, HEIGHT / 2)));

    let mut file = std::fs::File::create("out.gif").unwrap();
    let mut encoder = gif::Encoder::new(&mut file, WIDTH as u16, HEIGHT as u16, &[]).unwrap();

    for i in 0..100000 {
        grid.step(&mut turmites, &state_table);
        if i % 1000 == 0 {
            grid.write_frame(&mut encoder);
        }
        // if i % 1000 == 0 {
        //     println!("{}", grid);
        //     let mut str = String::new();
        //     std::io::stdin().read_line(&mut str);
        // }
    }

    // println!("{}", grid);

    // file size in bytes: 1 bpp, padded to 4 bytes per row
    // let file_size: u32 = (WIDTH * HEIGHT / 8) as u32; //pad(WIDTH, 32) * HEIGHT as u32 / 8;
    // println!(
    //     "File size raw: {}, padded: {}",
    //     WIDTH * HEIGHT / 8,
    //     file_size
    // );

    // let mut file = std::fs::File::create("out.bmp").unwrap();

    // // HEADER
    // file.write_all(&[b'B', b'M']).unwrap();
    // file.write_all(&(54 + file_size).to_le_bytes()).unwrap(); // total file space
    // file.write_all(&[0, 0, 0, 0]).unwrap();
    // file.write_all(&[54, 0, 0, 0]).unwrap();

    // // DIB HEADER
    // file.write_all(&[40, 0, 0, 0]).unwrap();
    // file.write_all(&(WIDTH as u32).to_le_bytes()).unwrap();
    // file.write_all(&(HEIGHT as u32).to_le_bytes()).unwrap();
    // file.write_all(&[1, 0]).unwrap(); // number of color planes
    // file.write_all(&[1, 0]).unwrap(); // bits per pixel
    // file.write_all(&[0, 0, 0, 0]).unwrap(); // compression scheme
    // file.write_all(&[0, 0, 0, 0]).unwrap();
    // file.write_all(&[64, 0, 0, 0]).unwrap(); // h dpi
    // file.write_all(&[64, 0, 0, 0]).unwrap(); // v dpi
    // file.write_all(&[0, 0, 0, 0]).unwrap(); // palette size
    // file.write_all(&[0, 0, 0, 0]).unwrap(); // all colors important?

    // // IMG DATA
    // let mut c = 0;
    // for row in grid.states.chunks_exact(HEIGHT) {
    //     for bits in row.chunks(8) {
    //         let mut byte = 0u8;
    //         for (i, s) in bits.iter().enumerate() {
    //             byte |= (s.to_val() as u8) << (7 - i);
    //         }
    //         c += 1;
    //         file.write_all(&[byte]).unwrap();
    //     }
    // }
}
