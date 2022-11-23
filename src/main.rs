use std::{path::PathBuf, time::Duration};

use clap::Parser;
use rumqttc::{AsyncClient, ConnAck, ConnectReturnCode, MqttOptions, Packet, QoS};
use tokio::task;

#[derive(Parser)]
struct Args {
    #[clap()]
    serial_path: PathBuf,
    #[clap(long, default_value = "mqtt://localhost?client_id=slimmemeter")]
    mqtt_url: String,
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let args = Args::parse();

    use serial::SerialPort;

    let path = args.serial_path;
    let mut port = serial::open(&path).expect("Could not open serial TTY path");

    port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud115200)?;
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    })
    .expect("Failed to configure serial TTY port");

    port.set_timeout(std::time::Duration::from_millis(11000))
        .unwrap();

    let bytes = port.bytes().map(|b| b.unwrap());

    use std::io::Read;
    let reader = dsmr5::Reader::new(bytes);

    let mut options = MqttOptions::parse_url(args.mqtt_url).unwrap();
    options.set_keep_alive(Duration::from_secs(5));

    let (client, mut eventloop) = AsyncClient::new(options, 10);
    log::info!("Connected to MQTT");

    task::spawn(async move {
        loop {
            let notification = eventloop.poll().await.unwrap();

            #[allow(clippy::single_match)]
            match notification {
                rumqttc::Event::Incoming(Packet::ConnAck(ConnAck {
                    code: ConnectReturnCode::Success,
                    ..
                })) => {
                    log::info!("Connected to MQTT");
                }
                _ => {}
            }
        }
    });

    for readout in reader {
        let telegram = readout.to_telegram().unwrap();
        let state = dsmr5::Result::<dsmr5::state::State>::from(&telegram).unwrap();

        log::info!(
            "Received P1: delivered {:?}kW",
            state.power_delivered.unwrap_or(0.)
        );

        client
            .publish(
                "slimmemeter",
                QoS::AtLeastOnce,
                false,
                serde_json::to_vec(&state).unwrap(),
            )
            .await
            .unwrap();
    }
}
