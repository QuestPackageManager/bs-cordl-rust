#[cfg(feature = "cordl_class_UnityEngine+Experimental+Rendering+GraphicsFormat")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GraphicsFormat {
    #[default]
    A10R10G10B10_XRSRGBPack32 = 85i32,
    A10R10G10B10_XRUNormPack32 = 86i32,
    A1R5G5B5_UNormPack16 = 72i32,
    A2B10G10R10_SIntPack32 = 77i32,
    A2B10G10R10_UIntPack32 = 76i32,
    A2B10G10R10_UNormPack32 = 75i32,
    A2R10G10B10_SIntPack32 = 80i32,
    A2R10G10B10_UIntPack32 = 79i32,
    A2R10G10B10_UNormPack32 = 78i32,
    A2R10G10B10_XRSRGBPack32 = 81i32,
    A2R10G10B10_XRUNormPack32 = 82i32,
    B10G11R11_UFloatPack32 = 74i32,
    B4G4R4A4_UNormPack16 = 67i32,
    B5G5R5A1_UNormPack16 = 71i32,
    B5G6R5_UNormPack16 = 69i32,
    B8G8R8A8_SInt = 65i32,
    B8G8R8A8_SNorm = 61i32,
    B8G8R8A8_SRGB = 57i32,
    B8G8R8A8_UInt = 63i32,
    B8G8R8A8_UNorm = 59i32,
    B8G8R8_SInt = 64i32,
    B8G8R8_SNorm = 60i32,
    B8G8R8_SRGB = 56i32,
    B8G8R8_UInt = 62i32,
    B8G8R8_UNorm = 58i32,
    D16_UNorm = 90i32,
    D16_UNorm_S8_UInt = 151i32,
    D24_UNorm = 91i32,
    D24_UNorm_S8_UInt = 92i32,
    D32_SFloat = 93i32,
    D32_SFloat_S8_UInt = 94i32,
    DepthAuto = 142i32,
    E5B9G9R9_UFloatPack32 = 73i32,
    None = 0i32,
    R10G10B10_XRSRGBPack32 = 83i32,
    R10G10B10_XRUNormPack32 = 84i32,
    R16G16B16A16_SFloat = 48i32,
    R16G16B16A16_SInt = 36i32,
    R16G16B16A16_SNorm = 28i32,
    R16G16B16A16_UInt = 32i32,
    R16G16B16A16_UNorm = 24i32,
    R16G16B16_SFloat = 47i32,
    R16G16B16_SInt = 35i32,
    R16G16B16_SNorm = 27i32,
    R16G16B16_UInt = 31i32,
    R16G16B16_UNorm = 23i32,
    R16G16_SFloat = 46i32,
    R16G16_SInt = 34i32,
    R16G16_SNorm = 26i32,
    R16G16_UInt = 30i32,
    R16G16_UNorm = 22i32,
    R16_SFloat = 45i32,
    R16_SInt = 33i32,
    R16_SNorm = 25i32,
    R16_UInt = 29i32,
    R16_UNorm = 21i32,
    R32G32B32A32_SFloat = 52i32,
    R32G32B32A32_SInt = 44i32,
    R32G32B32A32_UInt = 40i32,
    R32G32B32_SFloat = 51i32,
    R32G32B32_SInt = 43i32,
    R32G32B32_UInt = 39i32,
    R32G32_SFloat = 50i32,
    R32G32_SInt = 42i32,
    R32G32_UInt = 38i32,
    R32_SFloat = 49i32,
    R32_SInt = 41i32,
    R32_UInt = 37i32,
    R4G4B4A4_UNormPack16 = 66i32,
    R5G5B5A1_UNormPack16 = 70i32,
    R5G6B5_UNormPack16 = 68i32,
    R8G8B8A8_SInt = 20i32,
    R8G8B8A8_SNorm = 12i32,
    R8G8B8A8_SRGB = 4i32,
    R8G8B8A8_UInt = 16i32,
    R8G8B8A8_UNorm = 8i32,
    R8G8B8_SInt = 19i32,
    R8G8B8_SNorm = 11i32,
    R8G8B8_SRGB = 3i32,
    R8G8B8_UInt = 15i32,
    R8G8B8_UNorm = 7i32,
    R8G8_SInt = 18i32,
    R8G8_SNorm = 10i32,
    R8G8_SRGB = 2i32,
    R8G8_UInt = 14i32,
    R8G8_UNorm = 6i32,
    R8_SInt = 17i32,
    R8_SNorm = 9i32,
    R8_SRGB = 1i32,
    R8_UInt = 13i32,
    R8_UNorm = 5i32,
    RGBA_ASTC10X10_SRGB = 137i32,
    RGBA_ASTC10X10_UFloat = 149i32,
    RGBA_ASTC10X10_UNorm = 138i32,
    RGBA_ASTC12X12_SRGB = 139i32,
    RGBA_ASTC12X12_UFloat = 150i32,
    RGBA_ASTC12X12_UNorm = 140i32,
    RGBA_ASTC4X4_SRGB = 129i32,
    RGBA_ASTC4X4_UFloat = 145i32,
    RGBA_ASTC4X4_UNorm = 130i32,
    RGBA_ASTC5X5_SRGB = 131i32,
    RGBA_ASTC5X5_UFloat = 146i32,
    RGBA_ASTC5X5_UNorm = 132i32,
    RGBA_ASTC6X6_SRGB = 133i32,
    RGBA_ASTC6X6_UFloat = 147i32,
    RGBA_ASTC6X6_UNorm = 134i32,
    RGBA_ASTC8X8_SRGB = 135i32,
    RGBA_ASTC8X8_UFloat = 148i32,
    RGBA_ASTC8X8_UNorm = 136i32,
    RGBA_BC7_SRGB = 108i32,
    RGBA_BC7_UNorm = 109i32,
    RGBA_DXT1_SRGB = 96i32,
    RGBA_DXT1_UNorm = 97i32,
    RGBA_DXT3_SRGB = 98i32,
    RGBA_DXT3_UNorm = 99i32,
    RGBA_DXT5_SRGB = 100i32,
    RGBA_DXT5_UNorm = 101i32,
    RGBA_ETC2_SRGB = 123i32,
    RGBA_ETC2_UNorm = 124i32,
    RGBA_PVRTC_2Bpp_SRGB = 114i32,
    RGBA_PVRTC_2Bpp_UNorm = 115i32,
    RGBA_PVRTC_4Bpp_SRGB = 116i32,
    RGBA_PVRTC_4Bpp_UNorm = 117i32,
    RGB_A1_ETC2_SRGB = 121i32,
    RGB_A1_ETC2_UNorm = 122i32,
    RGB_BC6H_SFloat = 107i32,
    RGB_BC6H_UFloat = 106i32,
    RGB_ETC2_SRGB = 119i32,
    RGB_ETC2_UNorm = 120i32,
    RGB_ETC_UNorm = 118i32,
    RGB_PVRTC_2Bpp_SRGB = 110i32,
    RGB_PVRTC_2Bpp_UNorm = 111i32,
    RGB_PVRTC_4Bpp_SRGB = 112i32,
    RGB_PVRTC_4Bpp_UNorm = 113i32,
    RG_BC5_SNorm = 105i32,
    RG_BC5_UNorm = 104i32,
    RG_EAC_SNorm = 128i32,
    RG_EAC_UNorm = 127i32,
    R_BC4_SNorm = 103i32,
    R_BC4_UNorm = 102i32,
    R_EAC_SNorm = 126i32,
    R_EAC_UNorm = 125i32,
    S8_UInt = 95i32,
    ShadowAuto = 143i32,
    VideoAuto = 144i32,
    YUV2 = 141i32,
}
#[cfg(feature = "cordl_class_UnityEngine+Experimental+Rendering+GraphicsFormat")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Experimental::Rendering::GraphicsFormat {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Experimental.Rendering";
    const CLASS_NAME: &'static str = "GraphicsFormat";
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
#[cfg(feature = "cordl_class_UnityEngine+Experimental+Rendering+GraphicsFormat")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Experimental::Rendering::GraphicsFormat {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Experimental+Rendering+GraphicsFormat")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Experimental::Rendering::GraphicsFormat {
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
#[cfg(feature = "cordl_class_UnityEngine+Experimental+Rendering+GraphicsFormat")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Experimental::Rendering::GraphicsFormat {
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
#[cfg(feature = "cordl_class_UnityEngine+Experimental+Rendering+GraphicsFormat")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Experimental::Rendering::GraphicsFormat {
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
