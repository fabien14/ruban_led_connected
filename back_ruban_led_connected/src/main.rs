use back_ruban_led_connected::framework_bluetooth::Manager;
use back_ruban_led_connected::framework_api::Application;
use back_ruban_led_connected::configuration::get_configuration;

//use std::time::Duration;
//use tokio::time;
use tokio;


#[tokio::main]
async fn main() -> anyhow::Result<()>{
    let manager_bluetooth = Manager::new().await;

    /*manager_bluetooth.start_scan().await;
    let scan_duration: Duration = Duration::from_secs(10);

    time::sleep(scan_duration).await;
    manager_bluetooth.get_devices().await;

    println!("beetwen get devices call ##########################");

    time::sleep(scan_duration).await;
    manager_bluetooth.get_devices().await;*/

    //println!("Hello, world!");

    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(configuration.clone(), manager_bluetooth.clone()).await?;
    let _ = tokio::spawn(application.run_until_stopped()).await;
    Ok(())


}
