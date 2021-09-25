use std::{fmt::Display, ops::Index};

/// Represents an NxN cube.
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

/// These impls are safe since colour and face type are `repr(u8)`.
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

impl<const N: usize> Cube<N> {
    pub fn new() -> Self {
        Self {
            faces: [
                Face::new(FaceType::F),
                Face::new(FaceType::R),
                Face::new(FaceType::U),
                Face::new(FaceType::B),
                Face::new(FaceType::L),
                Face::new(FaceType::D),
            ],
        }
    }

    pub fn face(&self, ty: FaceType) -> &Face<N> {
        &self.faces[ty as usize]
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
                write!(f, "{} ", self.face(FaceType::U)[(i, j)].letter())?;
            }
            writeln!(f)?;
        }

        // Write the L, F, R, B faces.
        for i in 0..N {
            for face in [FaceType::L, FaceType::F, FaceType::R, FaceType::B] {
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
                write!(f, "{} ", self.face(FaceType::D)[(i, j)].letter())?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl<const N: usize> Face<N> {
    pub fn new(ty: FaceType) -> Self {
        Self {
            rows: [[ty.into(); N]; N],
        }
    }
}

impl<const N: usize> Index<(usize, usize)> for Face<N> {
    type Output = Colour;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.rows[row][col]
    }
}
