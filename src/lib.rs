use errno::{errno, Errno};
use num_enum::TryFromPrimitive;
use thiserror::Error;

pub use libryzenadj_sys;

///  Enumerates the possible errors returned from ryzenadj
#[derive(Error, Debug)]
pub enum RyzenAdjError {
    /// ryzenadj struct init failed, provides `errno` returned by the library
    #[error("ryzenadj init failed: {} {}", errno, errno.0)]
    InitError { errno: Errno },
    /// this error is returned when ryzenadj can not read out values from msr, setting values might still work
    #[error("ryzenadj table init failed: {0}, set functions might still work")]
    InitTableError(i32),
    /// reading given value returned a NaN float
    #[error("ryzenadj get returned NaN")]
    GetNaN,
    /// given cpu family is not supported by this crate
    #[error("ryzenadj familly: {0} is not know")]
    UnknowFamily(i32),
    /// given cpu family is not supported by ryzenadj
    #[error("ryzenadj adj family not supported")]
    AdjFamilyNotSupported,
    /// ryzenadj encured a memory access error
    #[error("ryzenadj adj memory access error")]
    AdjMemoryAccessError,
    /// the cpu smu rejected the given value
    #[error("ryzenadj adj smu rejected")]
    AdjSmuRejected,
    /// the cpu smu timeout out when trying to set the value
    #[error("ryzenadj adj smu timeout")]
    AdjSmuTimeout,
    /// the cpu smu responed with setting this value is unsupported
    #[error("ryzenadj adj smu unsupported")]
    AdjSmuUnsupported,
    /// unknow error ocured when tring to set give value
    #[error("ryzenadj adj unknow error {0}")]
    AdjUnknowError(i32),
    /// given value is out of allowed range
    #[error("ryzenadj adj value out or range")]
    AdjValueOutOfRange,
}
/// libryzenadj result type returned by all available functions
pub type RyzenAdjResult<T> = Result<T, RyzenAdjError>;

/// Struct holding access to an open instance of ryzenadj
pub struct RyzenAdj {
    ryzen_adj: libryzenadj_sys::ryzen_access,
    init_table_result: Option<i32>,
}

/// Enumerates supported CPU families
#[derive(Debug, TryFromPrimitive)]
#[non_exhaustive]
#[repr(i32)]
pub enum RyzenFamily {
    /// Unknow CPU family
    Unknow = libryzenadj_sys::ryzen_family_FAM_UNKNOWN,
    /// Ryzen 2XXX and a few Athlons
    Raven = libryzenadj_sys::ryzen_family_FAM_RAVEN,
    /// Ryzen 3XXX and a few Athlons
    Picassso = libryzenadj_sys::ryzen_family_FAM_PICASSO,
    /// Ryzen 4XXX
    Renoir = libryzenadj_sys::ryzen_family_FAM_RENOIR,
    /// Ryzen 5XXX APUs only
    Cezanne = libryzenadj_sys::ryzen_family_FAM_CEZANNE,
    /// a few lower power Ryzen 3XXX
    Dali = libryzenadj_sys::ryzen_family_FAM_DALI,
    /// a few lower power Ryzen 5XXX
    Lucienne = libryzenadj_sys::ryzen_family_FAM_LUCIENNE,
    /// Athlon 4XXX?
    Vangogh = libryzenadj_sys::ryzen_family_FAM_VANGOGH,
    /// Ryzen 6XXX
    Rembrandt = libryzenadj_sys::ryzen_family_FAM_REMBRANDT,
}

impl RyzenAdj {
    /// Returns a new RyzenAdj instance
    pub fn new() -> RyzenAdjResult<Self> {
        let ryzen_adj = unsafe { libryzenadj_sys::init_ryzenadj() };

        if ryzen_adj.is_null() {
            Err(RyzenAdjError::InitError { errno: errno() })
        } else {
            let init_table_result = unsafe { libryzenadj_sys::init_table(ryzen_adj) };

            let init_table_result = if init_table_result != 0 {
                Some(init_table_result)
            } else {
                None
            };

            Ok(Self {
                ryzen_adj,
                init_table_result,
            })
        }
    }

