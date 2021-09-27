use std::fmt::Display;

use crate::{cube::FaceType, group::*};

/// Represents a centre piece of an odd-sized cube.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CentreCubelet(FaceType);

impl Display for CentreCubelet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Enumerable for CentreCubelet {
    const N: usize = 6;

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

/// Represents an element of the symmetric group of the centre pieces of a odd-sized cube.
/// Ignores centre orientation.
pub type CentrePermutation = SymmetricGroup<CentreCubelet>;

#[cfg(test)]
mod tests {
    use super::CentrePermutation;
    use crate::cube::FaceType::*;
    use crate::{group::*, permute::CentreCubelet};

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
        println!("{}", rf);
        assert_eq!(e, rf.op(rf));
        assert_eq!(e.op(rf), rf.op(e));
        assert_eq!(e, e.op(e));
    }
}
