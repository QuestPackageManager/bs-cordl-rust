#[cfg(feature = "EmptyPlatformAchievementsHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct EmptyPlatformAchievementsHandler {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "EmptyPlatformAchievementsHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for EmptyPlatformAchievementsHandler => ""
    ."EmptyPlatformAchievementsHandler"
);
#[cfg(feature = "EmptyPlatformAchievementsHandler")]
impl std::ops::Deref for EmptyPlatformAchievementsHandler {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EmptyPlatformAchievementsHandler")]
impl std::ops::DerefMut for EmptyPlatformAchievementsHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EmptyPlatformAchievementsHandler")]
impl EmptyPlatformAchievementsHandler {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "EmptyPlatformAchievementsHandler")]
impl quest_hook::libil2cpp::ObjectType for EmptyPlatformAchievementsHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
