pub mod request;
pub use request::*;

pub mod response;
pub use response::*;

pub trait FromErr<T> {
    fn from(src: T) -> Self;
}
