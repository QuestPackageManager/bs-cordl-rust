#[cfg(feature = "TMPro+TagValueType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TagValueType {
    ColorValue = 4i32,
    None = 0i32,
    NumericalValue = 1i32,
    StringValue = 2i32,
}
#[cfg(feature = "TMPro+TagValueType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::TagValueType => "TMPro"."TagValueType"
);
