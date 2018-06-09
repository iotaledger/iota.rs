#[derive(PartialEq)]
pub enum IotaUnits {
    IOTA,
    KILO_IOTA,
    MEGA_IOTA,
    GIGA_IOTA,
    TERA_IOTA,
    PETA_IOTA,
}

impl IotaUnits {
    pub fn unit(&self) -> &'static str {
        match self {
            IotaUnits::IOTA => "i",
            IotaUnits::KILO_IOTA => "Ki",
            IotaUnits::MEGA_IOTA => "Mi",
            IotaUnits::GIGA_IOTA => "Gi",
            IotaUnits::TERA_IOTA => "Ti",
            IotaUnits::PETA_IOTA => "Pi",
        }
    }

    pub fn value(&self) -> u8 {
        match self {
            IotaUnits::IOTA => 0,
            IotaUnits::KILO_IOTA => 3,
            IotaUnits::MEGA_IOTA => 6,
            IotaUnits::GIGA_IOTA => 9,
            IotaUnits::TERA_IOTA => 12,
            IotaUnits::PETA_IOTA => 15,
        }
    }
}
