use std::fmt::Display;

use crate::cube::CornerType::*;
use crate::cube::EdgeType::*;
use crate::cube::FaceType::*;
use crate::{
    cube::{Axis, CornerType, EdgeType, FaceType, Move, MoveSequence, RotationType},
    group::*,
};

/// Represents a centre piece of an odd-sized cube.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CentreCubelet(pub FaceType);

impl Display for CentreCubelet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Enumerable for CentreCubelet {
    const N: usize = FaceType::N;

    fn enumerate() -> [Self; Self::N] {
        FaceType::enumerate().map(CentreCubelet)
    }

    fn from_index(idx: usize) -> Self {
        CentreCubelet(FaceType::from_index(idx))
    }

    fn index(&self) -> usize {
        self.0.index()
    }
}

/// Represents one of 12 centred edge pieces of an odd-sized cube.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct EdgeCubelet(pub EdgeType);

impl Display for EdgeCubelet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Enumerable for EdgeCubelet {
    const N: usize = EdgeType::N;

    fn enumerate() -> [Self; Self::N] {
        EdgeType::enumerate().map(EdgeCubelet)
    }

    fn from_index(idx: usize) -> Self {
        EdgeCubelet(EdgeType::from_index(idx))
    }

    fn index(&self) -> usize {
        self.0.index()
    }
}

/// Represents one of 8 corner pieces of a cube.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CornerCubelet(pub CornerType);

impl Display for CornerCubelet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Enumerable for CornerCubelet {
    const N: usize = CornerType::N;

    fn enumerate() -> [Self; Self::N] {
        CornerType::enumerate().map(CornerCubelet)
    }

    fn from_index(idx: usize) -> Self {
        CornerCubelet(CornerType::from_index(idx))
    }

    fn index(&self) -> usize {
        self.0.index()
    }
}

/// Represents an element of the symmetric group of the centre pieces of a odd-sized cube.
/// Ignores centre orientation.
pub type CentrePermutation = SymmetricGroup<CentreCubelet>;

/// Represents an element of the symmetric group of the 12 centred edge pieces of an odd-sized cube.
/// Edges have a rotational cyclic group of order 2.
/// That is, in any position, an edge may be positioned in one of two orientations.
/// Orientation 0 is oriented "correctly", that is, the key sticker is on the correct face.
pub type EdgePermutation = OrientedSymmetricGroup<EdgeCubelet, 2>;

/// Represents an element of the symmetric group of the 8 corner pieces on a cube.
/// Corners have a rotational cyclic group of order 3.
/// That is, in any position, a corner may be positioned in one of three orientations.
/// Orientation 0 is oriented "correctly", that is, the U/D colour is on the U/D face.
/// Orientations 1, 2 are clockwise 120-degree and 240-degree turns.
pub type CornerPermutation = OrientedSymmetricGroup<CornerCubelet, 3>;

/// Represents a permutation of a 3x3x3 cube.
/// This is the direct product of a centre permutation, edge permutation, and corner permutation group.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CubePermutation3 {
    centres: CentrePermutation,
    edges: EdgePermutation,
    corners: CornerPermutation,
}

impl CentrePermutation {
    /// Slice turns are inferred from their axis.
    /// The turn is taken to be a *clockwise* turn about that axis.
    /// In particular,
    /// - `FB => S`
    /// - `RL => M'`
    /// - `UD => E'`
    pub fn from_normal_slice_turn(axis: Axis) -> Self {
        match axis {
            // Cycle U R D L
            Axis::FB => CentrePermutation::new_unchecked([
                CentreCubelet(F),
                CentreCubelet(D),
                CentreCubelet(R),
                CentreCubelet(B),
                CentreCubelet(U),
                CentreCubelet(L),
            ]),
            // Cycle U B D F
            Axis::RL => CentrePermutation::new_unchecked([
                CentreCubelet(U),
                CentreCubelet(R),
                CentreCubelet(B),
                CentreCubelet(D),
                CentreCubelet(L),
                CentreCubelet(F),
            ]),
            // Cycle F L B R
            Axis::UD => CentrePermutation::new_unchecked([
                CentreCubelet(L),
                CentreCubelet(F),
                CentreCubelet(U),
                CentreCubelet(R),
                CentreCubelet(B),
                CentreCubelet(D),
            ]),
        }
    }

