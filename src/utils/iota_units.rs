#[derive(PartialEq, Clone, Copy, Debug)]
pub enum IotaUnits {
    Iota,
    KiloIota,
    MegaIota,
    GigaIota,
    TeraIota,
    PetaIota,
}

impl IotaUnits {
    pub fn unit(self) -> &'static str {
        match self {
            IotaUnits::Iota => "i",
            IotaUnits::KiloIota => "Ki",
            IotaUnits::MegaIota => "Mi",
            IotaUnits::GigaIota => "Gi",
            IotaUnits::TeraIota => "Ti",
            IotaUnits::PetaIota => "Pi",
        }
    }

    pub fn value(self) -> u8 {
        match self {
            IotaUnits::Iota => 0,
            IotaUnits::KiloIota => 3,
            IotaUnits::MegaIota => 6,
            IotaUnits::GigaIota => 9,
            IotaUnits::TeraIota => 12,
            IotaUnits::PetaIota => 15,
        }
    }
}
