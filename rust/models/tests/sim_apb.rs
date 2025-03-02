use hgl_models::apb_target_gpio::apb_target_gpio;
use hgl_models::t_apb_request;
use hgl_sim::prelude::sim::*;

#[test]
fn sim() -> Result<(), String> {
    let mut sim = Simulation::new();
    let clk = sim.add_clock("clk", 0, 1, 0)?;
    let cntr = sim.instantiate::<apb_target_gpio, _, _>("dut", || ())?;

    sim.connect_clock(clk, cntr, 0); // cntr_clk);

    sim.prepare_simulation();
    sim.start(true)?;

    fn f<'a>(m: &'a mut RefMutInstance<'_, apb_target_gpio>) -> &'a mut t_apb_request {
        &mut m.inputs.apb_request
    }

    for (a, d) in [(0, 0), (0, 1), (0, 4), (1, 3)] {
        let req = t_apb_request {
            psel: true.into(),
            penable: false.into(),
            pwrite: true.into(),
            paddr: a.into(),
            pwdata: d.into(),
        };

        *(f(&mut sim.inst_mut::<apb_target_gpio>(cntr))) = req;
        sim.fire_next_edges();
        f(&mut sim.inst_mut::<apb_target_gpio>(cntr)).penable = true.into();
        sim.fire_next_edges();
        f(&mut sim.inst_mut::<apb_target_gpio>(cntr)).psel = false.into();
        sim.fire_next_edges();

        sim.fire_next_edges();
        sim.fire_next_edges();
    }
    for _ in 0..1_000 {
        sim.fire_next_edges();
    }

    // *simm.inst_mut::<Counter<T>>(cntr).inputs.increment = false.into();

    sim.stop()?;
    dbg!(&sim);

    // assert!(false);

    Ok(())
}
