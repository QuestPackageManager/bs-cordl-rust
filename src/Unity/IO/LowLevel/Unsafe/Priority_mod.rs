#[cfg(feature = "Unity+IO+LowLevel+Unsafe+Priority")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
    PriorityHigh = 1i32,
    PriorityLow = 0i32,
}
#[cfg(feature = "Unity+IO+LowLevel+Unsafe+Priority")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::IO::LowLevel::Unsafe::Priority =>
    "Unity.IO.LowLevel.Unsafe"."Priority"
);
