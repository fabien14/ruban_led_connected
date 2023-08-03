mod startup;
mod routes;

pub use startup::Application;
pub use routes::{scan, devices, device};
