//hd44780
use std::thread;
use linux_embedded_hal::Pin;
use linux_embedded_hal::sysfs_gpio::Direction;
use hd44780_hal::HD44780;

//hx711
use rppal::{spi::{Spi, Bus, SlaveSelect, Mode, Error},hal::Delay};
use hx711_spi::Hx711;
use nb::block;
//use std::thread;


fn main() -> Result<(), Error> 
{
    //hx711 declarations
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 1_000_000, Mode::Mode0)?;
    let mut hx711 = Hx711::new(spi, Delay::new());

    //init the screen
	hx711.reset()?;

    let mut zero_value: f32 = 0.0;
    let one_kg_value: f32 = 130670.0;

    for i in 0..20 {
        let reading = block!(hx711.read()).unwrap() as f32;
        println!("{:>2}: {}", i, reading);
        zero_value += reading;
    }
    zero_value /= 20.0; //tara

    println!("Tara: {}", zero_value);

    let n = 5;

    //screen declarations
    let rs = Pin::new(13);
    let en = Pin::new(19);

    let db4 = Pin::new(26);
    let db5 = Pin::new(16);
    let db6 = Pin::new(20);
    let db7 = Pin::new(21);

    rs.export().unwrap();
    en.export().unwrap();
    
    db4.export().unwrap();
    db5.export().unwrap();
    db6.export().unwrap();
    db7.export().unwrap();

    rs.set_direction(Direction::Low).unwrap();
    en.set_direction(Direction::Low).unwrap();
    
    db4.set_direction(Direction::Low).unwrap();
    db5.set_direction(Direction::Low).unwrap();
    db6.set_direction(Direction::Low).unwrap();
    db7.set_direction(Direction::Low).unwrap();

    //4-bit communication with display
    let mut lcd = HD44780::new_4bit(
        rs,
        en,
    
        db4,
        db5,
        db6,
        db7,
        linux_embedded_hal::Delay,
    );
    
    lcd.reset();
    lcd.clear();
    lcd.set_display_mode(true, false, false);
    lcd.write_str("Customer nr. 4");

    lcd.set_cursor_pos(40);
    lcd.write_str("Calculating...");
    lcd.set_cursor_pos(30);
    lcd.write_str("0 kg.");

    loop {
        let mut value: f32 = 0.0;
        let mut tara_val: f32 = 0.0;
        let mut kg_val: f32 = 0.0;
        for _ in 0..n {
            let reading = block!(hx711.read()).unwrap() as f32;
            value += reading;
        }
        value /= n as f32;
        tara_val = value-zero_value;
        kg_val = tara_val/one_kg_value;
        println!(
            "Read: {} --- Tara val: {} --- kg: {:.2}", 
            value as i32, 
            tara_val as i32, 
            kg_val);
        thread::sleep_ms(10);
        lcd.set_cursor_pos(30);
        lcd.write_str("          ");
        lcd.set_cursor_pos(30);
        let s = format!("{:.0} g.", kg_val * 1000.0);
        lcd.write_str(&s);
        
        // for i in 1..6 {
        //     lcd.set_cursor_pos(0);
        //     lcd.write_str("Customer nr. ");
        //     let s = format!("{}", i);
        //     lcd.write_str(&s);
        //     thread::sleep(time::Duration::from_millis(1000));
        // }
    }
}

// fn blinking_dot()
// {
//     for i in 0..4 {
//         lcd.set_cursor_pos(40);
//         lcd.write_str("Calculating     ");
//         thread::sleep(time::Duration::from_millis(200));
//         lcd.set_cursor_pos(40);
//         lcd.write_str("Calculating.   ");
//         thread::sleep(time::Duration::from_millis(200));
//         lcd.set_cursor_pos(40);
//         lcd.write_str("Calculating..   ");
//         thread::sleep(time::Duration::from_millis(200));
//         lcd.set_cursor_pos(40);
//         lcd.write_str("Calculating...  ");
//         thread::sleep(time::Duration::from_millis(200));
//     }
//}
