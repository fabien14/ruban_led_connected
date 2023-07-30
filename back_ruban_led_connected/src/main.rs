use back_ruban_led_connected::framework_bluetooth::Manager;


#[tokio::main]
async fn main() {
    let manager_bluetooth = Manager::new().await;
    manager_bluetooth.start_scan().await;
    manager_bluetooth.get_devices().await;
    println!("Hello, world!");
}
