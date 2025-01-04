#[cfg(feature = "TMPro+MaskingTypes")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MaskingTypes {
    #[default]
    MaskHard = 1i32,
    MaskOff = 0i32,
    MaskSoft = 2i32,
}
#[cfg(feature = "TMPro+MaskingTypes")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::MaskingTypes => "TMPro"."MaskingTypes"
);
