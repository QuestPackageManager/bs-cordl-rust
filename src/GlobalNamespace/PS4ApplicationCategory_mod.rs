#[cfg(feature = "PS4ApplicationCategory")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PS4ApplicationCategory {
    Application = 0i32,
    Patch = 1i32,
    Remaster = 2i32,
}
#[cfg(feature = "PS4ApplicationCategory")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PS4ApplicationCategory => ""
    ."PS4ApplicationCategory"
);
