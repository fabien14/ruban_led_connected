mod routes;
mod startup;

pub use routes::{connect, device, devices, scan, BluetoothServerWS};
pub use startup::Application;
