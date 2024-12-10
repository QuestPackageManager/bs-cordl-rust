#[cfg(feature = "MultiplayerModeSettings")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerModeSettings {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub createServerPlayersCount: i32,
    pub quickPlayBeatmapDifficulty: crate::GlobalNamespace::BeatmapDifficultyMask,
    pub quickPlaySongPackMaskSerializedName: *mut quest_hook::libil2cpp::Il2CppString,
    pub quickPlayEnableLevelSelection: bool,
}
#[cfg(feature = "MultiplayerModeSettings")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MultiplayerModeSettings => ""
    ."MultiplayerModeSettings"
);
#[cfg(feature = "MultiplayerModeSettings")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerModeSettings {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerModeSettings")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerModeSettings {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerModeSettings")]
impl crate::GlobalNamespace::MultiplayerModeSettings {
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
}
#[cfg(feature = "MultiplayerModeSettings")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerModeSettings {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
