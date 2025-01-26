use hgl_models::Counter;
use hgl_sim::prelude::sim::*;

#[test]
fn sim_counter() -> Result<(), String> {
    type T = Bv<31>;
    let mut sim = Simulation::new();
    let clk = sim.add_clock("clk", 0, 1, 0)?;
    let cntr = sim.instantiate::<Counter<T>, _, _>("counter", || Some(T::of_u64(32)))?;

    let cntr_clk = sim
        .instance(cntr)
        .state_index(sim.find_name("clk").unwrap())
        .unwrap();

    sim.connect_clock(clk, cntr, 0); // cntr_clk);

    sim.prepare_simulation();

    dbg!(&sim);

    let q = sim
        .instance(cntr)
        .state_index(sim.find_name("q").unwrap())
        .unwrap();

    {
        let inst = sim.inst::<Counter<T>>(cntr);
        let q_value = inst.try_state_data(q).unwrap();
        assert_eq!(
            q_value.value::<T>().unwrap().try_as_u64().unwrap(),
            32,
            "Value at reset should be the reset value"
        );
        assert_eq!(
            inst.try_state_data(q)
                .and_then(|v| v.try_as_u64::<T>())
                .unwrap(),
            32,
            "Value at reset should be the reset value"
        );
        assert_eq!(
            sim.inst::<Counter<T>>(cntr).try_as_u64::<T>(q).unwrap(),
            32,
            "Value at reset should be the reset value"
        );
        assert_eq!(
            sim.inst::<Counter<T>>(cntr).as_t::<T>(q).try_as_u64(),
            Some(32),
            "Value at reset should be the reset value"
        );
    }

    {
        let mut inst = sim.inst_mut::<Counter<T>>(cntr);
        *inst.inputs.reset_n = true.into();
        *inst.inputs.decrement = true.into();
    }

    dbg!(&sim);
    let mut timer = hgl_utils::timer::Timer::default();
    timer.entry();
    for _ in 0..10_000 {
        sim.fire_next_edges();
    }
    *sim.inst_mut::<Counter<T>>(cntr).inputs.decrement = false.into();
    *sim.inst_mut::<Counter<T>>(cntr).inputs.increment = true.into();
    for _ in 0..1_000 {
        sim.fire_next_edges();
    }
    timer.exit();
    *sim.inst_mut::<Counter<T>>(cntr).inputs.increment = false.into();

    eprintln!("Timer ticks per cycle {}", timer.value() / 11_000);

    dbg!(&sim);
    let exp: u64 = (1 << 31) + 32 - 9000;
    assert_eq!(
        sim.inst_mut::<Counter<T>>(cntr)
            .outputs
            .data
            .try_as_u64()
            .unwrap(),
        exp
    );

    assert!(false);

    Ok(())
}
