#[cfg(feature = "TMPro+MaskingOffsetMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MaskingOffsetMode {
    #[default]
    Percentage = 0i32,
    Pixel = 1i32,
}
#[cfg(feature = "TMPro+MaskingOffsetMode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::MaskingOffsetMode => "TMPro"
    ."MaskingOffsetMode"
);
