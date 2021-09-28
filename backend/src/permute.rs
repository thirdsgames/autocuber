use std::fmt::Display;

use crate::cube::EdgeType::*;
use crate::cube::FaceType::*;
use crate::{
    cube::{EdgeType, FaceType, RotationType},
    group::*,
};

/// Represents a centre piece of an odd-sized cube.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CentreCubelet(FaceType);

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
pub struct EdgeCubelet(EdgeType);

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

/// Represents an element of the symmetric group of the centre pieces of a odd-sized cube.
/// Ignores centre orientation.
pub type CentrePermutation = SymmetricGroup<CentreCubelet>;

/// Represents an element of the symmetric group of the 12 centred edge pieces of an odd-sized cube.
/// Edges have a rotational cyclic group of order 2.
/// That is, in any position, an edge may be positioned in one of two orientations.
pub type EdgePermutation = OrientedSymmetricGroup<EdgeCubelet, 2>;

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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_operation() {
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
    fn test_edge_permutation() {
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
    fn test_face_turn() {
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
    fn test_alg() {
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
}
