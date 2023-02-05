use crate::report::Report;

pub trait Sensor {
    fn read(&self) -> Report;
}
