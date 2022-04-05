
This is an example of the 2004 LCD screen and HX711 ADC up and running on Rust.

More about the system:

This is a system called Smart Messroom, built in 2018 on Raspberry Pi and Windows IoT Core. It should be rebuilt and able to run on Rust for a high level of protection and speed. 
The system is not a weight scale! It measures food all the time. After a certain amount of food has been lifted from the food container, it starts calculating the difference in weight between NOW and the time the weight started to differ (food to be taken).

See more on the video below:

https://1drv.ms/v/s!Ak5sft2RFM38jb4jOWfsavIP3ROe2A?e=IfUv9y

Note: Give it a few seconds to load as onedrive is a bit slower.

At this stage it can only show some basic strings but it will get more complex as soon as we're able to read adequate data from the ADC and the load cell.

The near-future goal for the Smart Messroom is:

 (DONE!) - To make the ADC (Analogue-to-Digital-Converter) work with Rust and RPI2. (See repo HX711-rust-pi)

The long-term goals for this system would be the following:

 - To display the customer number, product and quantity on the LCD 2004 screen.
 - Having a touch-friendly GUI on the 7-inch Touchscreen for the worker to operate with the device.
 - Having a payment system integrated, eventually with cryptocurrency or fiat currency.
