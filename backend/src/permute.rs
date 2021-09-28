use std::fmt::Display;

use crate::{
    cube::{EdgeType, FaceType},
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cube::EdgeType::*;
    use crate::cube::FaceType::*;

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
        assert_eq!(e, g.op(g));
    }

    #[test]
    fn test_face_turn() {
        let turn_f = EdgePermutation::new_unchecked([
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
        ]);
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

        let turn_r = EdgePermutation::new_unchecked([
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
        ]);

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
    }
}
