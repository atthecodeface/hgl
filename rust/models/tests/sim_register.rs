use hgl_models::Register;
use hgl_sim::prelude::sim::*;

#[test]
fn sim_register() -> Result<(), String> {
    type T = Bv<31>;
    let mut sim = Simulation::new();
    let clk = sim.add_clock("clk", 0, 1, 0)?;
    let reg = sim.instantiate::<Register<T>, _, _>("reg", || Some(T::of_u64(32)))?;

    sim.connect_clock(clk, reg, 0); // cntr_clk);

    sim.prepare_simulation();
    let instances = sim.instances();
    sim.start(true)?;

    dbg!(&sim);

    instances.inst_mut::<Register<T>>(reg).inputs.reset_n = false.into();
    for _ in 0..3 {
        sim.fire_next_edges();
    }
    instances.inst_mut::<Register<T>>(reg).inputs.reset_n = true.into();

    for _ in 0..3 {
        sim.fire_next_edges();
    }

    assert_eq!(
        instances
            .inst_mut::<Register<T>>(reg)
            .outputs
            .data
            .try_as_u64()
            .unwrap(),
        32,
        "Expect data out at reset to be 32"
    );

    // instances.inst_mut::<Register<T>>(reg).inputs.enable = true.into();
    instances.inst_mut::<Register<T>>(reg).inputs.data = 0x1234567.into();
    for _ in 0..1 {
        sim.fire_next_edges();
    }

    assert_eq!(
        instances
            .inst_mut::<Register<T>>(reg)
            .outputs
            .data
            .try_as_u64()
            .unwrap(),
        32,
        "Expect data out at reset to be 32"
    );

    instances.inst_mut::<Register<T>>(reg).inputs.enable = true.into();
    for _ in 0..1 {
        sim.fire_next_edges();
    }
    instances.inst_mut::<Register<T>>(reg).inputs.enable = false.into();
    assert_eq!(
        instances
            .inst_mut::<Register<T>>(reg)
            .outputs
            .data
            .try_as_u64()
            .unwrap(),
        0x1234567,
        "Expect data out at reset to be 32"
    );

    dbg!(&sim);

    sim.stop()?;

    Ok(())
}
