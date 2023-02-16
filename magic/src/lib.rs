// pub mod iter;

pub use magic_proc::*;

pub type Magical<'a, M: Magic, T: 'a> = M::Type<'a, T>;

/// # Safety
///
/// - If `FLAVOUR == MagicFlavor::Owned`, then `Type<'a, T> == T`.
/// - If `FLAVOUR == MagicFlavor::Ref`, then `Type<'a, T> == &'a T`.
/// - If `FLAVOUR == MagicFlavor::Mut`, then `Type<'a, T> == &'a mut T`.
pub unsafe trait Magic {
    type Type<'a, T: 'a>;

    const FLAVOR: MagicFlavor;

    fn to_concrete<'a, T>(value: Magical<'a, Self, T>) -> ConcreteMagical<'a, T> {
        match Self::FLAVOR {
            MagicFlavor::Owned => ConcreteMagical::Owned({
                // SAFETY: `FLAVOUR == MagicFlavor::Owned`, so `Type<'a, T> == T`
                let concrete: T = unsafe { std::mem::transmute_copy(&value) };
                std::mem::forget(value);
                concrete
            }),
            MagicFlavor::Ref => ConcreteMagical::Ref({
                // SAFETY: `FLAVOUR == MagicFlavor::Ref`, so `Type<'a, T> == &'a T`
                let concrete: &'a T = unsafe { std::mem::transmute_copy(&value) };
                std::mem::forget(value);
                concrete
            }),
            MagicFlavor::Mut => ConcreteMagical::Mut({
                // SAFETY: `FLAVOUR == MagicFlavor::Mut`, so `Type<'a, T> == &'a mut T`
                let concrete: &'a mut T = unsafe { std::mem::transmute_copy(&value) };
                std::mem::forget(value);
                concrete
            }),
        }
    }
}

/// Tentative
pub trait MagicConvertFrom<M: Magic>: Magic + Sized {
    fn convert<T>(value: Magical<M, T>) -> Magical<M, Self>;
}

pub trait Explode: Sized {
    type Exploded<'a, M: Magic>
    where
        Self: 'a;

    fn explode<M: Magic>(this: M::Type<'_, Self>) -> Self::Exploded<'_, M>;
}

pub trait IterMagic<IM: Magic>: Sized {
    type Item;
    type Iter: Iterator<Item = Self::Item>;

    fn iter_magic(self) -> Self::Iter;
}

pub struct OwnedMagic;
pub struct RefMagic;
pub struct MutMagic;

pub enum MagicFlavor {
    Owned,
    Ref,
    Mut,
}

unsafe impl Magic for OwnedMagic {
    type Type<'a, T: 'a> = T;

    const FLAVOR: MagicFlavor = MagicFlavor::Owned;
}

unsafe impl Magic for RefMagic {
    type Type<'a, T: 'a> = &'a T;

    const FLAVOR: MagicFlavor = MagicFlavor::Ref;
}

unsafe impl Magic for MutMagic {
    type Type<'a, T: 'a> = &'a mut T;

    const FLAVOR: MagicFlavor = MagicFlavor::Mut;
}

pub enum ConcreteMagical<'a, T: 'a> {
    Owned(T),
    Ref(&'a T),
    Mut(&'a mut T),
}
