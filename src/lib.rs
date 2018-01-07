#![deny(warnings)]
#![no_std]

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}


pub trait Accelerometer {
    type Error;

    fn accel(&mut self) -> Result<Vec3, Self::Error>;
}

pub trait Barometer {
    type Error;

    fn pressure(&mut self) -> Result<f32, Self::Error>;
}

pub trait Gyroscope {
    type Error;

    fn gyro(&mut self) -> Result<Vec3, Self::Error>;
}

pub trait Humiditymeter {
    type Error;

    fn hum(&mut self) -> Result<f32, Self::Error>;
}

pub trait Magnetometer {
    type Error;

    fn mag(&mut self) -> Result<Vec3, Self::Error>;
}

pub trait Thermometer {
    type Error;

    fn temp(&mut self) -> Result<f32, Self::Error>;
}
