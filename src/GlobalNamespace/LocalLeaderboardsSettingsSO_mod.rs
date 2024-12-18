#[cfg(feature = "LocalLeaderboardsSettingsSO")]
#[repr(C)]
#[derive(Debug)]
pub struct LocalLeaderboardsSettingsSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _maxNumberOfScoresInLeaderboard: i32,
}
#[cfg(feature = "LocalLeaderboardsSettingsSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LocalLeaderboardsSettingsSO =>
    ""."LocalLeaderboardsSettingsSO"
);
#[cfg(feature = "LocalLeaderboardsSettingsSO")]
impl std::ops::Deref for crate::GlobalNamespace::LocalLeaderboardsSettingsSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LocalLeaderboardsSettingsSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::LocalLeaderboardsSettingsSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LocalLeaderboardsSettingsSO")]
impl crate::GlobalNamespace::LocalLeaderboardsSettingsSO {
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
    pub fn get_maxNumberOfScoresInLeaderboard(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("get_maxNumberOfScoresInLeaderboard", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_maxNumberOfScoresInLeaderboard(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_maxNumberOfScoresInLeaderboard", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "LocalLeaderboardsSettingsSO")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::LocalLeaderboardsSettingsSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
