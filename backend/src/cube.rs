use std::{fmt::Display, ops::Index, str::FromStr};

/// Represents a *valid* (i.e. has all of the required pieces, not necessarily solvable) NxN cube.
/// Not `Copy` primarily as a lint.
#[derive(Debug, Clone)]
pub struct Cube<const N: usize> {
    /// Faces of the cube, ordered F R U B L D.
    faces: [Face<N>; 6],
}

/// A face of an NxN cube.
/// Not `Copy` primarily as a lint.
#[derive(Debug, Clone)]
pub struct Face<const N: usize> {
    rows: [[Colour; N]; N],
}

/// The colour of a face on an NxN cube.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Colour {
    Green,
    Red,
    White,
    Blue,
    Orange,
    Yellow,
}

impl Colour {
    /// Gets the letter name of this colour.
    pub fn letter(self) -> char {
        match self {
            Colour::Green => 'g',
            Colour::Red => 'r',
            Colour::White => 'w',
            Colour::Blue => 'b',
            Colour::Orange => 'o',
            Colour::Yellow => 'y',
        }
    }
}

/// A face on a cube.
/// Represented in Singmaster notation.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum FaceType {
    F,
    R,
    U,
    B,
    L,
    D,
}
use FaceType::*;

impl FromStr for FaceType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "F" => Ok(F),
            "R" => Ok(R),
            "U" => Ok(U),
            "B" => Ok(B),
            "L" => Ok(L),
            "D" => Ok(D),
            _ => Err(()),
        }
    }
}

/// These impls are safe since colour and face type are `repr(u8)` and have the same possible discriminants.
impl From<FaceType> for Colour {
    fn from(face: FaceType) -> Self {
        unsafe { std::mem::transmute(face) }
    }
}
impl From<Colour> for FaceType {
    fn from(colour: Colour) -> Self {
        unsafe { std::mem::transmute(colour) }
    }
}

#[derive(Debug)]
pub enum RotationType {
    Normal,
    Double,
    Inverse,
}

#[derive(Debug)]
pub enum Move {
    Face {
        face: FaceType,
        rotation_type: RotationType,
        // We turn all slices from `start_depth` to `end_depth`.
        // If `start_depth = 0, end_depth = 1`, this is a normal turn.
        // If `start_depth = 1, end_depth = 2`, this is a slice turn.
        // If `start_depth = 0, end_depth = 2`, this is a wide turn.
        start_depth: usize,
        end_depth: usize,
    },
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        let face_char = chars.next().ok_or(())?;
        let turn_direction = match face_char {
            'M' => 'L',
            'E' => 'D',
            'S' => 'F',
            x => x,
        };
        let face: FaceType = turn_direction.to_uppercase().collect::<String>().parse()?;
        let mut end_depth = if face_char.is_lowercase() { 2 } else { 1 };
        let start_depth = match face_char {
            'M' | 'E' | 'S' => {
                end_depth = 2;
                1
            }
            _ => 0,
        };
        let mut rotation_type = RotationType::Normal;
        for modification in chars {
            match modification {
                'w' => end_depth = 2,
                '2' => rotation_type = RotationType::Double,
                '\'' => rotation_type = RotationType::Inverse,
                _ => return Err(()),
            }
        }
        Ok(Self::Face {
            face,
            rotation_type,
            start_depth,
            end_depth,
        })
    }
}

pub struct Algorithm {
    pub moves: Vec<Move>,
}

impl FromStr for Algorithm {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Self { moves: Vec::new() };
        for value in s.split(' ') {
            result.moves.push(value.parse()?);
        }
        Ok(result)
    }
}

impl<const N: usize> Cube<N> {
    pub fn new() -> Self {
        Self {
            faces: [
                Face::new(F),
                Face::new(R),
                Face::new(U),
                Face::new(B),
                Face::new(L),
                Face::new(D),
            ],
        }
    }

    pub fn face(&self, ty: FaceType) -> &Face<N> {
        &self.faces[ty as usize]
    }

    /// Asserts that this cube is valid.
    pub fn validate(&self) {}

