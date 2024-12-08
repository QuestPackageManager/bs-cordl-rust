#[cfg(feature = "System+Reflection+ExceptionHandlingClauseOptions")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExceptionHandlingClauseOptions {
    Clause = 0i32,
    Fault = 4i32,
    Filter = 1i32,
    Finally = 2i32,
}
#[cfg(feature = "System+Reflection+ExceptionHandlingClauseOptions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Reflection::ExceptionHandlingClauseOptions => "System.Reflection"
    ."ExceptionHandlingClauseOptions"
);
