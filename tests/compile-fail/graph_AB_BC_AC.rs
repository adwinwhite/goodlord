use deadlock_proof::LockAfter;
use deadlock_proof::impl_lock_order;

struct A;
struct B;
struct C;

impl_lock_order!(A => B);
impl_lock_order!(B => C);
impl_lock_order!(A => C);

fn main() {}
