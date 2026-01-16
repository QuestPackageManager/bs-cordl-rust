#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+FinalBlitPass")]
#[repr(C)]
#[derive(Debug)]
pub struct FinalBlitPass {
    __cordl_parent: crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
    pub m_Source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    pub m_PassData: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass_PassData,
    >,
    pub m_BlitMaterialData: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass_BlitMaterialData,
        >,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+FinalBlitPass")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal.Internal";
    const CLASS_NAME: &'static str = "FinalBlitPass";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+Internal+FinalBlitPass")]
impl std::ops::Deref
for crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass {
    type Target = crate::UnityEngine::Rendering::Universal::ScriptableRenderPass;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+Internal+FinalBlitPass")]
impl std::ops::DerefMut
for crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+Internal+FinalBlitPass")]
impl crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass {
    #[cfg(
        feature = "UnityEngine+Rendering+Universal+Internal+FinalBlitPass+BlitMaterialData"
    )]
    pub type BlitMaterialData = crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass_BlitMaterialData;
    #[cfg(
        feature = "UnityEngine+Rendering+Universal+Internal+FinalBlitPass+BlitPassNames"
    )]
    pub type BlitPassNames = crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass_BlitPassNames;
    #[cfg(feature = "UnityEngine+Rendering+Universal+Internal+FinalBlitPass+BlitType")]
    pub type BlitType = crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass_BlitType;
    #[cfg(feature = "UnityEngine+Rendering+Universal+Internal+FinalBlitPass+PassData")]
    pub type PassData = crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass_PassData;
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Dispose",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Execute(
        &mut self,
        context: crate::UnityEngine::Rendering::ScriptableRenderContext,
        renderingData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::RenderingData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Rendering::ScriptableRenderContext,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::Universal::RenderingData,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Execute",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (context, renderingData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ExecutePass(
        cmd: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RasterCommandBuffer,
        >,
        data: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass_PassData,
        >,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        destination: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
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
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass_PassData,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RTHandle,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RTHandle,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("ExecutePass")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExecutePass", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (cmd, data, source, destination, cameraData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InitPassData(
        &mut self,
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
        passData: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass_PassData,
            >,
        >,
        blitType: crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass_BlitType,
        enableAlphaOutput: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<
                                    crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass_PassData,
                                >,
                            >,
                            crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass_BlitType,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >("InitPassData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InitPassData", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (cameraData, passData, blitType, enableAlphaOutput),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        evt: crate::UnityEngine::Rendering::Universal::RenderPassEvent,
        blitMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        blitHDRMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (evt, blitMaterial, blitHDRMaterial))?;
        Ok(__cordl_object.into())
    }
    pub fn OnCameraSetup(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        renderingData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::RenderingData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::CommandBuffer,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::Universal::RenderingData,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("OnCameraSetup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnCameraSetup", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (cmd, renderingData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Render(
        &mut self,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        frameData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::ContextContainer,
        >,
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
        src: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        dest: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        >,
        overlayUITexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::ContextContainer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                            >,
                            quest_hook::libil2cpp::ByRefMut<
                                crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                            >,
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >("Render")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Render",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (renderGraph, frameData, cameraData, src, dest, overlayUITexture),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetupHDROutput(
        hdrDisplayColorGamut: crate::UnityEngine::ColorGamut,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        hdrOperation: crate::UnityEngine::Rendering::HDROutputUtils_Operation,
        hdrOutputParameters: crate::UnityEngine::Vector4,
        rendersOverlayUI: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::UnityEngine::ColorGamut,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            crate::UnityEngine::Rendering::HDROutputUtils_Operation,
                            crate::UnityEngine::Vector4,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >("SetupHDROutput")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetupHDROutput", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    (),
                    (
                        hdrDisplayColorGamut,
                        material,
                        hdrOperation,
                        hdrOutputParameters,
                        rendersOverlayUI,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Setup_RTHandle1(
        &mut self,
        baseDescriptor: crate::UnityEngine::RenderTextureDescriptor,
        colorHandle: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::RenderTextureDescriptor,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RTHandle,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Setup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Setup",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (baseDescriptor, colorHandle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Setup_RenderTargetHandle0(
        &mut self,
        baseDescriptor: crate::UnityEngine::RenderTextureDescriptor,
        colorHandle: crate::UnityEngine::Rendering::Universal::RenderTargetHandle,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::RenderTextureDescriptor,
                            crate::UnityEngine::Rendering::Universal::RenderTargetHandle,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("Setup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Setup",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (baseDescriptor, colorHandle))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        evt: crate::UnityEngine::Rendering::Universal::RenderPassEvent,
        blitMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        blitHDRMaterial: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::UnityEngine::Rendering::Universal::RenderPassEvent,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (evt, blitMaterial, blitHDRMaterial))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+FinalBlitPass")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+FinalBlitPass+BlitMaterialData"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct FinalBlitPass_BlitMaterialData {
    pub material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    pub nearestSamplerPass: i32,
    pub bilinearSamplerPass: i32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+FinalBlitPass+BlitMaterialData"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass_BlitMaterialData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal.Internal";
    const CLASS_NAME: &'static str = "FinalBlitPass/BlitMaterialData";
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+FinalBlitPass+BlitMaterialData"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass_BlitMaterialData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+FinalBlitPass+BlitMaterialData"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass_BlitMaterialData {
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+FinalBlitPass+BlitMaterialData"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass_BlitMaterialData {
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+FinalBlitPass+BlitMaterialData"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass_BlitMaterialData {
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+FinalBlitPass+BlitMaterialData"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass_BlitMaterialData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+Universal+Internal+FinalBlitPass+BlitMaterialData"
)]
impl crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass_BlitMaterialData {}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+FinalBlitPass+BlitPassNames"
)]
#[repr(C)]
#[derive(Debug)]
pub struct FinalBlitPass_BlitPassNames {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+FinalBlitPass+BlitPassNames"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass_BlitPassNames {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal.Internal";
    const CLASS_NAME: &'static str = "FinalBlitPass/BlitPassNames";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+Internal+FinalBlitPass+BlitPassNames")]
impl std::ops::Deref
for crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass_BlitPassNames {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+Internal+FinalBlitPass+BlitPassNames")]
impl std::ops::DerefMut
for crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass_BlitPassNames {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+Internal+FinalBlitPass+BlitPassNames")]
impl crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass_BlitPassNames {
    pub const BilinearSampler: &'static str = "BilinearDebugDraw";
    pub const NearestSampler: &'static str = "NearestDebugDraw";
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+FinalBlitPass+BlitPassNames"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass_BlitPassNames {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+FinalBlitPass+BlitType"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FinalBlitPass_BlitType {
    #[default]
    Core = 0i32,
    Count = 2i32,
    HDR = 1i32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+FinalBlitPass+BlitType"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass_BlitType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal.Internal";
    const CLASS_NAME: &'static str = "FinalBlitPass/BlitType";
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+FinalBlitPass+BlitType"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass_BlitType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+FinalBlitPass+BlitType"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass_BlitType {
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+FinalBlitPass+BlitType"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass_BlitType {
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+FinalBlitPass+BlitType"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass_BlitType {
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+FinalBlitPass+PassData"
)]
#[repr(C)]
#[derive(Debug)]
pub struct FinalBlitPass_PassData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub source: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub destination: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
    pub sourceID: i32,
    pub hdrOutputLuminanceParams: crate::UnityEngine::Vector4,
    pub requireSrgbConversion: bool,
    pub enableAlphaOutput: bool,
    pub blitMaterialData: crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass_BlitMaterialData,
    pub cameraData: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::Universal::UniversalCameraData,
    >,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+FinalBlitPass+PassData"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass_PassData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal.Internal";
    const CLASS_NAME: &'static str = "FinalBlitPass/PassData";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+Internal+FinalBlitPass+PassData")]
impl std::ops::Deref
for crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass_PassData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+Internal+FinalBlitPass+PassData")]
impl std::ops::DerefMut
for crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass_PassData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+Internal+FinalBlitPass+PassData")]
impl crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass_PassData {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+FinalBlitPass+PassData"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::Universal::Internal::FinalBlitPass_PassData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
