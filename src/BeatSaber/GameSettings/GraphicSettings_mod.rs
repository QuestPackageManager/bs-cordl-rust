#[cfg(feature = "BeatSaber+GameSettings+GraphicSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct GraphicSettings {
    __cordl_parent: crate::BGLib::SaveDataCore::VersionableSaveData,
    pub _performancePresetKey: *mut crate::System::String,
    pub _customPreset: *mut crate::BeatSaber::PerformancePresets::CustomPerformancePreset,
    pub _windowMode: crate::BeatSaber::GameSettings::WindowMode,
    pub _windowResolution: crate::UnityEngine::Vector2Int,
}
#[cfg(feature = "BeatSaber+GameSettings+GraphicSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::GameSettings::GraphicSettings =>
    "BeatSaber.GameSettings"."GraphicSettings"
);
#[cfg(feature = "BeatSaber+GameSettings+GraphicSettings")]
impl std::ops::Deref for crate::BeatSaber::GameSettings::GraphicSettings {
    type Target = crate::BGLib::SaveDataCore::VersionableSaveData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+GameSettings+GraphicSettings")]
impl std::ops::DerefMut for crate::BeatSaber::GameSettings::GraphicSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+GameSettings+GraphicSettings")]
impl crate::BeatSaber::GameSettings::GraphicSettings {
    pub fn set_isDirty(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_isDirty", (value))?;
        Ok(__cordl_ret)
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
    pub fn get_isDirty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isDirty", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_performancePresetKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_performancePresetKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_windowMode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BeatSaber::GameSettings::WindowMode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BeatSaber::GameSettings::WindowMode = __cordl_object
            .invoke("get_windowMode", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_windowMode(
        &mut self,
        value: crate::BeatSaber::GameSettings::WindowMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_windowMode", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_windowResolution(
        &mut self,
        value: crate::UnityEngine::Vector2Int,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_windowResolution", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_performancePresetKey(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_performancePresetKey", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_customPreset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::BeatSaber::PerformancePresets::CustomPerformancePreset,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::BeatSaber::PerformancePresets::CustomPerformancePreset = __cordl_object
            .invoke("get_customPreset", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_customPreset(
        &mut self,
        value: *mut crate::BeatSaber::PerformancePresets::CustomPerformancePreset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_customPreset", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_windowResolution(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2Int> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2Int = __cordl_object
            .invoke("get_windowResolution", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "BeatSaber+GameSettings+GraphicSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::GameSettings::GraphicSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
