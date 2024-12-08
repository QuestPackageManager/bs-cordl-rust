#[cfg(feature = "System+Runtime+ConstrainedExecution+Consistency")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Consistency {
    MayCorruptAppDomain = 1i32,
    MayCorruptInstance = 2i32,
    MayCorruptProcess = 0i32,
    WillNotCorruptState = 3i32,
}
#[cfg(feature = "System+Runtime+ConstrainedExecution+Consistency")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::ConstrainedExecution::Consistency =>
    "System.Runtime.ConstrainedExecution"."Consistency"
);
