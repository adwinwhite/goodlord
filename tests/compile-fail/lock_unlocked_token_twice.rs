use deadlock_proof::LockAfter;
use deadlock_proof::impl_lock_order;
use deadlock_proof::LockToken;
use deadlock_proof::GoodLock;
use deadlock_proof::Unlocked;


struct A;
struct B;

impl_lock_order!(Unlocked => A);
impl_lock_order!(A => B);

fn main() {
    let mut unlocked_token = LockToken::UNLOCKED;
    let lock_a: GoodLock<A, _> = GoodLock::new(0_i32);
    let lock_b: GoodLock<B, _> = GoodLock::new(0_i32);
    let (a, _) = lock_a.lock(&mut unlocked_token);
    let (b, _) = lock_b.lock(&mut unlocked_token);
    println!("{}, {}", a, b);
}
