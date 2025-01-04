#[cfg(feature = "Unity+Collections+Allocator")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Allocator {
    #[default]
    AudioKernel = 5i32,
    FirstUserIndex = 64i32,
    Invalid = 0i32,
    None = 1i32,
    Persistent = 4i32,
    Temp = 2i32,
    TempJob = 3i32,
}
#[cfg(feature = "Unity+Collections+Allocator")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Collections::Allocator =>
    "Unity.Collections"."Allocator"
);
