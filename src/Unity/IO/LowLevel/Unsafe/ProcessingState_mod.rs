#[cfg(feature = "Unity+IO+LowLevel+Unsafe+ProcessingState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessingState {
    Canceled = 5i32,
    Completed = 3i32,
    Failed = 4i32,
    InQueue = 1i32,
    Reading = 2i32,
    Unknown = 0i32,
}
#[cfg(feature = "Unity+IO+LowLevel+Unsafe+ProcessingState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::IO::LowLevel::Unsafe::ProcessingState =>
    "Unity.IO.LowLevel.Unsafe"."ProcessingState"
);