    /// Slice turns are inferred from their axis.
    /// The turn is taken to be a *clockwise* turn about that axis.
    /// In particular,
    /// - `FB => S`
    /// - `RL => M'`
    /// - `UD => E'`
    pub fn from_slice_turn(axis: Axis, rotation_type: RotationType) -> Self {
        let s = Self::from_normal_slice_turn(axis);
        match rotation_type {
            RotationType::Normal => s,
            RotationType::Double => s.op(s),
            RotationType::Inverse => s.inverse(),
        }
    }
}

impl EdgePermutation {
    pub fn from_normal_face_turn(face: FaceType) -> Self {
        match face {
            // Cycle UF FR DF FL (and adjust parity)
            F => EdgePermutation::new_unchecked([
                (EdgeCubelet(UR), CyclicGroup::new(0)),
                (EdgeCubelet(FR), CyclicGroup::new(1)),
                (EdgeCubelet(UL), CyclicGroup::new(0)),
                (EdgeCubelet(UB), CyclicGroup::new(0)),
                (EdgeCubelet(DR), CyclicGroup::new(0)),
                (EdgeCubelet(FL), CyclicGroup::new(1)),
                (EdgeCubelet(DL), CyclicGroup::new(0)),
                (EdgeCubelet(DB), CyclicGroup::new(0)),
                (EdgeCubelet(DF), CyclicGroup::new(1)),
                (EdgeCubelet(UF), CyclicGroup::new(1)),
                (EdgeCubelet(BR), CyclicGroup::new(0)),
                (EdgeCubelet(BL), CyclicGroup::new(0)),
            ]),
            // Cycle UR BR DR FR
            R => EdgePermutation::new_unchecked([
                (EdgeCubelet(BR), CyclicGroup::new(0)),
                (EdgeCubelet(UF), CyclicGroup::new(0)),
                (EdgeCubelet(UL), CyclicGroup::new(0)),
                (EdgeCubelet(UB), CyclicGroup::new(0)),
                (EdgeCubelet(FR), CyclicGroup::new(0)),
                (EdgeCubelet(DF), CyclicGroup::new(0)),
                (EdgeCubelet(DL), CyclicGroup::new(0)),
                (EdgeCubelet(DB), CyclicGroup::new(0)),
                (EdgeCubelet(UR), CyclicGroup::new(0)),
                (EdgeCubelet(FL), CyclicGroup::new(0)),
                (EdgeCubelet(DR), CyclicGroup::new(0)),
                (EdgeCubelet(BL), CyclicGroup::new(0)),
            ]),
            // Cycle UR UF UL UB
            U => EdgePermutation::new_unchecked([
                (EdgeCubelet(UF), CyclicGroup::new(0)),
                (EdgeCubelet(UL), CyclicGroup::new(0)),
                (EdgeCubelet(UB), CyclicGroup::new(0)),
                (EdgeCubelet(UR), CyclicGroup::new(0)),
                (EdgeCubelet(DR), CyclicGroup::new(0)),
                (EdgeCubelet(DF), CyclicGroup::new(0)),
                (EdgeCubelet(DL), CyclicGroup::new(0)),
                (EdgeCubelet(DB), CyclicGroup::new(0)),
                (EdgeCubelet(FR), CyclicGroup::new(0)),
                (EdgeCubelet(FL), CyclicGroup::new(0)),
                (EdgeCubelet(BR), CyclicGroup::new(0)),
                (EdgeCubelet(BL), CyclicGroup::new(0)),
            ]),
            // Cycle UB BL DB BR (and adjust parity)
            B => EdgePermutation::new_unchecked([
                (EdgeCubelet(UR), CyclicGroup::new(0)),
                (EdgeCubelet(UF), CyclicGroup::new(0)),
                (EdgeCubelet(UL), CyclicGroup::new(0)),
                (EdgeCubelet(BL), CyclicGroup::new(1)),
                (EdgeCubelet(DR), CyclicGroup::new(0)),
                (EdgeCubelet(DF), CyclicGroup::new(0)),
                (EdgeCubelet(DL), CyclicGroup::new(0)),
                (EdgeCubelet(BR), CyclicGroup::new(1)),
                (EdgeCubelet(FR), CyclicGroup::new(0)),
                (EdgeCubelet(FL), CyclicGroup::new(0)),
                (EdgeCubelet(UB), CyclicGroup::new(1)),
                (EdgeCubelet(DB), CyclicGroup::new(1)),
            ]),
            // Cycle UL FL DL BL
            L => EdgePermutation::new_unchecked([
                (EdgeCubelet(UR), CyclicGroup::new(0)),
                (EdgeCubelet(UF), CyclicGroup::new(0)),
                (EdgeCubelet(FL), CyclicGroup::new(0)),
                (EdgeCubelet(UB), CyclicGroup::new(0)),
                (EdgeCubelet(DR), CyclicGroup::new(0)),
                (EdgeCubelet(DF), CyclicGroup::new(0)),
                (EdgeCubelet(BL), CyclicGroup::new(0)),
                (EdgeCubelet(DB), CyclicGroup::new(0)),
                (EdgeCubelet(FR), CyclicGroup::new(0)),
                (EdgeCubelet(DL), CyclicGroup::new(0)),
                (EdgeCubelet(BR), CyclicGroup::new(0)),
                (EdgeCubelet(UL), CyclicGroup::new(0)),
            ]),
            // Cycle DR DB DL DF
            D => EdgePermutation::new_unchecked([
                (EdgeCubelet(UR), CyclicGroup::new(0)),
                (EdgeCubelet(UF), CyclicGroup::new(0)),
                (EdgeCubelet(UL), CyclicGroup::new(0)),
                (EdgeCubelet(UB), CyclicGroup::new(0)),
                (EdgeCubelet(DB), CyclicGroup::new(0)),
                (EdgeCubelet(DR), CyclicGroup::new(0)),
                (EdgeCubelet(DF), CyclicGroup::new(0)),
                (EdgeCubelet(DL), CyclicGroup::new(0)),
                (EdgeCubelet(FR), CyclicGroup::new(0)),
                (EdgeCubelet(FL), CyclicGroup::new(0)),
                (EdgeCubelet(BR), CyclicGroup::new(0)),
                (EdgeCubelet(BL), CyclicGroup::new(0)),
            ]),
        }
    }

