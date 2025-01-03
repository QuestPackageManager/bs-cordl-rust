#[cfg(feature = "AdaptiveSfxVolume")]
#[repr(C)]
#[derive(Debug)]
pub struct AdaptiveSfxVolume {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _audioManager: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::AudioManagerSO>,
    pub _initData: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AdaptiveSfxVolume_InitData,
    >,
    pub _minThreshold: f32,
    pub _buffer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MomentaryLoudnessBuffer,
    >,
    pub _lufsMeter: quest_hook::libil2cpp::Gc<crate::LufsMetering::LufsMeter>,
    pub _loudnessHistory: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::MomentaryLoudnessHistory,
    >,
}
#[cfg(feature = "AdaptiveSfxVolume")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::AdaptiveSfxVolume => ""
    ."AdaptiveSfxVolume"
);
#[cfg(feature = "AdaptiveSfxVolume")]
impl std::ops::Deref for crate::GlobalNamespace::AdaptiveSfxVolume {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AdaptiveSfxVolume")]
impl std::ops::DerefMut for crate::GlobalNamespace::AdaptiveSfxVolume {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AdaptiveSfxVolume")]
impl crate::GlobalNamespace::AdaptiveSfxVolume {
    pub const kAverageLoudnessFrames: i32 = 12i32;
    pub const kReadingsPerBuffer: i32 = 4i32;
    #[cfg(feature = "AdaptiveSfxVolume+InitData")]
    pub type InitData = crate::GlobalNamespace::AdaptiveSfxVolume_InitData;
    pub fn ApplyLoudness(
        &mut self,
        songLoudness: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyLoudness", (songLoudness))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnAudioFilterRead(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<f32>>,
        channels: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnAudioFilterRead", (data, channels))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret.into())
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
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
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
#[cfg(feature = "AdaptiveSfxVolume")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::AdaptiveSfxVolume {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "AdaptiveSfxVolume+InitData")]
#[repr(C)]
#[derive(Debug)]
pub struct AdaptiveSfxVolume_InitData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub userSettingsVolumeOffset: f32,
    pub adaptiveSfx: bool,
    pub frequency: i32,
}
#[cfg(feature = "AdaptiveSfxVolume+InitData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::AdaptiveSfxVolume_InitData =>
    ""."AdaptiveSfxVolume/InitData"
);
#[cfg(feature = "AdaptiveSfxVolume+InitData")]
impl std::ops::Deref for crate::GlobalNamespace::AdaptiveSfxVolume_InitData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AdaptiveSfxVolume+InitData")]
impl std::ops::DerefMut for crate::GlobalNamespace::AdaptiveSfxVolume_InitData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AdaptiveSfxVolume+InitData")]
impl crate::GlobalNamespace::AdaptiveSfxVolume_InitData {
    pub fn New(
        userSettingsVolumeOffset: f32,
        adaptiveSfx: bool,
        frequency: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (userSettingsVolumeOffset, adaptiveSfx, frequency))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        userSettingsVolumeOffset: f32,
        adaptiveSfx: bool,
        frequency: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (userSettingsVolumeOffset, adaptiveSfx, frequency))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "AdaptiveSfxVolume+InitData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::AdaptiveSfxVolume_InitData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
