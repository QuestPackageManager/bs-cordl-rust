#[cfg(feature = "System+IO+SearchOption")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchOption {
    AllDirectories = 1i32,
    TopDirectoryOnly = 0i32,
}
#[cfg(feature = "System+IO+SearchOption")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::IO::SearchOption => "System.IO"
    ."SearchOption"
);
