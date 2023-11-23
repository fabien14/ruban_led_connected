use back_ruban_led_connected::configuration::get_configuration;
use back_ruban_led_connected::framework_api::Application;
use back_ruban_led_connected::framework_bluetooth::Communication;

use std::sync::{Arc, Mutex};
use tokio;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let communication_manager_bluetooth = Arc::new(Mutex::new(Communication::new().await));

    let configuration = get_configuration().expect("Failed to read configuration.");
    let application = Application::build(
        configuration,
        communication_manager_bluetooth.clone(),
    )
    .await?;

    tokio::spawn(async move {
        application.run_until_stopped().await;
    })
    .await;

    Ok(())
}
