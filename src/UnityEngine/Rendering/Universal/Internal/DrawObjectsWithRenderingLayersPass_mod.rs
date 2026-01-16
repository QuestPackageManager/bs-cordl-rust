#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+DrawObjectsWithRenderingLayersPass"
)]
#[repr(C)]
#[derive(Debug)]
pub struct DrawObjectsWithRenderingLayersPass {
    __cordl_parent: crate::UnityEngine::Rendering::Universal::Internal::DrawObjectsPass,
    pub m_ColorTargetIndentifiers: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        >,
    >,
    pub m_DepthTargetIndentifiers:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+DrawObjectsWithRenderingLayersPass"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::Internal::DrawObjectsWithRenderingLayersPass
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal.Internal";
    const CLASS_NAME: &'static str = "DrawObjectsWithRenderingLayersPass";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "UnityEngine+Rendering+Universal+Internal+DrawObjectsWithRenderingLayersPass")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::Universal::Internal::DrawObjectsWithRenderingLayersPass
{
    type Target = crate::UnityEngine::Rendering::Universal::Internal::DrawObjectsPass;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+Internal+DrawObjectsWithRenderingLayersPass")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::Universal::Internal::DrawObjectsWithRenderingLayersPass
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+Internal+DrawObjectsWithRenderingLayersPass")]
impl crate::UnityEngine::Rendering::Universal::Internal::DrawObjectsWithRenderingLayersPass {
    #[cfg(
        feature = "UnityEngine+Rendering+Universal+Internal+DrawObjectsWithRenderingLayersPass+RenderingLayersPassData"
    )]
    pub type RenderingLayersPassData = crate::UnityEngine::Rendering::Universal::Internal::DrawObjectsWithRenderingLayersPass_RenderingLayersPassData;
    pub fn Configure(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        cameraTextureDescriptor: crate::UnityEngine::RenderTextureDescriptor,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        crate::UnityEngine::RenderTextureDescriptor,
                    ), quest_hook::libil2cpp::Void, 2usize>("Configure")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Configure",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (cmd, cameraTextureDescriptor))? };
        Ok(__cordl_ret.into())
    }
    pub fn Execute(
        &mut self,
        context: crate::UnityEngine::Rendering::ScriptableRenderContext,
        renderingData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::RenderingData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::ScriptableRenderContext,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::Universal::RenderingData,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("Execute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Execute",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (context, renderingData))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        profilerTag: crate::UnityEngine::Rendering::Universal::URPProfileId,
        opaque: bool,
        evt: crate::UnityEngine::Rendering::Universal::RenderPassEvent,
        renderQueueRange: crate::UnityEngine::Rendering::RenderQueueRange,
        layerMask: crate::UnityEngine::LayerMask,
        stencilState: crate::UnityEngine::Rendering::StencilState,
        stencilReference: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object).invoke_void(
            ".ctor",
            (
                profilerTag,
                opaque,
                evt,
                renderQueueRange,
                layerMask,
                stencilState,
                stencilReference,
            ),
        )?;
        Ok(__cordl_object.into())
    }
    pub fn Render(
        &mut self,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        frameData: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ContextContainer>,
        colorTarget: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        renderingLayersTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        depthTarget: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        mainShadowsTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        additionalShadowsTexture: crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
        maskSize: crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_MaskSize,
        batchLayerMask: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ContextContainer>,
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureHandle,
                        crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_MaskSize,
                        u32,
                    ), quest_hook::libil2cpp::Void, 9usize>("Render")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Render",
                            9usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    renderGraph,
                    frameData,
                    colorTarget,
                    renderingLayersTexture,
                    depthTarget,
                    mainShadowsTexture,
                    additionalShadowsTexture,
                    maskSize,
                    batchLayerMask,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Setup(
        &mut self,
        colorAttachment: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        renderingLayersTexture: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        depthAttachment: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                    ), quest_hook::libil2cpp::Void, 3usize>("Setup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Setup",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (colorAttachment, renderingLayersTexture, depthAttachment),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        profilerTag: crate::UnityEngine::Rendering::Universal::URPProfileId,
        opaque: bool,
        evt: crate::UnityEngine::Rendering::Universal::RenderPassEvent,
        renderQueueRange: crate::UnityEngine::Rendering::RenderQueueRange,
        layerMask: crate::UnityEngine::LayerMask,
        stencilState: crate::UnityEngine::Rendering::StencilState,
        stencilReference: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::UnityEngine::Rendering::Universal::URPProfileId,
                        bool,
                        crate::UnityEngine::Rendering::Universal::RenderPassEvent,
                        crate::UnityEngine::Rendering::RenderQueueRange,
                        crate::UnityEngine::LayerMask,
                        crate::UnityEngine::Rendering::StencilState,
                        i32,
                    ), quest_hook::libil2cpp::Void, 7usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    profilerTag,
                    opaque,
                    evt,
                    renderQueueRange,
                    layerMask,
                    stencilState,
                    stencilReference,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+DrawObjectsWithRenderingLayersPass"
)]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::Internal::DrawObjectsWithRenderingLayersPass
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+DrawObjectsWithRenderingLayersPass+RenderingLayersPassData"
)]
#[repr(C)]
#[derive(Debug)]
pub struct DrawObjectsWithRenderingLayersPass_RenderingLayersPassData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub basePassData: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::Universal::Internal::DrawObjectsPass_PassData,
    >,
    pub maskSize: crate::UnityEngine::Rendering::Universal::RenderingLayerUtils_MaskSize,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+DrawObjectsWithRenderingLayersPass+RenderingLayersPassData"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Universal::Internal::DrawObjectsWithRenderingLayersPass_RenderingLayersPassData {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal.Internal";
    const CLASS_NAME: &'static str = "DrawObjectsWithRenderingLayersPass/RenderingLayersPassData";
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
#[cfg(
    feature = "UnityEngine+Rendering+Universal+Internal+DrawObjectsWithRenderingLayersPass+RenderingLayersPassData"
)]
impl std::ops::Deref
for crate::UnityEngine::Rendering::Universal::Internal::DrawObjectsWithRenderingLayersPass_RenderingLayersPassData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+Universal+Internal+DrawObjectsWithRenderingLayersPass+RenderingLayersPassData"
)]
impl std::ops::DerefMut
for crate::UnityEngine::Rendering::Universal::Internal::DrawObjectsWithRenderingLayersPass_RenderingLayersPassData {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+Universal+Internal+DrawObjectsWithRenderingLayersPass+RenderingLayersPassData"
)]
impl crate::UnityEngine::Rendering::Universal::Internal::DrawObjectsWithRenderingLayersPass_RenderingLayersPassData {
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+Internal+DrawObjectsWithRenderingLayersPass+RenderingLayersPassData"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::Universal::Internal::DrawObjectsWithRenderingLayersPass_RenderingLayersPassData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
