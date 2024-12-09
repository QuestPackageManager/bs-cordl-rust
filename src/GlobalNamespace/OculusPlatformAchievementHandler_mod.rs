#[cfg(feature = "OculusPlatformAchievementHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusPlatformAchievementHandler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _achievementIdsModel: *mut crate::GlobalNamespace::AchievementIdsModelSO,
}
#[cfg(feature = "OculusPlatformAchievementHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OculusPlatformAchievementHandler => ""
    ."OculusPlatformAchievementHandler"
);
#[cfg(feature = "OculusPlatformAchievementHandler")]
impl std::ops::Deref for crate::GlobalNamespace::OculusPlatformAchievementHandler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusPlatformAchievementHandler")]
impl std::ops::DerefMut for crate::GlobalNamespace::OculusPlatformAchievementHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusPlatformAchievementHandler")]
impl crate::GlobalNamespace::OculusPlatformAchievementHandler {
    #[cfg(feature = "OculusPlatformAchievementHandler+__c__DisplayClass2_0")]
    pub type __c__DisplayClass2_0 = crate::GlobalNamespace::OculusPlatformAchievementHandler___c__DisplayClass2_0;
    #[cfg(feature = "OculusPlatformAchievementHandler+__c__DisplayClass3_0")]
    pub type __c__DisplayClass3_0 = crate::GlobalNamespace::OculusPlatformAchievementHandler___c__DisplayClass3_0;
    #[cfg(feature = "OculusPlatformAchievementHandler+__c__DisplayClass3_1")]
    pub type __c__DisplayClass3_1 = crate::GlobalNamespace::OculusPlatformAchievementHandler___c__DisplayClass3_1;
    pub fn GetUnlockedAchievements(
        &mut self,
        completionHandler: *mut crate::GlobalNamespace::IPlatformAchievementsHandler_GetUnlockedAchievementsCompletionHandler,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::HMAsyncRequest> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::HMAsyncRequest = __cordl_object
            .invoke("GetUnlockedAchievements", (completionHandler))?;
        Ok(__cordl_ret)
    }
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
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn UnlockAchievement(
        &mut self,
        achievementId: *mut quest_hook::libil2cpp::Il2CppString,
        completionHandler: *mut crate::GlobalNamespace::IPlatformAchievementsHandler_UnlockAchievementCompletionHandler,
    ) -> quest_hook::libil2cpp::Result<*mut crate::GlobalNamespace::HMAsyncRequest> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::GlobalNamespace::HMAsyncRequest = __cordl_object
            .invoke("UnlockAchievement", (achievementId, completionHandler))?;
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
#[cfg(feature = "OculusPlatformAchievementHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OculusPlatformAchievementHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
