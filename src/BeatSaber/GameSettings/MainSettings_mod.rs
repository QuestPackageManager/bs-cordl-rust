#[cfg(feature = "BeatSaber+GameSettings+MainSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct MainSettings {
    __cordl_parent: crate::BGLib::SaveDataCore::VersionableSaveData,
    pub _audioSettings_k__BackingField: *mut crate::BeatSaber::GameSettings::Audio,
    pub _smoothCameraSettings_k__BackingField: *mut crate::BeatSaber::GameSettings::SmoothCamera,
    pub _controllerSettings_k__BackingField: *mut crate::BeatSaber::GameSettings::Controllers,
    pub _customServerSettings_k__BackingField: *mut crate::BeatSaber::GameSettings::CustomServer,
    pub _debugSettings_k__BackingField: *mut crate::BeatSaber::GameSettings::DebugSettings,
    pub _language: *mut crate::System::String,
    pub _pauseButtonPressDurationLevel: i32,
    pub _roomCenter: crate::UnityEngine::Vector3,
    pub _roomRotation: f32,
    pub _hapticFeedback: bool,
    pub _createScreenshotDuringTheGame: bool,
}
#[cfg(feature = "BeatSaber+GameSettings+MainSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::GameSettings::MainSettings =>
    "BeatSaber.GameSettings"."MainSettings"
);
#[cfg(feature = "BeatSaber+GameSettings+MainSettings")]
impl std::ops::Deref for crate::BeatSaber::GameSettings::MainSettings {
    type Target = crate::BGLib::SaveDataCore::VersionableSaveData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+GameSettings+MainSettings")]
impl std::ops::DerefMut for crate::BeatSaber::GameSettings::MainSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+GameSettings+MainSettings")]
impl crate::BeatSaber::GameSettings::MainSettings {
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
    pub fn get_audioSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::BeatSaber::GameSettings::Audio> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::BeatSaber::GameSettings::Audio = __cordl_object
            .invoke("get_audioSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_controllerSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::BeatSaber::GameSettings::Controllers,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::BeatSaber::GameSettings::Controllers = __cordl_object
            .invoke("get_controllerSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_createScreenshotDuringTheGame(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_createScreenshotDuringTheGame", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_customServerSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::BeatSaber::GameSettings::CustomServer,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::BeatSaber::GameSettings::CustomServer = __cordl_object
            .invoke("get_customServerSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_debugSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::BeatSaber::GameSettings::DebugSettings,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::BeatSaber::GameSettings::DebugSettings = __cordl_object
            .invoke("get_debugSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_hapticFeedback(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hapticFeedback", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_isDirty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isDirty", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_language(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BGLib::Polyglot::Language> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BGLib::Polyglot::Language = __cordl_object
            .invoke("get_language", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_pauseButtonPressDurationLevel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_pauseButtonPressDurationLevel", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_roomCenter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_roomCenter", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_roomRotation(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_roomRotation", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_smoothCameraSettings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::BeatSaber::GameSettings::SmoothCamera,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::BeatSaber::GameSettings::SmoothCamera = __cordl_object
            .invoke("get_smoothCameraSettings", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_audioSettings(
        &mut self,
        value: *mut crate::BeatSaber::GameSettings::Audio,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_audioSettings", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_controllerSettings(
        &mut self,
        value: *mut crate::BeatSaber::GameSettings::Controllers,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_controllerSettings", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_createScreenshotDuringTheGame(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_createScreenshotDuringTheGame", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_customServerSettings(
        &mut self,
        value: *mut crate::BeatSaber::GameSettings::CustomServer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_customServerSettings", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_debugSettings(
        &mut self,
        value: *mut crate::BeatSaber::GameSettings::DebugSettings,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_debugSettings", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_hapticFeedback(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_hapticFeedback", (value))?;
        Ok(__cordl_ret)
    }
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
    pub fn set_language(
        &mut self,
        value: crate::BGLib::Polyglot::Language,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_language", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_pauseButtonPressDurationLevel(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_pauseButtonPressDurationLevel", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_roomCenter(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_roomCenter", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_roomRotation(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_roomRotation", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_smoothCameraSettings(
        &mut self,
        value: *mut crate::BeatSaber::GameSettings::SmoothCamera,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_smoothCameraSettings", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatSaber+GameSettings+MainSettings")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatSaber::GameSettings::MainSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
