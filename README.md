# RGI example
This is an example of an extremely simple program I would like to be able to make with rgi

## RGI Concept
RGI splits UI and logic in to two parts, the UI is handled by RGI files and the logic is handled by rust.

All component parts of UI elements (basically anything in an .rgi file) may have individual state structures.

These state structures define how a hierarchy of components responds to events, and what state changes must 
occur between events. State is immutable, every time state must change a new state structure is built.
State structures must not be mutable, nor should they possess interior mutability (with the exception of mutable resources that have no apparent effect on UI state).

Variables that act as a representation of system resources (files, devices, APIs over devices such as OpenGL or Vulkan, basically anything that doesn't represent application state) are stored in a `Context`. Only resources where storing history is not necessary should be in context, if a system resource result can be represented historically (the result of prior calls can be cached) then they may be placed inside of a struct implementing `State` instead.

Variables stored outside of a `Context` or `State` are internally stored in an `Rc(Box::<dyn Any>)`, this has obvious performance implications, though it's only relevent when accessing information on UI elements.
