#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+RenderingUtils")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct RenderingUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+RenderingUtils")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::RenderingUtils
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "RenderingUtils";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+RenderingUtils")]
impl std::ops::Deref for crate::UnityEngine::Rendering::Universal::RenderingUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+RenderingUtils")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::Universal::RenderingUtils {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+RenderingUtils")]
impl crate::UnityEngine::Rendering::Universal::RenderingUtils {
    pub fn AddStaleResourceToPoolOrRelease(
        desc: crate::UnityEngine::Rendering::RenderGraphModule::TextureDesc,
        handle: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Rendering::RenderGraphModule::TextureDesc,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "AddStaleResourceToPoolOrRelease"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "AddStaleResourceToPoolOrRelease",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (desc, handle))? };
        Ok(__cordl_ret.into())
    }
    pub fn Blit_ClearFlag_Color_Material_i32_0(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        viewport: crate::UnityEngine::Rect,
        destination: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        loadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        storeAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        clearFlag: crate::UnityEngine::Rendering::ClearFlag,
        clearColor: crate::UnityEngine::Color,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        passIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        crate::UnityEngine::Rect,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        crate::UnityEngine::Rendering::RenderBufferLoadAction,
                        crate::UnityEngine::Rendering::RenderBufferStoreAction,
                        crate::UnityEngine::Rendering::ClearFlag,
                        crate::UnityEngine::Color,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 10usize>("Blit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Blit",
                            10usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    cmd,
                    source,
                    viewport,
                    destination,
                    loadAction,
                    storeAction,
                    clearFlag,
                    clearColor,
                    material,
                    passIndex,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Blit_RTHandle_RenderBufferLoadAction_RenderBufferStoreAction_ClearFlag_Color_Material_i32_1(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        viewport: crate::UnityEngine::Rect,
        destinationColor: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        colorLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        colorStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        destinationDepthStencil: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        depthStencilLoadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        depthStencilStoreAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        clearFlag: crate::UnityEngine::Rendering::ClearFlag,
        clearColor: crate::UnityEngine::Color,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        passIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        crate::UnityEngine::Rect,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        crate::UnityEngine::Rendering::RenderBufferLoadAction,
                        crate::UnityEngine::Rendering::RenderBufferStoreAction,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        crate::UnityEngine::Rendering::RenderBufferLoadAction,
                        crate::UnityEngine::Rendering::RenderBufferStoreAction,
                        crate::UnityEngine::Rendering::ClearFlag,
                        crate::UnityEngine::Color,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 13usize>("Blit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Blit",
                            13usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    cmd,
                    source,
                    viewport,
                    destinationColor,
                    colorLoadAction,
                    colorStoreAction,
                    destinationDepthStencil,
                    depthStencilLoadAction,
                    depthStencilStoreAction,
                    clearFlag,
                    clearColor,
                    material,
                    passIndex,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClearSystemInfoCache() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "ClearSystemInfoCache",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ClearSystemInfoCache",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Contains_ClearFlag_ClearFlag1(
        a: crate::UnityEngine::Rendering::ClearFlag,
        b: crate::UnityEngine::Rendering::ClearFlag,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Rendering::ClearFlag,
                        crate::UnityEngine::Rendering::ClearFlag,
                    ), bool, 2usize>("Contains")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Contains",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (a, b))? };
        Ok(__cordl_ret.into())
    }
    pub fn Contains_Il2CppArray_RenderTargetIdentifier0(
        source: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::RenderTargetIdentifier,
            >,
        >,
        value: crate::UnityEngine::Rendering::RenderTargetIdentifier,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                crate::UnityEngine::Rendering::RenderTargetIdentifier,
                            >,
                        >,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                    ), bool, 2usize>("Contains")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Contains",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (source, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn CountDistinct(
        source: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
            >,
        >,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                    ), u32, 2usize>("CountDistinct")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CountDistinct",
                            2usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { cordl_method_info.invoke_unchecked((), (source, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateDrawingSettings_List_1_ByRefMut_SortingCriteria2(
        shaderTagIdList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Rendering::ShaderTagId>,
        >,
        renderingData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::RenderingData,
        >,
        sortingCriteria: crate::UnityEngine::Rendering::SortingCriteria,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::DrawingSettings> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::Rendering::ShaderTagId,
                            >,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::Universal::RenderingData,
                        >,
                        crate::UnityEngine::Rendering::SortingCriteria,
                    ), crate::UnityEngine::Rendering::DrawingSettings, 3usize>(
                        "CreateDrawingSettings",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateDrawingSettings",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::DrawingSettings = unsafe {
            cordl_method_info
                .invoke_unchecked((), (shaderTagIdList, renderingData, sortingCriteria))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateDrawingSettings_List_1_UniversalRenderingData_UniversalCameraData_UniversalLightData_SortingCriteria3(
        shaderTagIdList: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::UnityEngine::Rendering::ShaderTagId>,
        >,
        renderingData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalRenderingData,
        >,
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
        lightData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalLightData,
        >,
        sortingCriteria: crate::UnityEngine::Rendering::SortingCriteria,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::DrawingSettings> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::UnityEngine::Rendering::ShaderTagId,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalRenderingData,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalLightData,
                        >,
                        crate::UnityEngine::Rendering::SortingCriteria,
                    ), crate::UnityEngine::Rendering::DrawingSettings, 5usize>(
                        "CreateDrawingSettings",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateDrawingSettings",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::DrawingSettings = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    shaderTagIdList,
                    renderingData,
                    cameraData,
                    lightData,
                    sortingCriteria,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateDrawingSettings_ShaderTagId_ByRefMut_SortingCriteria0(
        shaderTagId: crate::UnityEngine::Rendering::ShaderTagId,
        renderingData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::RenderingData,
        >,
        sortingCriteria: crate::UnityEngine::Rendering::SortingCriteria,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::DrawingSettings> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Rendering::ShaderTagId,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::Universal::RenderingData,
                        >,
                        crate::UnityEngine::Rendering::SortingCriteria,
                    ), crate::UnityEngine::Rendering::DrawingSettings, 3usize>(
                        "CreateDrawingSettings",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateDrawingSettings",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::DrawingSettings = unsafe {
            cordl_method_info.invoke_unchecked((), (shaderTagId, renderingData, sortingCriteria))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateDrawingSettings_ShaderTagId_UniversalRenderingData_UniversalCameraData_UniversalLightData_SortingCriteria1(
        shaderTagId: crate::UnityEngine::Rendering::ShaderTagId,
        renderingData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalRenderingData,
        >,
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
        lightData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalLightData,
        >,
        sortingCriteria: crate::UnityEngine::Rendering::SortingCriteria,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::DrawingSettings> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Rendering::ShaderTagId,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalRenderingData,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalLightData,
                        >,
                        crate::UnityEngine::Rendering::SortingCriteria,
                    ), crate::UnityEngine::Rendering::DrawingSettings, 5usize>(
                        "CreateDrawingSettings",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateDrawingSettings",
                            5usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::DrawingSettings = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    shaderTagId,
                    renderingData,
                    cameraData,
                    lightData,
                    sortingCriteria,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateRendererListObjectsWithError_RenderGraph1(
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        cullResults: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::CullingResults>,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        filterSettings: crate::UnityEngine::Rendering::FilteringSettings,
        sortFlags: crate::UnityEngine::Rendering::SortingCriteria,
        rl: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::CullingResults,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        crate::UnityEngine::Rendering::FilteringSettings,
                        crate::UnityEngine::Rendering::SortingCriteria,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
                        >,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "CreateRendererListObjectsWithError"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateRendererListObjectsWithError",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    renderGraph,
                    cullResults,
                    camera,
                    filterSettings,
                    sortFlags,
                    rl,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateRendererListObjectsWithError_ScriptableRenderContext0(
        context: crate::UnityEngine::Rendering::ScriptableRenderContext,
        cullResults: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::CullingResults>,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        filterSettings: crate::UnityEngine::Rendering::FilteringSettings,
        sortFlags: crate::UnityEngine::Rendering::SortingCriteria,
        rl: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::RendererList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Rendering::ScriptableRenderContext,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::CullingResults,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        crate::UnityEngine::Rendering::FilteringSettings,
                        crate::UnityEngine::Rendering::SortingCriteria,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RendererList,
                        >,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "CreateRendererListObjectsWithError"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateRendererListObjectsWithError",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (context, cullResults, camera, filterSettings, sortFlags, rl),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateRendererListWithRenderStateBlock_RenderGraph1(
        renderGraph: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
        >,
        cullResults: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::CullingResults>,
        ds: crate::UnityEngine::Rendering::DrawingSettings,
        fs: crate::UnityEngine::Rendering::FilteringSettings,
        rsb: crate::UnityEngine::Rendering::RenderStateBlock,
        rl: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RenderGraphModule::RenderGraph,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::CullingResults,
                        >,
                        crate::UnityEngine::Rendering::DrawingSettings,
                        crate::UnityEngine::Rendering::FilteringSettings,
                        crate::UnityEngine::Rendering::RenderStateBlock,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::RendererListHandle,
                        >,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "CreateRendererListWithRenderStateBlock",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateRendererListWithRenderStateBlock",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (renderGraph, cullResults, ds, fs, rsb, rl))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateRendererListWithRenderStateBlock_ScriptableRenderContext0(
        context: crate::UnityEngine::Rendering::ScriptableRenderContext,
        cullResults: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::CullingResults>,
        ds: crate::UnityEngine::Rendering::DrawingSettings,
        fs: crate::UnityEngine::Rendering::FilteringSettings,
        rsb: crate::UnityEngine::Rendering::RenderStateBlock,
        rl: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::RendererList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Rendering::ScriptableRenderContext,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::CullingResults,
                        >,
                        crate::UnityEngine::Rendering::DrawingSettings,
                        crate::UnityEngine::Rendering::FilteringSettings,
                        crate::UnityEngine::Rendering::RenderStateBlock,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RendererList,
                        >,
                    ), quest_hook::libil2cpp::Void, 6usize>(
                        "CreateRendererListWithRenderStateBlock",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateRendererListWithRenderStateBlock",
                            6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (context, cullResults, ds, fs, rsb, rl))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateRendererParamsObjectsWithError(
        cullResults: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::CullingResults>,
        camera: quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
        filterSettings: crate::UnityEngine::Rendering::FilteringSettings,
        sortFlags: crate::UnityEngine::Rendering::SortingCriteria,
        param: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::RendererListParams>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::CullingResults,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Camera>,
                        crate::UnityEngine::Rendering::FilteringSettings,
                        crate::UnityEngine::Rendering::SortingCriteria,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RendererListParams,
                        >,
                    ), quest_hook::libil2cpp::Void, 5usize>(
                        "CreateRendererParamsObjectsWithError"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "CreateRendererParamsObjectsWithError",
                            5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (cullResults, camera, filterSettings, sortFlags, param))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn DrawRendererListObjectsWithError(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RasterCommandBuffer>,
        rl: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Rendering::RendererList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RasterCommandBuffer,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RendererList,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "DrawRendererListObjectsWithError"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "DrawRendererListObjectsWithError",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (cmd, rl))? };
        Ok(__cordl_ret.into())
    }
    pub fn FinalBlit(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        destination: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        loadAction: crate::UnityEngine::Rendering::RenderBufferLoadAction,
        storeAction: crate::UnityEngine::Rendering::RenderBufferStoreAction,
        material: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
        passIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        crate::UnityEngine::Rendering::RenderBufferLoadAction,
                        crate::UnityEngine::Rendering::RenderBufferStoreAction,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 8usize>("FinalBlit")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FinalBlit",
                            8usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    cmd,
                    cameraData,
                    source,
                    destination,
                    loadAction,
                    storeAction,
                    material,
                    passIndex,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetCameraTargetIdentifier(
        renderingData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::RenderingData,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::RenderTargetIdentifier> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::ByRefMut<
                        crate::UnityEngine::Rendering::Universal::RenderingData,
                    >), crate::UnityEngine::Rendering::RenderTargetIdentifier, 1usize>(
                        "GetCameraTargetIdentifier",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetCameraTargetIdentifier",
                            1usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::RenderTargetIdentifier =
            unsafe { cordl_method_info.invoke_unchecked((), (renderingData))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetFinalBlitScaleBias(
        source: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        destination: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        cameraData: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                        >,
                    ), crate::UnityEngine::Vector4, 3usize>(
                        "GetFinalBlitScaleBias"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetFinalBlitScaleBias",
                            3usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Vector4 =
            unsafe { cordl_method_info.invoke_unchecked((), (source, destination, cameraData))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetLastValidColorBufferIndex(
        colorBuffers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::UnityEngine::Rendering::RenderTargetIdentifier,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            crate::UnityEngine::Rendering::RenderTargetIdentifier,
                        >,
                    >), i32, 1usize>("GetLastValidColorBufferIndex")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetLastValidColorBufferIndex",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (colorBuffers))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetValidColorBufferCount(
        colorBuffers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        >,
                    >), u32, 1usize>("GetValidColorBufferCount")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetValidColorBufferCount",
                            1usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { cordl_method_info.invoke_unchecked((), (colorBuffers))? };
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf_RTHandle1(
        source: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
            >,
        >,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                    ), i32, 2usize>("IndexOf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IndexOf",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (source, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn IndexOf_RenderTargetIdentifier0(
        source: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
            >,
        >,
        value: crate::UnityEngine::Rendering::RenderTargetIdentifier,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                            >,
                        >,
                        crate::UnityEngine::Rendering::RenderTargetIdentifier,
                    ), i32, 2usize>("IndexOf")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IndexOf",
                            2usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (source, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsMRT(
        colorBuffers: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        >,
                    >), bool, 1usize>("IsMRT")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IsMRT",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (colorBuffers))? };
        Ok(__cordl_ret.into())
    }
    pub fn LastValid(
        source: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        >,
                    >), i32, 1usize>("LastValid")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "LastValid",
                            1usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked((), (source))? };
        Ok(__cordl_ret.into())
    }
    pub fn MultisampleDepthResolveSupported() -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), bool, 0usize>("MultisampleDepthResolveSupported")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "MultisampleDepthResolveSupported",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn RTHandleNeedsReAlloc(
        handle: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        descriptor: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::RenderGraphModule::TextureDesc,
        >,
        scaled: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::RenderGraphModule::TextureDesc,
                        >,
                        bool,
                    ), bool, 3usize>("RTHandleNeedsReAlloc")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "RTHandleNeedsReAlloc",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (handle, descriptor, scaled))? };
        Ok(__cordl_ret.into())
    }
    pub fn ReAllocateHandleIfNeeded_ByRefMut_FilterMode_TextureWrapMode_i32_f32_Il2CppString0(
        handle: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        >,
        descriptor: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RenderTextureDescriptor>,
        filterMode: crate::UnityEngine::FilterMode,
        wrapMode: crate::UnityEngine::TextureWrapMode,
        anisoLevel: i32,
        mipMapBias: f32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::RenderTextureDescriptor,
                        >,
                        crate::UnityEngine::FilterMode,
                        crate::UnityEngine::TextureWrapMode,
                        i32,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), bool, 7usize>("ReAllocateHandleIfNeeded")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReAllocateHandleIfNeeded",
                            7usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    handle, descriptor, filterMode, wrapMode, anisoLevel, mipMapBias, name,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReAllocateHandleIfNeeded_ScaleFunc_ByRefMut_FilterMode_TextureWrapMode_i32_f32_Il2CppString2(
        handle: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        >,
        scaleFunc: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ScaleFunc>,
        descriptor: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RenderTextureDescriptor>,
        filterMode: crate::UnityEngine::FilterMode,
        wrapMode: crate::UnityEngine::TextureWrapMode,
        anisoLevel: i32,
        mipMapBias: f32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ScaleFunc>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::RenderTextureDescriptor,
                        >,
                        crate::UnityEngine::FilterMode,
                        crate::UnityEngine::TextureWrapMode,
                        i32,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), bool, 8usize>("ReAllocateHandleIfNeeded")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReAllocateHandleIfNeeded",
                            8usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    handle, scaleFunc, descriptor, filterMode, wrapMode, anisoLevel, mipMapBias,
                    name,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReAllocateHandleIfNeeded_Vector2_ByRefMut_FilterMode_TextureWrapMode_i32_f32_Il2CppString1(
        handle: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        >,
        scaleFactor: crate::UnityEngine::Vector2,
        descriptor: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RenderTextureDescriptor>,
        filterMode: crate::UnityEngine::FilterMode,
        wrapMode: crate::UnityEngine::TextureWrapMode,
        anisoLevel: i32,
        mipMapBias: f32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        >,
                        crate::UnityEngine::Vector2,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::RenderTextureDescriptor,
                        >,
                        crate::UnityEngine::FilterMode,
                        crate::UnityEngine::TextureWrapMode,
                        i32,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), bool, 8usize>("ReAllocateHandleIfNeeded")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReAllocateHandleIfNeeded",
                            8usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    handle,
                    scaleFactor,
                    descriptor,
                    filterMode,
                    wrapMode,
                    anisoLevel,
                    mipMapBias,
                    name,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReAllocateIfNeeded_ByRefMut_FilterMode_TextureWrapMode__cordl_bool_i32_f32_Il2CppString0(
        handle: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        >,
        descriptor: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RenderTextureDescriptor>,
        filterMode: crate::UnityEngine::FilterMode,
        wrapMode: crate::UnityEngine::TextureWrapMode,
        isShadowMap: bool,
        anisoLevel: i32,
        mipMapBias: f32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::RenderTextureDescriptor,
                        >,
                        crate::UnityEngine::FilterMode,
                        crate::UnityEngine::TextureWrapMode,
                        bool,
                        i32,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), bool, 8usize>("ReAllocateIfNeeded")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReAllocateIfNeeded",
                            8usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    handle,
                    descriptor,
                    filterMode,
                    wrapMode,
                    isShadowMap,
                    anisoLevel,
                    mipMapBias,
                    name,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReAllocateIfNeeded_ScaleFunc_ByRefMut_FilterMode_TextureWrapMode__cordl_bool_i32_f32_Il2CppString2(
        handle: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        >,
        scaleFunc: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ScaleFunc>,
        descriptor: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RenderTextureDescriptor>,
        filterMode: crate::UnityEngine::FilterMode,
        wrapMode: crate::UnityEngine::TextureWrapMode,
        isShadowMap: bool,
        anisoLevel: i32,
        mipMapBias: f32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ScaleFunc>,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::RenderTextureDescriptor,
                        >,
                        crate::UnityEngine::FilterMode,
                        crate::UnityEngine::TextureWrapMode,
                        bool,
                        i32,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), bool, 9usize>("ReAllocateIfNeeded")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReAllocateIfNeeded",
                            9usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    handle,
                    scaleFunc,
                    descriptor,
                    filterMode,
                    wrapMode,
                    isShadowMap,
                    anisoLevel,
                    mipMapBias,
                    name,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReAllocateIfNeeded_Vector2_ByRefMut_FilterMode_TextureWrapMode__cordl_bool_i32_f32_Il2CppString1(
        handle: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
        >,
        scaleFactor: crate::UnityEngine::Vector2,
        descriptor: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RenderTextureDescriptor>,
        filterMode: crate::UnityEngine::FilterMode,
        wrapMode: crate::UnityEngine::TextureWrapMode,
        isShadowMap: bool,
        anisoLevel: i32,
        mipMapBias: f32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                        >,
                        crate::UnityEngine::Vector2,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::RenderTextureDescriptor,
                        >,
                        crate::UnityEngine::FilterMode,
                        crate::UnityEngine::TextureWrapMode,
                        bool,
                        i32,
                        f32,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    ), bool, 9usize>("ReAllocateIfNeeded")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ReAllocateIfNeeded",
                            9usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(
                (),
                (
                    handle,
                    scaleFactor,
                    descriptor,
                    filterMode,
                    wrapMode,
                    isShadowMap,
                    anisoLevel,
                    mipMapBias,
                    name,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SequenceEqual(
        left: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
            >,
        >,
        right: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                            >,
                        >,
                    ), bool, 2usize>("SequenceEqual")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SequenceEqual",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (left, right))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetMaxRTHandlePoolCapacity(capacity: i32) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(i32), bool, 1usize>("SetMaxRTHandlePoolCapacity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetMaxRTHandlePoolCapacity",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (capacity))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetScaleBiasRt_RTHandle0(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RasterCommandBuffer>,
        cameraData: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Rendering::Universal::UniversalCameraData,
            >,
        >,
        rTHandle: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RasterCommandBuffer,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::Universal::UniversalCameraData,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RTHandle>,
                    ), quest_hook::libil2cpp::Void, 3usize>("SetScaleBiasRt")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetScaleBiasRt",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (cmd, cameraData, rTHandle))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetScaleBiasRt_RasterCommandBuffer_ByRefMut1(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RasterCommandBuffer>,
        renderingData: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::Rendering::Universal::RenderingData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RasterCommandBuffer,
                        >,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::UnityEngine::Rendering::Universal::RenderingData,
                        >,
                    ), quest_hook::libil2cpp::Void, 2usize>("SetScaleBiasRt")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetScaleBiasRt",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (cmd, renderingData))? };
        Ok(__cordl_ret.into())
    }
    pub fn SetViewAndProjectionMatrices_CommandBuffer0(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
        viewMatrix: crate::UnityEngine::Matrix4x4,
        projectionMatrix: crate::UnityEngine::Matrix4x4,
        setInverseMatrices: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::CommandBuffer>,
                        crate::UnityEngine::Matrix4x4,
                        crate::UnityEngine::Matrix4x4,
                        bool,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetViewAndProjectionMatrices"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetViewAndProjectionMatrices",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (cmd, viewMatrix, projectionMatrix, setInverseMatrices))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetViewAndProjectionMatrices_RasterCommandBuffer1(
        cmd: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::RasterCommandBuffer>,
        viewMatrix: crate::UnityEngine::Matrix4x4,
        projectionMatrix: crate::UnityEngine::Matrix4x4,
        setInverseMatrices: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::RasterCommandBuffer,
                        >,
                        crate::UnityEngine::Matrix4x4,
                        crate::UnityEngine::Matrix4x4,
                        bool,
                    ), quest_hook::libil2cpp::Void, 4usize>(
                        "SetViewAndProjectionMatrices"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SetViewAndProjectionMatrices",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked((), (cmd, viewMatrix, projectionMatrix, setInverseMatrices))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SupportsGraphicsFormat(
        format: crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
        usage: crate::UnityEngine::Experimental::Rendering::FormatUsage,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::UnityEngine::Experimental::Rendering::GraphicsFormat,
                        crate::UnityEngine::Experimental::Rendering::FormatUsage,
                    ), bool, 2usize>("SupportsGraphicsFormat")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SupportsGraphicsFormat",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (format, usage))? };
        Ok(__cordl_ret.into())
    }
    pub fn SupportsLightLayers(
        _cordl_type: crate::UnityEngine::Rendering::GraphicsDeviceType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::UnityEngine::Rendering::GraphicsDeviceType),
                        bool,
                        1usize,
                    >("SupportsLightLayers")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SupportsLightLayers", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn SupportsRenderTextureFormat(
        format: crate::UnityEngine::RenderTextureFormat,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(crate::UnityEngine::RenderTextureFormat), bool, 1usize>(
                        "SupportsRenderTextureFormat",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SupportsRenderTextureFormat",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (format))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_emptyAttachment(
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Rendering::AttachmentDescriptor> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::UnityEngine::Rendering::AttachmentDescriptor,
                        0usize,
                    >("get_emptyAttachment")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_emptyAttachment", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::AttachmentDescriptor =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_errorMaterial(
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                        0usize,
                    >("get_errorMaterial")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_errorMaterial", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material> =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_fullscreenMesh(
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
                        0usize,
                    >("get_fullscreenMesh")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_fullscreenMesh", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh> =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_useStructuredBuffer() -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(), bool, 0usize>("get_useStructuredBuffer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_useStructuredBuffer",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+Rendering+Universal+RenderingUtils")]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::RenderingUtils
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
