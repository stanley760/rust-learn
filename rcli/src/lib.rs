pub mod operation {
    pub mod opts;
    pub use opts::{Opts, Subcommand};
}


pub mod process {
    pub mod csv;
    pub use csv::parse_csv;
}