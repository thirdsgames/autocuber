use std::fmt::{Debug, Display};

pub trait Magma {
    /// Apply the magma operation.
    fn op(self, other: Self) -> Self;
}

/// A marker trait denoting associativity on a magma.
pub trait Semigroup: Magma {}

/// Provides the identity element of a magma.
pub trait Unital: Magma {
    fn identity() -> Self;
}

/// Provides the (unique, left and right) inverse of a magma.
pub trait InverseSemigroup: Semigroup {
    fn inverse(&self) -> Self;
}

pub trait Group: InverseSemigroup + Unital {}
impl<G: InverseSemigroup + Unital> Group for G {}

/// A group can act on a set S.
pub trait GroupAction<S>: Group + Sized {
    fn act(&self, s: &S) -> S;

    fn unact(&self, s: &S) -> S {
        self.inverse().act(s)
    }
}

pub trait Enumerable: Sized {
    const N: usize;

    fn enumerate() -> [Self; Self::N];
    fn from_index(idx: usize) -> Self;
    fn index(&self) -> usize;
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct SymmetricGroup<S>
where
    S: Enumerable,
    // The following constraint comes from this issue:
    // <https://github.com/rust-lang/rust/issues/76560>
    // TODO report the ICE that happens when `[(); S::N]: Sized` is used everywhere after finding MCVE
    [(); S::N]: ,
{
    /// The value at index `i` is the image under the map of `i`,
    /// where `i` is the `i`th element of `S::enumerate()`.
    map: [S; S::N],
}

impl<S> SymmetricGroup<S>
where
    S: Enumerable,
    [(); S::N]: ,
{
    /// Does not check that the group axioms are upheld by this map.
    pub fn new_unchecked(map: [S; S::N]) -> Self {
        Self { map }
    }
}

impl<S> Default for SymmetricGroup<S>
where
    S: Enumerable,
    [(); S::N]: ,
{
    fn default() -> Self {
        // Return the identity map.
        Self {
            map: S::enumerate(),
        }
    }
}

/// Multiplies two permutations.
impl<S> Magma for SymmetricGroup<S>
where
    S: Enumerable + Clone,
    [(); S::N]: ,
{
    fn op(self, other: Self) -> Self {
        Self {
            map: other.map.map(|x| self.map[x.index()].clone()),
        }
    }
}

impl<S> Unital for SymmetricGroup<S>
where
    S: Enumerable + Clone,
    [(); S::N]: ,
{
    fn identity() -> Self {
        Self::default()
    }
}

impl<S> Semigroup for SymmetricGroup<S>
where
    S: Enumerable + Clone,
    [(); S::N]: ,
{
}

impl<S> InverseSemigroup for SymmetricGroup<S>
where
    S: Enumerable + Clone,
    [(); S::N]: ,
{
    fn inverse(&self) -> Self {
        let mut map = std::mem::MaybeUninit::<S>::uninit_array::<{ S::N }>();
        for i in 0..S::N {
            map[self.map[i].index()].write(S::from_index(i));
        }
        Self {
            map: unsafe { std::mem::transmute_copy(&map) },
        }
    }
}

impl<S> GroupAction<S> for SymmetricGroup<S>
where
    S: Enumerable + Clone,
    [(); S::N]: ,
{
    fn act(&self, s: &S) -> S {
        self.map[s.index()].clone()
    }
}

impl<S> Display for SymmetricGroup<S>
where
    S: Enumerable + Clone + Display,
    [(); S::N]: ,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\u{250c}")?;
        for s in S::enumerate() {
            write!(f, " {}", s)?;
        }
        writeln!(f, " \u{2510}")?;
        write!(f, "\u{2514}")?;
        for s in S::enumerate() {
            write!(f, " {}", self.act(&s))?;
        }
        writeln!(f, " \u{2518}")
    }
}

impl<S> Debug for SymmetricGroup<S>
where
    S: Enumerable + Clone + Display + Debug,
    [(); S::N]: ,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SymmetricGroup")
            .field("map", &self.map)
            .finish()
    }
}
