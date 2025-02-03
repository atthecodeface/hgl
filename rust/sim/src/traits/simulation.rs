//a Imports
use crate::simulation::{SimEdgeMask, SimNsName, SimReset, SimStateIndex, SimStateInfo};
use crate::values::{SimValueRef, SimValueRefMut};

//a Simulation traits
//tt SimHandle
/// A type providing [Component] can be an instance of a component
/// that can be simulated
///
/// Such types are constructed by a [ComponentBuilder]
///
/// The component builder has an 'instantiate' method invoked with a
pub trait SimHandle: Sized + Copy {}

//tt SimRegister
/// This trait is implemented by simulations to permit component
/// builders to instantiate instances of components with the
/// simulation.
///
/// This trait is not dyn-compatible as it contains the `Handle` type
pub trait SimRegister {
    //tp Handle
    /// The handle used by the simulation to presented to an instance
    /// so that it can update its administration within the simulation
    type Handle: SimHandle;

    //mp register_input_edge
    /// Registers that the edge of a signal is important to execution;
    /// usually this is a clock edge
    ///
    /// Called by a component at configuration time to indicate it
    /// uses an input edge
    ///
    /// Usually this is a clock posedge or negedge (or both)
    ///
    /// This takes an immutable 'self' as the instance invoking the
    /// call is probably part of 'self'
    fn register_input_edge(&self, handle: Self::Handle, input: usize, posedge: bool, negedge: bool);

    //mp comb_path
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

//a Component, ComponentBuilder, and Simulatable component traits
//tt Simulatable
/// This trait is dyn-compatible; it is in general used as `Box<dyn Simulatable + 'static>`
///
/// This cannot take a lifetime, as the 'Any' trait is used to
/// downcast values of types that support this trait into specific
/// `Component` instances, and `Any` requires 'static (as values of
/// the same type with different lifetimes have the same type_id).
///
/// A 'ready' method? Poll method?
///
/// If clock can return std::task::Poll::Pending, then the method should be passed a std::task::Waker?
pub trait Simulatable: std::any::Any {
    //mp as_any
    /// Return the instance as a 'dyn Any', so it can be downcast
    fn as_any(&self) -> &dyn std::any::Any;

    //mp as_mut_any
    /// Return the instance as a mutable 'dyn Any', so it can be downcast
    fn as_mut_any(&mut self) -> &mut dyn std::any::Any;

    //mp reset
    /// Reset the component
    ///
    /// The reason could be simulation restart, or something 'weaker'
    fn reset(&mut self, _reason: SimReset) {}

    //mp clock
    /// Clock the component, with mask indicating which edges have occurred
    ///
    /// This should use the values in its Inputs, and update its outputs.
    ///
    /// This might return a 'not ready' indication; or something that
    /// might be polled for completion.  If not ready is returned then
    /// no other calls to the component can be issued until ready is
    /// indicated
    fn clock(&mut self, _mask: SimEdgeMask) {}

    //mp propagate
    /// Propagate inputs through combinational paths and to all submodules
    ///
    /// This is not invoked for clocked-only modules, except when
    /// generating waveforms (or equivalent)
    ///
    /// For modules that declare (at config time) they have
    /// comb_path's, this will be called once for each such
    /// invocation, after any event that might change the inputs. The
    /// 'stage' indicates which set of inputs will now be valid (hence
    /// it is increased on each call, starting at 0 for the first
    /// after a clock edge)
    fn propagate(&mut self, _stage: usize) {}

    //ap state_info
    /// Return some of the state information
    ///
    /// The SimStateInfo indicates whether the state is an input, output,
    /// clock, internal state, etc
    ///
    /// If this returns None then the index is larger than the visible
    /// state of the component
    fn state_info(&self, index: SimStateIndex) -> Option<SimStateInfo>;

    //ap try_state_data
    /// Return state *data* for an index that matches that for
    /// state_info, if the data provides SimValueObject
    fn try_state_data(&self, _index: SimStateIndex) -> Option<SimValueRef> {
        None
    }

    //ap try_state_data_mut
    /// Return mutable state *data* for an index that matches that for
    /// state_info, if the data provides SimValueObject
    fn try_state_data_mut(&mut self, _index: SimStateIndex) -> Option<SimValueRefMut> {
        None
    }
}

//tt ComponentBuilder
/// This trait is provided by types that can create instances of a
/// component within a specific simulation
pub trait ComponentBuilder {
    //tp Build
    /// Type that is built by this builder
    type Build: Component;

    //bp instantiate
    /// Instantiate a component, which in turn can be configured using
    /// it 'configure' method in its 'Component' trait prior to first use
    fn instantiate<S: SimRegister>(sim: &mut S, name: SimNsName) -> Self::Build;
}

//tt Component
/// This trait is not dyn-compatible
///
/// The use of methods for types supporting this trait is usually
/// through borrowing (possibly immutably) and instance of a component
/// within a simulation using its *known* component type; for example
/// to copy data from its outputs, or to set some of its inputs.
pub trait Component: Simulatable {
    //tp Config
    /// Type that is used for
    type Config;

    //tp InputsMut
    /// Type returned by 'inputs_mut', to permit mutation of the
    /// inputs
    type InputsMut<'a>
    where
        Self: 'a;

    //tp Inputs
    /// Type returned by 'inputs', to permit inspection of the
    /// inputs
    type Inputs<'a>
    where
        Self: 'a;

    //tp Outputs
    /// Type returned by 'outputs', to permit inspection of the
    /// outputs
    type Outputs<'a>
    where
        Self: 'a;

    //mp configure
    /// Configure the component, called once after it is instantiated
    fn configure<S: SimRegister>(
        &mut self,
        _sim: &S,
        _handle: S::Handle,
        _config: Self::Config,
    ) -> Result<(), String> {
        Ok(())
    }

    //ap inputs_mut
    /// Borrow the inputs as mutable
    fn inputs_mut(&mut self) -> Self::InputsMut<'_>;

    //ap inputs
    /// Borrow the inputs as immutable
    fn inputs(&self) -> Self::Inputs<'_>;

    //ap outputs
    /// Borrow the outputs as immutable
    fn outputs(&self) -> Self::Outputs<'_>;
}
