use hgl_models::Counter;
use hgl_sim::prelude::sim::*;

#[test]
fn sim_counter() -> Result<(), String> {
    type T = Bv<31>;
    let mut sim = Simulation::new();
    sim.add_clock("clk", 0, 1, 0)?;
    let cntr = sim.instantiate::<Counter<T>, _, _>("counter", || Some(T::of_u64(32)))?;

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

    Ok(())
}
