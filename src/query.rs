pub mod length;
pub use length::{ Length, LengthQuery };

pub mod access_at;
pub use access_at::{ AccessAt, AccessAtQuery };

pub mod update_at;
pub use update_at::{ UpdateAt, UpdateAtQuery };

pub mod util;

pub mod init;

use crate::core::{ Named, QueryFail };
use rand::Rng;

pub trait Query: Named {
    type Target;
    type Checker;
    fn verify<R: Rng>(gen: &mut R, target: &mut Self::Target, checker: &mut Self::Checker) -> Result<(), QueryFail>;
}
