#[cfg(feature = "Unity+IO+LowLevel+Unsafe+FileReadType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FileReadType {
    #[default]
    Async = 1i32,
    Sync = 0i32,
}
#[cfg(feature = "Unity+IO+LowLevel+Unsafe+FileReadType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::IO::LowLevel::Unsafe::FileReadType =>
    "Unity.IO.LowLevel.Unsafe"."FileReadType"
);
