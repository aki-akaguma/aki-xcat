pub mod err;

mod opt_uc_x_param;
pub use self::opt_uc_x_param::OptUcXParam;
//pub use self::opt_uc_x_param::OptUcXParamParseError;

pub mod adapt_input;
pub use self::adapt_input::adapt_input;

pub mod file_type;
pub use self::file_type::{detect_file_type, FileType};
