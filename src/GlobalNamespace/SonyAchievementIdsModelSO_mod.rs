#[cfg(feature = "SonyAchievementIdsModelSO+AchievementIdData")]
#[repr(C)]
#[derive(Debug)]
pub struct SonyAchievementIdsModelSO_AchievementIdData {
    __cordl_parent: crate::System::Object,
    pub _trophyId: i32,
    pub _achievement: *mut AchievementSO,
}
#[cfg(feature = "SonyAchievementIdsModelSO+AchievementIdData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::SonyAchievementIdsModelSO_AchievementIdData => ""
    ."SonyAchievementIdsModelSO/AchievementIdData"
);
#[cfg(feature = "SonyAchievementIdsModelSO+AchievementIdData")]
impl std::ops::Deref
for crate::GlobalNamespace::SonyAchievementIdsModelSO_AchievementIdData {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SonyAchievementIdsModelSO+AchievementIdData")]
impl std::ops::DerefMut
for crate::GlobalNamespace::SonyAchievementIdsModelSO_AchievementIdData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SonyAchievementIdsModelSO+AchievementIdData")]
impl crate::GlobalNamespace::SonyAchievementIdsModelSO_AchievementIdData {
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
    pub fn get_achievementId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_achievementId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ps4TrophyId(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ps4TrophyId", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "SonyAchievementIdsModelSO+AchievementIdData")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SonyAchievementIdsModelSO_AchievementIdData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "SonyAchievementIdsModelSO")]
#[repr(C)]
#[derive(Debug)]
pub struct SonyAchievementIdsModelSO {
    __cordl_parent: PersistentScriptableObject,
    pub _achievementsIds: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::SonyAchievementIdsModelSO_AchievementIdData,
    >,
    pub _achievementIdToTrophyId: *mut crate::System::Collections::Generic::Dictionary_2<
        *mut crate::System::String,
        i32,
    >,
    pub _trophyIdToAchievementId: *mut crate::System::Collections::Generic::Dictionary_2<
        i32,
        *mut crate::System::String,
    >,
}
#[cfg(feature = "SonyAchievementIdsModelSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SonyAchievementIdsModelSO => ""
    ."SonyAchievementIdsModelSO"
);
#[cfg(feature = "SonyAchievementIdsModelSO")]
impl std::ops::Deref for SonyAchievementIdsModelSO {
    type Target = PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SonyAchievementIdsModelSO")]
impl std::ops::DerefMut for SonyAchievementIdsModelSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SonyAchievementIdsModelSO")]
impl SonyAchievementIdsModelSO {
    #[cfg(feature = "SonyAchievementIdsModelSO+AchievementIdData")]
    pub type AchievementIdData = crate::GlobalNamespace::SonyAchievementIdsModelSO_AchievementIdData;
    pub fn GetAchievementId(
        &mut self,
        trophyId: i32,
        achievementId: quest_hook::libil2cpp::ByRefMut<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetAchievementId", (trophyId, achievementId))?;
        Ok(__cordl_ret)
    }
    pub fn GetTrophyId(
        &mut self,
        achievementId: *mut crate::System::String,
        trophyId: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("GetTrophyId", (achievementId, trophyId))?;
        Ok(__cordl_ret)
    }
    pub fn GetTrophyIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<i32>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<i32> = __cordl_object
            .invoke("GetTrophyIds", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret)
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
    pub fn get_achievementsIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::SonyAchievementIdsModelSO_AchievementIdData,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::GlobalNamespace::SonyAchievementIdsModelSO_AchievementIdData,
        > = __cordl_object.invoke("get_achievementsIds", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "SonyAchievementIdsModelSO")]
impl quest_hook::libil2cpp::ObjectType for SonyAchievementIdsModelSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
