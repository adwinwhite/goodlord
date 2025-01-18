use goodlord::LockAfter;
use goodlord::impl_lock_order;
use goodlord::GoodLock;
use goodlord::Unlocked;
use goodlord::RootToken;


struct A;
struct B;

impl_lock_order!(Unlocked => A);
impl_lock_order!(A => B);

#[allow(non_snake_case)]
fn lock_AB() {
    let mut unlocked_token = RootToken::new();
    let lock_a: GoodLock<A, _> = GoodLock::new(0_i32);
    let lock_b: GoodLock<B, _> = GoodLock::new(0_i32);
    let (a, mut a_token) = lock_a.lock(unlocked_token.as_mut());
    let (b, _) = lock_b.lock(&mut a_token);
    println!("{}, {}", a, b);
}

fn main() {
    // We can new root token for multiple times.
    lock_AB();
    lock_AB();
    lock_AB();
}
