extern crate rppal;
extern crate embedded_hal as hal;

use embedded_ccs811::{prelude::*, Ccs811Awake, MeasurementMode, SlaveAddr};
use nb::block;

fn main() {
    let dev = rppal::i2c::I2c::new().unwrap();
    let address = SlaveAddr::default();
    let sensor = Ccs811Awake::new(dev, address);
    let mut sensor = sensor.start_application().ok().unwrap();
    sensor.set_mode(MeasurementMode::ConstantPower1s).unwrap();
    loop {
        if let Ok(data) = block!(sensor.data()) {
            println!("eCO2: {}, eTVOC: {}", data.eco2, data.etvoc);
        } else {
            println!("Data read error!");
        }
    }
}
