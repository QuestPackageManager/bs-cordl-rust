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
unsafe impl quest_hook::libil2cpp::Type for crate::BeatSaber::BeatAvatarSDK::AvatarPart {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "BeatSaber.BeatAvatarSDK";
    const CLASS_NAME: &'static str = "AvatarPart";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarPart")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::BeatSaber::BeatAvatarSDK::AvatarPart {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarPart")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::BeatSaber::BeatAvatarSDK::AvatarPart {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarPart")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::BeatSaber::BeatAvatarSDK::AvatarPart {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "BeatSaber+BeatAvatarSDK+AvatarPart")]
unsafe impl quest_hook::libil2cpp::Return
for crate::BeatSaber::BeatAvatarSDK::AvatarPart {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
