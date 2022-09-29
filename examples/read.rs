use libryzenadj::RyzenAdj;

fn main() {
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
