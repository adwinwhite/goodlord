use std::thread;
use std::sync::Arc;

use goodlord::LockAfter;
use goodlord::impl_lock_order;
use goodlord::LockToken;
use goodlord::GoodLock;
use goodlord::Unlocked;


struct A;

impl_lock_order!(Unlocked => A);

fn main() {
    let mut unlocked_token = LockToken::new();
    let lock_a1: Arc<GoodLock<A, _>> = GoodLock::new(1_i32).into();
    let lock_a2: Arc<GoodLock<A, _>> = GoodLock::new(2_i32).into();
    let handle = {
        let lock_a1 = lock_a1.clone();
        let lock_a2 = lock_a2.clone();
        thread::spawn(move || {
            let mut unlocked_token = LockToken::new();
            let (a1, a1_token) = lock_a1.lock(&mut unlocked_token);
            // We cannot use a1_token to lock lock_a2 due to we can't have `A => A`.
            let (a2, _) = lock_a2.lock(&mut a1_token);
            println!("{}, {}", a1, a2);
        })
    };
    let (a2, _) = lock_a2.lock(&mut unlocked_token);
    // We cannot use a token for multiple times in the same scope.
    let (a1, _) = lock_a1.lock(&mut unlocked_token);
    handle.join();
    println!("{}, {}", a2, a1);
}
