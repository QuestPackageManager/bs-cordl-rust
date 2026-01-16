#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+UniversalRenderPipelineGlobalSettings"
)]
#[repr(C)]
#[derive(Debug)]
pub struct UniversalRenderPipelineGlobalSettings {
    __cordl_parent: crate::UnityEngine::Rendering::RenderPipelineGlobalSettings_2<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalRenderPipelineGlobalSettings,
        >,
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalRenderPipeline,
        >,
    >,
    pub m_ShaderStrippingSetting:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ShaderStrippingSetting>,
    pub m_URPShaderStrippingSetting: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::Universal::URPShaderStrippingSetting,
    >,
    pub m_ShaderVariantLogLevel: crate::UnityEngine::Rendering::ShaderVariantLogLevel,
    pub m_ExportShaderVariants: bool,
    pub m_StripDebugVariants: bool,
    pub m_StripUnusedPostProcessingVariants: bool,
    pub m_StripUnusedVariants: bool,
    pub m_StripScreenCoordOverrideVariants: bool,
    pub supportRuntimeDebugDisplay: bool,
    pub m_EnableRenderGraph: bool,
    pub m_Settings: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Rendering::RenderPipelineGraphicsSettingsContainer,
    >,
    pub m_AssetVersion: i32,
    pub m_ObsoleteDefaultVolumeProfile:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::VolumeProfile>,
    pub m_RenderingLayerNames: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub m_ValidRenderingLayers: u32,
    pub lightLayerName0: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub lightLayerName1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub lightLayerName2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub lightLayerName3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub lightLayerName4: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub lightLayerName5: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub lightLayerName6: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub lightLayerName7: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub apvScenesData:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ProbeVolumeSceneData>,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+UniversalRenderPipelineGlobalSettings"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::UnityEngine::Rendering::Universal::UniversalRenderPipelineGlobalSettings
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "UniversalRenderPipelineGlobalSettings";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+UniversalRenderPipelineGlobalSettings")]
impl std::ops::Deref
    for crate::UnityEngine::Rendering::Universal::UniversalRenderPipelineGlobalSettings
{
    type Target = crate::UnityEngine::Rendering::RenderPipelineGlobalSettings_2<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalRenderPipelineGlobalSettings,
        >,
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::UniversalRenderPipeline,
        >,
    >;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+UniversalRenderPipelineGlobalSettings")]
impl std::ops::DerefMut
    for crate::UnityEngine::Rendering::Universal::UniversalRenderPipelineGlobalSettings
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+UniversalRenderPipelineGlobalSettings")]
impl crate::UnityEngine::Rendering::Universal::UniversalRenderPipelineGlobalSettings {
    pub const defaultAssetName: &'static str = "UniversalRenderPipelineGlobalSettings";
    pub const k_LastVersion: i32 = 8i32;
    pub fn GetOrCreateAPVSceneData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::ProbeVolumeSceneData>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::ProbeVolumeSceneData,
                        >,
                        0usize,
                    >("GetOrCreateAPVSceneData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetOrCreateAPVSceneData", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::ProbeVolumeSceneData,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetOrCreateDefaultVolumeProfile(
        defaultVolumeProfile: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::VolumeProfile,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::VolumeProfile>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::VolumeProfile,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::VolumeProfile,
                        >,
                        1usize,
                    >("GetOrCreateDefaultVolumeProfile")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetOrCreateDefaultVolumeProfile", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::VolumeProfile> =
            unsafe { cordl_method_info.invoke_unchecked((), (defaultVolumeProfile))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsAtLastVersion(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("IsAtLastVersion")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "IsAtLastVersion",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Reset(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Reset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Reset",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ResetRenderingLayerNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(
                        "ResetRenderingLayerNames",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "ResetRenderingLayerNames",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
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
    pub fn get_lightLayerNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        >,
                    >, 0usize>("get_lightLayerNames")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_lightLayerNames",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_prefixedLightLayerNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        quest_hook::libil2cpp::Il2CppArray<
                            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        >,
                    >, 0usize>("get_prefixedLightLayerNames")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_prefixedLightLayerNames",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_settingsList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::Rendering::IRenderPipelineGraphicsSettings,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::List_1<
                            quest_hook::libil2cpp::Gc<
                                crate::UnityEngine::Rendering::IRenderPipelineGraphicsSettings,
                            >,
                        >,
                    >, 0usize>("get_settingsList")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_settingsList",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::Rendering::IRenderPipelineGraphicsSettings,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_stripUnusedLODCrossFadeVariants(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_stripUnusedLODCrossFadeVariants")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_stripUnusedLODCrossFadeVariants",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_stripUnusedLODCrossFadeVariants(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(bool), quest_hook::libil2cpp::Void, 1usize>(
                        "set_stripUnusedLODCrossFadeVariants",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "set_stripUnusedLODCrossFadeVariants",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+UniversalRenderPipelineGlobalSettings"
)]
impl quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::Rendering::Universal::UniversalRenderPipelineGlobalSettings
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
