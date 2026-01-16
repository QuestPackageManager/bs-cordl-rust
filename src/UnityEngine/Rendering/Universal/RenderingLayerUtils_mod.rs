#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+RenderingLayerUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct RenderingLayerUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+RenderingLayerUtils")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Universal::RenderingLayerUtils {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "RenderingLayerUtils";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+RenderingLayerUtils")]
impl std::ops::Deref for crate::UnityEngine::Rendering::Universal::RenderingLayerUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+RenderingLayerUtils")]
impl std::ops::DerefMut
for crate::UnityEngine::Rendering::Universal::RenderingLayerUtils {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+RenderingLayerUtils")]
impl crate::UnityEngine::Rendering::Universal::RenderingLayerUtils {
    #[cfg(feature = "UnityEngine+Rendering+Universal+RenderingLayerUtils+Event")]
    pub type Event = crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_Event;
    #[cfg(feature = "UnityEngine+Rendering+Universal+RenderingLayerUtils+MaskSize")]
    pub type MaskSize = crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_MaskSize;
    pub fn CombineRendererEvents(
        isDeferred: bool,
        msaaSampleCount: i32,
        rendererEvent: crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_Event,
        combinedEvent: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_Event,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            bool,
                            i32,
                            crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_Event,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_Event,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("CombineRendererEvents")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CombineRendererEvents", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (isDeferred, msaaSampleCount, rendererEvent, combinedEvent),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Combine_RenderingLayerUtils_Event_RenderingLayerUtils_Event0(
        a: crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_Event,
        b: crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_Event,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_Event,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_Event,
                            crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_Event,
                        ),
                        crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_Event,
                        2usize,
                    >("Combine")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Combine",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_Event = unsafe {
            cordl_method_info.invoke_unchecked((), (a, b))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Combine_RenderingLayerUtils_MaskSize_RenderingLayerUtils_MaskSize1(
        a: crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_MaskSize,
        b: crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_MaskSize,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_MaskSize,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_MaskSize,
                            crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_MaskSize,
                        ),
                        crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_MaskSize,
                        2usize,
                    >("Combine")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Combine",
                            2usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_MaskSize = unsafe {
            cordl_method_info.invoke_unchecked((), (a, b))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetBits(
        maskSize: crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_MaskSize,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_MaskSize),
                        i32,
                        1usize,
                    >("GetBits")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "GetBits",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe {
            cordl_method_info.invoke_unchecked((), (maskSize))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetFormat(
        maskSize: crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_MaskSize,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_MaskSize),
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        1usize,
                    >("GetFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetFormat", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Experimental::Rendering::GraphicsFormat = unsafe {
            cordl_method_info.invoke_unchecked((), (maskSize))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetMaskSize(
        bits: i32,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_MaskSize,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (i32),
                        crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_MaskSize,
                        1usize,
                    >("GetMaskSize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetMaskSize", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_MaskSize = unsafe {
            cordl_method_info.invoke_unchecked((), (bits))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RequireRenderingLayers_List_1_RenderingMode__cordl_bool_i32_ByRefMut1(
        rendererFeatures: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::Rendering::Universal::ScriptableRendererFeature,
                >,
            >,
        >,
        renderingMode: crate::UnityEngine::Rendering::Universal::RenderingMode,
        accurateGbufferNormals: bool,
        msaaSampleCount: i32,
        combinedEvent: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_Event,
        >,
        combinedMaskSize: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_MaskSize,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::Rendering::Universal::ScriptableRendererFeature,
                                    >,
                                >,
                            >,
                            crate::UnityEngine::Rendering::Universal::RenderingMode,
                            bool,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_Event,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_MaskSize,
                            >,
                        ),
                        bool,
                        6usize,
                    >("RequireRenderingLayers")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RequireRenderingLayers", 6usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        rendererFeatures,
                        renderingMode,
                        accurateGbufferNormals,
                        msaaSampleCount,
                        combinedEvent,
                        combinedMaskSize,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn RequireRenderingLayers_UniversalRenderer_List_1_i32_ByRefMut0(
        universalRenderer: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalRenderer,
        >,
        rendererFeatures: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::Rendering::Universal::ScriptableRendererFeature,
                >,
            >,
        >,
        msaaSampleCount: i32,
        combinedEvent: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_Event,
        >,
        combinedMaskSize: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_MaskSize,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::UniversalRenderer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::List_1<
                                    quest_hook::libil2cpp::Gc<
                                        crate::UnityEngine::Rendering::Universal::ScriptableRendererFeature,
                                    >,
                                >,
                            >,
                            i32,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_Event,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_MaskSize,
                            >,
                        ),
                        bool,
                        5usize,
                    >("RequireRenderingLayers")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RequireRenderingLayers", 5usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        universalRenderer,
                        rendererFeatures,
                        msaaSampleCount,
                        combinedEvent,
                        combinedMaskSize,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetupProperties_CommandBuffer0(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        maskSize: crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_MaskSize,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_MaskSize,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetupProperties")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetupProperties", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, maskSize))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetupProperties_RasterCommandBuffer1(
        cmd: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RasterCommandBuffer,
        >,
        maskSize: crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_MaskSize,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RasterCommandBuffer,
                            >,
                            crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_MaskSize,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetupProperties")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetupProperties", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (cmd, maskSize))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToValidRenderingLayers(
        renderingLayers: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(u32), u32, 1usize>("ToValidRenderingLayers")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ToValidRenderingLayers", 1usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe {
            cordl_method_info.invoke_unchecked((), (renderingLayers))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+RenderingLayerUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::Universal::RenderingLayerUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+RenderingLayerUtils+Event")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RenderingLayerUtils_Event {
    #[default]
    DepthNormalPrePass = 0i32,
    Opaque = 1i32,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+RenderingLayerUtils+Event")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_Event {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "RenderingLayerUtils/Event";
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+RenderingLayerUtils+Event")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_Event {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+RenderingLayerUtils+Event")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_Event {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+RenderingLayerUtils+Event")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_Event {
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
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+RenderingLayerUtils+Event")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_Event {
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+RenderingLayerUtils+MaskSize"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RenderingLayerUtils_MaskSize {
    #[default]
    Bits16 = 1i32,
    Bits24 = 2i32,
    Bits32 = 3i32,
    Bits8 = 0i32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+RenderingLayerUtils+MaskSize"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_MaskSize {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "RenderingLayerUtils/MaskSize";
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+RenderingLayerUtils+MaskSize"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_MaskSize {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+RenderingLayerUtils+MaskSize"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_MaskSize {
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+RenderingLayerUtils+MaskSize"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_MaskSize {
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+RenderingLayerUtils+MaskSize"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_MaskSize {
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
