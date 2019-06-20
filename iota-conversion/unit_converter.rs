use super::iota_units::IotaUnits;

/// Converts an amount of iotas to a new unit
///
/// * `amount` - Amount to convert
/// * `from` - IotaUnit that `amount` is in
/// * `to` - Target IotaUnit
///```
/// extern crate iota_conversion;
/// use iota_conversion::{iota_units::IotaUnits, unit_converter};
///
/// let amount_in_new_unit = unit_converter::convert_units(1000, IotaUnits::TeraIota, IotaUnits::PetaIota);
/// assert_eq!(amount_in_new_unit, 1);
///```
pub fn convert_units(amount: u64, from: IotaUnits, to: IotaUnits) -> u64 {
    let amount_in_source = amount * 10_u64.pow(u32::from(from.value()));
    convert_units_helper(amount_in_source, to)
}

fn convert_units_helper(amount: u64, to: IotaUnits) -> u64 {
    amount / 10_u64.pow(u32::from(to.value()))
}

/// Converts an iota amount into the optimal unit for display
///
/// * `amount` - amount in base Iota unit
/// * `extended` - Whether to use two significant digests, or 15
///```
/// extern crate iota_conversion;
/// use iota_conversion::{iota_units::IotaUnits, unit_converter};
///
/// let s = unit_converter::convert_raw_iota_amount_to_display_text(1000000, false);
/// assert_eq!(s, "1.00 Mi");
///
/// let extended_s = unit_converter::convert_raw_iota_amount_to_display_text(1900000000000002, true);
/// assert_eq!(extended_s, "1.900000000000002 Pi");
///```
pub fn convert_raw_iota_amount_to_display_text(amount: u64, extended: bool) -> String {
    let unit = find_optimal_iota_unit_to_display(amount);
    let amount_in_display_unit = convert_amount_to(amount, unit);
    create_amount_with_unit_display_text(amount_in_display_unit, unit, extended)
}

fn create_amount_with_unit_display_text(amount: f64, unit: IotaUnits, extended: bool) -> String {
    if unit == IotaUnits::Iota {
        format!("{} {}", amount, unit.unit())
    } else if extended {
        format!("{:.15} {}", amount, unit.unit())
    } else {
        format!("{:.2} {}", amount, unit.unit())
    }
}

/// Converts an amount of iota to a unit
///
/// * `amount` - Amount in base Iota unit
/// * `target` - Target IotaUnit
///```
/// extern crate iota_conversion;
/// use iota_conversion::{iota_units::IotaUnits, unit_converter};
///
/// let unit = unit_converter::convert_amount_to(1000000, IotaUnits::GigaIota);
/// assert_eq!(unit, 0.001);
///```
pub fn convert_amount_to(amount: u64, target: IotaUnits) -> f64 {
    amount as f64 / 10_u64.pow(u32::from(target.value())) as f64
}

/// Finds the optimal unit for displaying an iota amount
///
/// * `amount` - Amount in base Iota unit
///```
/// extern crate iota_conversion;
/// use iota_conversion::{iota_units::IotaUnits, unit_converter};
///
/// let unit = unit_converter::find_optimal_iota_unit_to_display(1000000);
/// assert_eq!(unit, IotaUnits::MegaIota);
///```
pub fn find_optimal_iota_unit_to_display(amount: u64) -> IotaUnits {
    let length = amount.to_string().len();

    if length >= 1 && length <= 3 {
        IotaUnits::Iota
    } else if length > 3 && length <= 6 {
        IotaUnits::KiloIota
    } else if length > 6 && length <= 9 {
        IotaUnits::MegaIota
    } else if length > 9 && length <= 12 {
        IotaUnits::GigaIota
    } else if length > 12 && length <= 15 {
        IotaUnits::TeraIota
    } else if length > 15 && length <= 18 {
        IotaUnits::PetaIota
    } else {
        panic!("Invalid number")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_unit_i_to_ki() {
        assert_eq!(1, convert_units(1000, IotaUnits::Iota, IotaUnits::KiloIota));
    }

    #[test]
    fn test_convert_unit_ki_to_mi() {
        assert_eq!(
            1,
            convert_units(1000, IotaUnits::KiloIota, IotaUnits::MegaIota)
        );
    }

    #[test]
    fn test_convert_unit_mi_to_gi() {
        assert_eq!(
            1,
            convert_units(1000, IotaUnits::MegaIota, IotaUnits::GigaIota)
        );
    }

    #[test]
    fn test_convert_unit_gi_to_ti() {
        assert_eq!(
            1,
            convert_units(1000, IotaUnits::GigaIota, IotaUnits::TeraIota)
        );
    }

    #[test]
    fn test_convert_unit_ti_to_pi() {
        assert_eq!(
            1,
            convert_units(1000, IotaUnits::TeraIota, IotaUnits::PetaIota)
        );
    }

    #[test]
    fn test_find_optimize_unit_to_display() {
        assert_eq!(find_optimal_iota_unit_to_display(1), IotaUnits::Iota);
        assert_eq!(find_optimal_iota_unit_to_display(1000), IotaUnits::KiloIota);
        assert_eq!(
            find_optimal_iota_unit_to_display(1000000),
            IotaUnits::MegaIota
        );
        assert_eq!(
            find_optimal_iota_unit_to_display(1000000000),
            IotaUnits::GigaIota
        );
        assert_eq!(
            find_optimal_iota_unit_to_display(1000000000000),
            IotaUnits::TeraIota
        );
        assert_eq!(
            find_optimal_iota_unit_to_display(1000000000000000),
            IotaUnits::PetaIota
        );
    }

    #[test]
    fn test_convert_raw_iota_amount_to_display_text() {
        assert_eq!(convert_raw_iota_amount_to_display_text(1, false), "1 i");
        assert_eq!(
            convert_raw_iota_amount_to_display_text(1000, false),
            "1.00 Ki"
        );
        assert_eq!(
            convert_raw_iota_amount_to_display_text(1000000, false),
            "1.00 Mi"
        );
        assert_eq!(
            convert_raw_iota_amount_to_display_text(1000000000, false),
            "1.00 Gi"
        );
        assert_eq!(
            convert_raw_iota_amount_to_display_text(1000000000000, false),
            "1.00 Ti"
        );
        assert_eq!(
            convert_raw_iota_amount_to_display_text(1000000000000000, false),
            "1.00 Pi"
        );
        assert_eq!(
            convert_raw_iota_amount_to_display_text(1900000000000002, true),
            "1.900000000000002 Pi"
        );
    }
}
