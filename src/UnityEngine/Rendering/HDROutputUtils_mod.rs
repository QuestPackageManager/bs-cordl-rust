#[cfg(feature = "cordl_class_UnityEngine+Rendering+HDROutputUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct HDROutputUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+HDROutputUtils")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::HDROutputUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "HDROutputUtils";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+Rendering+HDROutputUtils")]
impl std::ops::Deref for crate::UnityEngine::Rendering::HDROutputUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+HDROutputUtils")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::HDROutputUtils {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+HDROutputUtils")]
impl crate::UnityEngine::Rendering::HDROutputUtils {
    #[cfg(feature = "UnityEngine+Rendering+HDROutputUtils+HDRDisplayInformation")]
    pub type HDRDisplayInformation = crate::UnityEngine::Rendering::HDROutputUtils_HDRDisplayInformation;
    #[cfg(feature = "UnityEngine+Rendering+HDROutputUtils+Operation")]
    pub type Operation = crate::UnityEngine::Rendering::HDROutputUtils_Operation;
    #[cfg(feature = "UnityEngine+Rendering+HDROutputUtils+ShaderKeywords")]
    pub type ShaderKeywords = crate::UnityEngine::Rendering::HDROutputUtils_ShaderKeywords;
    #[cfg(feature = "UnityEngine+Rendering+HDROutputUtils+ShaderPropertyId")]
    pub type ShaderPropertyId = crate::UnityEngine::Rendering::HDROutputUtils_ShaderPropertyId;
    pub fn ConfigureHDROutput_ComputeShader_ColorGamut_HDROutputUtils_Operation3(
        computeShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
        gamut: crate::UnityEngine::ColorGamut,
        operations: crate::UnityEngine::Rendering::HDROutputUtils_Operation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::ComputeShader>,
                            crate::UnityEngine::ColorGamut,
                            crate::UnityEngine::Rendering::HDROutputUtils_Operation,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("ConfigureHDROutput")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConfigureHDROutput", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (computeShader, gamut, operations))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConfigureHDROutput_MaterialPropertyBlock_ColorGamut1(
        properties: quest_hook::libil2cpp::Gc<crate::UnityEngine::MaterialPropertyBlock>,
        gamut: crate::UnityEngine::ColorGamut,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::MaterialPropertyBlock,
                            >,
                            crate::UnityEngine::ColorGamut,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("ConfigureHDROutput")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConfigureHDROutput", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (properties, gamut))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConfigureHDROutput_Material_ColorGamut_HDROutputUtils_Operation0(
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        gamut: crate::UnityEngine::ColorGamut,
        operations: crate::UnityEngine::Rendering::HDROutputUtils_Operation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            crate::UnityEngine::ColorGamut,
                            crate::UnityEngine::Rendering::HDROutputUtils_Operation,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("ConfigureHDROutput")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConfigureHDROutput", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (material, gamut, operations))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ConfigureHDROutput_Material_HDROutputUtils_Operation2(
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        operations: crate::UnityEngine::Rendering::HDROutputUtils_Operation,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            crate::UnityEngine::Rendering::HDROutputUtils_Operation,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("ConfigureHDROutput")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ConfigureHDROutput", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (material, operations))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetColorEncodingForGamut(
        gamut: crate::UnityEngine::ColorGamut,
        encoding: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::ColorGamut,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                        ),
                        bool,
                        2usize,
                    >("GetColorEncodingForGamut")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetColorEncodingForGamut", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (gamut, encoding))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetColorSpaceForGamut(
        gamut: crate::UnityEngine::ColorGamut,
        colorspace: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::ColorGamut,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                        ),
                        bool,
                        2usize,
                    >("GetColorSpaceForGamut")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetColorSpaceForGamut", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (gamut, colorspace))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsShaderVariantValid(
        shaderKeywordSet: crate::UnityEngine::Rendering::ShaderKeywordSet,
        isHDREnabled: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Rendering::ShaderKeywordSet, bool),
                        bool,
                        2usize,
                    >("IsShaderVariantValid")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsShaderVariantValid", 2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked((), (shaderKeywordSet, isHDREnabled))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+HDROutputUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::HDROutputUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+HDROutputUtils+HDRDisplayInformation"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HDROutputUtils_HDRDisplayInformation {
    pub maxFullFrameToneMapLuminance: i32,
    pub maxToneMapLuminance: i32,
    pub minToneMapLuminance: i32,
    pub paperWhiteNits: f32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+HDROutputUtils+HDRDisplayInformation"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::HDROutputUtils_HDRDisplayInformation {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "HDROutputUtils/HDRDisplayInformation";
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+HDROutputUtils+HDRDisplayInformation"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::HDROutputUtils_HDRDisplayInformation {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+HDROutputUtils+HDRDisplayInformation"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::HDROutputUtils_HDRDisplayInformation {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+HDROutputUtils+HDRDisplayInformation"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::HDROutputUtils_HDRDisplayInformation {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+HDROutputUtils+HDRDisplayInformation"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::HDROutputUtils_HDRDisplayInformation {
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+HDROutputUtils+HDRDisplayInformation"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::HDROutputUtils_HDRDisplayInformation {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Rendering+HDROutputUtils+HDRDisplayInformation")]
impl crate::UnityEngine::Rendering::HDROutputUtils_HDRDisplayInformation {
    pub fn _ctor(
        &mut self,
        maxFullFrameToneMapLuminance: i32,
        maxToneMapLuminance: i32,
        minToneMapLuminance: i32,
        hdrPaperWhiteNits: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (i32, i32, i32, f32),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        maxFullFrameToneMapLuminance,
                        maxToneMapLuminance,
                        minToneMapLuminance,
                        hdrPaperWhiteNits,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+HDROutputUtils+Operation")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HDROutputUtils_Operation {
    #[default]
    ColorConversion = 1i32,
    ColorEncoding = 2i32,
    None = 0i32,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+HDROutputUtils+Operation")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::HDROutputUtils_Operation {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "HDROutputUtils/Operation";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+HDROutputUtils+Operation")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::HDROutputUtils_Operation {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+HDROutputUtils+Operation")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::HDROutputUtils_Operation {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+HDROutputUtils+Operation")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::HDROutputUtils_Operation {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+HDROutputUtils+Operation")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::HDROutputUtils_Operation {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+HDROutputUtils+ShaderKeywords")]
#[repr(C)]
#[derive(Debug)]
pub struct HDROutputUtils_ShaderKeywords {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+HDROutputUtils+ShaderKeywords")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::HDROutputUtils_ShaderKeywords {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "HDROutputUtils/ShaderKeywords";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+Rendering+HDROutputUtils+ShaderKeywords")]
impl std::ops::Deref for crate::UnityEngine::Rendering::HDROutputUtils_ShaderKeywords {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+HDROutputUtils+ShaderKeywords")]
impl std::ops::DerefMut
for crate::UnityEngine::Rendering::HDROutputUtils_ShaderKeywords {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+HDROutputUtils+ShaderKeywords")]
impl crate::UnityEngine::Rendering::HDROutputUtils_ShaderKeywords {
    pub const HDR_COLORSPACE_CONVERSION: &'static str = "HDR_COLORSPACE_CONVERSION";
    pub const HDR_COLORSPACE_CONVERSION_AND_ENCODING: &'static str = "HDR_COLORSPACE_CONVERSION_AND_ENCODING";
    pub const HDR_ENCODING: &'static str = "HDR_ENCODING";
    pub const HDR_INPUT: &'static str = "HDR_INPUT";
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+HDROutputUtils+ShaderKeywords")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::HDROutputUtils_ShaderKeywords {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+HDROutputUtils+ShaderPropertyId")]
#[repr(C)]
#[derive(Debug)]
pub struct HDROutputUtils_ShaderPropertyId {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+HDROutputUtils+ShaderPropertyId")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::HDROutputUtils_ShaderPropertyId {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "HDROutputUtils/ShaderPropertyId";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+Rendering+HDROutputUtils+ShaderPropertyId")]
impl std::ops::Deref for crate::UnityEngine::Rendering::HDROutputUtils_ShaderPropertyId {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+HDROutputUtils+ShaderPropertyId")]
impl std::ops::DerefMut
for crate::UnityEngine::Rendering::HDROutputUtils_ShaderPropertyId {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+HDROutputUtils+ShaderPropertyId")]
impl crate::UnityEngine::Rendering::HDROutputUtils_ShaderPropertyId {}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+HDROutputUtils+ShaderPropertyId")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::HDROutputUtils_ShaderPropertyId {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
