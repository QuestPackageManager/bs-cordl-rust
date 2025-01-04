#[cfg(feature = "Unity+IO+LowLevel+Unsafe+FileState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FileState {
    #[default]
    Absent = 0i32,
    Exists = 1i32,
}
#[cfg(feature = "Unity+IO+LowLevel+Unsafe+FileState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::IO::LowLevel::Unsafe::FileState =>
    "Unity.IO.LowLevel.Unsafe"."FileState"
);
