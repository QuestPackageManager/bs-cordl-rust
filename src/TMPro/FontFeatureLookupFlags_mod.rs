#[cfg(feature = "TMPro+FontFeatureLookupFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FontFeatureLookupFlags {
    IgnoreLigatures = 4i32,
    IgnoreSpacingAdjustments = 256i32,
    None = 0i32,
}
#[cfg(feature = "TMPro+FontFeatureLookupFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::TMPro::FontFeatureLookupFlags => "TMPro"
    ."FontFeatureLookupFlags"
);