    pub fn perform(self, mv: Move) -> Self {
        // Heavily optimised move-performing logic.
        macro_rules! face {
            ( $start_depth:ident, $end_depth:ident, ($($x:tt)*) ) => {
                // Unbox parentheses.
                face!($start_depth, $end_depth, $($x)*)
            };
            ( $start_depth:ident, $end_depth:ident, $face:ident ) => {
                self.face($face).clone()
            };
            ( $start_depth:ident, $end_depth:ident, $face:ident cw ) => {
                if $start_depth == 0 {
                    self.face($face).rotate_cw()
                } else {
                    self.face($face).clone()
                }
            };
            ( $start_depth:ident, $end_depth:ident, $face:ident 2 ) => {
                if $start_depth == 0 {
                    self.face($face).rotate_double()
                } else {
                    self.face($face).clone()
                }
            };
            ( $start_depth:ident, $end_depth:ident, $face:ident ccw ) => {
                if $start_depth == 0 {
                    self.face($face).rotate_ccw()
                } else {
                    self.face($face).clone()
                }
            };
            ( $start_depth:ident, $end_depth:ident, $face:ident $target:ident $source_face:ident $source_type:ident ) => {
                self.face($face).overwrite_from(
                    $start_depth,
                    $end_depth,
                    $target,
                    self.face($source_face),
                    $source_type,
                )
            };
        }

        macro_rules! perform {
            ( $start_depth:ident, $end_depth:ident, $($x:tt)* ) => {
                [$(face!($start_depth, $end_depth, $x),)*]
            };
        }

        Self {
            faces: match mv {
                // F
                Move::Face {
                    face: F,
                    rotation_type: RotationType::Normal,
                    start_depth,
                    end_depth,
                } => perform!(start_depth, end_depth,
                    // Read this:
                    // "F clockwise"
                    (F cw)
                    // "R left comes from U bottom"
                    // (the left part of R's face is copied from the bottom part of U's face)
                    (R Left U Bottom)
                    (U Bottom L Right)
                    // "B is unchanged"
                    (B)
                    (L Right D Top)
                    (D Top R Left)
                ),
                Move::Face {
                    face: F,
                    rotation_type: RotationType::Double,
                    start_depth,
                    end_depth,
                } => perform!(start_depth, end_depth,
                    (F 2)
                    (R Left L Right)
                    (U Bottom D Top)
                    (B)
                    (L Right R Left)
                    (D Top U Bottom)
                ),
                Move::Face {
                    face: F,
                    rotation_type: RotationType::Inverse,
                    start_depth,
                    end_depth,
                } => perform!(start_depth, end_depth,
                    (F ccw)
                    (R Left D Top)
                    (U Bottom R Left)
                    (B)
                    (L Right U Bottom)
                    (D Top L Right)
                ),
                // R
                Move::Face {
                    face: R,
                    rotation_type: RotationType::Normal,
                    start_depth,
                    end_depth,
                } => perform!(start_depth, end_depth,
                    (F Right D Right)
                    (R cw)
                    (U Right F Right)
                    (B Left U Right)
                    (L)
                    (D Right B Left)
                ),
                Move::Face {
                    face: R,
                    rotation_type: RotationType::Double,
                    start_depth,
                    end_depth,
                } => perform!(start_depth, end_depth,
                    (F Right B Left)
                    (R 2)
                    (U Right D Right)
                    (B Left F Right)
                    (L)
                    (D Right U Right)
                ),
                Move::Face {
                    face: R,
                    rotation_type: RotationType::Inverse,
                    start_depth,
                    end_depth,
                } => perform!(start_depth, end_depth,
                    (F Right U Right)
                    (R ccw)
                    (U Right B Left)
                    (B Left D Right)
                    (L)
                    (D Right F Right)
                ),
                _ => panic!("move {:#?} not recognised", mv),
            },
        }
    }
}

