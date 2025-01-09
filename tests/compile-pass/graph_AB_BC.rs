use goodlord::LockAfter;
use goodlord::impl_lock_order;

struct A;
struct B;
struct C;

impl_lock_order!(A => B);
impl_lock_order!(B => C);

fn main() {}
