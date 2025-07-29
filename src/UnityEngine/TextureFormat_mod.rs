#[cfg(feature = "cordl_class_UnityEngine+TextureFormat")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TextureFormat {
    #[default]
    ARGB32 = 5i32,
    ARGB4444 = 2i32,
    ASTC_10x10 = 52i32,
    ASTC_12x12 = 53i32,
    ASTC_4x4 = 48i32,
    ASTC_5x5 = 49i32,
    ASTC_6x6 = 50i32,
    ASTC_8x8 = 51i32,
    ASTC_HDR_10x10 = 70i32,
    ASTC_HDR_12x12 = 71i32,
    ASTC_HDR_4x4 = 66i32,
    ASTC_HDR_5x5 = 67i32,
    ASTC_HDR_6x6 = 68i32,
    ASTC_HDR_8x8 = 69i32,
    ASTC_RGBA_10x10 = 58i32,
    ASTC_RGBA_12x12 = 59i32,
    ASTC_RGBA_4x4 = 54i32,
    ASTC_RGBA_5x5 = 55i32,
    ASTC_RGBA_6x6 = 56i32,
    ASTC_RGBA_8x8 = 57i32,
    Alpha8 = 1i32,
    BC4 = 26i32,
    BC5 = 27i32,
    BC6H = 24i32,
    BC7 = 25i32,
    BGRA32 = 14i32,
    DXT1 = 10i32,
    DXT1Crunched = 28i32,
    DXT5 = 12i32,
    DXT5Crunched = 29i32,
    EAC_R = 41i32,
    EAC_RG = 43i32,
    EAC_RG_SIGNED = 44i32,
    EAC_R_SIGNED = 42i32,
    ETC2_RGB = 45i32,
    ETC2_RGBA1 = 46i32,
    ETC2_RGBA8 = 47i32,
    ETC2_RGBA8Crunched = 65i32,
    ETC_RGB4 = 34i32,
    ETC_RGB4Crunched = 64i32,
    ETC_RGB4_3DS = 60i32,
    ETC_RGBA8_3DS = 61i32,
    PVRTC_RGB2 = 30i32,
    PVRTC_RGB4 = 32i32,
    PVRTC_RGBA2 = 31i32,
    PVRTC_RGBA4 = 33i32,
    R16 = 9i32,
    R8 = 63i32,
    RFloat = 18i32,
    RG16 = 62i32,
    RG32 = 72i32,
    RGB24 = 3i32,
    RGB48 = 73i32,
    RGB565 = 7i32,
    RGB9e5Float = 22i32,
    RGBA32 = 4i32,
    RGBA4444 = 13i32,
    RGBA64 = 74i32,
    RGBAFloat = 20i32,
    RGBAHalf = 17i32,
    RGFloat = 19i32,
    RGHalf = 16i32,
    RHalf = 15i32,
    YUY2 = 21i32,
}
#[cfg(feature = "cordl_class_UnityEngine+TextureFormat")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::TextureFormat {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "TextureFormat";
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
#[cfg(feature = "cordl_class_UnityEngine+TextureFormat")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::TextureFormat {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+TextureFormat")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::TextureFormat {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+TextureFormat")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::TextureFormat {
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
#[cfg(feature = "cordl_class_UnityEngine+TextureFormat")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::TextureFormat {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
