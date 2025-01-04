#[cfg(feature = "System+IO+FileMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FileMode {
    #[default]
    Append = 6i32,
    Create = 2i32,
    CreateNew = 1i32,
    Open = 3i32,
    OpenOrCreate = 4i32,
    Truncate = 5i32,
}
#[cfg(feature = "System+IO+FileMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::IO::FileMode => "System.IO"."FileMode"
);
