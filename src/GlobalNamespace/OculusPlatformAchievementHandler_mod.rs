#[cfg(feature = "OculusPlatformAchievementHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusPlatformAchievementHandler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _achievementIdsModel: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::AchievementIdsModelSO,
    >,
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
    pub fn GetUnlockedAchievements(
        &mut self,
        completionHandler: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IPlatformAchievementsHandler_GetUnlockedAchievementsCompletionHandler,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::HMAsyncRequest>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::HMAsyncRequest,
        > = __cordl_object.invoke("GetUnlockedAchievements", (completionHandler))?;
        Ok(__cordl_ret.into())
    }
    pub fn Initialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Initialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UnlockAchievement(
        &mut self,
        achievementId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        completionHandler: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IPlatformAchievementsHandler_UnlockAchievementCompletionHandler,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::HMAsyncRequest>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::HMAsyncRequest,
        > = __cordl_object
            .invoke("UnlockAchievement", (achievementId, completionHandler))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "OculusPlatformAchievementHandler")]
impl AsRef<crate::GlobalNamespace::IPlatformAchievementsHandler>
for crate::GlobalNamespace::OculusPlatformAchievementHandler {
    fn as_ref(&self) -> &crate::GlobalNamespace::IPlatformAchievementsHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OculusPlatformAchievementHandler")]
impl AsMut<crate::GlobalNamespace::IPlatformAchievementsHandler>
for crate::GlobalNamespace::OculusPlatformAchievementHandler {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IPlatformAchievementsHandler {
        unsafe { std::mem::transmute(self) }
    }
}
