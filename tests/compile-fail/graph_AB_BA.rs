use goodlord::LockAfter;
use goodlord::impl_lock_order;

struct A;
struct B;

impl_lock_order!(A => B);
impl_lock_order!(B => A);

fn main() {}
