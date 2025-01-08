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
    let (b, locked_b) = lock_b.lock(&mut unlocked_token);
    let (a, _) = lock_a.lock(&mut locked_b);
    println!("{}, {}", a, b);
}
