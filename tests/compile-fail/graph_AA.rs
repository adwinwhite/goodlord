use deadlock_proof::LockAfter;
use deadlock_proof::impl_lock_order;

struct A;

impl_lock_order!(A => A);

fn main() {}