    /// TODO: Cache the results of this function if profiling indicates it is a hot path.
    /// For example, using the `memoize` crate.
    pub fn from_face_turn(face: FaceType, rotation_type: RotationType) -> Self {
        let s = Self::from_normal_face_turn(face);
        match rotation_type {
            RotationType::Normal => s,
            RotationType::Double => s.op(s),
            RotationType::Inverse => s.inverse(),
        }
    }

    /// Slice turns are inferred from their axis.
    /// The turn is taken to be a *clockwise* turn about that axis.
    /// In particular,
    /// - `FB => S`
    /// - `RL => M'`
    /// - `UD => E'`
    pub fn from_normal_slice_turn(axis: Axis) -> Self {
        match axis {
            // Cycle UL UR DR DL
            Axis::FB => EdgePermutation::new_unchecked([
                (EdgeCubelet(DR), CyclicGroup::new(1)),
                (EdgeCubelet(UF), CyclicGroup::new(0)),
                (EdgeCubelet(UR), CyclicGroup::new(1)),
                (EdgeCubelet(UB), CyclicGroup::new(0)),
                (EdgeCubelet(DL), CyclicGroup::new(1)),
                (EdgeCubelet(DF), CyclicGroup::new(0)),
                (EdgeCubelet(UL), CyclicGroup::new(1)),
                (EdgeCubelet(DB), CyclicGroup::new(0)),
                (EdgeCubelet(FR), CyclicGroup::new(0)),
                (EdgeCubelet(FL), CyclicGroup::new(0)),
                (EdgeCubelet(BR), CyclicGroup::new(0)),
                (EdgeCubelet(BL), CyclicGroup::new(0)),
            ]),
            // Cycle UF UB DB DF
            Axis::RL => EdgePermutation::new_unchecked([
                (EdgeCubelet(UR), CyclicGroup::new(0)),
                (EdgeCubelet(UB), CyclicGroup::new(1)),
                (EdgeCubelet(UL), CyclicGroup::new(0)),
                (EdgeCubelet(DB), CyclicGroup::new(1)),
                (EdgeCubelet(DR), CyclicGroup::new(0)),
                (EdgeCubelet(UF), CyclicGroup::new(1)),
                (EdgeCubelet(DL), CyclicGroup::new(0)),
                (EdgeCubelet(DF), CyclicGroup::new(1)),
                (EdgeCubelet(FR), CyclicGroup::new(0)),
                (EdgeCubelet(FL), CyclicGroup::new(0)),
                (EdgeCubelet(BR), CyclicGroup::new(0)),
                (EdgeCubelet(BL), CyclicGroup::new(0)),
            ]),
            // Cycle FR FL BL BR
            Axis::UD => EdgePermutation::new_unchecked([
                (EdgeCubelet(UR), CyclicGroup::new(0)),
                (EdgeCubelet(UF), CyclicGroup::new(0)),
                (EdgeCubelet(UL), CyclicGroup::new(0)),
                (EdgeCubelet(UB), CyclicGroup::new(0)),
                (EdgeCubelet(DR), CyclicGroup::new(0)),
                (EdgeCubelet(DF), CyclicGroup::new(0)),
                (EdgeCubelet(DL), CyclicGroup::new(0)),
                (EdgeCubelet(DB), CyclicGroup::new(0)),
                (EdgeCubelet(FL), CyclicGroup::new(1)),
                (EdgeCubelet(BL), CyclicGroup::new(1)),
                (EdgeCubelet(FR), CyclicGroup::new(1)),
                (EdgeCubelet(BR), CyclicGroup::new(1)),
            ]),
        }
    }

