use goodlord::LockAfter;
use goodlord::impl_lock_order;
use goodlord::Unlocked;

struct A;

impl_lock_order!(Unlocked => A);

fn main() {}
