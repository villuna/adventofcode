pub const DIRECTIONS: [Dir; 4] = [Dir::Up, Dir::Right, Dir::Down, Dir::Left];

#[derive(Default, Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum Dir {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

pub fn add_coords(c1: (isize, isize), c2: (isize, isize)) -> (isize, isize) {
    (c1.0 + c2.0, c1.1 + c2.1)
}

#[derive(Default, Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
pub struct Coord(pub isize, pub isize);

impl Coord {
    pub fn in_bounds(&self, lower: (isize, isize), upper: (isize, isize)) -> bool {
        self.0 >= lower.0 && self.1 >= lower.1 && self.0 < upper.0 && self.1 < upper.1
    }

    pub fn in_bounds_positive(&self, bounds: (isize, isize)) -> bool {
        self.in_bounds((0, 0), bounds)
    }
}

impl std::ops::Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl std::ops::Add<Dir> for Coord {
    type Output = Self;

    fn add(self, rhs: Dir) -> Self::Output {
        let rhs = rhs.increment();
        Coord(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Mul<isize> for Coord {
    type Output = Self;

    fn mul(self, rhs: isize) -> Self::Output {
        Coord(self.0 * rhs, self.1 * rhs)
    }
}

impl Dir {
    pub fn increment(&self) -> (isize, isize) {
        match self {
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
        }
    }

    pub fn cincrement(&self) -> Coord {
        match self {
            Dir::Up => Coord(0, -1),
            Dir::Down => Coord(0, 1),
            Dir::Left => Coord(-1, 0),
            Dir::Right => Coord(1, 0),
        }
    }

    pub fn from_char(c: char) -> Dir {
        match c {
            'U' => Dir::Up,
            'D' => Dir::Down,
            'L' => Dir::Left,
            'R' => Dir::Right,
            _ => panic!(),
        }
    }

    pub fn opposite(&self) -> Dir {
        match self {
            Dir::Up => Dir::Down,
            Dir::Down => Dir::Up,
            Dir::Left => Dir::Right,
            Dir::Right => Dir::Left,
        }
    }

    pub fn rotate_cw(&self) -> Dir {
        match self {
            Dir::Up => Dir::Right,
            Dir::Right => Dir::Down,
            Dir::Down => Dir::Left,
            Dir::Left => Dir::Up,
        }
    }
}

pub fn digits_in(int: u64) -> u32 {
    if int == 0 {
        1
    } else {
        int.ilog10() + 1
    }
}

pub fn unconcat(whole: u64, suffix: u64) -> Option<u64> {
    let suffix_digits = digits_in(suffix);
    (suffix == whole % 10u64.pow(suffix_digits)).then_some(whole / 10u64.pow(suffix_digits))
}

pub fn split(int: u64) -> Option<(u64, u64)> {
    let n = digits_in(int);
    (n % 2 == 0).then_some((int / 10u64.pow(n / 2), int % 10u64.pow(n / 2)))
}
