#[cfg(feature = "PlayingDifficultyBeatmapRichPresenceData")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayingDifficultyBeatmapRichPresenceData {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _apiName_k__BackingField: *mut quest_hook::libil2cpp::Il2CppString,
    pub _localizedDescription: *mut quest_hook::libil2cpp::Il2CppString,
    pub _beatmapKey: crate::GlobalNamespace::BeatmapKey,
    pub _beatmapLevel: *mut crate::GlobalNamespace::BeatmapLevel,
}
#[cfg(feature = "PlayingDifficultyBeatmapRichPresenceData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PlayingDifficultyBeatmapRichPresenceData => ""
    ."PlayingDifficultyBeatmapRichPresenceData"
);
#[cfg(feature = "PlayingDifficultyBeatmapRichPresenceData")]
impl std::ops::Deref
for crate::GlobalNamespace::PlayingDifficultyBeatmapRichPresenceData {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayingDifficultyBeatmapRichPresenceData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PlayingDifficultyBeatmapRichPresenceData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayingDifficultyBeatmapRichPresenceData")]
impl crate::GlobalNamespace::PlayingDifficultyBeatmapRichPresenceData {
    pub fn New(
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (beatmapKey, beatmapLevel))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        beatmapLevel: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatmapLevel>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (beatmapKey, beatmapLevel))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_apiName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_apiName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_localizedDescription(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_localizedDescription", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PlayingDifficultyBeatmapRichPresenceData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayingDifficultyBeatmapRichPresenceData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
