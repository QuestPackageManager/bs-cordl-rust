#[cfg(feature = "RenderingParamsSetupApplicator")]
#[repr(C)]
#[derive(Debug)]
pub struct RenderingParamsSetupApplicator {
    __cordl_parent: crate::System::Object,
    pub _mainSettingsHandler: *mut crate::BeatSaber::GameSettings::MainSettingsHandler,
    pub _graphicsSettingsHandler: *mut crate::BeatSaber::GameSettings::GraphicSettingsHandler,
    pub _settingsApplicator: *mut crate::GlobalNamespace::SettingsApplicatorSO,
}
#[cfg(feature = "RenderingParamsSetupApplicator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::RenderingParamsSetupApplicator
    => ""."RenderingParamsSetupApplicator"
);
#[cfg(feature = "RenderingParamsSetupApplicator")]
impl std::ops::Deref for crate::GlobalNamespace::RenderingParamsSetupApplicator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "RenderingParamsSetupApplicator")]
impl std::ops::DerefMut for crate::GlobalNamespace::RenderingParamsSetupApplicator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "RenderingParamsSetupApplicator")]
impl crate::GlobalNamespace::RenderingParamsSetupApplicator {
    pub fn Apply(
        &mut self,
        sceneType: crate::GlobalNamespace::SceneType,
        optionalEnvironmentSerializedName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Apply", (sceneType, optionalEnvironmentSerializedName))?;
        Ok(__cordl_ret)
    }
    pub fn ApplyGraphicsSettings(
        &mut self,
        sceneType: crate::GlobalNamespace::SceneType,
        optionalEnvironmentSerializedName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ApplyGraphicsSettings",
                (sceneType, optionalEnvironmentSerializedName),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ApplyMainSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyMainSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "RenderingParamsSetupApplicator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::RenderingParamsSetupApplicator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
