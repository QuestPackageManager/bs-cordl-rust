#[cfg(feature = "SonyLeaderboardIdsModel")]
#[repr(C)]
#[derive(Debug)]
pub struct SonyLeaderboardIdsModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _leaderboardIds: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            u32,
        >,
    >,
}
#[cfg(feature = "SonyLeaderboardIdsModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SonyLeaderboardIdsModel => ""
    ."SonyLeaderboardIdsModel"
);
#[cfg(feature = "SonyLeaderboardIdsModel")]
impl std::ops::Deref for crate::GlobalNamespace::SonyLeaderboardIdsModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SonyLeaderboardIdsModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::SonyLeaderboardIdsModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SonyLeaderboardIdsModel")]
impl crate::GlobalNamespace::SonyLeaderboardIdsModel {
    pub fn GetMaxId(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("GetMaxId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        idsMaps: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::SonyLeaderboardIdsSO,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (idsMaps))?;
        Ok(__cordl_object.into())
    }
    pub fn TryGetSonyLeaderboardId(
        &mut self,
        beatmapKey: quest_hook::libil2cpp::ByRefMut<crate::GlobalNamespace::BeatmapKey>,
        sonyLeaderboardId: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetSonyLeaderboardId", (beatmapKey, sonyLeaderboardId))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        idsMaps: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::GlobalNamespace::SonyLeaderboardIdsSO,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (idsMaps))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "SonyLeaderboardIdsModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SonyLeaderboardIdsModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
