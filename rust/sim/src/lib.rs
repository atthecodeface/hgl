//a Documentation
//!
//! # Simulation Values
//!
//! Values are used within simulations for all kinds of storage; some
//! values must be used in common ways - for example for state
//! interrogation, or waveform file generation
//!
//! ## Kinds of values
//!
//! The simples kinds of values are completely bit-copyable; they
//! would support the Rust Copy trait, and have state that is
//! completely conveyed by the bit-level encoding and can be saved and
//! restored by literal memcpy. Not all Rust Copy types are like this:
//! references, in Rust, are Copy, but simulation values containing
//! references are not. These kinds of values would include bits, bit
//! vectors, structures of bit-copyable values, arrays of bit-copyable
//! values, and tagged unions of bit-copyable values where the tag is
//! encoded as a bit vector. Rust values that contain an UnsafeCell
//! (or a RefCell, by implication) are *not* bit-copyable.
//!
//! By their very nature these simple kinds of values are Rust Send and Sync
//!
//! Another kind of value is an array type that might be implemented
//! as a Rust Vec. This is not bit-copyable; usually these values
//! would not be assigned in one state storage element from another,
//! but the individual contents might be. A memory of some form would
//! match this kind of value. Waveform generation, checkpoint and
//! restore all require more work for these kinds of values; however,
//! they effectively hold the state of a piece of hardware, which does
//! have a temporal concept of state and state changes, and thus need
//! to be supported in simulation. These kinds of values might provide
//! a Rust Clone trait that is not too heavyweight.
//!
//! A third kind of value would be a sparse array implementing a large
//! memory (such as a DRAM or disk) - possibly backed by a read-only
//! data file that is external to the simulation. Support for
//! checkpoint and restore may be possible for these types using
//! transaction logs, and waveforms might be abstracted into read and
//! write transactions (for example). These kinds of values might
//! provide a heavyweight Rust Clone implementation, but more likely
//! they would sit behing a Rust Rc; indeed, to operate correctly in a
//! multithreaded simulation they would need to use an Arc with
//! probably a RwLock or Mutex guard.
//!
//! A fourth kind of value would be an abstract simulation type, such
//! as a log file, which can not readily be considered to mirror any
//! hardware state; it does not have state transitions in the same way
//! that a register or memory does. These kinds of values in general
//! don't support any waveform access, or checkpointing and
//! restoration. The cloning of these would always be through a Rust
//! Arc, with mutability managed through a RwLock or Mutex guard.
//!
//! ## Multithreading
//!
//! Simulations will in general be multithreaded; the simulation
//! engine itself, which coordinates the execution of models, is
//! likely to run in a single thread, but individual models may
//! execute in one thread or more, simultaneously.
//!
//! The simulation engine will own the model state, and manage it
//! atomically using RwLocks; the model methods themselves should have
//! exclusive access to their state during the time when they are
//! evaluating their next states, but outside of this the state should
//! be accessible from other threads.
//!
//! # Traits
//!
//! The basic kinds of value that a simulation can deal with must be
//! accessible as 'dyn' objects; i.e. the base trait must be
//! dyn-compatible, as this provides for standardized state
//! interaction. This basic trait is [SimValueObject]; it provides for
//! upcasting to a 'dyn any', so it can be appropriately downcast by
//! clients that know its actual type; it provides basic interrogation
//! to probe into sublements (e.g. fields of a struct) of the value;
//! it provides simple formatting, and data value interrogation.
//!
//! These basic aspects allow:
//!
//! *  the contents of arrays and structures to be determined
//!
//! *  a [SimValueObject] to be used in a
//!     waveform output - by copying values out, comparing them, and
//!     formatting them if required
//!
//! *  values to be checkpointed and restored... not yet
//!
//! # Simulation Values that are *Copy*
//!
//! Many simulation values will support Copy; this would generally
//! include bits, bit vectors, structs of such types and other similat
//! structs, and arrays of these, and so on.
//!
//! These types can be taken to support copying by conversion to
//! slices of u8 and 'memcpy' of the slices; comparison of values for
//! 'might be equal' can be performed with 'memcmp'; checkpoint and
//! restore can be performed by checkpointing the byte data. This is a
//! significant kind of value.
//!
//! The [SimCopyValue] trait is provided for these kinds of values.
//!
//! # Simulation Value that are Not *Copy*
//!
//! A simulation value might be a sparse array (such as a DRAM memory
//! content). Comparison for might-be-equal may not be
//! possible. Checkpoint and restore can be handled sparsely.
//!
//! Other types that might not be copy could be files; such values may
//! not need to be exposed to the simulation, but if they are then
//! they might not support comparison and may be incompatible entirely
//! with checkpointing (or may just opt out).
//!
//! # Simulation
//!
//! A simulation is the main data type provided. A simulation is managed in the following phases:
//!
//! * Basic simulation configuration
//!
//! * Construction of the model to be simulated (instantiation of models)
//!
//! * Creation and connection of clocks to models (and other edge sensitivity)
//!
//! * Prepare for simulation - individual model instances are mutable
//!   in multiple threads through separate RwLocks, but the main control
//!   structure is owned by the main simulation thread
//!
//! * Start of simulation - additional threads are created by models
//!   if required, cloning part of the simulation as SimulationWorker
//!
//! * Main thread controls forward progress of the model (indicates
//!   how many clock edges to move on, etc)
//!
//! * Additional threads can use SimulationWorker to wait for specific
//!   events, such as a signal asserted when a clock edge fires
//!
//! * Main thread controls termination of the simulation (after a
//!   certain number of clock edges, for example)
//!
//! * Main thread halts simulation; other model threads must drop
//!   their SimulationWorkers (and generally terminate)
//!
//! * Simulation models can be deconstructed (if required) and results analyzed
//!
//! Simulation operates from a main thread, and can have other threads interacting with it; the simulation main thread
//!
//! SimulationWorkers can invoke 'wait_for_edge<F:Send +
//! FnMut(SimEdgeMask) -> SimWaitResult>'; this will wait on a
//! sync::Condvar until the simulation is stopped or the relevant edge
//! occurs with the condition set
//!
//! Maybe have the worker set its filter with Arc<Mutex<Box<dyn FnMut>>>
//!
//! The main simulation thread calculates when the next clock edge is
//! to occur, and it moves time forward to that point; it calculates
//! the SimEdgeMask for that time (more than one clock edge might
//! occur); all SimulationWorkers must at this point be waiting for an edge.
//!
//! The conditions for all the SimulationWorkers are evaluated before
//! releasing the sync::Condvar's; the main thread has a thread
//! barrier and it increments the number of workers for each
//! released. When the workers return (or drop!) the increment the
//! thread barrier count, and the main thread can continue execution
//! when *it* and the worker's are all ready
//!

//a Modules
pub(crate) mod data;
pub mod prelude;
pub(crate) mod simulation;
pub(crate) mod traits;
pub(crate) mod value_types;
pub(crate) mod values;

pub mod sync;
