/// Provides an enum representing all units of IOTA
#[derive(PartialEq, Clone, Copy, Debug)]
pub enum IotaUnits {
    /// Base unit of IOTA
    Iota,
    /// 1,000 Iota
    KiloIota,
    /// 1,000,000 Iota
    MegaIota,
    /// 1,000,000,000 Iota
    GigaIota,
    /// 1,000,000,000,000 Iota
    TeraIota,
    /// 1,000,000,000,000,000 Iota
    PetaIota,
}

impl IotaUnits {
    /// Provides the unit string associated with this unit
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

    /// Provides the number of significant digits associated with this unit
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
