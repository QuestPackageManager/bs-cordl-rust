#[cfg(feature = "EnvironmentColorType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnvironmentColorType {
    Color0 = 0i32,
    Color1 = 1i32,
    ColorW = 2i32,
    None = -1i32,
}
#[cfg(feature = "EnvironmentColorType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for EnvironmentColorType => ""."EnvironmentColorType"
);
