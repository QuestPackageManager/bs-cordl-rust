#[cfg(feature = "System+Runtime+Serialization+StreamingContextStates")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StreamingContextStates {
    All = 255i32,
    Clone = 64i32,
    CrossAppDomain = 128i32,
    CrossMachine = 2i32,
    CrossProcess = 1i32,
    File = 4i32,
    Other = 32i32,
    Persistence = 8i32,
    Remoting = 16i32,
}
#[cfg(feature = "System+Runtime+Serialization+StreamingContextStates")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::Serialization::StreamingContextStates =>
    "System.Runtime.Serialization"."StreamingContextStates"
);
