# slimmemeter
Quick and dirty DSMR5 serial TTY to MQTT exporter for your smart meter.

Will emit readings for high and low tariffs, current usage, voltage, gas usage from slave meter and number of incidents to the `slimmemeter` topic as JSON.

## How to run
Install the slimmemeter by running `cargo install slimmemeter`.
Then run using `slimmemeter /dev/ttyUSB0`, or whatever you are using as a serial interface.

DSMR5 smart meters invert the TX-pin, thus you might need to configure your serial interface (like FTDI) to adapt to this.
You can also invert the signal yourself using a transistor, but that is a bit fiddly.

## Example data
```json
{
    "datetime": {
        "year": 22,
        "month": 11,
        "day": 23,
        "hour": 19,
        "minute": 19,
        "second": 52,
        "dst": false
    },
    "meterreadings": [
        {
            "to": 6514.304,
            "by": 0.0
        },
        {
            "to": 7076.58,
            "by": 0.0
        }
    ],
    "tariff_indicator": [
        0,
        2
    ],
    "power_delivered": 0.352,
    "power_received": 0.0,
    "power_failures": 3,
    "long_power_failures": 4,
    "lines": [
        {
            "voltage_sags": 2,
            "voltage_swells": 0,
            "voltage": null,
            "current": 1,
            "active_power_plus": 0.353,
            "active_power_neg": 0.0
        },
        {
            "voltage_sags": 0,
            "voltage_swells": 0,
            "voltage": null,
            "current": 0,
            "active_power_plus": 0.0,
            "active_power_neg": 0.0
        },
        {
            "voltage_sags": 0,
            "voltage_swells": 0,
            "voltage": null,
            "current": 0,
            "active_power_plus": 0.0,
            "active_power_neg": 0.0
        }
    ],
    "slaves": [
        {
            "device_type": 3,
            "meter_reading": [
                {
                    "year": 22,
                    "month": 11,
                    "day": 23,
                    "hour": 19,
                    "minute": 0,
                    "second": 0,
                    "dst": false
                },
                4877.691
            ]
        },
        {
            "device_type": null,
            "meter_reading": null
        },
        {
            "device_type": null,
            "meter_reading": null
        },
        {
            "device_type": null,
            "meter_reading": null
        }
    ]
}
```