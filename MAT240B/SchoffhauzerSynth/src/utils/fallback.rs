// pub struct Fallback<T, F: Fn() -> T> {
//     value: Option<T>,
//     fallback: F,
// }
//
// impl<T, F: Fn() -> T> Fallback<T, F> {
//     pub fn new(value: Option<T>, fallback: F) -> Self {
//         Self { value, fallback }
//     }
// }
//
// impl<T: Clone, F: Fn() -> T> Fallback<T, F> {
//     pub fn get(&self) -> T {
//         self.value.clone().unwrap_or_else(&self.fallback)
//     }
// }
//
// macro_rules! impl_from_fallback_ref {
//     ($t: ty) => {
//         impl<F: Fn() -> $t> From<&Fallback<$t, F>> for $t {
//             fn from(value: &Fallback<$t, F>) -> Self {
//                 value.get()
//             }
//         }
//     };
// }
//
// impl_from_fallback_ref!(f32);
// impl_from_fallback_ref!(f64);
//
// pub trait WithFallback<T> where Self: Sized {
//     fn with_fallback<F: Fn() -> T>(self, fallback: F) -> Fallback<T, F>;
// }
//
// impl<T> WithFallback<T> for Option<T> {
//     fn with_fallback<F: Fn() -> T>(self, fallback: F) -> Fallback<T, F> {
//         Fallback::new(self, fallback)
//     }
// }
