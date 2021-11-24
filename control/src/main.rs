use brain;
use mpu6050; // TODO Fix (bad)
use linux_embedded_hal::{I2cdev, Delay};
use i2cdev::linux::LinuxI2CError; // Haha this implies I care about errors

use std::error::Error; // probably shouldn't panic when we can't write
use csv::Writer;


/// Oh god. Also Jack I can't unit test this code lol, wanna know why? Are youuuu building the docs connected to an MPU6050 over i2c??? I'm not building the docs connected to an MPU6050 over i2c
fn main() {
    println!("send. help.");
    
    let i2c = I2cdev::new("/dev/i2c-1").unwrap(); // From example, but also probably correct
    let mut delay = Delay; // From digging through code it's some delay functionality thing we don't care that much about

    let mut location = brain::Locale::new((0.0,0.0,0.0), 500); // using documented values bc I don't care about fidelity and zeroing is a good idea

    let mut csv_accels = Writer::from_path("accelerations.csv").unwrap();
    let mut csv_localizations = Writer::from_path("localizations.csv").unwrap();
    
    let mut mpu = mpu6050::Mpu6050::new(i2c);
    mpu.init(&mut delay).unwrap();

    loop {
	// get roll and pitch estimate
        let wtfthiscodedumb = mpu.get_acc_angles().unwrap();
        println!("r/p: {:?}", wtfthiscodedumb);

        // get sensor temp
        let temp = mpu.get_temp().unwrap();
        println!("temp: {:?}c", temp);

        // get gyro data, scaled with sensitivity 
        let gyro = mpu.get_gyro().unwrap();
        println!("gyro: {:?}", gyro);

        // get accelerometer data, scaled with sensitivity
        let acc = mpu.get_acc().unwrap();
        println!("acc: {:?}", acc);

	location.update(acc, gyro).unwrap();

	csv_accels.write_record(&acc).unwrap();
	csv_localizations.write_record(&location.position).unwrap();
    }
    
}
