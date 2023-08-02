use back_ruban_led_connected::framework_bluetooth::Manager;
use back_ruban_led_connected::framework_api::{Application, get_configuration};

use std::time::Duration;
use tokio::time;
use tokio;


#[tokio::main]
async fn main() {
    let manager_bluetooth = Manager::new().await;

    manager_bluetooth.start_scan().await;
    let scan_duration: Duration = Duration::from_secs(10);

    time::sleep(scan_duration).await;
    manager_bluetooth.get_devices().await;

    println!("beetwen get devices call ##########################");

    time::sleep(scan_duration).await;
    manager_bluetooth.get_devices().await;

    println!("Hello, world!");

    let configuration = get_configuration().expect("Failed to read configuration.");
    //let application = Application::build(configuration.clone()).await?;

}