impl<const N: usize> Display for Cube<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Write the U face.
        for i in 0..N {
            // Write each row.
            for _ in 0..N {
                // Add a gap at the start for the L face.
                write!(f, "  ")?;
            }
            // Display the row.
            for j in 0..N {
                write!(f, "{} ", self.face(U)[(i, j)].letter())?;
            }
            writeln!(f)?;
        }

        // Write the L, F, R, B faces.
        for i in 0..N {
            for face in [L, F, R, B] {
                for j in 0..N {
                    write!(f, "{} ", self.face(face)[(i, j)].letter())?;
                }
            }
            writeln!(f)?;
        }

        // Write the D face.
        for i in 0..N {
            // Write each row.
            for _ in 0..N {
                // Add a gap at the start for the L face.
                write!(f, "  ")?;
            }
            // Display the row.
            for j in 0..N {
                write!(f, "{} ", self.face(D)[(i, j)].letter())?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

#[derive(Clone, Copy)]
enum FaceSegment {
    Top,
    Right,
    Bottom,
    Left,
}
use FaceSegment::*;

impl<const N: usize> Face<N> {
    pub fn new(ty: FaceType) -> Self {
        Self {
            rows: [[ty.into(); N]; N],
        }
    }

    fn row(&self, row: usize) -> [Colour; N] {
        self.rows[row]
    }

    fn row_rev(&self, row: usize) -> [Colour; N] {
        let mut array: [_; N] = std::mem::MaybeUninit::uninit_array();
        for i in 0..N {
            array[i].write(self[(row, N - 1 - i)]);
        }
        unsafe { std::mem::transmute_copy(&array) }
    }

    fn col(&self, col: usize) -> [Colour; N] {
        let mut array: [_; N] = std::mem::MaybeUninit::uninit_array();
        for i in 0..N {
            array[i].write(self[(i, col)]);
        }
        unsafe { std::mem::transmute_copy(&array) }
    }

    fn col_rev(&self, col: usize) -> [Colour; N] {
        let mut array: [_; N] = std::mem::MaybeUninit::uninit_array();
        for i in 0..N {
            array[i].write(self[(N - 1 - i, col)]);
        }
        unsafe { std::mem::transmute_copy(&array) }
    }

    fn rotate_cw(&self) -> Self {
        let mut array: [_; N] = std::mem::MaybeUninit::uninit_array();
        for i in 0..N {
            array[i].write(self.col_rev(i));
        }
        Self {
            rows: unsafe { std::mem::transmute_copy(&array) },
        }
    }

    fn rotate_ccw(&self) -> Self {
        let mut array: [_; N] = std::mem::MaybeUninit::uninit_array();
        for i in 0..N {
            array[i].write(self.col(N - 1 - i));
        }
        Self {
            rows: unsafe { std::mem::transmute_copy(&array) },
        }
    }

    fn rotate_double(&self) -> Self {
        let mut array: [_; N] = std::mem::MaybeUninit::uninit_array();
        for i in 0..N {
            array[i].write(self.row_rev(N - 1 - i));
        }
        Self {
            rows: unsafe { std::mem::transmute_copy(&array) },
        }
    }

    fn set_row(&mut self, row: usize, data: [Colour; N]) {
        self.rows[row] = data;
    }

    fn set_col(&mut self, col: usize, data: [Colour; N]) {
        for i in 0..N {
            self.rows[i][col] = data[i];
        }
    }

    /// Read this function:
    /// "overwrite \[depth\] slices on the \[target_type\] from \[source\]'s \[source_type\]"
    #[inline(always)]
    fn overwrite_from(
        &self,
        start_depth: usize,
        end_depth: usize,
        target_type: FaceSegment,
        source: &Face<N>,
        source_type: FaceSegment,
    ) -> Self {
        // Considering the face segments on the source and the target,
        // when we collect an individual row or column from the source,
        // we might need to flip it such that its image on the target is correctly oriented.

        // The source/target is said to go "clockwise" if the row/column index increases as we rotate clockwise around the given face.
        let source_clockwise = matches!(source_type, Top | Right);
        let target_clockwise = matches!(target_type, Top | Right);
        // If the source and target's orientations differ, we must reverse the indices of each element in the source,
        // that is, reverse the row or column itself.
        let reverse_direction = source_clockwise != target_clockwise;

        let mut face = self.clone();
        // i counts from left to right.
        for i in start_depth..end_depth {
            // j counts from right to left.
            let j = N - 1 - i;
            let source_row = match (source_type, reverse_direction) {
                (Top, false) => source.row(i),
                (Top, true) => source.row_rev(i),
                (Right, false) => source.col(j),
                (Right, true) => source.col_rev(j),
                (Bottom, false) => source.row(j),
                (Bottom, true) => source.row_rev(j),
                (Left, false) => source.col(i),
                (Left, true) => source.col_rev(i),
            };

            match target_type {
                Top => face.set_row(i, source_row),
                Right => face.set_col(j, source_row),
                Bottom => face.set_row(j, source_row),
                Left => face.set_col(i, source_row),
            };
        }
        face
    }
}

impl<const N: usize> Index<(usize, usize)> for Face<N> {
    type Output = Colour;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.rows[row][col]
    }
}
