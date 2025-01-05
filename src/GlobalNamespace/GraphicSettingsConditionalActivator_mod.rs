#[cfg(feature = "GraphicSettingsConditionalActivator")]
#[repr(C)]
#[derive(Debug)]
pub struct GraphicSettingsConditionalActivator {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
    pub _activatorType: crate::GlobalNamespace::GraphicSettingsConditionalActivator_ActivatorType,
    pub _activateOnFalse: bool,
    pub _settingsManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::SettingsManager,
    >,
    pub _sceneSetupData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::GameplayCoreSceneSetupData,
    >,
}
#[cfg(feature = "GraphicSettingsConditionalActivator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GraphicSettingsConditionalActivator => ""
    ."GraphicSettingsConditionalActivator"
);
#[cfg(feature = "GraphicSettingsConditionalActivator")]
impl std::ops::Deref for crate::GlobalNamespace::GraphicSettingsConditionalActivator {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GraphicSettingsConditionalActivator")]
impl std::ops::DerefMut for crate::GlobalNamespace::GraphicSettingsConditionalActivator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GraphicSettingsConditionalActivator")]
impl crate::GlobalNamespace::GraphicSettingsConditionalActivator {
    #[cfg(feature = "GraphicSettingsConditionalActivator+ActivatorType")]
    pub type ActivatorType = crate::GlobalNamespace::GraphicSettingsConditionalActivator_ActivatorType;
    pub fn GetGraphicsActivatorType(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetGraphicsActivatorType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
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
}
#[cfg(feature = "GraphicSettingsConditionalActivator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GraphicSettingsConditionalActivator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "GraphicSettingsConditionalActivator+ActivatorType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GraphicSettingsConditionalActivator_ActivatorType {
    #[default]
    burnmarkTrails = 2i32,
    depthTexture = 1i32,
    screenDisplacement = 3i32,
    smoke = 0i32,
}
#[cfg(feature = "GraphicSettingsConditionalActivator+ActivatorType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GraphicSettingsConditionalActivator_ActivatorType => ""
    ."GraphicSettingsConditionalActivator/ActivatorType"
);
