#[cfg(feature = "GameServersFilter")]
#[repr(C)]
#[derive(Debug)]
pub struct GameServersFilter {
    __cordl_parent: crate::System::Object,
    pub filterByDifficulty: bool,
    pub filteredDifficulty: crate::GlobalNamespace::BeatmapDifficultyMask,
    pub filterByModifiers: bool,
    pub filteredModifiers: crate::GlobalNamespace::GameplayModifierMask,
    pub filterBySongPacks: bool,
    pub filteredSongPacks: crate::GlobalNamespace::SongPackMask,
    pub showFull: bool,
    pub showProtected: bool,
    pub showInternetGames: bool,
}
#[cfg(feature = "GameServersFilter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::GameServersFilter => ""
    ."GameServersFilter"
);
#[cfg(feature = "GameServersFilter")]
impl std::ops::Deref for crate::GlobalNamespace::GameServersFilter {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameServersFilter")]
impl std::ops::DerefMut for crate::GlobalNamespace::GameServersFilter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameServersFilter")]
impl crate::GlobalNamespace::GameServersFilter {
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
#[cfg(feature = "GameServersFilter")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::GameServersFilter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
