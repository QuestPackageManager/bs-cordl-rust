#[cfg(feature = "OculusPlatformAchievementHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusPlatformAchievementHandler {
    __cordl_parent: crate::System::Object,
    pub _achievementIdsModel: *mut AchievementIdsModelSO,
}
#[cfg(feature = "OculusPlatformAchievementHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for OculusPlatformAchievementHandler => ""
    ."OculusPlatformAchievementHandler"
);
#[cfg(feature = "OculusPlatformAchievementHandler")]
impl std::ops::Deref for OculusPlatformAchievementHandler {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusPlatformAchievementHandler")]
impl std::ops::DerefMut for OculusPlatformAchievementHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusPlatformAchievementHandler")]
impl OculusPlatformAchievementHandler {
    #[cfg(feature = "OculusPlatformAchievementHandler+__c__DisplayClass3_0")]
    pub type __c__DisplayClass3_0 = crate::GlobalNamespace::OculusPlatformAchievementHandler___c__DisplayClass3_0;
    #[cfg(feature = "OculusPlatformAchievementHandler+__c__DisplayClass3_1")]
    pub type __c__DisplayClass3_1 = crate::GlobalNamespace::OculusPlatformAchievementHandler___c__DisplayClass3_1;
    #[cfg(feature = "OculusPlatformAchievementHandler+__c__DisplayClass2_0")]
    pub type __c__DisplayClass2_0 = crate::GlobalNamespace::OculusPlatformAchievementHandler___c__DisplayClass2_0;
    pub fn UnlockAchievement(
        &mut self,
        achievementId: *mut crate::System::String,
        completionHandler: *mut crate::GlobalNamespace::IPlatformAchievementsHandler_UnlockAchievementCompletionHandler,
    ) -> quest_hook::libil2cpp::Result<*mut HMAsyncRequest> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut HMAsyncRequest = __cordl_object
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
    pub fn GetUnlockedAchievements(
        &mut self,
        completionHandler: *mut crate::GlobalNamespace::IPlatformAchievementsHandler_GetUnlockedAchievementsCompletionHandler,
    ) -> quest_hook::libil2cpp::Result<*mut HMAsyncRequest> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut HMAsyncRequest = __cordl_object
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "OculusPlatformAchievementHandler")]
impl quest_hook::libil2cpp::ObjectType for OculusPlatformAchievementHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
