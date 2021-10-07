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

/// Groups must support Eq since we often need to compare elements.
pub trait Group: InverseSemigroup + Unital + Eq + Clone + Sized {
    /// Returns the order of an element in a group.
    fn order(&self) -> usize {
        let mut x = self.clone();
        let mut i = 1;
        let e = Self::identity();
        while x != e {
            x = x.op(self.clone());
            i += 1;
        }
        i
    }
}
impl<G: InverseSemigroup + Unital + Eq + Clone + Sized> Group for G {}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TrivialGroup;

impl Magma for TrivialGroup {
    fn op(self, _: Self) -> Self {
        Self
    }
}

impl Semigroup for TrivialGroup {}

impl InverseSemigroup for TrivialGroup {
    fn inverse(&self) -> Self {
        Self
    }
}

impl Unital for TrivialGroup {
    fn identity() -> Self {
        Self
    }
}

impl Default for TrivialGroup {
    fn default() -> Self {
        Self
    }
}

/// Represents integers under addition, using modular arithmetic.
/// In particular, this is the group `Z/kZ`.
///
/// Cube symmetric groups are denoted by the symmetry of the positions,
/// as well as the rotation that each piece experiences.
/// The rotation is denoted with an element of this cyclic group.
///
/// The inner value is the index of element in the cyclic group.
/// The element is constrained to the range `0..K`.
///
/// `Ord` is not derived, since the elements of the cyclic group form a circle.
/// Therefore, there is no well-defined partial order on the elements.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub struct CyclicGroup<const K: u8>(u8);

impl<const K: u8> CyclicGroup<K> {
    pub fn new(value: u8) -> Self {
        Self(value % K)
    }

    pub fn get_value(self) -> u8 {
        self.0
    }
}

impl<const K: u8> Magma for CyclicGroup<K> {
    fn op(self, other: Self) -> Self {
        Self::new(self.0 + other.0)
    }
}

impl<const K: u8> Semigroup for CyclicGroup<K> {}

impl<const K: u8> InverseSemigroup for CyclicGroup<K> {
    fn inverse(&self) -> Self {
        Self::new(K - self.get_value())
    }
}

impl<const K: u8> Unital for CyclicGroup<K> {
    fn identity() -> Self {
        Self(0)
    }
}

impl<const K: u8> Display for CyclicGroup<K> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

/// Represents an element from the symmetric group on `S`.
///
/// Note that this symmetric group acts like a group, that is, in cycle notation,
/// `(a b) (b c) = (a b c)` - the symmetries act in reverse order.
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct SymmetricGroup<S>
where
    S: Enumerable,
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
    S: Enumerable + Clone + Eq,
    [(); S::N]: ,
{
    fn act(&self, s: &S) -> S {
        self.map[s.index()].clone()
    }
}

impl<S> Display for SymmetricGroup<S>
where
    S: Enumerable + Clone + Display + Eq,
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

/// Represents an element from the symmetric group on `S`,
/// but where each element may have an orientation which is an element of the cyclic group of order `K`.
///
/// Note that this symmetric group acts like a group, that is, in cycle notation,
/// `(a b) (b c) = (a b c)` - the symmetries act in reverse order.
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct OrientedSymmetricGroup<S, const K: u8>
where
    S: Enumerable,
    [(); S::N]: ,
{
    /// The value at index `i` is the image under the map of `i`,
    /// where `i` is the `i`th element of `S::enumerate()`.
    map: [(S, CyclicGroup<K>); S::N],
}

impl<S, const K: u8> OrientedSymmetricGroup<S, K>
where
    S: Enumerable,
    [(); S::N]: ,
{
    /// Does not check that the group axioms are upheld by this map.
    pub fn new_unchecked(map: [(S, CyclicGroup<K>); S::N]) -> Self {
        Self { map }
    }
}

impl<S, const K: u8> Default for OrientedSymmetricGroup<S, K>
where
    S: Enumerable,
    [(); S::N]: ,
{
    fn default() -> Self {
        // Return the identity map.
        Self {
            map: S::enumerate().map(|s| (s, CyclicGroup::identity())),
        }
    }
}

/// Multiplies two permutations.
impl<S, const K: u8> Magma for OrientedSymmetricGroup<S, K>
where
    S: Enumerable + Clone,
    [(); S::N]: ,
{
    fn op(self, other: Self) -> Self {
        Self {
            map: other.map.map(|(s, r)| {
                let (s2, r2) = self.map[s.index()].clone();
                (s2, r.op(r2))
            }),
        }
    }
}

impl<S, const K: u8> Unital for OrientedSymmetricGroup<S, K>
where
    S: Enumerable + Clone,
    [(); S::N]: ,
{
    fn identity() -> Self {
        Self::default()
    }
}

impl<S, const K: u8> Semigroup for OrientedSymmetricGroup<S, K>
where
    S: Enumerable + Clone,
    [(); S::N]: ,
{
}

impl<S, const K: u8> InverseSemigroup for OrientedSymmetricGroup<S, K>
where
    S: Enumerable + Clone,
    [(); S::N]: ,
{
    fn inverse(&self) -> Self {
        let mut map = std::mem::MaybeUninit::<(S, CyclicGroup<K>)>::uninit_array::<{ S::N }>();
        for i in 0..S::N {
            let (s, r) = &self.map[i];
            map[s.index()].write((S::from_index(i), r.inverse()));
        }
        Self {
            map: unsafe { std::mem::transmute_copy(&map) },
        }
    }
}

impl<S, const K: u8> GroupAction<(S, CyclicGroup<K>)> for OrientedSymmetricGroup<S, K>
where
    S: Enumerable + Clone + Eq,
    [(); S::N]: ,
{
    fn act(&self, (s, r): &(S, CyclicGroup<K>)) -> (S, CyclicGroup<K>) {
        let (s2, r2) = self.map[s.index()].clone();
        (s2, r.op(r2))
    }
}

impl<S, const K: u8> Display for OrientedSymmetricGroup<S, K>
where
    S: Enumerable + Clone + Display + Eq,
    [(); S::N]: ,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\u{250c}")?;
        for s in S::enumerate() {
            write!(f, " {}", s)?;
        }
        writeln!(f, " \u{2510}")?;
        write!(f, "\u{2502}")?;
        let mut width = 0;
        for s in S::enumerate() {
            let displayed = self.act(&(s, CyclicGroup::identity())).0.to_string();
            width = displayed.len();
            write!(f, " {}", displayed)?;
        }
        writeln!(f, " \u{2502}")?;
        write!(f, "\u{2514}")?;
        for s in S::enumerate() {
            write!(f, " {:width$}", self.act(&(s, CyclicGroup::identity())).1)?;
        }
        writeln!(f, " \u{2518}")
    }
}

impl<S, const K: u8> Debug for OrientedSymmetricGroup<S, K>
where
    S: Enumerable + Clone + Display + Debug,
    [(); S::N]: ,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OrientedSymmetricGroup")
            .field("map", &self.map)
            .finish()
    }
}