    fn is_init_table(&self) -> RyzenAdjResult<()> {
        if let Some(init_table_result) = self.init_table_result {
            Err(RyzenAdjError::InitTableError(init_table_result))
        } else {
            Ok(())
        }
    }

    fn is_nan(value: f32) -> RyzenAdjResult<f32> {
        if value.is_nan() {
            Err(RyzenAdjError::GetNaN)
        } else {
            Ok(value)
        }
    }

    fn adj_code(code: i32) -> RyzenAdjResult<()> {
        match code {
            0 => Ok(()),
            libryzenadj_sys::ADJ_ERR_FAM_UNSUPPORTED => Err(RyzenAdjError::AdjFamilyNotSupported),
            libryzenadj_sys::ADJ_ERR_MEMORY_ACCESS => Err(RyzenAdjError::AdjMemoryAccessError),
            libryzenadj_sys::ADJ_ERR_SMU_REJECTED => Err(RyzenAdjError::AdjSmuRejected),
            libryzenadj_sys::ADJ_ERR_SMU_TIMEOUT => Err(RyzenAdjError::AdjSmuTimeout),
            libryzenadj_sys::ADJ_ERR_SMU_UNSUPPORTED => Err(RyzenAdjError::AdjSmuUnsupported),
            _ => Err(RyzenAdjError::AdjUnknowError(code)),
        }
    }
    /// Refresh current readed values from the CPU
    pub fn refresh(&self) -> RyzenAdjResult<()> {
        self.is_init_table()?;
        let result = unsafe { libryzenadj_sys::refresh_table(self.ryzen_adj) };
        if result != 0 {
            Err(RyzenAdjError::InitTableError(result))
        } else {
            Ok(())
        }
    }
    /// Gets the APU skin temperature limit
    pub fn get_apu_skin_temp_limit(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_apu_skin_temp_limit(self.ryzen_adj) })
    }
    /// Gets the APU skin temperature value
    pub fn get_apu_skin_temp_value(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_apu_skin_temp_value(self.ryzen_adj) })
    }
    /// Gets the APU slow limit
    pub fn get_apu_slow_limit(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_apu_slow_limit(self.ryzen_adj) })
    }
    /// Gets the APU slow value
    pub fn get_apu_slow_value(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_apu_slow_value(self.ryzen_adj) })
    }
    /// Gets bios ver
    pub fn get_bios_if_ver(&self) -> RyzenAdjResult<i32> {
        self.is_init_table()?;
        Ok(unsafe { libryzenadj_sys::get_bios_if_ver(self.ryzen_adj) })
    }
    /// Gets cclk busy value
    pub fn get_cclk_busy_value(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_cclk_busy_value(self.ryzen_adj) })
    }
    /// Gets cclk setpoint
    pub fn get_cclk_setpoint(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_cclk_setpoint(self.ryzen_adj) })
    }
    /// Gets current core clk
    pub fn get_core_clk(&self, core: u32) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_core_clk(self.ryzen_adj, core) })
    }
    /// Gets current core power
    pub fn get_core_power(&self, core: u32) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_core_power(self.ryzen_adj, core) })
    }
    /// Gets current core temp
    pub fn get_core_temp(&self, core: u32) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_core_temp(self.ryzen_adj, core) })
    }
    /// Gets current core volt
    pub fn get_core_volt(&self, core: u32) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_core_volt(self.ryzen_adj, core) })
    }
    /// Gets the cpu family
    pub fn get_cpu_family(&self) -> RyzenAdjResult<RyzenFamily> {
        self.is_init_table()?;
        let family_int = unsafe { libryzenadj_sys::get_cpu_family(self.ryzen_adj) };
        RyzenFamily::try_from(family_int).map_err(|_| RyzenAdjError::UnknowFamily(family_int))
    }
    /// Gets the dgpu skin temp limit
    pub fn get_dgpu_skin_temp_limit(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_dgpu_skin_temp_limit(self.ryzen_adj) })
    }
    /// Gets the dgpu skin temp value
    pub fn get_dgpu_skin_temp_value(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_dgpu_skin_temp_value(self.ryzen_adj) })
    }
    /// Gets the fast limit
    pub fn get_fast_limit(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_fast_limit(self.ryzen_adj) })
    }
    /// Gets the fast value
    pub fn get_fast_value(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_fast_value(self.ryzen_adj) })
    }
    /// Gets fclk
    pub fn get_fclk(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_fclk(self.ryzen_adj) })
    }
    /// Gets gfx tmp
    pub fn get_gfx_temp(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_gfx_temp(self.ryzen_adj) })
    }
    /// Gets gfx clk
    pub fn get_gfx_clk(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_gfx_clk(self.ryzen_adj) })
    }
    /// Gets gfx volt
    pub fn get_gfx_volt(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_gfx_volt(self.ryzen_adj) })
    }
    /// Gets l3 cache clk
    pub fn get_l3_clk(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_l3_clk(self.ryzen_adj) })
    }
    /// Gets l3 logic
    pub fn get_l3_logic(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_l3_logic(self.ryzen_adj) })
    }
    /// Gets l3 temp
    pub fn get_l3_temp(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_l3_temp(self.ryzen_adj) })
    }
    /// Gets l3 vddm
    pub fn get_l3_vddm(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_l3_vddm(self.ryzen_adj) })
    }
    /// Gets mem clk
    pub fn get_mem_clk(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_mem_clk(self.ryzen_adj) })
    }
    /// Gets psi0 current
    pub fn get_psi0_current(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_psi0_current(self.ryzen_adj) })
    }
    /// Gets psi0soc current
    pub fn get_psi0soc_current(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_psi0soc_current(self.ryzen_adj) })
    }
    /// Gets slow limit
    pub fn get_slow_limit(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_slow_limit(self.ryzen_adj) })
    }
    /// Gets slow time
    pub fn get_slow_time(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_slow_time(self.ryzen_adj) })
    }
    /// Gets slow value
    pub fn get_slow_value(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_slow_value(self.ryzen_adj) })
    }
    /// Gets soc power
    pub fn get_soc_power(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_soc_power(self.ryzen_adj) })
    }
    /// Gets soc voltage
    pub fn get_soc_volt(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_soc_volt(self.ryzen_adj) })
    }
    /// Gets socket power
    pub fn get_socket_power(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_socket_power(self.ryzen_adj) })
    }
    /// Gets stamp limit
    pub fn get_stapm_limit(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_stapm_limit(self.ryzen_adj) })
    }
    /// Gets stamp time
    pub fn get_stapm_time(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_stapm_time(self.ryzen_adj) })
    }
    /// Gets stamp value
    pub fn get_stapm_value(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_stapm_value(self.ryzen_adj) })
    }
    /// Gets tctl temp
    pub fn get_tctl_temp(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_tctl_temp(self.ryzen_adj) })
    }
    /// Gets tctl temp value
    pub fn get_tctl_temp_value(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_tctl_temp(self.ryzen_adj) })
    }
    /// Gets vrm current
    pub fn get_vrm_current(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_vrm_current(self.ryzen_adj) })
    }
    /// Gets vrm current value
    pub fn get_vrm_current_value(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_vrm_current_value(self.ryzen_adj) })
    }
    /// Gets vrmmax current
    pub fn get_vrmmax_current(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_vrmmax_current(self.ryzen_adj) })
    }
    /// Gets vrmmax current value
    pub fn get_vrmmax_current_value(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_vrmmax_current_value(self.ryzen_adj) })
    }
    /// Gets vrmsoc current
    pub fn get_vrmsoc_current(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_vrmsoc_current(self.ryzen_adj) })
    }
    /// Gets vrmsoc current value
    pub fn get_vrmsoc_current_value(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_vrmsoc_current_value(self.ryzen_adj) })
    }
    /// Gets vrmsocmax current
    pub fn get_vrmsocmax_current(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_vrmsocmax_current(self.ryzen_adj) })
    }
    /// Gets vrmsocmax current value
    pub fn get_vrmsocmax_current_value(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_vrmsocmax_current_value(self.ryzen_adj) })
    }
    /// Sets the apu skin temp limit
    pub fn set_apu_skin_temp_limit(&self, value: u32) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe { libryzenadj_sys::set_apu_skin_temp_limit(self.ryzen_adj, value) })
    }
    /// Sets the apu slow limit
    pub fn set_apu_slow_limit(&self, value: u32) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe { libryzenadj_sys::set_apu_slow_limit(self.ryzen_adj, value) })
    }
    /// Sets the all core curve optimiser
    ///
    /// Calling this function with a wrong value might crash you system
    /// those marking it as unsafe
    ///
    /// # Safety
    /// - `value` needs to be in proper range, the base of this value is 0x100000 and can go -/+ 30 decimal
    pub unsafe fn set_unsafe_coall(&self, value: u32) -> RyzenAdjResult<()> {
        Self::adj_code(libryzenadj_sys::set_coall(self.ryzen_adj, value))
    }
    /// Sets the igpu curve optimiser
    ///
    /// Calling this function with a wrong value might crash you system
    /// those marking it as unsafe
    ///
    /// # Safety
    /// - `value` needs to be in proper range, the base of this value is 0x100000 and can go -/+ 30 decimal
    pub unsafe fn set_unsafe_cogfx(&self, value: u32) -> RyzenAdjResult<()> {
        Self::adj_code(libryzenadj_sys::set_cogfx(self.ryzen_adj, value))
    }
    /// Sets the per core curve optimiser
    ///
    /// Calling this function with a wrong value might crash you system
    /// those marking it as unsafe
    ///
    /// # Safety
    /// - `value` needs to be in proper range, the base of this value is 0x100000
    /// the formula for per core Curve Optimizer (on a single CCD mobile APU) is <core number> * 0x100000 + ((0x100000 + <value>) & 0xFFFFF).
    /// for example to set -10 on core no.2 and -5 on core no.3 it's:
    pub unsafe fn set_unsafe_coper(&self, value: u32) -> RyzenAdjResult<()> {
        Self::adj_code(libryzenadj_sys::set_coall(self.ryzen_adj, value))
    }

    /// Sets the all core curve optimiser
    pub fn set_coall(&self, value: i32) -> RyzenAdjResult<()> {
        if (-30..=30).contains(&value) {
            let value = 0x100000 - value;
            Self::adj_code(unsafe { libryzenadj_sys::set_coall(self.ryzen_adj, value as u32) })
        } else {
            Err(RyzenAdjError::AdjValueOutOfRange)
        }
    }

    /// Sets the per core curve optimiser
    pub fn set_coper(&self, core: u32, value: i32) -> RyzenAdjResult<()> {
        if (-30..=30).contains(&value) {
            let value = (core * 0x100000) as i32 - value;
            Self::adj_code(unsafe { libryzenadj_sys::set_coall(self.ryzen_adj, value as u32) })
        } else {
            Err(RyzenAdjError::AdjValueOutOfRange)
        }
    }

    /// Sets the dgpu skin temp limit
    pub fn set_dgpu_skin_temp_limit(&self, value: u32) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe { libryzenadj_sys::set_coall(self.ryzen_adj, value) })
    }
    /// Enable overclock (Renoir and up Only)
    pub fn set_enable_oc(&self) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe { libryzenadj_sys::set_enable_oc(self.ryzen_adj) })
    }
    /// Disable overclock (Renoir and up Only)
    pub fn set_disable_oc(&self) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe { libryzenadj_sys::set_disable_oc(self.ryzen_adj) })
    }
    /// Sets the fast limit
    pub fn set_fast_limit(&self, value: u32) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe { libryzenadj_sys::set_fast_limit(self.ryzen_adj, value) })
    }
    /// Sets the gfx clk
    pub fn set_gfx_clk(&self, value: u32) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe { libryzenadj_sys::set_gfx_clk(self.ryzen_adj, value) })
    }
    /// Sets maximum Transmission (CPU-GPU) Frequency
    pub fn set_max_fclk_freq(&self, value: u32) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe { libryzenadj_sys::set_max_fclk_freq(self.ryzen_adj, value) })
    }
    /// Sets max gfxclk frequency
    pub fn set_max_gfxclk_freq(&self, value: u32) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe { libryzenadj_sys::set_max_gfxclk_freq(self.ryzen_adj, value) })
    }
    /// Sets the max lclk
    pub fn set_max_lclk(&self, value: u32) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe { libryzenadj_sys::set_max_lclk(self.ryzen_adj, value) })
    }
    /// Sets max perfomence mode
    pub fn set_max_performance(&self) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe { libryzenadj_sys::set_max_performance(self.ryzen_adj) })
    }
    /// Sets max socclk freq
    pub fn set_max_socclk_freq(&self, value: u32) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe { libryzenadj_sys::set_max_socclk_freq(self.ryzen_adj, value) })
    }
    /// Sets maximum Video Core Next freq
    pub fn set_max_vcn(&self, value: u32) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe { libryzenadj_sys::set_max_vcn(self.ryzen_adj, value) })
    }
    /// Sets minimum Transmission (CPU-GPU) Frequency (MHz)
    pub fn set_min_fclk_freq(&self, value: u32) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe { libryzenadj_sys::set_min_fclk_freq(self.ryzen_adj, value) })
    }
    /// Sets min gfxclk frequency
    pub fn set_min_gfxclk_freq(&self, value: u32) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe { libryzenadj_sys::set_min_gfxclk_freq(self.ryzen_adj, value) })
    }
    /// Sets min lclk
    pub fn set_min_lclk(&self, value: u32) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe { libryzenadj_sys::set_min_lclk(self.ryzen_adj, value) })
    }
    /// Sets min socclk freq
    pub fn set_min_socclk_freq(&self, value: u32) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe { libryzenadj_sys::set_min_socclk_freq(self.ryzen_adj, value) })
    }
    /// Sets min Video Core Next freq
    pub fn set_min_vcn(&self, value: u32) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe { libryzenadj_sys::set_min_vcn(self.ryzen_adj, value) })
    }
    /// Sets forced Core Clock Speed in MHz (Renoir and up Only)
    pub fn set_oc_clk(&self, value: u32) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe { libryzenadj_sys::set_oc_clk(self.ryzen_adj, value) })
    }
    /// Sets forced Core VID: Must follow this calcuation (1.55 - [VID you want to set e.g. 1.25 for 1.25v]) / 0.00625 (Renoir and up Only)
    pub fn set_oc_volt(&self, value: u32) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe { libryzenadj_sys::set_oc_volt(self.ryzen_adj, value) })
    }
    /// Sets forced per Core Clock Speed in MHz (Renoir and up Only)
    pub fn set_per_core_oc_clk(&self, value: u32) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe { libryzenadj_sys::set_per_core_oc_clk(self.ryzen_adj, value) })
    }
    /// Sets power saving mode
    pub fn set_power_saving(&self) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe { libryzenadj_sys::set_power_saving(self.ryzen_adj) })
    }
    /// Sets Ramp Time After Prochot is Deasserted: limit power based on value, higher values does apply tighter limits after prochot is over
    pub fn set_prochot_deassertion_ramp(&self, value: u32) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe {
            libryzenadj_sys::set_prochot_deassertion_ramp(self.ryzen_adj, value)
        })
    }
    /// Sets PSI0 VDD Current Limit (mA)
    pub fn set_psi0_current(&self, value: u32) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe { libryzenadj_sys::set_psi0_current(self.ryzen_adj, value) })
    }
    /// Sets PSI0 SoC Current Limit (mA)
    pub fn set_psi0soc_current(&self, value: u32) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe { libryzenadj_sys::set_psi0soc_current(self.ryzen_adj, value) })
    }
    /// Sets PSI3 CPU Current Limit (mA)
    pub fn set_psi3cpu_current(&self, value: u32) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe { libryzenadj_sys::set_psi3cpu_current(self.ryzen_adj, value) })
    }
    /// Sets PSI3 GFX Current Limit (mA)
    pub fn set_psi3gfx_current(&self, value: u32) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe { libryzenadj_sys::set_psi3gfx_current(self.ryzen_adj, value) })
    }
    /// Sets Skin Temperature Power Limit (mW)
    pub fn set_skin_temp_power_limit(&self, value: u32) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe { libryzenadj_sys::set_skin_temp_power_limit(self.ryzen_adj, value) })
    }
    /// Sets Average Power Limit - PPT LIMIT SLOW (mW)
    pub fn set_slow_limit(&self, value: u32) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe { libryzenadj_sys::set_slow_limit(self.ryzen_adj, value) })
    }
    /// Sets Slow PPT Constant Time (s)
    pub fn set_slow_time(&self, value: u32) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe { libryzenadj_sys::set_slow_time(self.ryzen_adj, value) })
    }
    /// Sets Sustained Power Limit - STAPM LIMIT (mW)
    pub fn set_stapm_limit(&self, value: u32) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe { libryzenadj_sys::set_stapm_limit(self.ryzen_adj, value) })
    }
    /// Sets STAPM constant time (s)
    pub fn set_stapm_time(&self, value: u32) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe { libryzenadj_sys::set_stapm_time(self.ryzen_adj, value) })
    }
    /// Sets Tctl Temperature Limit (degree C)
    pub fn set_tctl_temp(&self, value: u32) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe { libryzenadj_sys::set_tctl_temp(self.ryzen_adj, value) })
    }
    /// Sets VRM Current Limit - TDC LIMIT VDD (mA)
    pub fn set_vrm_current(&self, value: u32) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe { libryzenadj_sys::set_vrm_current(self.ryzen_adj, value) })
    }
    /// Sets VRM CVIP Current Limit - TDC LIMIT CVIP (mA)
    pub fn set_vrmcvip_current(&self, value: u32) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe { libryzenadj_sys::set_vrmcvip_current(self.ryzen_adj, value) })
    }
    /// Sets VRM GFX Current Limit - TDC LIMIT GFX (mA)
    pub fn set_vrmgfx_current(&self, value: u32) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe { libryzenadj_sys::set_vrmgfx_current(self.ryzen_adj, value) })
    }
    /// Sets VRM GFX Maximum Current Limit - EDC LIMIT GFX (mA)
    pub fn set_vrmgfxmax_current(&self, value: u32) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe { libryzenadj_sys::set_vrmgfxmax_current(self.ryzen_adj, value) })
    }
    /// Sets VRM Maximum Current Limit - EDC LIMIT VDD (mA)
    pub fn set_vrmmax_current(&self, value: u32) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe { libryzenadj_sys::set_vrmmax_current(self.ryzen_adj, value) })
    }
    /// Sets VRM SoC Current Limit - TDC LIMIT SoC (mA)
    pub fn set_vrmsoc_current(&self, value: u32) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe { libryzenadj_sys::set_vrmsoc_current(self.ryzen_adj, value) })
    }
    /// Sets VRM SoC Maximum Current Limit - EDC LIMIT SoC (mA)
    pub fn set_vrmsocmax_current(&self, value: u32) -> RyzenAdjResult<()> {
        Self::adj_code(unsafe { libryzenadj_sys::set_vrmsocmax_current(self.ryzen_adj, value) })
    }
}

impl Drop for RyzenAdj {
    fn drop(&mut self) {
        unsafe {
            libryzenadj_sys::cleanup_ryzenadj(self.ryzen_adj);
        }
    }
}
