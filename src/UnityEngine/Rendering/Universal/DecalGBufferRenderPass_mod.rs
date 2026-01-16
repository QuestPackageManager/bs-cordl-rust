#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+DecalGBufferRenderPass")]
#[repr(C)]
#[derive(Debug)]
pub struct DecalGBufferRenderPass {
    __cordl_parent: crate::UnityEngine::Rendering::Universal::ScriptableRenderPass,
    pub m_FilteringSettings: crate::UnityEngine::Rendering::FilteringSettings,
    pub m_ShaderTagIdList: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<crate::UnityEngine::Rendering::ShaderTagId>,
    >,
    pub m_DrawSystem:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::DecalDrawGBufferSystem>,
    pub m_Settings: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::Universal::DecalScreenSpaceSettings,
    >,
    pub m_DeferredLights: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::Universal::Internal::DeferredLights,
    >,
    pub m_GbufferAttachments: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        >,
    >,
    pub m_DecalLayers: bool,
    pub m_PassData: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::Universal::DecalGBufferRenderPass_PassData,
    >,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+DecalGBufferRenderPass")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::DecalGBufferRenderPass
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "DecalGBufferRenderPass";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+DecalGBufferRenderPass")]
impl std::ops::Deref for crate::UnityEngine::Rendering::Universal::DecalGBufferRenderPass {
    type Target = crate::UnityEngine::Rendering::Universal::ScriptableRenderPass;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+DecalGBufferRenderPass")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::Universal::DecalGBufferRenderPass {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+DecalGBufferRenderPass")]
impl crate::UnityEngine::Rendering::Universal::DecalGBufferRenderPass {
    #[cfg(feature = "UnityEngine+Rendering+Universal+DecalGBufferRenderPass+PassData")]
    pub type PassData = crate::UnityEngine::Rendering::Universal::DecalGBufferRenderPass_PassData;
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
    pub fn ExecutePass(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RasterCommandBuffer>,
        passData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DecalGBufferRenderPass_PassData,
        >,
        rendererList: crate::UnityEngine::Rendering::RendererList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::RasterCommandBuffer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::DecalGBufferRenderPass_PassData,
                            >,
                            crate::UnityEngine::Rendering::RendererList,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("ExecutePass")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ExecutePass", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (cmd, passData, rendererList))? };
        Ok(__cordl_ret.into())
    }
    pub fn InitPassData(
        &mut self,
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
        passData: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::Universal::DecalGBufferRenderPass_PassData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
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
                                    crate::UnityEngine::Rendering::Universal::DecalGBufferRenderPass_PassData,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("InitPassData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InitPassData", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (cameraData, passData))? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        settings: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DecalScreenSpaceSettings,
        >,
        drawSystem: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DecalDrawGBufferSystem,
        >,
        decalLayers: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (settings, drawSystem, decalLayers))?;
        Ok(__cordl_object.into())
    }
    pub fn OnCameraCleanup(
        &mut self,
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::CommandBuffer,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OnCameraCleanup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnCameraCleanup", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (cmd))? };
        Ok(__cordl_ret.into())
    }
    pub fn RecordRenderGraph(
        &mut self,
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        frameData: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ContextContainer>,
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
                    ), quest_hook::libil2cpp::Void, 2usize>("RecordRenderGraph")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RecordRenderGraph",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (renderGraph, frameData))? };
        Ok(__cordl_ret.into())
    }
    pub fn Setup(
        &mut self,
        deferredLights: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::Internal::DeferredLights,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::Rendering::Universal::Internal::DeferredLights,
                    >), quest_hook::libil2cpp::Void, 1usize>("Setup")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Setup",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (deferredLights))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        settings: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DecalScreenSpaceSettings,
        >,
        drawSystem: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DecalDrawGBufferSystem,
        >,
        decalLayers: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::DecalScreenSpaceSettings,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::DecalDrawGBufferSystem,
                        >,
                        bool,
                    ), quest_hook::libil2cpp::Void, 3usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (settings, drawSystem, decalLayers))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+DecalGBufferRenderPass")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::DecalGBufferRenderPass
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+DecalGBufferRenderPass+PassData")]
#[repr(C)]
#[derive(Debug)]
pub struct DecalGBufferRenderPass_PassData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub drawSystem:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::DecalDrawGBufferSystem>,
    pub settings: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::Universal::DecalScreenSpaceSettings,
    >,
    pub decalLayers: bool,
    pub cameraData:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::Universal::UniversalCameraData>,
    pub rendererList: crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+DecalGBufferRenderPass+PassData")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::DecalGBufferRenderPass_PassData
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "DecalGBufferRenderPass/PassData";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+DecalGBufferRenderPass+PassData")]
impl std::ops::Deref for crate::UnityEngine::Rendering::Universal::DecalGBufferRenderPass_PassData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+DecalGBufferRenderPass+PassData")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::Universal::DecalGBufferRenderPass_PassData
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+DecalGBufferRenderPass+PassData")]
impl crate::UnityEngine::Rendering::Universal::DecalGBufferRenderPass_PassData {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+DecalGBufferRenderPass+PassData")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::DecalGBufferRenderPass_PassData
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
