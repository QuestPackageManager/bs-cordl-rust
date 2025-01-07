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
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FallbackLightmapperByRef", (lightmapperPtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn FallbackMixedLightingModeByRef(
        fallbackModePtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FallbackMixedLightingModeByRef", (fallbackModePtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsAutoAmbientProbeBakingSupported(
        isSupportedPtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsAutoAmbientProbeBakingSupported", (isSupportedPtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsAutoDefaultReflectionProbeBakingSupported(
        isSupportedPtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsAutoDefaultReflectionProbeBakingSupported", (isSupportedPtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsLightmapBakeTypeSupported(
        bakeType: crate::UnityEngine::LightmapBakeType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsLightmapBakeTypeSupported", (bakeType))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsLightmapBakeTypeSupportedByRef(
        bakeType: crate::UnityEngine::LightmapBakeType,
        isSupportedPtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsLightmapBakeTypeSupportedByRef", (bakeType, isSupportedPtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsLightmapperSupportedByRef(
        lightmapper: i32,
        isSupportedPtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsLightmapperSupportedByRef", (lightmapper, isSupportedPtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsLightmapsModeSupportedByRef(
        mode: crate::UnityEngine::LightmapsMode,
        isSupportedPtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsLightmapsModeSupportedByRef", (mode, isSupportedPtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsMixedLightingModeSupported(
        mixedMode: crate::UnityEngine::MixedLightingMode,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsMixedLightingModeSupported", (mixedMode))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsMixedLightingModeSupportedByRef(
        mixedMode: crate::UnityEngine::MixedLightingMode,
        isSupportedPtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsMixedLightingModeSupportedByRef", (mixedMode, isSupportedPtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsUIOverlayRenderedBySRP(
        isSupportedPtr: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsUIOverlayRenderedBySRP", (isSupportedPtr))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OverridesLightProbeSystem", (overridesPtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_active() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::SupportedRenderingFeatures,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::SupportedRenderingFeatures,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_active", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_autoAmbientProbeBaking(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_autoAmbientProbeBaking", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_autoDefaultReflectionProbeBaking(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_autoDefaultReflectionProbeBaking", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_defaultMixedLightingModes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::SupportedRenderingFeatures_LightmapMixedBakeModes,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rendering::SupportedRenderingFeatures_LightmapMixedBakeModes = __cordl_object
            .invoke("get_defaultMixedLightingModes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_enlighten(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enlighten", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_enlightenLightmapper(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enlightenLightmapper", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lightmapBakeTypes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::LightmapBakeType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::LightmapBakeType = __cordl_object
            .invoke("get_lightmapBakeTypes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lightmapsModes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::LightmapsMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::LightmapsMode = __cordl_object
            .invoke("get_lightmapsModes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_mixedLightingModes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Rendering::SupportedRenderingFeatures_LightmapMixedBakeModes,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Rendering::SupportedRenderingFeatures_LightmapMixedBakeModes = __cordl_object
            .invoke("get_mixedLightingModes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_overridesLightProbeSystem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_overridesLightProbeSystem", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rendersUIOverlay(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_rendersUIOverlay", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_active(
        value: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Rendering::SupportedRenderingFeatures,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_active", (value))?;
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
    const CLASS_NAME: &'static str = "LightmapMixedBakeModes";
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
    const CLASS_NAME: &'static str = "ReflectionProbeModes";
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
