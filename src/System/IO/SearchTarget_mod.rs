#[cfg(feature = "System+IO+SearchTarget")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SearchTarget {
    #[default]
    Both = 3i32,
    Directories = 2i32,
    Files = 1i32,
}
#[cfg(feature = "System+IO+SearchTarget")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::IO::SearchTarget => "System.IO"
    ."SearchTarget"
);