    /// Slice turns are inferred from their axis.
    /// The turn is taken to be a *clockwise* turn about that axis.
    /// In particular,
    /// - `FB => S`
    /// - `RL => M'`
    /// - `UD => E'`
    pub fn from_slice_turn(axis: Axis, rotation_type: RotationType) -> Self {
        let s = Self::from_normal_slice_turn(axis);
        match rotation_type {
            RotationType::Normal => s,
            RotationType::Double => s.op(s),
            RotationType::Inverse => s.inverse(),
        }
    }
}

impl CornerPermutation {
    pub fn from_normal_face_turn(face: FaceType) -> Self {
        match face {
            // Cycle FUL FUR FDR FDL
            F => CornerPermutation::new_unchecked([
                (CornerCubelet(FDR), CyclicGroup::new(2)),
                (CornerCubelet(FUR), CyclicGroup::new(1)),
                (CornerCubelet(FDL), CyclicGroup::new(1)),
                (CornerCubelet(FUL), CyclicGroup::new(2)),
                (CornerCubelet(BUR), CyclicGroup::new(0)),
                (CornerCubelet(BUL), CyclicGroup::new(0)),
                (CornerCubelet(BDR), CyclicGroup::new(0)),
                (CornerCubelet(BDL), CyclicGroup::new(0)),
            ]),
            // Cycle FUR BUR BDR FDR
            R => CornerPermutation::new_unchecked([
                (CornerCubelet(BUR), CyclicGroup::new(1)),
                (CornerCubelet(FUL), CyclicGroup::new(0)),
                (CornerCubelet(FUR), CyclicGroup::new(2)),
                (CornerCubelet(FDL), CyclicGroup::new(0)),
                (CornerCubelet(BDR), CyclicGroup::new(2)),
                (CornerCubelet(BUL), CyclicGroup::new(0)),
                (CornerCubelet(FDR), CyclicGroup::new(1)),
                (CornerCubelet(BDL), CyclicGroup::new(0)),
            ]),
            // Cycle FUR FUL BUL BUR
            U => CornerPermutation::new_unchecked([
                (CornerCubelet(FUL), CyclicGroup::new(0)),
                (CornerCubelet(BUL), CyclicGroup::new(0)),
                (CornerCubelet(FDR), CyclicGroup::new(0)),
                (CornerCubelet(FDL), CyclicGroup::new(0)),
                (CornerCubelet(FUR), CyclicGroup::new(0)),
                (CornerCubelet(BUR), CyclicGroup::new(0)),
                (CornerCubelet(BDR), CyclicGroup::new(0)),
                (CornerCubelet(BDL), CyclicGroup::new(0)),
            ]),
            // Cycle BUR BUL BDL BDR
            B => CornerPermutation::new_unchecked([
                (CornerCubelet(FUR), CyclicGroup::new(0)),
                (CornerCubelet(FUL), CyclicGroup::new(0)),
                (CornerCubelet(FDR), CyclicGroup::new(0)),
                (CornerCubelet(FDL), CyclicGroup::new(0)),
                (CornerCubelet(BUL), CyclicGroup::new(1)),
                (CornerCubelet(BDL), CyclicGroup::new(2)),
                (CornerCubelet(BUR), CyclicGroup::new(2)),
                (CornerCubelet(BDR), CyclicGroup::new(1)),
            ]),
            // Cycle BUL FUL FDL BDL
            L => CornerPermutation::new_unchecked([
                (CornerCubelet(FUR), CyclicGroup::new(0)),
                (CornerCubelet(FDL), CyclicGroup::new(2)),
                (CornerCubelet(FDR), CyclicGroup::new(0)),
                (CornerCubelet(BDL), CyclicGroup::new(1)),
                (CornerCubelet(BUR), CyclicGroup::new(0)),
                (CornerCubelet(FUL), CyclicGroup::new(1)),
                (CornerCubelet(BDR), CyclicGroup::new(0)),
                (CornerCubelet(BUL), CyclicGroup::new(2)),
            ]),
            // Cycle FDL FDR BDR BDL
            D => CornerPermutation::new_unchecked([
                (CornerCubelet(FUR), CyclicGroup::new(0)),
                (CornerCubelet(FUL), CyclicGroup::new(0)),
                (CornerCubelet(BDR), CyclicGroup::new(0)),
                (CornerCubelet(FDR), CyclicGroup::new(0)),
                (CornerCubelet(BUR), CyclicGroup::new(0)),
                (CornerCubelet(BUL), CyclicGroup::new(0)),
                (CornerCubelet(BDL), CyclicGroup::new(0)),
                (CornerCubelet(FDL), CyclicGroup::new(0)),
            ]),
        }
    }

