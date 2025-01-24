use hgl_sim::Simulation;

#[test]
fn sim_clocks() -> Result<(), String> {
    let mut sim = Simulation::new();
    assert_eq!(sim.add_clock("clk21", 0, 21, 0)?, 0.into());
    assert_eq!(sim.add_clock("clk5", 17, 5, 3)?, 1.into());
    assert_eq!(sim.add_clock("clk14", 3, 14, 0)?, 2.into());
    sim.prepare_simulation();

    for _ in 0..100 {
        let edges = sim.next_edges();
        eprintln!("{}: Edges {:?}", sim.time(), edges);
    }
    Ok(())
}

#[test]
#[should_panic]
fn sim_bad_period() {
    let mut sim = Simulation::new();
    assert_eq!(sim.add_clock("clk", 10, 0, 0).unwrap(), 0.into());
}

#[test]
#[should_panic]
fn sim_bad_negedge() {
    let mut sim = Simulation::new();
    assert_eq!(sim.add_clock("clk", 10, 5, 5).unwrap(), 0.into());
}

#[test]
#[should_panic]
fn sim_no_schedule() {
    let mut sim = Simulation::new();
    assert_eq!(sim.add_clock("clk", 0, 21, 0).unwrap(), 0.into());

    let _edges = sim.next_edges();
}

#[test]
#[should_panic]
fn sim_no_schedule2() {
    let mut sim = Simulation::new();
    assert_eq!(sim.add_clock("clk", 0, 21, 0).unwrap(), 0.into());

    let _edges = sim.time();
}
