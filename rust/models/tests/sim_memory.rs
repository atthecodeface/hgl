use hgl_models::Memory;
use hgl_sim::prelude::component::SimEdgeMask;
use hgl_sim::prelude::sim::*;

type Mem32x31 = Memory<Bv<31>, Bv<5>>;
#[test]
fn sim_memory() -> Result<(), String> {
    let mut sim = Simulation::new();
    sim.add_clock("clk", 0, 1, 0)?;
    let mem1 = sim.instantiate::<Mem32x31, _, _>("memory", || 32)?;
    let mem2 = sim.instantiate::<Mem32x31, _, _>("memory_2", || 32)?;

    sim.prepare_simulation();
    let instances = sim.instances();
    sim.start(true)?;

    {
        let _mem = instances.inst::<Mem32x31>(mem1);
        let _mem2 = instances.inst::<Mem32x31>(mem2);
        let _mema = instances.inst::<Mem32x31>(mem1);
        let _mem2a = instances.inst::<Mem32x31>(mem2);
    }
    let address_name = sim
        .find_name("address")
        .expect("Memory must have declared the 'address' state");
    let address_index = instances
        .instance(mem1)
        .state_index(address_name)
        .expect("Memory must have declared the 'address' state");
    {
        let mut mem = instances.inst_mut::<Mem32x31>(mem1);
        let inputs = mem.inputs_mut();
        inputs.read_enable |= true;
    }

    dbg!(&sim);

    // mem : RefMutInstance<Mem32x32>
    let mut mem = instances.inst_mut::<Mem32x31>(mem1);
    mem.inputs_mut().read_enable &= false;
    let clk = SimEdgeMask::default().add_posedge(0);
    mem.clock(clk);
    assert!(
        mem.outputs().read_valid.is_false(),
        "Read data should not be valid if no read took place"
    );
    mem.inputs_mut().read_enable &= false;
    mem.inputs_mut().write_enable |= true;
    mem.inputs_mut().address.set_u64(3);
    mem.inputs_mut().write_data.set_u64(724);
    mem.clock(clk);
    assert!(
        mem.outputs().read_valid.is_false(),
        "Read data should not be valid if no read took place"
    );
    mem.inputs_mut().read_enable |= true;
    mem.inputs_mut().write_enable &= false;
    mem.clock(clk);
    assert!(
        mem.outputs().read_valid.is_true(),
        "Read data should be valid if read took place"
    );
    assert_eq!(
        mem.outputs().read_data.try_as_u64().unwrap(),
        724,
        "Read data should be value written"
    );

    let mut x = mem.inputs_mut().address;
    x += x;
    let port_data_address = mem.try_state_data(address_index).unwrap();
    assert_eq!(
        port_data_address.sim_value().try_as_u8s().unwrap(),
        [3, 0, 0, 0, 0, 0, 0, 0]
    );
    let mut port_data_address = mem.try_state_data_mut(address_index).unwrap();
    assert!(
        port_data_address.set_u8s(x.try_as_u8s().unwrap()),
        "Should correctly set data"
    );
    assert_eq!(
        port_data_address.sim_value().try_as_u8s().unwrap(),
        [6, 0, 0, 0, 0, 0, 0, 0]
    );

    // If we do not drop then dbg of sim will not show the values
    drop(mem);

    sim.stop()?;
    dbg!(&sim);

    Ok(())
}
