#[cfg(feature = "System+IO+FileAccess")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FileAccess {
    #[default]
    Read = 1i32,
    ReadWrite = 3i32,
    Write = 2i32,
}
#[cfg(feature = "System+IO+FileAccess")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::IO::FileAccess => "System.IO"
    ."FileAccess"
);
