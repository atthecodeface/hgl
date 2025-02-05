use hgl_models::Threaded;
use hgl_sim::prelude::sim::*;

#[test]
fn sim_threaded() -> Result<(), String> {
    let mut sim = Simulation::new();
    let clk = sim.add_clock("clk", 0, 1, 0)?;
    let m = sim.instantiate::<Threaded, _, _>("threaded", || ())?;

    let mclk = sim
        .instance(m)
        .state_index(sim.find_name("clk").unwrap())
        .unwrap();

    sim.connect_clock(clk, m, 0); // mclk);

    sim.prepare_simulation();
    sim.start(true)?;

    sim.inst_mut::<Threaded>(m).inputs.reset_n = true;
    sim.fire_next_edges();

    sim.inst_mut::<Threaded>(m).inputs.start = true;
    sim.fire_next_edges();
    sim.inst_mut::<Threaded>(m).inputs.start = false;

    for _ in 0..10_000 {
        sim.fire_next_edges();
    }

    eprintln!("Time after edges {}", sim.inst_mut::<Threaded>(m).outputs.q);

    sim.inst_mut::<Threaded>(m).inputs.stop = true;
    sim.fire_next_edges();
    sim.inst_mut::<Threaded>(m).inputs.stop = false;

    for _ in 0..10_000 {
        sim.fire_next_edges();
    }

    eprintln!("Time after edges {}", sim.inst_mut::<Threaded>(m).outputs.q);

    dbg!(&sim);

    sim.stop()?;
    assert!(false);

    Ok(())
}
