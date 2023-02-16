use std::marker::PhantomData;

use crate::{to_concrete, ConcreteMagical, Magic, MagicFlavor, Magical};

pub trait IterMagic: Sized {
    type Item<M: Magic>;
    type Iter<M: Magic>: Iterator<Item = Self::Item<M>>;

    fn iter_magic<'a, M: Magic>(this: Magical<'a, M, Self>) -> Self::Iter<M>;
}

impl<T> IterMagic for Vec<T> {
    type Item<M: Magic> = Magical<'a, M, T>;
    type Iter<M: Magic> = VecMagicIterator<M>;

    fn iter_magic<'a, M: Magic>(this: Magical<'a, M, Self>) -> Self::Iter<M> {
        match M::FLAVOR {
            MagicFlavor::Owned => {
                let concrete: Self = unsafe { std::mem::transmute_copy(&this) };
                std::mem::forget(this);
                let iter = concrete.into_iter();
                let magic_iter = 
            }
            MagicFlavor::Ref => todo!(),
            MagicFlavor::Mut => todo!(),
        }
    }
}

pub trait VecIterationMagic {
    type Iter;
}

impl VecIterationMagic for M {
    
}