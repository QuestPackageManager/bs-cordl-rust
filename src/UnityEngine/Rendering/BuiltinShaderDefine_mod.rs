#[cfg(feature = "cordl_class_UnityEngine+Rendering+BuiltinShaderDefine")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BuiltinShaderDefine {
    #[default]
    SHADER_API_DESKTOP = 18i32,
    SHADER_API_GLES30 = 31i32,
    SHADER_API_MOBILE = 17i32,
    UNITY_ASTC_NORMALMAP_ENCODING = 30i32,
    UNITY_COLORSPACE_GAMMA = 22i32,
    UNITY_ENABLE_DETAIL_NORMALMAP = 16i32,
    UNITY_ENABLE_NATIVE_SHADOW_LOOKUPS = 5i32,
    UNITY_ENABLE_REFLECTION_BUFFERS = 3i32,
    UNITY_FRAMEBUFFER_FETCH_AVAILABLE = 4i32,
    UNITY_HALF_PRECISION_FRAGMENT_SHADER_REGISTERS = 24i32,
    UNITY_HARDWARE_TIER1 = 19i32,
    UNITY_HARDWARE_TIER2 = 20i32,
    UNITY_HARDWARE_TIER3 = 21i32,
    UNITY_LIGHTMAP_DLDR_ENCODING = 25i32,
    UNITY_LIGHTMAP_FULL_HDR = 27i32,
    UNITY_LIGHTMAP_RGBM_ENCODING = 26i32,
    UNITY_LIGHT_PROBE_PROXY_VOLUME = 23i32,
    UNITY_METAL_SHADOWS_USE_POINT_FILTERING = 6i32,
    UNITY_NEEDS_RENDERPASS_FBFETCH_FALLBACK = 35i32,
    UNITY_NO_CUBEMAP_ARRAY = 7i32,
    UNITY_NO_DXT5nm = 0i32,
    UNITY_NO_FULL_STANDARD_SHADER = 13i32,
    UNITY_NO_RGBM = 1i32,
    UNITY_NO_SCREENSPACE_SHADOWS = 8i32,
    UNITY_PBS_USE_BRDF1 = 10i32,
    UNITY_PBS_USE_BRDF2 = 11i32,
    UNITY_PBS_USE_BRDF3 = 12i32,
    UNITY_PLATFORM_SUPPORTS_DEPTH_FETCH = 36i32,
    UNITY_PLATFORM_SUPPORTS_WAVE_32 = 33i32,
    UNITY_PLATFORM_SUPPORTS_WAVE_64 = 34i32,
    UNITY_PRETRANSFORM_TO_DISPLAY_ORIENTATION = 29i32,
    UNITY_SPECCUBE_BLENDING = 15i32,
    UNITY_SPECCUBE_BOX_PROJECTION = 14i32,
    UNITY_UNIFIED_SHADER_PRECISION_MODEL = 32i32,
    UNITY_USE_DITHER_MASK_FOR_ALPHABLENDED_SHADOWS = 9i32,
    UNITY_USE_NATIVE_HDR = 2i32,
    UNITY_VIRTUAL_TEXTURING = 28i32,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+BuiltinShaderDefine")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::BuiltinShaderDefine {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "BuiltinShaderDefine";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+BuiltinShaderDefine")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::BuiltinShaderDefine {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+BuiltinShaderDefine")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::BuiltinShaderDefine {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+BuiltinShaderDefine")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::BuiltinShaderDefine {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+BuiltinShaderDefine")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::BuiltinShaderDefine {
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
