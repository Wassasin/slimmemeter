use prometheus::{__register_gauge, opts, register_gauge, register_int_gauge};
use prometheus_exporter::{FinishedUpdate, PrometheusExporter};
use std::env;
use std::net::SocketAddr;

fn main() {
    use serial::SerialPort;

    let path = env::args_os()
        .skip(1)
        .next()
        .expect("Please specify the serial TTY path");
    let mut port = serial::open(&path).unwrap();

    port.reconfigure(&|settings| {
        settings.set_baud_rate(serial::Baud115200)?;
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    })
    .expect("Failed to configure serial TTY port");

    port.set_timeout(std::time::Duration::from_millis(2000))
        .unwrap();

    let addr_raw = "0.0.0.0:9185";
    let addr: SocketAddr = addr_raw.parse().expect("Can not parse listen addr");

    let (request_receiver, finished_sender) =
        PrometheusExporter::run_and_repeat(addr, std::time::Duration::from_millis(10));

    let power_delivered = register_gauge!(
        "power_delivered",
        "Power delivered in the last second (kWs)"
    )
    .unwrap();
    let power_received =
        register_gauge!("power_received", "Power generated in the last second (kWs)").unwrap();
    let power_failures = register_int_gauge!("power_failures", "Number of power failures").unwrap();
    let long_power_failures =
        register_int_gauge!("long_power_failures", "Number of long power failures").unwrap();
    let meterreadings_tariff1_to = register_gauge!(
        "meterreadings_tariff1_to",
        "Total power consumed under tariff 1"
    )
    .unwrap();
    let meterreadings_tariff2_to = register_gauge!(
        "meterreadings_tariff2_to",
        "Total power consumed under tariff 2"
    )
    .unwrap();
    let meterreadings_tariff1_by = register_gauge!(
        "meterreadings_tariff1_by",
        "Total power generated under tariff 1"
    )
    .unwrap();
    let meterreadings_tariff2_by = register_gauge!(
        "meterreadings_tariff2_by",
        "Total power generated under tariff 2"
    )
    .unwrap();

    let voltage_sags = register_int_gauge!("voltage_sags", "Number of voltage sags").unwrap();
    let voltage_swells = register_int_gauge!("voltage_swells", "Number of voltage swells").unwrap();
    let voltage = register_gauge!("voltage", "Amount of voltage currently").unwrap();
    let current = register_int_gauge!("current", "Amount of amperes currently").unwrap();

    let gas_delivered = register_gauge!("gas_delivered", "Total gas delivered").unwrap();

    use std::io::Read;
    let reader = dsmr5::Reader::new(port.bytes().map(|b| b.unwrap()));

    for readout in reader {
        // Will block until exporter receives http request.
        request_receiver.recv().unwrap();

        let telegram = readout.to_telegram().unwrap();
        let state = dsmr5::Result::<dsmr5::state::State>::from(&telegram).unwrap();

        println!("{:?}", state);

        power_delivered.set(state.power_delivered.unwrap());
        power_received.set(state.power_received.unwrap());
        power_failures.set(state.power_failures.unwrap() as i64);
        long_power_failures.set(state.long_power_failures.unwrap() as i64);

        meterreadings_tariff1_to.set(
            state.meterreadings[dsmr5::Tariff::Tariff1 as usize]
                .to
                .unwrap(),
        );
        meterreadings_tariff2_to.set(
            state.meterreadings[dsmr5::Tariff::Tariff2 as usize]
                .to
                .unwrap(),
        );
        meterreadings_tariff1_by.set(
            state.meterreadings[dsmr5::Tariff::Tariff1 as usize]
                .by
                .unwrap(),
        );
        meterreadings_tariff2_by.set(
            state.meterreadings[dsmr5::Tariff::Tariff2 as usize]
                .by
                .unwrap(),
        );

        let line = &state.lines[dsmr5::Line::Line1 as usize];

        voltage_sags.set(line.voltage_sags.unwrap() as i64);
        voltage_swells.set(line.voltage_swells.unwrap() as i64);
        voltage.set(line.voltage.unwrap());
        current.set(line.current.unwrap() as i64);

        let gas = &state.slaves[dsmr5::Slave::Slave1 as usize];

        gas_delivered.set(gas.meter_reading.as_ref().unwrap().1);

        // Notify exporter that all metrics have been updated so the caller client can
        // receive a response.
        finished_sender.send(FinishedUpdate).unwrap();
    }
}
