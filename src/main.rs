use embedded_graphics::prelude::*;
use embedded_graphics::primitives::PrimitiveStyle;
use embedded_graphics::primitives::Rectangle;
use epd_driver::color::*;
use epd_driver::prelude::*;

use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::prelude::*;
use esp_idf_hal::spi;
use esp_idf_sys as _; // If using the `binstart` feature of `esp-idf-sys`, always keep this module imported

fn main() {
    // Temporary. Will disappear once ESP-IDF 4.4 is released, but for now it is necessary to call this function once,
    // or else some patches to the runtime implemented by esp-idf-sys might not link properly.
    esp_idf_sys::link_patches();

    println!("Hello, world!");
    let peripherals = Peripherals::take().unwrap();

    let cs = peripherals.pins.gpio5;
    // let res = peripherals.pins.gpio9;
    let miso = peripherals.pins.gpio10;
    let sclk = peripherals.pins.gpio18;
    // let busy = peripherals.pins.gpio19;
    let mosi = peripherals.pins.gpio23;

    let config = <spi::config::Config as Default>::default()
        .baudrate(1.MHz().into())
        .data_mode(embedded_hal::spi::MODE_0);

    let mut spi = spi::Master::<spi::SPI3, _, _, _, _>::new(
        peripherals.spi3,
        spi::Pins {
            sclk,
            sdo: miso,
            sdi: Some(mosi),
            cs: Some(cs),
        },
        config,
    )
    .expect("spi");

    let peripherals_2: Peripherals;
    unsafe {
        peripherals_2 = Peripherals::new();
    }
    let cs = peripherals_2.pins.gpio5;
    let res = peripherals_2.pins.gpio9;
    let dc = peripherals_2.pins.gpio10;
    let busy = peripherals_2.pins.gpio19;

    let mut ssd1618 = Ssd1681::new(
        &mut spi,
        cs.into_output().expect("cs pin"),
        busy.into_input().expect("busy pin"),
        dc.into_output().expect("dc pin"),
        res.into_output().expect("res pin"),
        &mut esp_idf_hal::delay::Ets,
    )
    .expect("ssd1618");

    ssd1618
        .clear_frames(&mut spi)
        .expect("failed to clear display frames");

    let mut display = Display1in54::new();

    display.set_rotation(DisplayRotation::Rotate0);

    Rectangle::new(Point::new(50, 50), Size::new(50, 50))
        .into_styled(PrimitiveStyle::with_fill(White))
        .draw(&mut display)
        .expect("draw failed");

    ssd1618
        .update_frame1(&mut spi, display.buffer1())
        .expect("updating frame1 failed");
    ssd1618
        .update_frame2(&mut spi, display.buffer2())
        .expect("updating frame2 failed");

    ssd1618.display_frame(&mut spi).expect("failed to display frame");

    println!("Done!")
}
