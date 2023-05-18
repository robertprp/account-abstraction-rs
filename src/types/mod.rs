pub mod user_operation;

pub trait FromErr<T> {
    fn from(src: T) -> Self;
}
