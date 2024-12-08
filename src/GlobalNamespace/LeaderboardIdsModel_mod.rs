#[cfg(feature = "LeaderboardIdsModel")]
#[repr(C)]
#[derive(Debug)]
pub struct LeaderboardIdsModel {
    __cordl_parent: crate::System::Object,
    pub _leaderboardIds: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        *mut crate::System::String,
    >,
}
#[cfg(feature = "LeaderboardIdsModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::LeaderboardIdsModel => ""
    ."LeaderboardIdsModel"
);
#[cfg(feature = "LeaderboardIdsModel")]
impl std::ops::Deref for crate::GlobalNamespace::LeaderboardIdsModel {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "LeaderboardIdsModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::LeaderboardIdsModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "LeaderboardIdsModel")]
impl crate::GlobalNamespace::LeaderboardIdsModel {
    pub fn New(
        idsMaps: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::GlobalNamespace::LeaderboardIdsSO,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (idsMaps))?;
        Ok(__cordl_object)
    }
    pub fn TryGetPlatformLeaderboardId(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        platformLeaderboardId: quest_hook::libil2cpp::ByRefMut<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetPlatformLeaderboardId", (beatmapKey, platformLeaderboardId))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        idsMaps: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::GlobalNamespace::LeaderboardIdsSO,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (idsMaps))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "LeaderboardIdsModel")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::LeaderboardIdsModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
