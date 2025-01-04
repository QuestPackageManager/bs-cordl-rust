#[cfg(feature = "System+Threading+Tasks+CausalityTraceLevel")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CausalityTraceLevel {
    #[default]
    Important = 1i32,
    Required = 0i32,
    Verbose = 2i32,
}
#[cfg(feature = "System+Threading+Tasks+CausalityTraceLevel")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::Tasks::CausalityTraceLevel =>
    "System.Threading.Tasks"."CausalityTraceLevel"
);
