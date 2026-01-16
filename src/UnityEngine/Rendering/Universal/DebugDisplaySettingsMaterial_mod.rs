#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial"
)]
#[repr(C)]
#[derive(Debug)]
pub struct DebugDisplaySettingsMaterial {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_AlbedoDebugValidationPresetData: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_AlbedoDebugValidationPresetData,
        >,
    >,
    pub m_AlbedoValidationPreset: crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_AlbedoDebugValidationPreset,
    pub _albedoMinLuminance_k__BackingField: f32,
    pub _albedoMaxLuminance_k__BackingField: f32,
    pub m_AlbedoHueTolerance: f32,
    pub m_AlbedoSaturationTolerance: f32,
    pub _albedoCompareColor_k__BackingField: crate::UnityEngine::Color,
    pub _metallicMinValue_k__BackingField: f32,
    pub _metallicMaxValue_k__BackingField: f32,
    pub _renderingLayersSelectedLight_k__BackingField: bool,
    pub _selectedLightShadowLayerMask_k__BackingField: bool,
    pub _renderingLayerMask_k__BackingField: u32,
    pub debugRenderingLayersColors: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
    >,
    pub _materialValidationMode_k__BackingField: crate::UnityEngine::Rendering::Universal::DebugMaterialValidationMode,
    pub _materialDebugMode_k__BackingField: crate::UnityEngine::Rendering::Universal::DebugMaterialMode,
    pub _vertexAttributeDebugMode_k__BackingField: crate::UnityEngine::Rendering::Universal::DebugVertexAttributeMode,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "DebugDisplaySettingsMaterial";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial")]
impl std::ops::Deref
for crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial")]
impl std::ops::DerefMut
for crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial")]
impl crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial {
    #[cfg(
        feature = "UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial+AlbedoDebugValidationPreset"
    )]
    pub type AlbedoDebugValidationPreset = crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_AlbedoDebugValidationPreset;
    #[cfg(
        feature = "UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial+AlbedoDebugValidationPresetData"
    )]
    pub type AlbedoDebugValidationPresetData = crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_AlbedoDebugValidationPresetData;
    #[cfg(
        feature = "UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial+SettingsPanel"
    )]
    pub type SettingsPanel = crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_SettingsPanel;
    #[cfg(
        feature = "UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial+Strings"
    )]
    pub type Strings = crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_Strings;
    #[cfg(
        feature = "UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial+WidgetFactory"
    )]
    pub type WidgetFactory = crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_WidgetFactory;
    pub fn GetDebugLightLayersMask(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), u32, 0usize>("GetDebugLightLayersMask")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetDebugLightLayersMask", 0usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UnityEngine_Rendering_IDebugDisplaySettingsData_CreatePanel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::IDebugDisplaySettingsPanelDisposable,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::IDebugDisplaySettingsPanelDisposable,
                        >,
                        0usize,
                    >("UnityEngine.Rendering.IDebugDisplaySettingsData.CreatePanel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnityEngine.Rendering.IDebugDisplaySettingsData.CreatePanel",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::IDebugDisplaySettingsPanelDisposable,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
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
    pub fn get_AreAnySettingsActive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_AreAnySettingsActive")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_AreAnySettingsActive", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsLightingActive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_IsLightingActive")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_IsLightingActive", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsPostProcessingAllowed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_IsPostProcessingAllowed")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_IsPostProcessingAllowed", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_albedoCompareColor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::Color,
                        0usize,
                    >("get_albedoCompareColor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_albedoCompareColor", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Color = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_albedoHueTolerance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), f32, 0usize>("get_albedoHueTolerance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_albedoHueTolerance", 0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_albedoMaxLuminance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), f32, 0usize>("get_albedoMaxLuminance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_albedoMaxLuminance", 0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_albedoMinLuminance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), f32, 0usize>("get_albedoMinLuminance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_albedoMinLuminance", 0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_albedoSaturationTolerance(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), f32, 0usize>("get_albedoSaturationTolerance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_albedoSaturationTolerance", 0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_albedoValidationPreset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_AlbedoDebugValidationPreset,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_AlbedoDebugValidationPreset,
                        0usize,
                    >("get_albedoValidationPreset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_albedoValidationPreset", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_AlbedoDebugValidationPreset = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_materialDebugMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::Universal::DebugMaterialMode,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::Rendering::Universal::DebugMaterialMode,
                        0usize,
                    >("get_materialDebugMode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_materialDebugMode", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::Universal::DebugMaterialMode = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_materialValidationMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::Universal::DebugMaterialValidationMode,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::Rendering::Universal::DebugMaterialValidationMode,
                        0usize,
                    >("get_materialValidationMode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_materialValidationMode", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::Universal::DebugMaterialValidationMode = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_metallicMaxValue(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), f32, 0usize>("get_metallicMaxValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_metallicMaxValue", 0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_metallicMinValue(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), f32, 0usize>("get_metallicMinValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_metallicMinValue", 0usize
                        )
                    })
            });
        let __cordl_ret: f32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_renderingLayerMask(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), u32, 0usize>("get_renderingLayerMask")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_renderingLayerMask", 0usize
                        )
                    })
            });
        let __cordl_ret: u32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_renderingLayersSelectedLight(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_renderingLayersSelectedLight")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_renderingLayersSelectedLight", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_selectedLightShadowLayerMask(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_selectedLightShadowLayerMask")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_selectedLightShadowLayerMask", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_vertexAttributeDebugMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::Universal::DebugVertexAttributeMode,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::UnityEngine::Rendering::Universal::DebugVertexAttributeMode,
                        0usize,
                    >("get_vertexAttributeDebugMode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_vertexAttributeDebugMode", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::UnityEngine::Rendering::Universal::DebugVertexAttributeMode = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_albedoCompareColor(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Color),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_albedoCompareColor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_albedoCompareColor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_albedoHueTolerance(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (f32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_albedoHueTolerance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_albedoHueTolerance", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_albedoMaxLuminance(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (f32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_albedoMaxLuminance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_albedoMaxLuminance", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_albedoMinLuminance(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (f32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_albedoMinLuminance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_albedoMinLuminance", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_albedoSaturationTolerance(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (f32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_albedoSaturationTolerance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_albedoSaturationTolerance", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_albedoValidationPreset(
        &mut self,
        value: crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_AlbedoDebugValidationPreset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_AlbedoDebugValidationPreset),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_albedoValidationPreset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_albedoValidationPreset", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_materialDebugMode(
        &mut self,
        value: crate::UnityEngine::Rendering::Universal::DebugMaterialMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::Universal::DebugMaterialMode),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_materialDebugMode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_materialDebugMode", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_materialValidationMode(
        &mut self,
        value: crate::UnityEngine::Rendering::Universal::DebugMaterialValidationMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::Universal::DebugMaterialValidationMode),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_materialValidationMode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_materialValidationMode", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_metallicMaxValue(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (f32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_metallicMaxValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_metallicMaxValue", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_metallicMinValue(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (f32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_metallicMinValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_metallicMinValue", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_renderingLayerMask(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (u32),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_renderingLayerMask")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_renderingLayerMask", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_renderingLayersSelectedLight(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_renderingLayersSelectedLight")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_renderingLayersSelectedLight", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_selectedLightShadowLayerMask(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_selectedLightShadowLayerMask")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_selectedLightShadowLayerMask", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_vertexAttributeDebugMode(
        &mut self,
        value: crate::UnityEngine::Rendering::Universal::DebugVertexAttributeMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::Rendering::Universal::DebugVertexAttributeMode),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_vertexAttributeDebugMode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_vertexAttributeDebugMode", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial")]
impl AsRef<crate::UnityEngine::Rendering::IDebugDisplaySettingsData>
for crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial {
    fn as_ref(&self) -> &crate::UnityEngine::Rendering::IDebugDisplaySettingsData {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial")]
impl AsMut<crate::UnityEngine::Rendering::IDebugDisplaySettingsData>
for crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::Rendering::IDebugDisplaySettingsData {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial")]
impl AsRef<crate::UnityEngine::Rendering::IDebugDisplaySettingsQuery>
for crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial {
    fn as_ref(&self) -> &crate::UnityEngine::Rendering::IDebugDisplaySettingsQuery {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial")]
impl AsMut<crate::UnityEngine::Rendering::IDebugDisplaySettingsQuery>
for crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::Rendering::IDebugDisplaySettingsQuery {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial+AlbedoDebugValidationPreset"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DebugDisplaySettingsMaterial_AlbedoDebugValidationPreset {
    #[default]
    BlackAcrylicPaint = 1i32,
    BlueSky = 12i32,
    Custom = 14i32,
    DarkSoil = 2i32,
    DefaultLuminance = 0i32,
    DryClaySoil = 4i32,
    DrySand = 8i32,
    Foliage = 13i32,
    FreshSnow = 11i32,
    GreenGrass = 5i32,
    NewConcrete = 9i32,
    OldConcrete = 6i32,
    RedClayTile = 7i32,
    WhiteAcrylicPaint = 10i32,
    WornAsphalt = 3i32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial+AlbedoDebugValidationPreset"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_AlbedoDebugValidationPreset {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "DebugDisplaySettingsMaterial/AlbedoDebugValidationPreset";
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial+AlbedoDebugValidationPreset"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_AlbedoDebugValidationPreset {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial+AlbedoDebugValidationPreset"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_AlbedoDebugValidationPreset {
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial+AlbedoDebugValidationPreset"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_AlbedoDebugValidationPreset {
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial+AlbedoDebugValidationPreset"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_AlbedoDebugValidationPreset {
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial+AlbedoDebugValidationPresetData"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DebugDisplaySettingsMaterial_AlbedoDebugValidationPresetData {
    pub name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub color: crate::UnityEngine::Color,
    pub minLuminance: f32,
    pub maxLuminance: f32,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial+AlbedoDebugValidationPresetData"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_AlbedoDebugValidationPresetData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "DebugDisplaySettingsMaterial/AlbedoDebugValidationPresetData";
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial+AlbedoDebugValidationPresetData"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_AlbedoDebugValidationPresetData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial+AlbedoDebugValidationPresetData"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_AlbedoDebugValidationPresetData {
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial+AlbedoDebugValidationPresetData"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_AlbedoDebugValidationPresetData {
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial+AlbedoDebugValidationPresetData"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_AlbedoDebugValidationPresetData {
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
    feature = "cordl_class_UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial+AlbedoDebugValidationPresetData"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_AlbedoDebugValidationPresetData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial+AlbedoDebugValidationPresetData"
)]
impl crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_AlbedoDebugValidationPresetData {}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial+SettingsPanel"
)]
#[repr(C)]
#[derive(Debug)]
pub struct DebugDisplaySettingsMaterial_SettingsPanel {
    __cordl_parent: crate::UnityEngine::Rendering::DebugDisplaySettingsPanel_1<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial,
        >,
    >,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial+SettingsPanel"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_SettingsPanel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "DebugDisplaySettingsMaterial/SettingsPanel";
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
    feature = "UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial+SettingsPanel"
)]
impl std::ops::Deref
for crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_SettingsPanel {
    type Target = crate::UnityEngine::Rendering::DebugDisplaySettingsPanel_1<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial,
        >,
    >;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial+SettingsPanel"
)]
impl std::ops::DerefMut
for crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_SettingsPanel {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial+SettingsPanel"
)]
impl crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_SettingsPanel {
    pub fn New(
        data: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (data))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        data: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (data))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial+SettingsPanel"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_SettingsPanel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial+Strings"
)]
#[repr(C)]
#[derive(Debug)]
pub struct DebugDisplaySettingsMaterial_Strings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial+Strings"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_Strings {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "DebugDisplaySettingsMaterial/Strings";
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
#[cfg(feature = "UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial+Strings")]
impl std::ops::Deref
for crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_Strings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial+Strings")]
impl std::ops::DerefMut
for crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_Strings {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial+Strings")]
impl crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_Strings {
    pub const AlbedoSettingsContainerName: &'static str = "Albedo Settings";
    pub const MetallicSettingsContainerName: &'static str = "Metallic Settings";
    pub const RenderingLayerMasksSettingsContainerName: &'static str = "Rendering Layer Masks Settings";
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial+Strings"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_Strings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial+WidgetFactory"
)]
#[repr(C)]
#[derive(Debug)]
pub struct DebugDisplaySettingsMaterial_WidgetFactory {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial+WidgetFactory"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_WidgetFactory {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering.Universal";
    const CLASS_NAME: &'static str = "DebugDisplaySettingsMaterial/WidgetFactory";
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
    feature = "UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial+WidgetFactory"
)]
impl std::ops::Deref
for crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_WidgetFactory {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial+WidgetFactory"
)]
impl std::ops::DerefMut
for crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_WidgetFactory {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial+WidgetFactory"
)]
impl crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_WidgetFactory {
    pub fn CreateAlbedoCustomColor(
        panel: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_SettingsPanel,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugUI_Widget>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_SettingsPanel,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::DebugUI_Widget,
                        >,
                        1usize,
                    >("CreateAlbedoCustomColor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateAlbedoCustomColor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::DebugUI_Widget,
        > = unsafe { cordl_method_info.invoke_unchecked((), (panel))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateAlbedoHueTolerance(
        panel: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_SettingsPanel,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugUI_Widget>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_SettingsPanel,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::DebugUI_Widget,
                        >,
                        1usize,
                    >("CreateAlbedoHueTolerance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateAlbedoHueTolerance", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::DebugUI_Widget,
        > = unsafe { cordl_method_info.invoke_unchecked((), (panel))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateAlbedoMaxLuminance(
        panel: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_SettingsPanel,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugUI_Widget>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_SettingsPanel,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::DebugUI_Widget,
                        >,
                        1usize,
                    >("CreateAlbedoMaxLuminance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateAlbedoMaxLuminance", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::DebugUI_Widget,
        > = unsafe { cordl_method_info.invoke_unchecked((), (panel))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateAlbedoMinLuminance(
        panel: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_SettingsPanel,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugUI_Widget>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_SettingsPanel,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::DebugUI_Widget,
                        >,
                        1usize,
                    >("CreateAlbedoMinLuminance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateAlbedoMinLuminance", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::DebugUI_Widget,
        > = unsafe { cordl_method_info.invoke_unchecked((), (panel))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateAlbedoPreset(
        panel: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_SettingsPanel,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugUI_Widget>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_SettingsPanel,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::DebugUI_Widget,
                        >,
                        1usize,
                    >("CreateAlbedoPreset")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateAlbedoPreset", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::DebugUI_Widget,
        > = unsafe { cordl_method_info.invoke_unchecked((), (panel))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateAlbedoSaturationTolerance(
        panel: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_SettingsPanel,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugUI_Widget>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_SettingsPanel,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::DebugUI_Widget,
                        >,
                        1usize,
                    >("CreateAlbedoSaturationTolerance")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateAlbedoSaturationTolerance", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::DebugUI_Widget,
        > = unsafe { cordl_method_info.invoke_unchecked((), (panel))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateFilterRenderingLayerMasks(
        panel: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_SettingsPanel,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugUI_Widget>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_SettingsPanel,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::DebugUI_Widget,
                        >,
                        1usize,
                    >("CreateFilterRenderingLayerMasks")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateFilterRenderingLayerMasks", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::DebugUI_Widget,
        > = unsafe { cordl_method_info.invoke_unchecked((), (panel))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateMaterialOverride(
        panel: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_SettingsPanel,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugUI_Widget>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_SettingsPanel,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::DebugUI_Widget,
                        >,
                        1usize,
                    >("CreateMaterialOverride")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateMaterialOverride", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::DebugUI_Widget,
        > = unsafe { cordl_method_info.invoke_unchecked((), (panel))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateMaterialValidationMode(
        panel: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_SettingsPanel,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugUI_Widget>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_SettingsPanel,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::DebugUI_Widget,
                        >,
                        1usize,
                    >("CreateMaterialValidationMode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateMaterialValidationMode", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::DebugUI_Widget,
        > = unsafe { cordl_method_info.invoke_unchecked((), (panel))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateMetallicMaxValue(
        panel: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_SettingsPanel,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugUI_Widget>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_SettingsPanel,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::DebugUI_Widget,
                        >,
                        1usize,
                    >("CreateMetallicMaxValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateMetallicMaxValue", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::DebugUI_Widget,
        > = unsafe { cordl_method_info.invoke_unchecked((), (panel))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateMetallicMinValue(
        panel: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_SettingsPanel,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugUI_Widget>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_SettingsPanel,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::DebugUI_Widget,
                        >,
                        1usize,
                    >("CreateMetallicMinValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateMetallicMinValue", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::DebugUI_Widget,
        > = unsafe { cordl_method_info.invoke_unchecked((), (panel))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateRenderingLayersSelectedLight(
        panel: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_SettingsPanel,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugUI_Widget>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_SettingsPanel,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::DebugUI_Widget,
                        >,
                        1usize,
                    >("CreateRenderingLayersSelectedLight")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateRenderingLayersSelectedLight", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::DebugUI_Widget,
        > = unsafe { cordl_method_info.invoke_unchecked((), (panel))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateSelectedLightShadowLayerMask(
        panel: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_SettingsPanel,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugUI_Widget>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_SettingsPanel,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::DebugUI_Widget,
                        >,
                        1usize,
                    >("CreateSelectedLightShadowLayerMask")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateSelectedLightShadowLayerMask", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::DebugUI_Widget,
        > = unsafe { cordl_method_info.invoke_unchecked((), (panel))? };
        Ok(__cordl_ret.into())
    }
    pub fn CreateVertexAttribute(
        panel: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_SettingsPanel,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Rendering::DebugUI_Widget>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_SettingsPanel,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::UnityEngine::Rendering::DebugUI_Widget,
                        >,
                        1usize,
                    >("CreateVertexAttribute")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateVertexAttribute", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::DebugUI_Widget,
        > = unsafe { cordl_method_info.invoke_unchecked((), (panel))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "cordl_class_UnityEngine+Rendering+Universal+DebugDisplaySettingsMaterial+WidgetFactory"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::Universal::DebugDisplaySettingsMaterial_WidgetFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
