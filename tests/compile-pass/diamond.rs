use std::sync::Arc;

use goodlord::LockAfter;
use goodlord::impl_lock_order;
use goodlord::LockToken;
use goodlord::GoodLock;
use goodlord::Unlocked;

//   A
//  / \
// B   C
//  \ /
//   D
// For diamond graph, we just assign an arbitrary order between branches.
// E.g. A -> B -> C -> D or A -> C -> B -> D.

struct A;
struct B;
struct C;
struct D;

impl_lock_order!(Unlocked => A);
impl_lock_order!(A => B);
impl_lock_order!(B => C);
impl_lock_order!(C => D);

fn main() {
    let lock_a: Arc<GoodLock<A, _>> = GoodLock::new(0_i32).into();
    let lock_b: Arc<GoodLock<B, _>> = GoodLock::new(0_i32).into();
    let lock_c: Arc<GoodLock<C, _>> = GoodLock::new(0_i32).into();
    let lock_d: Arc<GoodLock<D, _>> = GoodLock::new(0_i32).into();
    {
        let lock_a = lock_a.clone();
        let lock_c = lock_c.clone();
        let lock_d = lock_d.clone();
        std::thread::spawn(move || {
            let mut unlocked_token = LockToken::new();
            let (a, mut a_token) = lock_a.lock(&mut unlocked_token);
            let (c, mut c_token) = lock_c.lock(&mut a_token);
            let (d, _) = lock_d.lock(&mut c_token);
            println!("{}, {}, {}", a, c, d);
        });
    }
    let mut unlocked_token = LockToken::new();
    let (a, mut a_token) = lock_a.lock(&mut unlocked_token);
    let (b, mut b_token) = lock_b.lock(&mut a_token);
    let (d, _) = lock_d.lock(&mut b_token);
    println!("{}, {}, {}", a, b, d);
}
