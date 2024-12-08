#[cfg(feature = "System+AttributeTargets")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AttributeTargets {
    All = 32767i32,
    Assembly = 1i32,
    Class = 4i32,
    Constructor = 32i32,
    Delegate = 4096i32,
    Enum = 16i32,
    Event = 512i32,
    Field = 256i32,
    GenericParameter = 16384i32,
    Interface = 1024i32,
    Method = 64i32,
    Module = 2i32,
    Parameter = 2048i32,
    Property = 128i32,
    ReturnValue = 8192i32,
    Struct = 8i32,
}
#[cfg(feature = "System+AttributeTargets")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::AttributeTargets => "System"
    ."AttributeTargets"
);
