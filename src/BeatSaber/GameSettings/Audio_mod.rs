#[cfg(feature = "BeatSaber+GameSettings+Audio")]
#[repr(C)]
#[derive(Debug)]
pub struct Audio {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _isDirty_k__BackingField: bool,
    pub _volume: f32,
    pub _ambientVolumeScale: f32,
    pub _overrideAudioLatency: bool,
    pub _audioLatency: f32,
}
#[cfg(feature = "BeatSaber+GameSettings+Audio")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::GameSettings::Audio =>
    "BeatSaber.GameSettings"."Audio"
);
#[cfg(feature = "BeatSaber+GameSettings+Audio")]
impl std::ops::Deref for crate::BeatSaber::GameSettings::Audio {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+GameSettings+Audio")]
impl std::ops::DerefMut for crate::BeatSaber::GameSettings::Audio {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+GameSettings+Audio")]
impl crate::BeatSaber::GameSettings::Audio {
    pub fn GetAudioLatency(
        &mut self,
        graphicSettingsHandler: quest_hook::libil2cpp::Gc<
            crate::BeatSaber::GameSettings::GraphicSettingsHandler,
        >,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("GetAudioLatency", (graphicSettingsHandler))?;
        Ok(__cordl_ret.into())
    }
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
    pub fn get_ambientVolumeScale(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_ambientVolumeScale", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isDirty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isDirty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_overrideAudioLatency(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_overrideAudioLatency", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_volume(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_volume", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ambientVolumeScale(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ambientVolumeScale", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_audioLatency(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_audioLatency", (value))?;
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
    pub fn set_overrideAudioLatency(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_overrideAudioLatency", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_volume(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_volume", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+GameSettings+Audio")]
impl quest_hook::libil2cpp::ObjectType for crate::BeatSaber::GameSettings::Audio {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatSaber+GameSettings+Audio")]
impl AsRef<crate::BGLib::SaveDataCore::ISerializableSaveData>
for crate::BeatSaber::GameSettings::Audio {
    fn as_ref(&self) -> &crate::BGLib::SaveDataCore::ISerializableSaveData {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaber+GameSettings+Audio")]
impl AsMut<crate::BGLib::SaveDataCore::ISerializableSaveData>
for crate::BeatSaber::GameSettings::Audio {
    fn as_mut(&mut self) -> &mut crate::BGLib::SaveDataCore::ISerializableSaveData {
        unsafe { std::mem::transmute(self) }
    }
}
