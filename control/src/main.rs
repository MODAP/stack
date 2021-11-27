use brain;
use mpu6050;
use linux_embedded_hal::{I2cdev, Delay};
use i2cdev::linux::LinuxI2CError; // Haha this implies I care about errors

use std::error::Error; // probably shouldn't panic when we can't write but ditto
use csv::Writer;


/// Oh god. Also Jack I can't unit test this code lol, wanna know why? Are youuuu building the docs connected to an MPU6050 over i2c??? I'm not building the docs connected to an MPU6050 over i2c
fn main() -> Result<(), mpu6050::Mpu6050Error<LinuxI2CError>> {
    println!("send. help.");
    
    let i2c = I2cdev::new("/dev/i2c-0").map_err(mpu6050::Mpu6050Error::I2c)?; // From example, but also probably correct
    let mut delay = Delay; // Uhhhh from digging through code it's some delay functionality thing we don't care that much about

    let mut location = brain::Locale::new((0.0,0.0,0.0), 500); // FIXME using documented values bc I don't care about fidelity and zeroing is a good idea

    let mut csv_accels = Writer::from_path("accelerations.csv").unwrap();
    let mut csv_localizations = Writer::from_path("localizations.csv").unwrap();
    
    let mut mpu = mpu6050::Mpu6050::new(i2c);
    mpu.init(&mut delay)?;

    loop {
	// get roll and pitch estimate
        let wtfthiscodedumb = mpu.get_acc_angles()?;
        println!("r/p: {:?}", wtfthiscodedumb);

        // get sensor temp
        let temp = mpu.get_temp()?;
        println!("temp: {:?}c", temp);

        // get gyro data, scaled with sensitivity 
        let gyro = mpu.get_gyro()?;
        println!("gyro: {:?}", gyro);

        // get accelerometer data, scaled with sensitivity
        let acc = mpu.get_acc()?;
        println!("acc: {:?}", acc);

	let acc_tuple = (acc.x, acc.y, acc.z);
	let gyro_tuple = (gyro.x, gyro.y, gyro.z);
	
	location.update(acc_tuple, gyro_tuple).unwrap(); // you can't make me jack

	// write to CSV
	csv_accels.write_record(&[acc.to_string(), gyro.to_string()]).unwrap();
	csv_localizations.write_record(&[location.position[0].to_string(), location.position[1].to_string(), location.position[2].to_string()]).unwrap();

	// flush CSV writer
	csv_accels.flush().unwrap();
	csv_localizations.flush().unwrap();
    }
    
}
