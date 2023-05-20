pub mod request;
pub use request::*;

pub trait FromErr<T> {
    fn from(src: T) -> Self;
}
