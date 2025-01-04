#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarPart")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarPart {
    #[default]
    All = 1i32,
    ClothesModel = 10i32,
    ClothesModelDetailColor = 13i32,
    ClothesModelPrimaryColor = 11i32,
    ClothesModelSecondaryColor = 12i32,
    FacialHairColor = 7i32,
    GlassesColor = 6i32,
    HandsColor = 9i32,
    HandsModel = 8i32,
    HeadTopModel = 3i32,
    HeadTopPrimaryColor = 4i32,
    HeadTopSecondaryColor = 5i32,
    SkinColor = 2i32,
    Unknown = 0i32,
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarPart")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::BeatAvatarSDK::AvatarPart =>
    "BeatSaber.BeatAvatarSDK"."AvatarPart"
);
