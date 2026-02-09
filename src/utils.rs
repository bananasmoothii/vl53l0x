pub(crate) fn decode_timeout(register_value: u16) -> u16 {
    // format: "(LSByte * 2^MSByte) + 1"
    ((register_value & 0x00FF) << ((register_value & 0xFF00) >> 8)) + 1
}

pub(crate) fn encode_timeout(timeout_mclks: u16) -> u16 {
    if timeout_mclks == 0 {
        return 0;
    }
    let mut ls_byte: u32;
    let mut ms_byte: u16 = 0;

    ls_byte = (timeout_mclks as u32) - 1;

    while (ls_byte & 0xFFFFFF00) > 0 {
        ls_byte >>= 1;
        ms_byte += 1;
    }

    (ms_byte << 8) | ((ls_byte & 0xFF) as u16)
}

pub(crate) fn calc_macro_period(vcsel_period_pclks: u8) -> u32 {
    ((2304u32 * (vcsel_period_pclks as u32) * 1655u32) + 500u32) / 1000u32
}

pub(crate) fn timeout_mclks_to_microseconds(
    timeout_period_mclks: u16,
    vcsel_period_pclks: u8,
) -> u32 {
    let macro_period_nanoseconds: u32 = calc_macro_period(vcsel_period_pclks);
    (((timeout_period_mclks as u32) * macro_period_nanoseconds)
        + (macro_period_nanoseconds / 2))
        / 1000
}

pub(crate) fn timeout_microseconds_to_mclks(
    timeout_period_microseconds: u32,
    vcsel_period_pclks: u8,
) -> u32 {
    let macro_period_nanoseconds: u32 = calc_macro_period(vcsel_period_pclks);

    ((timeout_period_microseconds * 1000) + (macro_period_nanoseconds / 2))
        / macro_period_nanoseconds
}

// Decode VCSEL (vertical cavity surface emitting laser) pulse period in PCLKs from register value based on VL53L0X_decode_vcsel_period()
pub(crate) fn decode_vcsel_period(register_value: u8) -> u8 {
    ((register_value) + 1) << 1
}

// Encode VCSEL pulse period register value from period in PCLKs based on VL53L0X_encode_vcsel_period()
pub(crate) fn encode_vcsel_period(period_pclks: u8) -> u8 {
    ((period_pclks) >> 1) - 1
}
