pub struct Assert<const CHECK: bool>;
pub trait IsTrue {}
impl IsTrue for Assert<true> {}
