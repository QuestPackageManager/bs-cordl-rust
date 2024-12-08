#[cfg(feature = "TMPro+SpriteAssetUtilities+SpriteAssetImportFormats")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpriteAssetImportFormats {
    None = 0i32,
    TexturePackerJsonArray = 1i32,
}
#[cfg(feature = "TMPro+SpriteAssetUtilities+SpriteAssetImportFormats")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::TMPro::SpriteAssetUtilities::SpriteAssetImportFormats =>
    "TMPro.SpriteAssetUtilities"."SpriteAssetImportFormats"
);
