pub mod data;
pub mod index;
pub mod iter;
pub mod slice;
pub mod vec;

pub mod prelude {
    pub use super::index::*;
    pub use super::slice::*;
    pub use super::vec::*;
}
