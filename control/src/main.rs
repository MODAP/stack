use brain;
use brain::utils::ResultExt;

use csv::Writer;

use linux_embedded_hal::{Delay, I2cdev};

fn setup_mpu() -> anyhow::Result<mpu6050::Mpu6050<I2cdev>> {
    // TODO @jemoka please clarify
    let i2c = I2cdev::new("/dev/i2c-0").map_err(mpu6050::Mpu6050Error::I2c).anyhow()?; // From example, but also probably correct

    // Dummy struct that handles delay functionality using thread::sleep
    let mut delay = Delay;

    let mut mpu = mpu6050::Mpu6050::new(i2c);
    mpu.set_gyro_range(mpu6050::device::GyroRange::D2000).anyhow()?;
    mpu.set_accel_range(mpu6050::device::AccelRange::G16).anyhow()?;
    mpu.init(&mut delay).anyhow()?;
    Ok(mpu)
}

fn main() -> anyhow::Result<()> {
    if let Err(e) = brain::utils::setup_logging() {
        println!("Logging failed to start with error {:?}", e);
    }

    log::info!("commit: {}", env!("VERGEN_GIT_SHA"));
    log::info!("timestamp: {}", env!("VERGEN_GIT_COMMIT_TIMESTAMP"));
    log::info!("target: {}", env!("VERGEN_RUSTC_HOST_TRIPLE"));

    let mut csv_accels =
        Writer::from_path("accelerations.csv").log_error("Accelerations CSV logging failed");
    let mut csv_localizations =
        Writer::from_path("localizations.csv").log_error("Localizations CSV logging failed");

    let mut mpu = setup_mpu().ok();

    let mut cycles: usize = 0;
    loop {
        cycles += 1;

        if let Some(mpu) = &mut mpu {
            let temp = mpu.get_temp().log_error("Cannot grab temperature");
            let gyro = mpu.get_gyro().log_error("Cannot grab gyroscope info");
            let acc = mpu.get_acc().log_error("Cannot grab acceleration info");

            // HACK Could be better.
            if cycles % 500 == 0 {
                if let Some(temp) = temp {
                    log::trace!("temp: {}c", temp);
                }
                if let Some(gyro) = gyro {
                    log::trace!("gyro: {:?}", gyro);
                }
                if let Some(acc) = acc {
                    log::trace!("accel: {:?}", acc);
                }
            }

            if let (Some(acc), Some(gyro)) = (&acc, &gyro) {
                let acc_tuple = (acc.x, acc.y, acc.z);
                let gyro_tuple = (gyro.x, gyro.y, gyro.z);

                // FIXME using documented values bc I don't care about fidelity and zeroing is a good idea
                let mut location = brain::Locale::new((0.0, 0.0, 0.0), 10);
                location.update(acc_tuple, gyro_tuple).log_error("Failed to fetch new location");

                if let Some(csv_accels) = &mut csv_accels {
                    csv_accels
                        .write_record(&[acc.to_string(), gyro.to_string()])
                        .log_error("Could not write accel CSV");
                    csv_accels.flush().log_error("Could not flush CSV Writer");
                };
                if let (location, Some(csv_localizations)) = (location, &mut csv_localizations) {
                    csv_localizations
                        .write_record(&[
                            location.position[0].to_string(),
                            location.position[1].to_string(),
                            location.position[2].to_string(),
                        ])
                        .log_error("Could not write localization CSV");
                    csv_localizations.flush().log_error("Could not flush CSV Writer");
                }
            }
        }
    }
}
