#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+ScheduleMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScheduleMode {
    Batched = 1i32,
    Run = 0i32,
    Single = 2i32,
}
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+ScheduleMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Jobs::LowLevel::Unsafe::ScheduleMode =>
    "Unity.Jobs.LowLevel.Unsafe"."ScheduleMode"
);