    /// TODO: Cache the results of this function if profiling indicates it is a hot path.
    pub fn from_face_turn(face: FaceType, rotation_type: RotationType) -> Self {
        let s = Self::from_normal_face_turn(face);
        match rotation_type {
            RotationType::Normal => s,
            RotationType::Double => s.op(s),
            RotationType::Inverse => s.inverse(),
        }
    }
}

impl Magma for CubePermutation3 {
    fn op(self, other: Self) -> Self {
        Self {
            centres: self.centres.op(other.centres),
            edges: self.edges.op(other.edges),
            corners: self.corners.op(other.corners),
        }
    }
}

impl Semigroup for CubePermutation3 {}

impl InverseSemigroup for CubePermutation3 {
    fn inverse(&self) -> Self {
        Self {
            centres: self.centres.inverse(),
            edges: self.edges.inverse(),
            corners: self.corners.inverse(),
        }
    }
}

impl Unital for CubePermutation3 {
    fn identity() -> Self {
        Self {
            centres: CentrePermutation::identity(),
            edges: EdgePermutation::identity(),
            corners: CornerPermutation::identity(),
        }
    }
}

impl Display for CubePermutation3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = self.corners.to_string();
        let e = self.edges.to_string();
        let x = self.centres.to_string();

        // Concatenate each *line* of the strings together.
        for ((c_line, e_line), x_line) in c
            .lines()
            .zip(e.lines())
            .zip(x.lines().chain(std::iter::once("")))
        {
            writeln!(f, "{} {} {}", c_line, e_line, x_line)?;
        }

        Ok(())
    }
}

