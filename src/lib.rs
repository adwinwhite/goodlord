#![feature(negative_impls)]

use std::{
    marker::PhantomData,
    sync::{Mutex, MutexGuard},
};

// Each lock is named.
pub struct GoodLock<Id, T> {
    lock: Mutex<T>,
    id: PhantomData<Id>,
}

impl<Id, T> GoodLock<Id, T> {
    pub fn new(data: T) -> Self {
        Self {
            lock: Mutex::new(data),
            id: PhantomData,
        }
    }

    // We can only lock if we have a token from previous lock.
    // Make the exclusive borrowed token and guard share lifetime so our token can only be used
    // again after the guard is dropped.
    // Can this mut reference keep our previous token exclusive?
    pub fn lock<'t, 's, PrevId>(
        &'s self,
        _token: &'t mut LockToken<PrevId>,
    ) -> (MutexGuard<'t, T>, LockToken<Id>)
    where
        Id: LockAfter<PrevId>,
        's: 't,
    {
        (self.lock.lock().unwrap(), LockToken { id: PhantomData })
    }
}

impl<Id, T> From<T> for GoodLock<Id, T> {
    fn from(data: T) -> Self {
        Self::new(data)
    }
}

pub struct LockToken<Id> {
    id: PhantomData<Id>,
}

pub struct Unlocked;

// FIXME: We can create as many `UNLOCKED` as we want.
impl LockToken<Unlocked> {
    pub const UNLOCKED: Self = LockToken { id: PhantomData };
}



/// # Safety
/// Self must always lock after M.
pub unsafe trait LockAfter<M> {}


// FIXME: negative impl doesn't solve conflicting impls.
// impl !LockAfter<Unlocked> for Unlocked {}

#[macro_export]
macro_rules! impl_lock_order {
    // Special branch to avoid orphan rule conflicts.
    (Unlocked => $B:ty) => {
        /// # Safety
        /// LockAfter(b, Unlocked).
        unsafe impl LockAfter<Unlocked> for $B {}
    };
    ($A:ty => $B:ty) => {
        /// # Safety
        /// LockAfter(b, a).
        unsafe impl LockAfter<$A> for $B {}

        /// # Safety
        /// LockAfter(b, X):- LockAfter(a, X), LockAfter(b, a).
        unsafe impl<X> LockAfter<X> for $B where $A: LockAfter<X> {}
    };
}
