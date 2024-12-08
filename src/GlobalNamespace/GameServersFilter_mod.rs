#[cfg(feature = "GameServersFilter")]
#[repr(C)]
#[derive(Debug)]
pub struct GameServersFilter {
    __cordl_parent: crate::System::Object,
    pub filterByDifficulty: bool,
    pub filteredDifficulty: BeatmapDifficultyMask,
    pub filterByModifiers: bool,
    pub filteredModifiers: GameplayModifierMask,
    pub filterBySongPacks: bool,
    pub filteredSongPacks: SongPackMask,
    pub showFull: bool,
    pub showProtected: bool,
    pub showInternetGames: bool,
}
#[cfg(feature = "GameServersFilter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for GameServersFilter => ""."GameServersFilter"
);
#[cfg(feature = "GameServersFilter")]
impl std::ops::Deref for GameServersFilter {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameServersFilter")]
impl std::ops::DerefMut for GameServersFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameServersFilter")]
impl GameServersFilter {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "GameServersFilter")]
impl quest_hook::libil2cpp::ObjectType for GameServersFilter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
