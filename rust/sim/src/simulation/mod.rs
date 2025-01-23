//a Imports
use std::cell::{RefCell, RefMut};
use std::marker::PhantomData;
use std::sync::{RwLock, RwLockReadGuard, RwLockWriteGuard};

//a Types
//tp Simulation
pub struct EdgeUse {}
pub enum SimReset {
    Restart,
}

//tt SimHandle
/// A type providing [Component] can be an instance of a component
/// that can be simulated
///
/// Such types are constructed by a [ComponentBuilder]
pub trait SimHandle: Sized + Copy {}

//tt Component
pub trait Component: Simulatable {
    type Config;
    type InputsMut<'a>
    where
        Self: 'a;
    type Inputs<'a>
    where
        Self: 'a;
    type Outputs<'a>
    where
        Self: 'a;

    /// Configure the component, called at most once after it is instantiated
    fn configure<S: SimRegister>(
        &mut self,
        _sim: &S,
        _handle: S::Handle,
        _config: Self::Config,
    ) -> Result<(), String> {
        Ok(())
    }

    /// Borrow the inputs as mutable
    fn inputs_mut<'a>(&'a mut self) -> Self::InputsMut<'a>;

    /// Borrow the inputs as immutable
    fn inputs<'a>(&'a self) -> Self::Inputs<'a>;

    /// Borrow the outputs as immutable
    fn outputs<'a>(&'a self) -> Self::Outputs<'a>;
}
//tt Simulatable
pub trait Simulatable: std::any::Any {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_mut_any(&mut self) -> &mut dyn std::any::Any;

    /// Reset the component
    fn reset(&mut self, _reason: SimReset) {}

    /// Clock the component, with mask indicating which edges have occurred
    fn clock(&mut self, _mask: u32) {}

    /// Propagate inputs through combinational paths and to all submodules
    ///
    /// This is not invoked for clocked-only modules, except when
    /// generating waveforms (or equivalent)
    ///
    /// For modules that declare (at config time) they have
    /// comb_path's, this will be called once for each such
    /// invocation, after any event that might change the inputs. The
    /// 'stage' indicates which set of inputs will now be valid (hence
    /// it is increased on each call)
    fn propagate(&mut self, _stage: usize) {}
}

pub trait SimRegister {
    type Handle: SimHandle;
    /// Called by a component at configuration time to indicate it uses an input edge
    ///
    /// Usually this is a clock posedge or negedge (or both)
    ///
    /// This takes an immutable 'self' as the instance invoking the
    /// call is probably part of 'self'
    fn register_input_edge(&self, handle: Self::Handle, input: usize, posedge: bool, negedge: bool);

    /// Called by a component to indicate there are combinational
    /// paths from a set of inputs to some outputs
    ///
    /// Inputs and outputs of a module can be considered to be valid
    /// at some percentage of a clock period; a clocked output is
    /// valid after 0%, and an input used on a clock valid at 100%.
    ///
    /// Combinational paths from inputs to outputs mean the output is
    /// valid after the input. Hence the inputs must be valid at, say,
    /// X% and the outputs become valid at, say, Y%, where Y > X.
    ///
    /// This method indicates that the 'propagate' function is
    /// required to be called between the 'times' X and Y.
    ///
    /// The invocation provides the set of outputs that are invalid
    /// *before* the call, and the set of inputs that can be invalid
    /// *before* the call, plus the set of outputs that are valid
    /// *after* the call.
    ///
    /// (It uses 'invalid' sets, so that the simulator can get away
    /// without knowing how many actual inputs and outputs there are)
    ///
    /// So if the input and output validity for a module is
    ///
    ///   state -> oA
    ///   iA, iB -> oB,
    ///   iC, iD -> oC, oD
    ///   iE -> clock
    ///
    /// Then the first invocation indicates that:
    ///
    ///   * oB, oC, oD are invalid before
    ///   * iC, iD, iE may be invalid before
    ///   * oC, oD may invalid afterwards
    ///
    /// The second invocation indicates that:
    ///
    ///   * oC, oD are invalid before
    ///   * iE may be invalid before
    ///   * nothing will be invalid afterwards
    ///
    /// This takes an immutable 'self' as the instance invoking the
    /// call is probably part of 'self'
    fn comb_path(
        &self,
        handle: Self::Handle,
        outputs_ib: &[u8],
        inputs_ib: &[u8],
        outputs_ia: &[u8],
    );
}
pub trait ComponentBuilder {
    type Build: Component;
    fn instantiate<S: SimRegister>(sim: &mut S, name: &FullName) -> Self::Build;
}

