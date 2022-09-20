use errno::{errno, Errno};
use num_enum::TryFromPrimitive;
use thiserror::Error;

pub use libryzenadj_sys;

#[derive(Error, Debug)]
pub enum RyzenAdjError {
    #[error("ryzenadj init failed: {} {}", errno, errno.0)]
    InitError { errno: Errno },
    #[error("ryzenadj table init failed: {0}, set functions might still work")]
    InitTableError(i32),
    #[error("ryzenadj get returned NaN")]
    GetNaN,
    #[error("ryzenadj familly: {0} is not know")]
    UnknowFamily(i32),
}

pub type RyzenAdjResult<T> = Result<T, RyzenAdjError>;

pub struct RyzenAdj {
    ryzen_adj: libryzenadj_sys::ryzen_access,
    init_table_result: Option<i32>,
}

#[derive(Debug, TryFromPrimitive)]
#[non_exhaustive]
#[repr(i32)]
pub enum RyzenFamily {
    Unknow = libryzenadj_sys::ryzen_family_FAM_UNKNOWN,
    Raven = libryzenadj_sys::ryzen_family_FAM_RAVEN,
    Picassso = libryzenadj_sys::ryzen_family_FAM_PICASSO,
    Renoir = libryzenadj_sys::ryzen_family_FAM_RENOIR,
    Cezanne = libryzenadj_sys::ryzen_family_FAM_CEZANNE,
    Dali = libryzenadj_sys::ryzen_family_FAM_DALI,
    Lucienne = libryzenadj_sys::ryzen_family_FAM_LUCIENNE,
    Vangogh = libryzenadj_sys::ryzen_family_FAM_VANGOGH,
    Rembrandt = libryzenadj_sys::ryzen_family_FAM_REMBRANDT,
    End = libryzenadj_sys::ryzen_family_FAM_END,
}

impl RyzenAdj {
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

    pub fn refresh(&self) -> RyzenAdjResult<()> {
        self.is_init_table()?;
        let result = unsafe { libryzenadj_sys::refresh_table(self.ryzen_adj) };
        if result != 0 {
            Err(RyzenAdjError::InitTableError(result))
        } else {
            Ok(())
        }
    }

    pub fn get_apu_skin_temp_limit(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_apu_skin_temp_limit(self.ryzen_adj) })
    }

    pub fn get_apu_skin_temp_value(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_apu_skin_temp_value(self.ryzen_adj) })
    }

    pub fn get_apu_slow_limit(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_apu_slow_limit(self.ryzen_adj) })
    }

    pub fn get_apu_slow_value(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_apu_slow_value(self.ryzen_adj) })
    }

    pub fn get_bios_if_ver(&self) -> RyzenAdjResult<i32> {
        self.is_init_table()?;
        Ok(unsafe { libryzenadj_sys::get_bios_if_ver(self.ryzen_adj) })
    }

    pub fn get_cclk_busy_value(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_cclk_busy_value(self.ryzen_adj) })
    }

    pub fn get_cclk_setpoint(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_cclk_setpoint(self.ryzen_adj) })
    }

    pub fn get_core_clk(&self, core: u32) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_core_clk(self.ryzen_adj, core) })
    }

    pub fn get_core_power(&self, core: u32) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_core_power(self.ryzen_adj, core) })
    }

    pub fn get_core_temp(&self, core: u32) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_core_temp(self.ryzen_adj, core) })
    }

    pub fn get_core_volt(&self, core: u32) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_core_volt(self.ryzen_adj, core) })
    }

    pub fn get_cpu_family(&self) -> RyzenAdjResult<RyzenFamily> {
        self.is_init_table()?;
        let family_int = unsafe { libryzenadj_sys::get_cpu_family(self.ryzen_adj) };
        RyzenFamily::try_from(family_int).map_err(|_| RyzenAdjError::UnknowFamily(family_int))
    }

    pub fn get_dgpu_skin_temp_limit(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_dgpu_skin_temp_limit(self.ryzen_adj) })
    }

    pub fn get_dgpu_skin_temp_value(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_dgpu_skin_temp_value(self.ryzen_adj) })
    }

    pub fn get_fast_limit(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_fast_limit(self.ryzen_adj) })
    }

    pub fn get_fast_value(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_fast_value(self.ryzen_adj) })
    }

    pub fn get_fclk(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_fclk(self.ryzen_adj) })
    }

    pub fn get_gfx_temp(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_gfx_temp(self.ryzen_adj) })
    }

    pub fn get_gfx_clk(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_gfx_clk(self.ryzen_adj) })
    }

    pub fn get_gfx_volt(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_gfx_volt(self.ryzen_adj) })
    }

    pub fn get_l3_clk(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_l3_clk(self.ryzen_adj) })
    }

    pub fn get_l3_logic(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_l3_logic(self.ryzen_adj) })
    }

    pub fn get_l3_temp(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_l3_temp(self.ryzen_adj) })
    }

    pub fn get_l3_vddm(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_l3_vddm(self.ryzen_adj) })
    }

    pub fn get_mem_clk(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_mem_clk(self.ryzen_adj) })
    }

    pub fn get_psi0_current(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_psi0_current(self.ryzen_adj) })
    }

    pub fn get_psi0soc_current(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_psi0soc_current(self.ryzen_adj) })
    }

    pub fn get_slow_limit(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_slow_limit(self.ryzen_adj) })
    }

    pub fn get_slow_time(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_slow_time(self.ryzen_adj) })
    }

    pub fn get_slow_value(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_slow_value(self.ryzen_adj) })
    }

    pub fn get_soc_power(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_soc_power(self.ryzen_adj) })
    }

    pub fn get_soc_volt(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_soc_volt(self.ryzen_adj) })
    }

    pub fn get_socket_power(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_socket_power(self.ryzen_adj) })
    }

    pub fn get_stapm_limit(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_stapm_limit(self.ryzen_adj) })
    }

    pub fn get_stapm_time(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_stapm_time(self.ryzen_adj) })
    }

    pub fn get_stapm_value(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_stapm_value(self.ryzen_adj) })
    }

    pub fn get_tctl_temp(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_tctl_temp(self.ryzen_adj) })
    }

    pub fn get_tctl_temp_value(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_tctl_temp(self.ryzen_adj) })
    }

    pub fn get_vrm_current(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_vrm_current(self.ryzen_adj) })
    }

    pub fn get_vrm_current_value(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_vrm_current_value(self.ryzen_adj) })
    }

    pub fn get_vrmmax_current(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_vrmmax_current(self.ryzen_adj) })
    }

    pub fn get_vrmmax_current_value(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_vrmmax_current_value(self.ryzen_adj) })
    }

    pub fn get_vrmsoc_current(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_vrmsoc_current(self.ryzen_adj) })
    }

    pub fn get_vrmsoc_current_value(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_vrmsoc_current_value(self.ryzen_adj) })
    }

    pub fn get_vrmsocmax_current(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_vrmsocmax_current(self.ryzen_adj) })
    }

    pub fn get_vrmsocmax_current_value(&self) -> RyzenAdjResult<f32> {
        self.is_init_table()?;
        Self::is_nan(unsafe { libryzenadj_sys::get_vrmsocmax_current_value(self.ryzen_adj) })
    }
}

