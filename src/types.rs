#[derive(Debug, Copy, Clone)]
pub(crate) enum VcselPeriodType {
    VcselPeriodPreRange = 0,
    VcselPeriodFinalRange = 1,
}

/// Which event is routed to the GPIO pin.
#[derive(Copy, Clone, Debug)]
pub enum GpioFunctionality {
    /// Interrupt on new sample ready (default after init)
    NewSampleReady,
    /// Interrupt on range below low threshold
    LevelLow,
    /// Interrupt on range above high threshold
    LevelHigh,
    /// Interrupt on range outside window (below low OR above high)
    OutOfWindow,
}

/// Active level of the interrupt pin.
#[derive(Copy, Clone, Debug)]
pub enum GpioPolarity {
    /// GPIO pulls low when interrupt occurs (default after init)
    ActiveLow,
    /// GPIO pulls high when interrupt occurs
    ActiveHigh,
}

pub(crate) struct SeqStepEnables {
    pub(crate) tcc: bool,
    pub(crate) dss: bool,
    pub(crate) msrc: bool,
    pub(crate) pre_range: bool,
    pub(crate) final_range: bool,
}

pub(crate) struct SeqStepTimeouts {
    pub(crate) pre_range_vcselperiod_pclks: u8,
    pub(crate) final_range_vcsel_period_pclks: u8,
    pub(crate) msrc_dss_tcc_mclks: u8,
    pub(crate) pre_range_mclks: u16,
    pub(crate) final_range_mclks: u16,
    pub(crate) msrc_dss_tcc_microseconds: u32,
    pub(crate) pre_range_microseconds: u32,
    pub(crate) final_range_microseconds: u32,
}
