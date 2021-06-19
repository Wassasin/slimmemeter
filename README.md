# slimmemeter
Quick and dirty DSMR5 serial TTY to Prometheus exporter for your smart meter.

Will open a listening socket for the HTTP endpoints on `:9185`, and emit some default metrics for my home.
You will probably need to adapt the program to make the specific metrics from your home visible.
As such I do not recommend using this package.

Will emit readings for high and low tariffs, current usage, voltage, gas usage from slave meter and number of incidents.

## How to run
Build it and upload the binary to the machine attached to your smart meter. You might need to cross compile for ARM if you're
using a Raspberry Pi or similar. Then run using `./slimmemeter /dev/ttyUSB0`, or whatever you are using as a serial interface.

DSMR5 smart meters invert the TX-pin, thus you might need to configure your serial interface (like FTDI) to adapt to this.
You can also invert the signal yourself using a transistor, but that is a bit fiddly.

## Cross compile (for ARM v6)
```
cargo install cross
cross build --target arm-unknown-linux-gnueabihf
```
