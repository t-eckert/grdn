pub struct SoilSensor {
    pub name: String,
    pin: u8,
}

impl SoilSensor {
    pub fn new(name: String, pin: u8) -> SoilSensor {
        SoilSensor { name, pin }
    }
}

impl Sensor for SoilSensor {
    fn read(&self) -> Report {
        let value = 0.0;
        Report {
            source: self.name.clone(),
            value,
            unit: "percent".to_string(),
        }
    }
}
