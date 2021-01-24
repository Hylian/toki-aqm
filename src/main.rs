extern crate embedded_hal as hal;

use bme280::BME280;
use embedded_ccs811::{prelude::*, Ccs811Awake, MeasurementMode, SlaveAddr};
use embedded_graphics::fonts::{Font6x8, Text};
use embedded_graphics::style::{TextStyle, TextStyleBuilder};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use nb::block;
use ssd1306::{mode::GraphicsMode, Builder, I2CDIBuilder, displaysize::*, prelude::*};

fn main() {
    let dev = rppal::i2c::I2c::new().unwrap();
    let address = SlaveAddr::default();
    let sensor = Ccs811Awake::new(dev, address);
    let mut sensor = sensor.start_application().ok().unwrap();
    sensor.set_mode(MeasurementMode::ConstantPower1s).unwrap();

    let dev = rppal::i2c::I2c::new().unwrap();
    let delay = linux_embedded_hal::Delay;
    let mut bme280 = BME280::new_secondary(dev, delay);
    bme280.init().unwrap();

    let dev = rppal::i2c::I2c::new().unwrap();
    let interface = I2CDIBuilder::new().init(dev);
    let mut disp: GraphicsMode<_, _> = Builder::new()
        .size(DisplaySize128x32)
        .connect(interface)
        .into();

    disp.init().unwrap();
    disp.flush().unwrap();
    
	let style = TextStyleBuilder::new(Font6x8)
        .text_color(BinaryColor::On)
        .build();

    loop {
        if let (Ok(ccs_data), Ok(bme_data)) = (block!(sensor.data()), bme280.measure()) {
			println!("eCO2: {} eTVOC: {} Rel. Humidity: {} Temperature: {}C Pressure: {}", 
				 ccs_data.eco2, ccs_data.etvoc, bme_data.humidity, bme_data.temperature, bme_data.pressure);

            disp.clear();
			let line_1 = format!("eCO2: {}  eTVOC: {}", 
				 ccs_data.eco2, ccs_data.etvoc);
			let line_2 = format!("Hmd:  {:.2}", bme_data.humidity);
			let line_3 = format!("Temp: {:.2}C", bme_data.temperature);
			let line_4 = format!("Prs:  {:.2}", bme_data.pressure);

            Text::new(&line_1, Point::zero())
                .into_styled(style)
                .draw(&mut disp)
                .ok();
            Text::new(&line_2, Point::new(0, 9))
                .into_styled(style)
                .draw(&mut disp)
                .ok();
            Text::new(&line_3, Point::new(0, 18))
                .into_styled(style)
                .draw(&mut disp)
                .ok();
            Text::new(&line_4, Point::new(0, 26))
                .into_styled(style)
                .draw(&mut disp)
                .ok();

            disp.flush().ok();
        } else {
            println!("Data read error!");
        }
    }
}