//tp Component
/// A component that *can* be instantiated
pub struct Instantiable {
    name: FullName,
}

pub struct SubInstance {}

#[derive(Default)]
pub struct Clock {}

#[derive(Default)]
pub struct ClockArray {
    clocks: Vec<Clock>,
}
pub struct ClockIndex(usize);
impl std::ops::Index<ClockIndex> for ClockArray {
    type Output = Clock;
    fn index(&self, n: ClockIndex) -> &Clock {
        &self.clocks[n.0]
    }
}
impl From<usize> for ClockIndex {
    fn from(n: usize) -> ClockIndex {
        ClockIndex(n)
    }
}

/// For now derive Debug and Default, but not in the future
///
/// Quite probably the simulatino needs to be accessible from more
/// than one thread
pub struct FullName {
    namespace: String,
    name: String,
}

impl FullName {
    fn new(namespace: (), name: &str) -> Result<Self, String> {
        Ok(Self {
            namespace: "".into(),
            name: name.into(),
        })
    }
}

/// Toplevel instance of a component
///
/// This probably needs to have some Sync wrapper so that different
/// Instance's can be clocked simultaneously
pub struct Instance {
    simulatable: RwLock<Box<dyn Simulatable + 'static>>,
}

pub struct RefMutInstance<'a, C: Component + 'static> {
    l: RwLockWriteGuard<'a, Box<dyn Simulatable + 'static>>,
    phantom: PhantomData<&'a C>,
}
impl<'a, C: Component + 'static> std::ops::Deref for RefMutInstance<'a, C> {
    type Target = C;
    fn deref(&self) -> &C {
        self.l.as_any().downcast_ref::<C>().unwrap()
    }
}
impl<'a, C: Component + 'static> std::ops::DerefMut for RefMutInstance<'a, C> {
    fn deref_mut(&mut self) -> &mut C {
        self.l.as_mut_any().downcast_mut::<C>().unwrap()
    }
}
pub struct RefInstance<'a, C: Component + 'static> {
    l: RwLockReadGuard<'a, Box<dyn Simulatable + 'static>>,
    phantom: PhantomData<&'a C>,
}
impl<'a, C: Component + 'static> std::ops::Deref for RefInstance<'a, C> {
    type Target = C;
    fn deref(&self) -> &C {
        self.l.as_any().downcast_ref::<C>().unwrap()
    }
}
impl<'a, C: Component + 'static> RefMutInstance<'a, C> {
    ///  Borrow the inputs as mutable
    pub fn inputs_mut<'i>(&'i mut self) -> C::InputsMut<'i> {
        self.l
            .as_mut_any()
            .downcast_mut::<C>()
            .unwrap()
            .inputs_mut()
    }

    /// Borrow the inputs as immutable
    pub fn inputs<'i>(&'i self) -> C::Inputs<'i> {
        self.l.as_any().downcast_ref::<C>().unwrap().inputs()
    }

    /// Borrow the outputs as immutable
    pub fn outputs<'i>(&'i self) -> C::Outputs<'i> {
        self.l.as_any().downcast_ref::<C>().unwrap().outputs()
    }
}
impl<'a, C: Component + 'static> RefInstance<'a, C> {
    /// Borrow the inputs as immutable
    pub fn inputs<'i>(&'i self) -> C::Inputs<'i> {
        self.l.as_any().downcast_ref::<C>().unwrap().inputs()
    }

    /// Borrow the outputs as immutable
    pub fn outputs<'i>(&'i self) -> C::Outputs<'i> {
        self.l.as_any().downcast_ref::<C>().unwrap().outputs()
    }
}
impl Instance {
    fn new<S: Simulatable + 'static>(s: S) -> Self {
        let s: Box<dyn Simulatable + 'static> = Box::new(s);
        let simulatable = RwLock::new(s);
        Self { simulatable }
    }
    fn map<C: Component + 'static, R, F: FnOnce(&mut C) -> R>(&self, f: F) -> R {
        f(&mut *self.borrow_mut())
    }
    fn borrow_mut<'a, C: Component + 'static>(&'a self) -> RefMutInstance<'a, C> {
        let l = self.simulatable.try_write();
        match l {
            Ok(l) => RefMutInstance {
                l,
                phantom: PhantomData,
            },
            Err(e) => {
                panic!("Failed to get RwLock on instance; probably already locked for reads {e:?}");
            }
        }
    }
    fn borrow<'a, C: Component + 'static>(&'a self) -> RefInstance<'a, C> {
        let l = self.simulatable.read().unwrap();
        RefInstance {
            l,
            phantom: PhantomData,
        }
    }
}

struct SimulationControl {
    namespace: (),
    clocks: ClockArray,
}

pub struct Simulation {
    control: SimulationControl,
    instances: Vec<Instance>,
}

#[derive(Debug, Clone, Copy)]
pub struct InstanceHandle(usize);
impl InstanceHandle {
    fn new(n: usize) -> Self {
        Self(n)
    }
}

impl SimHandle for InstanceHandle {}

// impl std::ops::Index<InstanceHandle> for Simulation {
//     type Output = Box<dyn Simulatable>;
//     fn index(&self, n: InstanceHandle) -> &Box<dyn Simulatable> {
//         &self.instances[n.0].simulatable
//     }
// }
// impl std::ops::IndexMut<InstanceHandle> for Simulation {
//     fn index_mut(&mut self, n: InstanceHandle) -> &mut Box<dyn Simulatable> {
//         &mut self.instances[n.0].simulatable
//     }
// }
impl Simulation {
    pub fn new() -> Self {
        let namespace = ();
        let clocks = ClockArray::default();
        let control = SimulationControl { namespace, clocks };
        let instances = vec![];
        Self { control, instances }
    }

    pub fn add_clock(&mut self, name: &str, delay: usize, period: usize, negedge_offset: usize) {}

    pub fn instantiate<
        CB: ComponentBuilder<Build = C>,
        C: Component,
        F: FnOnce() -> <C as Component>::Config,
    >(
        &mut self,
        name: &str,
        f: F,
    ) -> Result<InstanceHandle, String> {
        let full_name = FullName::new(self.control.namespace, name)?;
        let component = CB::instantiate(self, &full_name);
        let instance = Instance::new(component);
        let handle = InstanceHandle::new(self.instances.len());
        self.instances.push(instance);
        self.instances[handle.0].map::<C, _, _>(|c: &mut C| c.configure(&*self, handle, f()))?;
        Ok(handle)
    }

    pub fn inst<C: Component>(&self, handle: InstanceHandle) -> RefInstance<C> {
        self.instances[handle.0].borrow()
    }
    pub fn inst_mut<C: Component>(&self, handle: InstanceHandle) -> RefMutInstance<C> {
        self.instances[handle.0].borrow_mut()
    }
}

impl SimRegister for Simulation {
    type Handle = InstanceHandle;
    fn register_input_edge(
        &self,
        handle: Self::Handle,
        input: usize,
        posedge: bool,
        negedge: bool,
    ) {
    }
    fn comb_path(
        &self,
        handle: Self::Handle,
        outputs_ib: &[u8],
        inputs_ib: &[u8],
        outputs_ia: &[u8],
    ) {
    }
}