impl CubePermutation3 {
    pub fn from_face_turn(face: FaceType, rotation_type: RotationType) -> Self {
        Self {
            centres: CentrePermutation::identity(),
            edges: EdgePermutation::from_face_turn(face, rotation_type),
            corners: CornerPermutation::from_face_turn(face, rotation_type),
        }
    }

    /// Slice turns are inferred from their axis.
    /// The turn is taken to be a *clockwise* turn about that axis.
    /// In particular,
    /// - `FB => S`
    /// - `RL => M'`
    /// - `UD => E'`
    pub fn from_slice_turn(axis: Axis, rotation_type: RotationType) -> Self {
        Self {
            centres: CentrePermutation::from_slice_turn(axis, rotation_type),
            edges: EdgePermutation::from_slice_turn(axis, rotation_type),
            corners: CornerPermutation::identity(),
        }
    }

    pub fn from_move(mv: Move) -> Self {
        // Construct the move from commuting slice moves.
        let mut g = Self::identity();

        let front = match mv.axis {
            Axis::FB => F,
            Axis::RL => R,
            Axis::UD => U,
        };
        let back = match mv.axis {
            Axis::FB => B,
            Axis::RL => L,
            Axis::UD => D,
        };

        for i in mv.start_depth..mv.end_depth {
            let h = match i {
                0 => Self::from_face_turn(front, mv.rotation_type),
                1 => Self::from_slice_turn(mv.axis, mv.rotation_type),
                2 => Self::from_face_turn(back, mv.rotation_type.inverse()),
                _ => panic!("invalid move on a 3x3x3 cube: {:?}", mv),
            };
            g = g.op(h);
        }

        g
    }

    pub fn from_move_sequence(moves: MoveSequence) -> Self {
        let mut g = Self::identity();
        for mv in moves.moves.into_iter().rev() {
            g = g.op(Self::from_move(mv));
        }
        g
    }

    /// Get a reference to the cube permutation's centres.
    pub fn centres(&self) -> &CentrePermutation {
        &self.centres
    }

    /// Get a reference to the cube permutation's edges.
    pub fn edges(&self) -> &EdgePermutation {
        &self.edges
    }

