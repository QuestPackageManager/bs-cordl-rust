#[cfg(feature = "TMPro+TagUnitType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TagUnitType {
    FontUnits = 1i32,
    Percentage = 2i32,
    Pixels = 0i32,
}
#[cfg(feature = "TMPro+TagUnitType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TagUnitType => "TMPro"."TagUnitType"
);
