// our utilities
use brain;

// std utilities
use std::fs;
use csv::Writer;

// I/O tools
use mpu6050;
use linux_embedded_hal::{I2cdev, Delay};
use i2cdev::linux::LinuxI2CError;


// Logging
use log::{info};
use simplelog::*;

// Error Handling
use anyhow::{Result};

fn main() -> Result<(), mpu6050::Mpu6050Error<LinuxI2CError>> {
    // Initialized system loggers
    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Auto)
        ]
    ).unwrap_or(()); 
    // TODO
    
    info!("\n///MODAP/stack///\n  commit: {}\n  timestamp: {}\n  target: {}\n",
          env!("VERGEN_GIT_SHA"),
          env!("VERGEN_GIT_COMMIT_TIMESTAMP"),
          env!("VERGEN_RUSTC_HOST_TRIPLE"));

    let i2c = I2cdev::new("/dev/i2c-0").map_err(mpu6050::Mpu6050Error::I2c)?; // From example, but also probably correct
	// Dummy struct that handles delay functionality using thread::sleep
    let mut delay = Delay; 

    let mut location = brain::Locale::new((0.0,0.0,0.0), 10); // FIXME using documented values bc I don't care about fidelity and zeroing is a good idea

    if std::path::Path::new("accelerations.csv").exists() { //Check for files and delete them if they exist
        fs::remove_file("accelerations.csv").unwrap();
    }

    if std::path::Path::new("localizations.csv").exists() {
        fs::remove_file("localizations.csv").unwrap();
    }

    let mut csv_accels = Writer::from_path("accelerations.csv").unwrap();
    let mut csv_localizations = Writer::from_path("localizations.csv").unwrap();

    let mut mpu = mpu6050::Mpu6050::new(i2c);
    mpu.set_gyro_range(mpu6050::device::GyroRange::D2000)?;
    mpu.set_accel_range(mpu6050::device::AccelRange::G16)?;
    mpu.init(&mut delay)?;

    let mut number = 0;

    loop {
        number += 1;

		// get roll and pitch estimate
        //let wtfthiscodedumb = mpu.get_acc_angles()?;
        //println!("r/p: {:?}", wtfthiscodedumb);

        let temp = mpu.get_temp()?;
        let gyro = mpu.get_gyro()?;
        let acc = mpu.get_acc()?;

        if number % 500 == 0 {
            // get sensor temp
            println!("temp: {:?}c", temp);

            // get gyro data, scaled with sensitivity 
            println!("gyro: {:?}", gyro);

            // get accelerometer data, scaled with sensitivity
            println!("acc: {:?}", acc);
        }

		let acc_tuple = (acc.x, acc.y, acc.z);
		let gyro_tuple = (gyro.x, gyro.y, gyro.z);
		
		location.update(acc_tuple, gyro_tuple).unwrap(); // you can't make me jack

		// write to CSV
		csv_accels.write_record(&[acc.to_string(), gyro.to_string()]).unwrap();
		csv_localizations.write_record(&[
			location.position[0].to_string(),
			location.position[1].to_string(),
			location.position[2].to_string()
		]).unwrap();

		// flush CSV writer
		csv_accels.flush().unwrap();
		csv_localizations.flush().unwrap();
    }
    
}
