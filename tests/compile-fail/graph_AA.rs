use goodlord::LockAfter;
use goodlord::impl_lock_order;

struct A;

impl_lock_order!(A => A);

fn main() {}
