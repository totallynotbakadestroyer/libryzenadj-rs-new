use libryzenadj::RyzenAdj;

fn main() {
    let ryzen_adj = RyzenAdj::new().unwrap();
    ryzen_adj.set_apu_skin_temp_limit(50).unwrap();
}
