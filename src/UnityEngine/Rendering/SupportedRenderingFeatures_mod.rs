#[cfg(feature = "UnityEngine+Rendering+SupportedRenderingFeatures")]
#[repr(C)]
#[derive(Debug)]
pub struct SupportedRenderingFeatures {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _reflectionProbeModes_k__BackingField: crate::UnityEngine::Rendering::SupportedRenderingFeatures_ReflectionProbeModes,
    pub _defaultMixedLightingModes_k__BackingField: crate::UnityEngine::Rendering::SupportedRenderingFeatures_LightmapMixedBakeModes,
    pub _mixedLightingModes_k__BackingField: crate::UnityEngine::Rendering::SupportedRenderingFeatures_LightmapMixedBakeModes,
    pub _lightmapBakeTypes_k__BackingField: crate::UnityEngine::LightmapBakeType,
    pub _lightmapsModes_k__BackingField: crate::UnityEngine::LightmapsMode,
    pub _enlightenLightmapper_k__BackingField: bool,
    pub _enlighten_k__BackingField: bool,
    pub _lightProbeProxyVolumes_k__BackingField: bool,
    pub _motionVectors_k__BackingField: bool,
    pub _receiveShadows_k__BackingField: bool,
    pub _reflectionProbes_k__BackingField: bool,
    pub _reflectionProbesBlendDistance_k__BackingField: bool,
    pub _rendererPriority_k__BackingField: bool,
    pub _rendersUIOverlay_k__BackingField: bool,
    pub _overridesEnvironmentLighting_k__BackingField: bool,
    pub _overridesFog_k__BackingField: bool,
    pub _overridesRealtimeReflectionProbes_k__BackingField: bool,
    pub _overridesOtherLightingSettings_k__BackingField: bool,
    pub _editableMaterialRenderQueue_k__BackingField: bool,
    pub _overridesLODBias_k__BackingField: bool,
    pub _overridesMaximumLODLevel_k__BackingField: bool,
    pub _overridesEnableLODCrossFade_k__BackingField: bool,
    pub _rendererProbes_k__BackingField: bool,
    pub _particleSystemInstancing_k__BackingField: bool,
    pub _autoAmbientProbeBaking_k__BackingField: bool,
    pub _autoDefaultReflectionProbeBaking_k__BackingField: bool,
    pub _overridesShadowmask_k__BackingField: bool,
    pub _overridesLightProbeSystem_k__BackingField: bool,
    pub _supportsHDR_k__BackingField: bool,
    pub _overridesLightProbeSystemWarningMessage_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "UnityEngine+Rendering+SupportedRenderingFeatures")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::SupportedRenderingFeatures {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "SupportedRenderingFeatures";
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
#[cfg(feature = "UnityEngine+Rendering+SupportedRenderingFeatures")]
impl std::ops::Deref for crate::UnityEngine::Rendering::SupportedRenderingFeatures {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+SupportedRenderingFeatures")]
impl std::ops::DerefMut for crate::UnityEngine::Rendering::SupportedRenderingFeatures {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rendering+SupportedRenderingFeatures")]
impl crate::UnityEngine::Rendering::SupportedRenderingFeatures {
    #[cfg(
        feature = "UnityEngine+Rendering+SupportedRenderingFeatures+LightmapMixedBakeModes"
    )]
    pub type LightmapMixedBakeModes = crate::UnityEngine::Rendering::SupportedRenderingFeatures_LightmapMixedBakeModes;
    #[cfg(
        feature = "UnityEngine+Rendering+SupportedRenderingFeatures+ReflectionProbeModes"
    )]
    pub type ReflectionProbeModes = crate::UnityEngine::Rendering::SupportedRenderingFeatures_ReflectionProbeModes;
    pub fn FallbackLightmapperByRef(
        lightmapperPtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("FallbackLightmapperByRef")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FallbackLightmapperByRef", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (lightmapperPtr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn FallbackMixedLightingModeByRef(
        fallbackModePtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("FallbackMixedLightingModeByRef")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FallbackMixedLightingModeByRef", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (fallbackModePtr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsAutoAmbientProbeBakingSupported(
        isSupportedPtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("IsAutoAmbientProbeBakingSupported")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsAutoAmbientProbeBakingSupported", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (isSupportedPtr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsAutoDefaultReflectionProbeBakingSupported(
        isSupportedPtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("IsAutoDefaultReflectionProbeBakingSupported")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsAutoDefaultReflectionProbeBakingSupported", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (isSupportedPtr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsLightmapBakeTypeSupported(
        bakeType: crate::UnityEngine::LightmapBakeType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::LightmapBakeType),
                bool,
                1usize,
            >("IsLightmapBakeTypeSupported")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsLightmapBakeTypeSupported", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (bakeType)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsLightmapBakeTypeSupportedByRef(
        bakeType: crate::UnityEngine::LightmapBakeType,
        isSupportedPtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::LightmapBakeType, crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                2usize,
            >("IsLightmapBakeTypeSupportedByRef")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsLightmapBakeTypeSupportedByRef", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (bakeType, isSupportedPtr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsLightmapperSupportedByRef(
        lightmapper: i32,
        isSupportedPtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                2usize,
            >("IsLightmapperSupportedByRef")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsLightmapperSupportedByRef", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (lightmapper, isSupportedPtr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsLightmapsModeSupportedByRef(
        mode: crate::UnityEngine::LightmapsMode,
        isSupportedPtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::LightmapsMode, crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                2usize,
            >("IsLightmapsModeSupportedByRef")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsLightmapsModeSupportedByRef", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (mode, isSupportedPtr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsMixedLightingModeSupported(
        mixedMode: crate::UnityEngine::MixedLightingMode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::MixedLightingMode),
                bool,
                1usize,
            >("IsMixedLightingModeSupported")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsMixedLightingModeSupported", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (mixedMode)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsMixedLightingModeSupportedByRef(
        mixedMode: crate::UnityEngine::MixedLightingMode,
        isSupportedPtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::MixedLightingMode, crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                2usize,
            >("IsMixedLightingModeSupportedByRef")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsMixedLightingModeSupportedByRef", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (mixedMode, isSupportedPtr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsUIOverlayRenderedBySRP(
        isSupportedPtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("IsUIOverlayRenderedBySRP")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsUIOverlayRenderedBySRP", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (isSupportedPtr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OverridesLightProbeSystem(
        overridesPtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OverridesLightProbeSystem")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OverridesLightProbeSystem", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (overridesPtr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_active() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::SupportedRenderingFeatures,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::Rendering::SupportedRenderingFeatures,
                >,
                0usize,
            >("get_active")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_active", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::SupportedRenderingFeatures,
        > = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_autoAmbientProbeBaking(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_autoAmbientProbeBaking")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_autoAmbientProbeBaking", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_autoDefaultReflectionProbeBaking(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_autoDefaultReflectionProbeBaking")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_autoDefaultReflectionProbeBaking", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultMixedLightingModes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::SupportedRenderingFeatures_LightmapMixedBakeModes,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::Rendering::SupportedRenderingFeatures_LightmapMixedBakeModes,
                0usize,
            >("get_defaultMixedLightingModes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_defaultMixedLightingModes", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Rendering::SupportedRenderingFeatures_LightmapMixedBakeModes = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_enlighten(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_enlighten")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_enlighten", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_enlightenLightmapper(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_enlightenLightmapper")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_enlightenLightmapper", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_lightmapBakeTypes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::LightmapBakeType> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::LightmapBakeType,
                0usize,
            >("get_lightmapBakeTypes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_lightmapBakeTypes", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::LightmapBakeType = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_lightmapsModes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::LightmapsMode> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::LightmapsMode,
                0usize,
            >("get_lightmapsModes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_lightmapsModes", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::LightmapsMode = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_mixedLightingModes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::SupportedRenderingFeatures_LightmapMixedBakeModes,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::Rendering::SupportedRenderingFeatures_LightmapMixedBakeModes,
                0usize,
            >("get_mixedLightingModes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_mixedLightingModes", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Rendering::SupportedRenderingFeatures_LightmapMixedBakeModes = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_overridesLightProbeSystem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_overridesLightProbeSystem")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_overridesLightProbeSystem", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_rendersUIOverlay(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_rendersUIOverlay")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_rendersUIOverlay", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn set_active(
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::SupportedRenderingFeatures,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::Rendering::SupportedRenderingFeatures,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_active")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_active", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (value))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Rendering+SupportedRenderingFeatures")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Rendering::SupportedRenderingFeatures {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+SupportedRenderingFeatures+LightmapMixedBakeModes"
)]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SupportedRenderingFeatures_LightmapMixedBakeModes {
    #[default]
    IndirectOnly = 1i32,
    None = 0i32,
    Shadowmask = 4i32,
    Subtractive = 2i32,
}
#[cfg(
    feature = "UnityEngine+Rendering+SupportedRenderingFeatures+LightmapMixedBakeModes"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::SupportedRenderingFeatures_LightmapMixedBakeModes {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "SupportedRenderingFeatures/LightmapMixedBakeModes";
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
    feature = "UnityEngine+Rendering+SupportedRenderingFeatures+LightmapMixedBakeModes"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::SupportedRenderingFeatures_LightmapMixedBakeModes {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+SupportedRenderingFeatures+LightmapMixedBakeModes"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::SupportedRenderingFeatures_LightmapMixedBakeModes {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(
    feature = "UnityEngine+Rendering+SupportedRenderingFeatures+LightmapMixedBakeModes"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::SupportedRenderingFeatures_LightmapMixedBakeModes {
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
    feature = "UnityEngine+Rendering+SupportedRenderingFeatures+LightmapMixedBakeModes"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::SupportedRenderingFeatures_LightmapMixedBakeModes {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+Rendering+SupportedRenderingFeatures+ReflectionProbeModes")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SupportedRenderingFeatures_ReflectionProbeModes {
    #[default]
    None = 0i32,
    Rotation = 1i32,
}
#[cfg(feature = "UnityEngine+Rendering+SupportedRenderingFeatures+ReflectionProbeModes")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::Rendering::SupportedRenderingFeatures_ReflectionProbeModes {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.Rendering";
    const CLASS_NAME: &'static str = "SupportedRenderingFeatures/ReflectionProbeModes";
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
#[cfg(feature = "UnityEngine+Rendering+SupportedRenderingFeatures+ReflectionProbeModes")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::Rendering::SupportedRenderingFeatures_ReflectionProbeModes {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+Rendering+SupportedRenderingFeatures+ReflectionProbeModes")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::Rendering::SupportedRenderingFeatures_ReflectionProbeModes {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "UnityEngine+Rendering+SupportedRenderingFeatures+ReflectionProbeModes")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::Rendering::SupportedRenderingFeatures_ReflectionProbeModes {
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
#[cfg(feature = "UnityEngine+Rendering+SupportedRenderingFeatures+ReflectionProbeModes")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::Rendering::SupportedRenderingFeatures_ReflectionProbeModes {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
