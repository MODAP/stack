use brain;
use mpu6050; // TODO Fix (bad)
use linux_embedded_hal::{I2cdev, Delay};
use i2cdev::linux::LinuxI2CError;

/// Oh god.
fn main() {
    let i2c = I2cdev::new("/dev/i2c-1").unwrap(); // From example, but also probably correct
    let mut delay = Delay; // From digging through code it's some delay functionality thing we don't care that much about

    let mut mpu = mpu6050::Mpu6050::new(i2c);
    mpu.init(&mut delay).unwrap();

    loop {
	// get roll and pitch estimate
        let acc = mpu.get_acc_angles().unwrap();
        println!("r/p: {:?}", acc);

        // get sensor temp
        let temp = mpu.get_temp().unwrap();
        println!("temp: {:?}c", temp);

        // get gyro data, scaled with sensitivity 
        let gyro = mpu.get_gyro().unwrap();
        println!("gyro: {:?}", gyro);

        // get accelerometer data, scaled with sensitivity
        let acc = mpu.get_acc().unwrap();
        println!("acc: {:?}", acc);
    }
    
    println!("send. help.");
}
