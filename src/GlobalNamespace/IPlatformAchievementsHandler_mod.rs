#[cfg(feature = "IPlatformAchievementsHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct IPlatformAchievementsHandler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IPlatformAchievementsHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IPlatformAchievementsHandler =>
    ""."IPlatformAchievementsHandler"
);
#[cfg(feature = "IPlatformAchievementsHandler")]
impl std::ops::Deref for crate::GlobalNamespace::IPlatformAchievementsHandler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IPlatformAchievementsHandler")]
impl std::ops::DerefMut for crate::GlobalNamespace::IPlatformAchievementsHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IPlatformAchievementsHandler")]
impl crate::GlobalNamespace::IPlatformAchievementsHandler {
    #[cfg(
        feature = "IPlatformAchievementsHandler+GetUnlockedAchievementsCompletionHandler"
    )]
    pub type GetUnlockedAchievementsCompletionHandler = crate::GlobalNamespace::IPlatformAchievementsHandler_GetUnlockedAchievementsCompletionHandler;
    #[cfg(feature = "IPlatformAchievementsHandler+GetUnlockedAchievementsResult")]
    pub type GetUnlockedAchievementsResult = crate::GlobalNamespace::IPlatformAchievementsHandler_GetUnlockedAchievementsResult;
    #[cfg(feature = "IPlatformAchievementsHandler+UnlockAchievementCompletionHandler")]
    pub type UnlockAchievementCompletionHandler = crate::GlobalNamespace::IPlatformAchievementsHandler_UnlockAchievementCompletionHandler;
    #[cfg(feature = "IPlatformAchievementsHandler+UnlockAchievementResult")]
    pub type UnlockAchievementResult = crate::GlobalNamespace::IPlatformAchievementsHandler_UnlockAchievementResult;
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
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IPlatformAchievementsHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IPlatformAchievementsHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "IPlatformAchievementsHandler+GetUnlockedAchievementsCompletionHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct IPlatformAchievementsHandler_GetUnlockedAchievementsCompletionHandler {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "IPlatformAchievementsHandler+GetUnlockedAchievementsCompletionHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::IPlatformAchievementsHandler_GetUnlockedAchievementsCompletionHandler
    => ""."IPlatformAchievementsHandler/GetUnlockedAchievementsCompletionHandler"
);
#[cfg(feature = "IPlatformAchievementsHandler+GetUnlockedAchievementsCompletionHandler")]
impl std::ops::Deref
for crate::GlobalNamespace::IPlatformAchievementsHandler_GetUnlockedAchievementsCompletionHandler {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IPlatformAchievementsHandler+GetUnlockedAchievementsCompletionHandler")]
impl std::ops::DerefMut
for crate::GlobalNamespace::IPlatformAchievementsHandler_GetUnlockedAchievementsCompletionHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IPlatformAchievementsHandler+GetUnlockedAchievementsCompletionHandler")]
impl crate::GlobalNamespace::IPlatformAchievementsHandler_GetUnlockedAchievementsCompletionHandler {
    pub fn BeginInvoke(
        &mut self,
        result: crate::GlobalNamespace::IPlatformAchievementsHandler_GetUnlockedAchievementsResult,
        unlockedAchievementsIds: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (result, unlockedAchievementsIds, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        result: crate::GlobalNamespace::IPlatformAchievementsHandler_GetUnlockedAchievementsResult,
        unlockedAchievementsIds: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (result, unlockedAchievementsIds))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "IPlatformAchievementsHandler+GetUnlockedAchievementsCompletionHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IPlatformAchievementsHandler_GetUnlockedAchievementsCompletionHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "IPlatformAchievementsHandler+GetUnlockedAchievementsResult")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IPlatformAchievementsHandler_GetUnlockedAchievementsResult {
    #[default]
    Failed = 1i32,
    OK = 0i32,
}
#[cfg(feature = "IPlatformAchievementsHandler+GetUnlockedAchievementsResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::IPlatformAchievementsHandler_GetUnlockedAchievementsResult => ""
    ."IPlatformAchievementsHandler/GetUnlockedAchievementsResult"
);
#[cfg(feature = "IPlatformAchievementsHandler+UnlockAchievementCompletionHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct IPlatformAchievementsHandler_UnlockAchievementCompletionHandler {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "IPlatformAchievementsHandler+UnlockAchievementCompletionHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::IPlatformAchievementsHandler_UnlockAchievementCompletionHandler =>
    ""."IPlatformAchievementsHandler/UnlockAchievementCompletionHandler"
);
#[cfg(feature = "IPlatformAchievementsHandler+UnlockAchievementCompletionHandler")]
impl std::ops::Deref
for crate::GlobalNamespace::IPlatformAchievementsHandler_UnlockAchievementCompletionHandler {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IPlatformAchievementsHandler+UnlockAchievementCompletionHandler")]
impl std::ops::DerefMut
for crate::GlobalNamespace::IPlatformAchievementsHandler_UnlockAchievementCompletionHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IPlatformAchievementsHandler+UnlockAchievementCompletionHandler")]
impl crate::GlobalNamespace::IPlatformAchievementsHandler_UnlockAchievementCompletionHandler {
    pub fn BeginInvoke(
        &mut self,
        result: crate::GlobalNamespace::IPlatformAchievementsHandler_UnlockAchievementResult,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (result, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        result: crate::GlobalNamespace::IPlatformAchievementsHandler_UnlockAchievementResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "IPlatformAchievementsHandler+UnlockAchievementCompletionHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IPlatformAchievementsHandler_UnlockAchievementCompletionHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "IPlatformAchievementsHandler+UnlockAchievementResult")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum IPlatformAchievementsHandler_UnlockAchievementResult {
    #[default]
    Failed = 1i32,
    OK = 0i32,
}
#[cfg(feature = "IPlatformAchievementsHandler+UnlockAchievementResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::IPlatformAchievementsHandler_UnlockAchievementResult => ""
    ."IPlatformAchievementsHandler/UnlockAchievementResult"
);
