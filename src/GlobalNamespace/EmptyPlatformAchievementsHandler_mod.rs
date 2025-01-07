#[cfg(feature = "EmptyPlatformAchievementsHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct EmptyPlatformAchievementsHandler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "EmptyPlatformAchievementsHandler")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::EmptyPlatformAchievementsHandler {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "EmptyPlatformAchievementsHandler";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "EmptyPlatformAchievementsHandler")]
impl std::ops::Deref for crate::GlobalNamespace::EmptyPlatformAchievementsHandler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EmptyPlatformAchievementsHandler")]
impl std::ops::DerefMut for crate::GlobalNamespace::EmptyPlatformAchievementsHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EmptyPlatformAchievementsHandler")]
impl crate::GlobalNamespace::EmptyPlatformAchievementsHandler {
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
#[cfg(feature = "EmptyPlatformAchievementsHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::EmptyPlatformAchievementsHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "EmptyPlatformAchievementsHandler")]
impl AsRef<crate::GlobalNamespace::IPlatformAchievementsHandler>
for crate::GlobalNamespace::EmptyPlatformAchievementsHandler {
    fn as_ref(&self) -> &crate::GlobalNamespace::IPlatformAchievementsHandler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "EmptyPlatformAchievementsHandler")]
impl AsMut<crate::GlobalNamespace::IPlatformAchievementsHandler>
for crate::GlobalNamespace::EmptyPlatformAchievementsHandler {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IPlatformAchievementsHandler {
        unsafe { std::mem::transmute(self) }
    }
}
