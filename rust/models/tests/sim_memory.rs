use hgl_models::Memory;
use hgl_sim::prelude::sim::*;
use hgl_sim::Simulation;

type Mem32x31 = Memory<Bv<31>, Bv<5>>;
#[test]
fn sim_memory() -> Result<(), String> {
    let mut sim = Simulation::new();
    sim.add_clock("clk", 0, 1, 0)?;
    let mem1 = sim.instantiate::<Mem32x31, _, _>("memory", || 32)?;
    let mem2 = sim.instantiate::<Mem32x31, _, _>("memory_2", || 32)?;

    sim.prepare_simulation();

    {
        let _mem = sim.inst::<Mem32x31>(mem1);
        let _mem2 = sim.inst::<Mem32x31>(mem2);
        let _mema = sim.inst::<Mem32x31>(mem1);
        let _mem2a = sim.inst::<Mem32x31>(mem2);
    }
    {
        let mut mem = sim.inst_mut::<Mem32x31>(mem1);
        let inputs = mem.inputs_mut();
        inputs.read_enable |= true;
    }
    // mem : RefMutInstance<Mem32x32>
    let mut mem = sim.inst_mut::<Mem32x31>(mem1);
    mem.inputs_mut().read_enable &= false;
    mem.clock(1);
    assert!(
        mem.outputs().read_valid.is_false(),
        "Read data should not be valid if no read took place"
    );
    mem.inputs_mut().read_enable &= false;
    mem.inputs_mut().write_enable |= true;
    mem.inputs_mut().address.set_u64(3);
    mem.inputs_mut().write_data.set_u64(724);
    mem.clock(1);
    assert!(
        mem.outputs().read_valid.is_false(),
        "Read data should not be valid if no read took place"
    );
    mem.inputs_mut().read_enable |= true;
    mem.inputs_mut().write_enable &= false;
    mem.clock(1);
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
    let port_data_address = mem.try_state_data(0).unwrap();
    assert_eq!(
        port_data_address.value().try_as_u8s().unwrap(),
        [3, 0, 0, 0, 0, 0, 0, 0]
    );
    let mut port_data_address = mem.try_state_data_mut(0).unwrap();
    assert!(
        port_data_address.set_u8s(x.try_as_u8s().unwrap()),
        "Should correctly set data"
    );
    assert_eq!(
        port_data_address.value().try_as_u8s().unwrap(),
        [6, 0, 0, 0, 0, 0, 0, 0]
    );

    // assert!(false);

    Ok(())
}
