//hd44780
use std::thread;
use linux_embedded_hal::Pin;
use linux_embedded_hal::sysfs_gpio::Direction;
use hd44780_hal::HD44780;

//hx711
use rppal::{spi::{Spi, Bus, SlaveSelect, Mode, Error},hal::Delay};
use hx711_spi::Hx711;
use nb::block;

const one_kg_value: f32 = 130670.0;
const N: f32 = 30.0;
const readLoopCount: u8 = 5;

struct AdcData{
    adcRawVal: f32,
    adcVal : f32,
    zeroVal: f32,
    taraVal: f32,
    kgVal: f32,
    previousKgVal: f32,
}

fn main() -> Result<(), Error> 
{
    let mut ADC = AdcData{
        adcRawVal: 0.0,
        adcVal : 0.0,
        zeroVal: 0.0,
        taraVal: 0.0,
        kgVal: 0.0,
        previousKgVal: 0.0,
    };

    //hx711 declarations
    let spi = Spi::new(Bus::Spi0, SlaveSelect::Ss0, 1_000_000, Mode::Mode0)?;
    let mut hx711 = Hx711::new(spi, Delay::new());
    
    //init the screen
	hx711.reset()?;

    //calibration (tara)
    for i in 0..N as i32 {
        let reading = block!(hx711.read()).unwrap() as f32;
        println!("{:>2}: {}", i, reading);
        ADC.zeroVal += reading;
    }
    ADC.zeroVal /= N; //tara
    println!("Tara: {}", ADC.zeroVal);

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
    lcd.write_str("WELCOME!");
    lcd.set_cursor_pos(30);
    lcd.write_str("--- g.");


    let mut counter: u8 = 0;
    let mut flag: bool = false;
    loop {
        ADC.previousKgVal = ADC.kgVal;
        ADC.adcVal = 0.0;
        
        for _ in 0..readLoopCount {
            ADC.adcRawVal = block!(hx711.read()).unwrap() as f32;
            ADC.adcVal += ADC.adcRawVal;
        }
        ADC.adcVal /= readLoopCount as f32;
        ADC.taraVal = ADC.adcVal-ADC.zeroVal;
        ADC.kgVal = ADC.taraVal/one_kg_value;
        println!(
            "Read: {} --- Tara val: {} --- kg: {:.3}", 
            ADC.adcVal as i32, 
            ADC.taraVal as i32, 
            ADC.kgVal);
                
        if (ADC.kgVal - ADC.previousKgVal) > 0.002 {
            println!("--- START LISTENING --- {}", ADC.kgVal - ADC.previousKgVal);
            lcd.set_cursor_pos(40);
            lcd.write_str("Calculating...");
        }

        if (ADC.previousKgVal - ADC.kgVal) > 0.002 {
            println!("---STOP LISTENING--- To be added: {:.3}", ADC.kgVal);
            lcd.set_cursor_pos(40);
            lcd.write_str("Almost there...");

            flag = true;
            counter = 0;
        }
        
        if flag == true {
            counter += 1;
            lcd.set_cursor_pos(30);
            let s = format!("{:.0} g.   ", (ADC.kgVal * 1000.0).abs());
            lcd.write_str(&s);
            if counter == 5{
                println!("{:.3} added to customer", ADC.kgVal);
                
                lcd.set_cursor_pos(40);
                lcd.write_str("Green salad:   ");
                flag = false;
                counter = 0;
            }
        }
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
// }