impl Drop for RyzenAdj {
    fn drop(&mut self) {
        unsafe {
            libryzenadj_sys::cleanup_ryzenadj(self.ryzen_adj);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RyzenAdj;
    #[test]
    fn init_test() {
        let cpus = num_cpus::get_physical() as u32;

        let ryzen_adj = RyzenAdj::new().unwrap();

        let apu_skin_temp_limit = ryzen_adj.get_apu_skin_temp_limit().unwrap();
        let apu_skin_temp_value = ryzen_adj.get_apu_skin_temp_value().unwrap();
        let apu_slow_limit = ryzen_adj.get_apu_slow_limit().unwrap();
        let apu_slow_value = ryzen_adj.get_apu_slow_value().unwrap();
        let bios_if_ver = ryzen_adj.get_bios_if_ver().unwrap();
        let cclk_busy_value = ryzen_adj.get_cclk_busy_value().unwrap();
        let cclk_setpoint = ryzen_adj.get_cclk_setpoint().unwrap();
        let core_clk: Vec<f32> = (0..cpus)
            .into_iter()
            .map(|c| ryzen_adj.get_core_clk(c).unwrap())
            .collect();
        let core_power: Vec<f32> = (0..cpus)
            .into_iter()
            .map(|c| ryzen_adj.get_core_power(c).unwrap())
            .collect();
        let core_temp: Vec<f32> = (0..cpus)
            .into_iter()
            .map(|c| ryzen_adj.get_core_temp(c).unwrap())
            .collect();
        let core_volt: Vec<f32> = (0..cpus)
            .into_iter()
            .map(|c| ryzen_adj.get_core_volt(c).unwrap())
            .collect();
        let cpu_family = ryzen_adj.get_cpu_family().unwrap();
        let dgpu_skin_temp_limit = ryzen_adj.get_dgpu_skin_temp_limit().unwrap();
        let dgpu_skin_temp_value = ryzen_adj.get_dgpu_skin_temp_value().unwrap();
        let fast_limit = ryzen_adj.get_fast_limit().unwrap();
        let fast_value = ryzen_adj.get_fast_value().unwrap();
        let fclk = ryzen_adj.get_fclk().unwrap();
        let gfx_clk = ryzen_adj.get_gfx_clk().unwrap();
        let gfx_temp = ryzen_adj.get_gfx_temp().unwrap();
        let gfx_volt = ryzen_adj.get_gfx_volt().unwrap();
        let l3_clk = ryzen_adj.get_l3_clk().unwrap();
        let l3_logic = ryzen_adj.get_l3_logic().unwrap();
        let l3_temp = ryzen_adj.get_l3_temp().unwrap();
        let l3_vddm = ryzen_adj.get_l3_vddm().unwrap();
        let mem_clk = ryzen_adj.get_mem_clk().unwrap();
        let psi0_current = ryzen_adj.get_psi0_current().unwrap();
        let psi0soc_current = ryzen_adj.get_psi0soc_current().unwrap();
        let slow_limit = ryzen_adj.get_slow_limit().unwrap();
        let slow_time = ryzen_adj.get_slow_time().unwrap();
        let slow_value = ryzen_adj.get_slow_value().unwrap();
        let soc_power = ryzen_adj.get_soc_power().unwrap();
        let soc_volt = ryzen_adj.get_soc_volt().unwrap();
        let socket_power = ryzen_adj.get_socket_power().unwrap();
        let stamp_limit = ryzen_adj.get_stapm_limit().unwrap();
        let stapm_time = ryzen_adj.get_stapm_time().unwrap();
        let stapm_value = ryzen_adj.get_stapm_value().unwrap();
        let tctl_temp = ryzen_adj.get_tctl_temp().unwrap();
        let tctl_temp_value = ryzen_adj.get_tctl_temp_value().unwrap();
        let vrm_current = ryzen_adj.get_vrm_current().unwrap();
        let vrm_current_value = ryzen_adj.get_vrm_current_value().unwrap();
        let vrmmax_current = ryzen_adj.get_vrmmax_current().unwrap();
        let vrmmax_current_value = ryzen_adj.get_vrmmax_current_value().unwrap();
        let vrmsoc_current = ryzen_adj.get_vrmsoc_current().unwrap();
        let vrmsoc_current_value = ryzen_adj.get_vrmsoc_current_value().unwrap();
        let vrmsocmax_current = ryzen_adj.get_vrmsocmax_current().unwrap();
        let vrmsocmax_current_value = ryzen_adj.get_vrmsocmax_current_value().unwrap();

        println!("apu_skin_temp_limit: {}", apu_skin_temp_limit);
        println!("apu_skin_temp_value: {}", apu_skin_temp_value);
        println!("apu_slow_limit: {}", apu_slow_limit);
        println!("apu_slow_value: {}", apu_slow_value);
        println!("bios_if_ver: {}", bios_if_ver);
        println!("cclk_busy_value: {}", cclk_busy_value);
        println!("cclk_setpoint: {}", cclk_setpoint);
        println!("core_clk: {:?}", core_clk);
        println!("core_power: {:?}", core_power);
        println!("core_temp: {:?}", core_temp);
        println!("core_volt: {:?}", core_volt);
        println!("cpu_family: {:?}", cpu_family);
        println!("dgpu_skin_temp_limit: {}", dgpu_skin_temp_limit);
        println!("dgpu_skin_temp_value: {}", dgpu_skin_temp_value);
        println!("fast_limit: {}", fast_limit);
        println!("fast_value: {}", fast_value);
        println!("fclk: {}", fclk);
        println!("gfx_clk: {}", gfx_clk);
        println!("gfx_temp: {}", gfx_temp);
        println!("gfx_volt: {}", gfx_volt);
        println!("l3_clk: {}", l3_clk);
        println!("l3_logic: {}", l3_logic);
        println!("l3_temp: {}", l3_temp);
        println!("l3_vddm: {}", l3_vddm);
        println!("mem_clk: {}", mem_clk);
        println!("psi0_current: {}", psi0_current);
        println!("psi0soc_current: {}", psi0soc_current);
        println!("slow_limit: {}", slow_limit);
        println!("slow_time: {}", slow_time);
        println!("slow_value: {}", slow_value);
        println!("soc_power: {}", soc_power);
        println!("soc_volt: {}", soc_volt);
        println!("socket_power: {}", socket_power);
        println!("stamp_limit: {}", stamp_limit);
        println!("stapm_time: {}", stapm_time);
        println!("stapm_value: {}", stapm_value);
        println!("tctl_temp: {}", tctl_temp);
        println!("tctl_temp_value: {}", tctl_temp_value);
        println!("vrm_current: {}", vrm_current);
        println!("vrm_current_value: {}", vrm_current_value);
        println!("vrmmax_current: {}", vrmmax_current);
        println!("vrmmax_current_value: {}", vrmmax_current_value);
        println!("vrmsoc_current: {}", vrmsoc_current);
        println!("vrmsoc_current_value: {}", vrmsoc_current_value);
        println!("vrmsocmax_current: {}", vrmsocmax_current);
        println!("vrmsocmax_current_value: {}", vrmsocmax_current_value);
    }
}
