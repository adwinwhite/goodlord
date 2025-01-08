use deadlock_proof::LockAfter;
use deadlock_proof::impl_lock_order;
use deadlock_proof::Unlocked;

struct A;

impl_lock_order!(Unlocked => A);

fn main() {}
