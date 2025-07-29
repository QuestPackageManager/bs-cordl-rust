#[cfg(feature = "cordl_class_PyramidBloomRendererSO")]
#[repr(C)]
#[derive(Debug)]
pub struct PyramidBloomRendererSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _shader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub _material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub _pyramid: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::GlobalNamespace::PyramidBloomRendererSO_Level,
        >,
    >,
    pub kIsScreenspaceEffectKeyword: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub kLegacyAutoExposureKeyword: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _initialized: bool,
}
#[cfg(feature = "cordl_class_PyramidBloomRendererSO")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PyramidBloomRendererSO {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PyramidBloomRendererSO";
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
#[cfg(feature = "cordl_class_PyramidBloomRendererSO")]
impl std::ops::Deref for crate::GlobalNamespace::PyramidBloomRendererSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "cordl_class_PyramidBloomRendererSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::PyramidBloomRendererSO {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PyramidBloomRendererSO")]
impl crate::GlobalNamespace::PyramidBloomRendererSO {
    pub const kMaxPyramidSize: i32 = 16i32;
    #[cfg(feature = "PyramidBloomRendererSO+Level")]
    pub type Level = crate::GlobalNamespace::PyramidBloomRendererSO_Level;
    #[cfg(feature = "PyramidBloomRendererSO+Pass")]
    pub type Pass = crate::GlobalNamespace::PyramidBloomRendererSO_Pass;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnDisable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnDisable", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnEnable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnEnable", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RenderBloom__cordl_bool__cordl_bool__cordl_bool0(
        &mut self,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        dest: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        radius: f32,
        alphaWeights: bool,
        betterQuality: bool,
        gammaCorrection: bool,
        legacyAutoExposure: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                            f32,
                            bool,
                            bool,
                            bool,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        7usize,
                    >("RenderBloom")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RenderBloom", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        src,
                        dest,
                        radius,
                        alphaWeights,
                        betterQuality,
                        gammaCorrection,
                        legacyAutoExposure,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RenderBloom_f32_f32_f32__cordl_bool_f32_f32_f32_f32_PyramidBloomRendererSO_Pass_PyramidBloomRendererSO_Pass_PyramidBloomRendererSO_Pass_PyramidBloomRendererSO_Pass__cordl_bool__cordl_bool1(
        &mut self,
        src: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        dest: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        radius: f32,
        intensity: f32,
        autoExposureLimit: f32,
        downIntensityOffset: f32,
        uniformPyramidWeights: bool,
        downsampleOnFirstPass: bool,
        pyramidWeightsParam: f32,
        alphaWeights: f32,
        firstUpsampleBrightness: f32,
        finalUpsampleBrightness: f32,
        preFilterPass: crate::GlobalNamespace::PyramidBloomRendererSO_Pass,
        downsamplePass: crate::GlobalNamespace::PyramidBloomRendererSO_Pass,
        upsamplePass: crate::GlobalNamespace::PyramidBloomRendererSO_Pass,
        finalUpsamplePass: crate::GlobalNamespace::PyramidBloomRendererSO_Pass,
        legacyAutoExposure: bool,
        isScreenspaceEffect: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
                            f32,
                            f32,
                            f32,
                            f32,
                            bool,
                            bool,
                            f32,
                            f32,
                            f32,
                            f32,
                            crate::GlobalNamespace::PyramidBloomRendererSO_Pass,
                            crate::GlobalNamespace::PyramidBloomRendererSO_Pass,
                            crate::GlobalNamespace::PyramidBloomRendererSO_Pass,
                            crate::GlobalNamespace::PyramidBloomRendererSO_Pass,
                            bool,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        18usize,
                    >("RenderBloom")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RenderBloom", 18usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        src,
                        dest,
                        radius,
                        intensity,
                        autoExposureLimit,
                        downIntensityOffset,
                        uniformPyramidWeights,
                        downsampleOnFirstPass,
                        pyramidWeightsParam,
                        alphaWeights,
                        firstUpsampleBrightness,
                        finalUpsampleBrightness,
                        preFilterPass,
                        downsamplePass,
                        upsamplePass,
                        finalUpsamplePass,
                        legacyAutoExposure,
                        isScreenspaceEffect,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_PyramidBloomRendererSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PyramidBloomRendererSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_PyramidBloomRendererSO+Level")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PyramidBloomRendererSO_Level {
    pub down: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
    pub up: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
}
#[cfg(feature = "cordl_class_PyramidBloomRendererSO+Level")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PyramidBloomRendererSO_Level {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PyramidBloomRendererSO/Level";
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
#[cfg(feature = "cordl_class_PyramidBloomRendererSO+Level")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::PyramidBloomRendererSO_Level {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_PyramidBloomRendererSO+Level")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::PyramidBloomRendererSO_Level {
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
#[cfg(feature = "cordl_class_PyramidBloomRendererSO+Level")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::PyramidBloomRendererSO_Level {
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
#[cfg(feature = "cordl_class_PyramidBloomRendererSO+Level")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::PyramidBloomRendererSO_Level {
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
#[cfg(feature = "cordl_class_PyramidBloomRendererSO+Level")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::PyramidBloomRendererSO_Level {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "PyramidBloomRendererSO+Level")]
impl crate::GlobalNamespace::PyramidBloomRendererSO_Level {}
#[cfg(feature = "cordl_class_PyramidBloomRendererSO+Pass")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PyramidBloomRendererSO_Pass {
    #[default]
    Bilinear = 9i32,
    BilinearGamma = 10i32,
    Downsample13 = 2i32,
    Downsample4 = 3i32,
    DownsampleBilinearGamma = 4i32,
    Prefilter13 = 0i32,
    Prefilter4 = 1i32,
    UpsampleBox = 6i32,
    UpsampleBoxGamma = 8i32,
    UpsampleTent = 5i32,
    UpsampleTentAndACESToneMapping = 12i32,
    UpsampleTentAndACESToneMappingGlobalIntensity = 13i32,
    UpsampleTentAndReinhardToneMapping = 11i32,
    UpsampleTentGamma = 7i32,
}
#[cfg(feature = "cordl_class_PyramidBloomRendererSO+Pass")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PyramidBloomRendererSO_Pass {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PyramidBloomRendererSO/Pass";
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
#[cfg(feature = "cordl_class_PyramidBloomRendererSO+Pass")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::PyramidBloomRendererSO_Pass {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_PyramidBloomRendererSO+Pass")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::PyramidBloomRendererSO_Pass {
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
#[cfg(feature = "cordl_class_PyramidBloomRendererSO+Pass")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::PyramidBloomRendererSO_Pass {
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
#[cfg(feature = "cordl_class_PyramidBloomRendererSO+Pass")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::PyramidBloomRendererSO_Pass {
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
