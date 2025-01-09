#![feature(negative_impls)]

use std::{
    cell::Cell, marker::PhantomData, sync::{Mutex, MutexGuard}
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

thread_local! {
    static HAS_LOCK_ROOT: Cell<bool> = const { Cell::new(false) };
}

impl LockToken<Unlocked> {
    pub fn new() -> Self {
        if HAS_LOCK_ROOT.get() {
            panic!("A thread can only have a single `UNLOCKED` token.");
        } else {
            HAS_LOCK_ROOT.set(true);
            LockToken { id: PhantomData }
        }
    }
}

impl Default for LockToken<Unlocked> {
    fn default() -> Self {
        Self::new()
    }
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

#[cfg(test)]
mod tests {
    use crate::LockToken;

    #[test]
    #[should_panic]
    fn duplicate_unlocked_tokens() {

        let _unlocked_token1 = LockToken::new();
        let _unlocked_token2 = LockToken::new();
    }

    #[test]
    fn one_token_one_thread() {
        use std::thread;

        let _unlocked_token0 = LockToken::new();

        let handle1 = thread::spawn(|| {
            let _unlocked_token1 = LockToken::new();
        });

        let handle2 = thread::spawn(|| {
            let _unlocked_token2 = LockToken::new();
        });

        assert!(handle1.join().is_ok());
        assert!(handle2.join().is_ok());
    }
}
