#[cfg(feature = "AchievementsModelSO")]
#[repr(C)]
#[derive(Debug)]
pub struct AchievementsModelSO {
    __cordl_parent: crate::GlobalNamespace::PersistentScriptableObject,
    pub _platformAchievementsHandler: *mut crate::GlobalNamespace::IPlatformAchievementsHandler,
    pub _unlockedAchievementIds: *mut crate::System::Collections::Generic::HashSet_1<
        *mut crate::System::String,
    >,
    pub _initialized: bool,
}
#[cfg(feature = "AchievementsModelSO")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::AchievementsModelSO => ""
    ."AchievementsModelSO"
);
#[cfg(feature = "AchievementsModelSO")]
impl std::ops::Deref for crate::GlobalNamespace::AchievementsModelSO {
    type Target = crate::GlobalNamespace::PersistentScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "AchievementsModelSO")]
impl std::ops::DerefMut for crate::GlobalNamespace::AchievementsModelSO {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "AchievementsModelSO")]
impl crate::GlobalNamespace::AchievementsModelSO {
    #[cfg(feature = "AchievementsModelSO+__c__DisplayClass4_0")]
    pub type __c__DisplayClass4_0 = crate::GlobalNamespace::AchievementsModelSO___c__DisplayClass4_0;
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn IsAchievementUnlocked(
        &mut self,
        achievement: *mut crate::GlobalNamespace::AchievementSO,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("IsAchievementUnlocked", (achievement))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn UnlockAchievement(
        &mut self,
        achievement: *mut crate::GlobalNamespace::AchievementSO,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnlockAchievement", (achievement))?;
        Ok(__cordl_ret)
    }
    pub fn _Initialize_b__3_0(
        &mut self,
        result: crate::GlobalNamespace::IPlatformAchievementsHandler_GetUnlockedAchievementsResult,
        achievementIds: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<Initialize>b__3_0", (result, achievementIds))?;
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
}
#[cfg(feature = "AchievementsModelSO")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::AchievementsModelSO {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
