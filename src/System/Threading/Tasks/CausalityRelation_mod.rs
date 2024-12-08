#[cfg(feature = "System+Threading+Tasks+CausalityRelation")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CausalityRelation {
    AssignDelegate = 0i32,
    Cancel = 3i32,
    Choice = 2i32,
    Error = 4i32,
    Join = 1i32,
}
#[cfg(feature = "System+Threading+Tasks+CausalityRelation")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::Tasks::CausalityRelation =>
    "System.Threading.Tasks"."CausalityRelation"
);