    /// Get a reference to the cube permutation's corners.
    pub fn corners(&self) -> &CornerPermutation {
        &self.corners
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn group_operation() {
        let e = CentrePermutation::identity();
        println!("{}", e);
        let rf = CentrePermutation::new_unchecked([
            CentreCubelet(R),
            CentreCubelet(F),
            CentreCubelet(U),
            CentreCubelet(B),
            CentreCubelet(L),
            CentreCubelet(D),
        ]);
        let uf = CentrePermutation::new_unchecked([
            CentreCubelet(U),
            CentreCubelet(R),
            CentreCubelet(F),
            CentreCubelet(B),
            CentreCubelet(L),
            CentreCubelet(D),
        ]);
        let ruf = CentrePermutation::new_unchecked([
            CentreCubelet(R),
            CentreCubelet(U),
            CentreCubelet(F),
            CentreCubelet(B),
            CentreCubelet(L),
            CentreCubelet(D),
        ]);
        let urf = CentrePermutation::new_unchecked([
            CentreCubelet(U),
            CentreCubelet(F),
            CentreCubelet(R),
            CentreCubelet(B),
            CentreCubelet(L),
            CentreCubelet(D),
        ]);
        assert_eq!(e, rf.op(rf));
        assert_eq!(e.op(rf), rf.op(e));
        assert_eq!(e, e.op(e));
        assert_eq!(ruf, uf.op(rf));
        assert_eq!(urf, rf.op(uf));
    }

    #[test]
    fn edge_permutation() {
        let e = EdgePermutation::identity();
        let g = EdgePermutation::new_unchecked([
            (EdgeCubelet(UF), CyclicGroup::new(0)),
            (EdgeCubelet(UR), CyclicGroup::new(0)),
            (EdgeCubelet(UL), CyclicGroup::new(0)),
            (EdgeCubelet(UB), CyclicGroup::new(0)),
            (EdgeCubelet(DR), CyclicGroup::new(0)),
            (EdgeCubelet(DF), CyclicGroup::new(0)),
            (EdgeCubelet(DL), CyclicGroup::new(1)),
            (EdgeCubelet(DB), CyclicGroup::new(0)),
            (EdgeCubelet(FR), CyclicGroup::new(0)),
            (EdgeCubelet(FL), CyclicGroup::new(0)),
            (EdgeCubelet(BR), CyclicGroup::new(0)),
            (EdgeCubelet(BL), CyclicGroup::new(0)),
        ]);
        assert_eq!(e.order(), 1);
        assert_eq!(e, g.op(g));
    }

    #[test]
    fn face_turn() {
        let turn_f = EdgePermutation::from_face_turn(F, RotationType::Normal);
        let turn_f2 = EdgePermutation::new_unchecked([
            (EdgeCubelet(UR), CyclicGroup::new(0)),
            (EdgeCubelet(DF), CyclicGroup::new(0)),
            (EdgeCubelet(UL), CyclicGroup::new(0)),
            (EdgeCubelet(UB), CyclicGroup::new(0)),
            (EdgeCubelet(DR), CyclicGroup::new(0)),
            (EdgeCubelet(UF), CyclicGroup::new(0)),
            (EdgeCubelet(DL), CyclicGroup::new(0)),
            (EdgeCubelet(DB), CyclicGroup::new(0)),
            (EdgeCubelet(FL), CyclicGroup::new(0)),
            (EdgeCubelet(FR), CyclicGroup::new(0)),
            (EdgeCubelet(BR), CyclicGroup::new(0)),
            (EdgeCubelet(BL), CyclicGroup::new(0)),
        ]);

        assert_eq!(turn_f2, turn_f.op(turn_f));
        assert_eq!(turn_f.inverse(), turn_f.op(turn_f).op(turn_f));

        let turn_r = EdgePermutation::from_face_turn(R, RotationType::Normal);

        // Order is reversed to speedcubing notation!
        let rf = turn_f.op(turn_r);
        let rf_manual = EdgePermutation::new_unchecked([
            (EdgeCubelet(BR), CyclicGroup::new(0)),
            (EdgeCubelet(FR), CyclicGroup::new(1)),
            (EdgeCubelet(UL), CyclicGroup::new(0)),
            (EdgeCubelet(UB), CyclicGroup::new(0)),
            (EdgeCubelet(DF), CyclicGroup::new(1)),
            (EdgeCubelet(FL), CyclicGroup::new(1)),
            (EdgeCubelet(DL), CyclicGroup::new(0)),
            (EdgeCubelet(DB), CyclicGroup::new(0)),
            (EdgeCubelet(UR), CyclicGroup::new(0)),
            (EdgeCubelet(UF), CyclicGroup::new(1)),
            (EdgeCubelet(DR), CyclicGroup::new(0)),
            (EdgeCubelet(BL), CyclicGroup::new(0)),
        ]);
        assert_eq!(rf, rf_manual);

        let fr = turn_r.op(turn_f);
        let fr_manual = EdgePermutation::new_unchecked([
            (EdgeCubelet(BR), CyclicGroup::new(0)),
            (EdgeCubelet(UR), CyclicGroup::new(1)),
            (EdgeCubelet(UL), CyclicGroup::new(0)),
            (EdgeCubelet(UB), CyclicGroup::new(0)),
            (EdgeCubelet(FR), CyclicGroup::new(0)),
            (EdgeCubelet(FL), CyclicGroup::new(1)),
            (EdgeCubelet(DL), CyclicGroup::new(0)),
            (EdgeCubelet(DB), CyclicGroup::new(0)),
            (EdgeCubelet(DF), CyclicGroup::new(1)),
            (EdgeCubelet(UF), CyclicGroup::new(1)),
            (EdgeCubelet(DR), CyclicGroup::new(0)),
            (EdgeCubelet(BL), CyclicGroup::new(0)),
        ]);
        assert_eq!(fr, fr_manual);

        assert_eq!(turn_f.order(), 4);
        assert_eq!(turn_r.order(), 4);
        // Surprisingly, the move sequence RF has order 7 on edges.
        assert_eq!(rf.order(), 7);
        assert_eq!(fr.order(), 7);
    }

    #[test]
    fn u_perm() {
        // R' U R' U' R' U' R' U R U R2 is a U permutation.
        let moves = [
            EdgePermutation::from_face_turn(R, RotationType::Inverse),
            EdgePermutation::from_face_turn(U, RotationType::Normal),
            EdgePermutation::from_face_turn(R, RotationType::Inverse),
            EdgePermutation::from_face_turn(U, RotationType::Inverse),
            EdgePermutation::from_face_turn(R, RotationType::Inverse),
            EdgePermutation::from_face_turn(U, RotationType::Inverse),
            EdgePermutation::from_face_turn(R, RotationType::Inverse),
            EdgePermutation::from_face_turn(U, RotationType::Normal),
            EdgePermutation::from_face_turn(R, RotationType::Normal),
            EdgePermutation::from_face_turn(U, RotationType::Normal),
            EdgePermutation::from_face_turn(R, RotationType::Double),
        ];
        let mut operation = EdgePermutation::identity();
        for mv in moves.into_iter().rev() {
            operation = operation.op(mv);
        }
        // Thus, it should have order 3.
        assert_eq!(operation.order(), 3);
        // It should also be the 3-cycle (UR UL UB).
        assert_eq!(
            operation.act(&(EdgeCubelet(UR), CyclicGroup::new(0))),
            (EdgeCubelet(UL), CyclicGroup::new(0))
        );
        assert_eq!(
            operation.act(&(EdgeCubelet(UL), CyclicGroup::new(0))),
            (EdgeCubelet(UB), CyclicGroup::new(0))
        );
        assert_eq!(
            operation.act(&(EdgeCubelet(UB), CyclicGroup::new(0))),
            (EdgeCubelet(UR), CyclicGroup::new(0))
        );
    }

    #[test]
    fn a_perm() {
        // L2 D2 L' U' L D2 L' U L' is an A permutation.
        let moves = [
            CornerPermutation::from_face_turn(L, RotationType::Double),
            CornerPermutation::from_face_turn(D, RotationType::Double),
            CornerPermutation::from_face_turn(L, RotationType::Inverse),
            CornerPermutation::from_face_turn(U, RotationType::Inverse),
            CornerPermutation::from_face_turn(L, RotationType::Normal),
            CornerPermutation::from_face_turn(D, RotationType::Double),
            CornerPermutation::from_face_turn(L, RotationType::Inverse),
            CornerPermutation::from_face_turn(U, RotationType::Normal),
            CornerPermutation::from_face_turn(L, RotationType::Inverse),
        ];
        let mut operation = CornerPermutation::identity();
        for mv in moves.into_iter().rev() {
            operation = operation.op(mv);
        }
        // Thus, it should have order 3.
        assert_eq!(operation.order(), 3);
        // It should also be the 3-cycle (BDL BDR BUL).
        assert_eq!(
            operation.act(&(CornerCubelet(BDL), CyclicGroup::new(0))),
            (CornerCubelet(BDR), CyclicGroup::new(1))
        );
        assert_eq!(
            operation.act(&(CornerCubelet(BDR), CyclicGroup::new(0))),
            (CornerCubelet(BUL), CyclicGroup::new(0))
        );
        assert_eq!(
            operation.act(&(CornerCubelet(BUL), CyclicGroup::new(0))),
            (CornerCubelet(BDL), CyclicGroup::new(2))
        );
    }

    #[test]
    fn h_perm() {
        let m2 = CubePermutation3::from_slice_turn(Axis::RL, RotationType::Double);
        let u = CubePermutation3::from_face_turn(U, RotationType::Normal);
        let h = m2.op(u).op(m2).op(u).op(u).op(m2).op(u).op(m2);
        assert_eq!(h.order(), 2);
    }

    #[test]
    fn alg_parsing() {
        // The superflip flips every edge on the cube.
        // Therefore, flipping each edge twice should be a no-op.
        let superflip = "U R2 F B R B2 R U2 L B2 R U' D' R2 F R' L B2 U2 F2"
            .parse::<MoveSequence>()
            .unwrap();
        let g = CubePermutation3::from_move_sequence(superflip);
        assert_eq!(g.order(), 2);
    }
}
