use std::thread;
use linux_embedded_hal::{Delay, I2cdev};
use std::sync::mpsc::{channel, Sender, Receiver};

// Result extension
use brain::utils::ResultExt;

/// A class representing MPU6050 info
#[derive(Clone, Copy)]
pub struct MpuGrab {
    pub temp: f32,
    pub acc: (f32,f32,f32),
    pub gyro: (f32,f32,f32)
}

/// A bridge to the MPU6050
pub struct MpuBridge {
    // the private i2c bridge
    i2c: I2cdev,
    // and the public mpu6050
    pub mpu: mpu6050::Mpu6050<I2cdev>,
    // value store
    sender: Sender<MpuGrab>,
    reciever: Receiver<MpuGrab>,
    // the thread object
    grabber_thread: thread::Thread
}

// https://doc.rust-lang.org/std/sync/mpsc/fn.channel.html

impl MpuBridge {
    // 
}


