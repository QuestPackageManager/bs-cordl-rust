#[cfg(feature = "BeatSaber+GameSettings+DebugSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct DebugSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _isDirty_k__BackingField: bool,
    pub _enableFpsCounter: bool,
    pub _enableFpsRecorder: bool,
    pub _enableMemoryTracker: bool,
    pub _showBeatmapLevelVersion: bool,
}
#[cfg(feature = "BeatSaber+GameSettings+DebugSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::GameSettings::DebugSettings =>
    "BeatSaber.GameSettings"."DebugSettings"
);
#[cfg(feature = "BeatSaber+GameSettings+DebugSettings")]
impl std::ops::Deref for crate::BeatSaber::GameSettings::DebugSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+GameSettings+DebugSettings")]
impl std::ops::DerefMut for crate::BeatSaber::GameSettings::DebugSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+GameSettings+DebugSettings")]
impl crate::BeatSaber::GameSettings::DebugSettings {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn get_enableFpsCounter(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enableFpsCounter", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_enableFpsRecorder(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enableFpsRecorder", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_enableMemoryTracker(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_enableMemoryTracker", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isDirty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isDirty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_showBeatmapLevelVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_showBeatmapLevelVersion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_enableFpsCounter(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_enableFpsCounter", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_enableFpsRecorder(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_enableFpsRecorder", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_enableMemoryTracker(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_enableMemoryTracker", (value))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn set_showBeatmapLevelVersion(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_showBeatmapLevelVersion", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+GameSettings+DebugSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::GameSettings::DebugSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatSaber+GameSettings+DebugSettings")]
impl AsRef<crate::BGLib::SaveDataCore::ISerializableSaveData>
for crate::BeatSaber::GameSettings::DebugSettings {
    fn as_ref(&self) -> &crate::BGLib::SaveDataCore::ISerializableSaveData {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaber+GameSettings+DebugSettings")]
impl AsMut<crate::BGLib::SaveDataCore::ISerializableSaveData>
for crate::BeatSaber::GameSettings::DebugSettings {
    fn as_mut(&mut self) -> &mut crate::BGLib::SaveDataCore::ISerializableSaveData {
        unsafe { std::mem::transmute(self) }
    }
}
