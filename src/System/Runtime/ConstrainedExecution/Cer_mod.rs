#[cfg(feature = "System+Runtime+ConstrainedExecution+Cer")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cer {
    MayFail = 1i32,
    None = 0i32,
    Success = 2i32,
}
#[cfg(feature = "System+Runtime+ConstrainedExecution+Cer")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::ConstrainedExecution::Cer =>
    "System.Runtime.ConstrainedExecution"."Cer"
);
