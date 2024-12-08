#[cfg(feature = "PlayingDifficultyBeatmapRichPresenceData")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayingDifficultyBeatmapRichPresenceData {
    __cordl_parent: crate::System::Object,
    pub _apiName_k__BackingField: *mut crate::System::String,
    pub _localizedDescription: *mut crate::System::String,
    pub _beatmapKey: BeatmapKey,
    pub _beatmapLevel: *mut BeatmapLevel,
}
#[cfg(feature = "PlayingDifficultyBeatmapRichPresenceData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for PlayingDifficultyBeatmapRichPresenceData => ""
    ."PlayingDifficultyBeatmapRichPresenceData"
);
#[cfg(feature = "PlayingDifficultyBeatmapRichPresenceData")]
impl std::ops::Deref for PlayingDifficultyBeatmapRichPresenceData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayingDifficultyBeatmapRichPresenceData")]
impl std::ops::DerefMut for PlayingDifficultyBeatmapRichPresenceData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayingDifficultyBeatmapRichPresenceData")]
impl PlayingDifficultyBeatmapRichPresenceData {
    pub fn get_apiName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_apiName", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<BeatmapKey>,
        beatmapLevel: *mut BeatmapLevel,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (beatmapKey, beatmapLevel))?;
        Ok(__cordl_ret)
    }
    pub fn get_localizedDescription(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_localizedDescription", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        beatmapKey: quest_hook::libil2cpp::ByRefMut<BeatmapKey>,
        beatmapLevel: *mut BeatmapLevel,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (beatmapKey, beatmapLevel))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "PlayingDifficultyBeatmapRichPresenceData")]
impl quest_hook::libil2cpp::ObjectType for PlayingDifficultyBeatmapRichPresenceData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
