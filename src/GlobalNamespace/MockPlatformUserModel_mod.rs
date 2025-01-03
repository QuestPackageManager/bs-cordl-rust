#[cfg(feature = "MockPlatformUserModel")]
#[repr(C)]
#[derive(Debug)]
pub struct MockPlatformUserModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub platformUserInfoDidChangeEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<*mut crate::GlobalNamespace::UserInfo>,
    >,
}
#[cfg(feature = "MockPlatformUserModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MockPlatformUserModel => ""
    ."MockPlatformUserModel"
);
#[cfg(feature = "MockPlatformUserModel")]
impl std::ops::Deref for crate::GlobalNamespace::MockPlatformUserModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlatformUserModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::MockPlatformUserModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlatformUserModel")]
impl crate::GlobalNamespace::MockPlatformUserModel {
    pub fn GetUserAuthToken(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::GlobalNamespace::PlatformUserAuthTokenData,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::GlobalNamespace::PlatformUserAuthTokenData,
            >,
        > = __cordl_object.invoke("GetUserAuthToken", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUserFriendsUserIds(
        &mut self,
        cached: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::System::Collections::Generic::IReadOnlyList_1<
                    *mut quest_hook::libil2cpp::Il2CppString,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::System::Collections::Generic::IReadOnlyList_1<
                    *mut quest_hook::libil2cpp::Il2CppString,
                >,
            >,
        > = __cordl_object.invoke("GetUserFriendsUserIds", (cached))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUserInfo(
        &mut self,
        ctx: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::GlobalNamespace::UserInfo,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::GlobalNamespace::UserInfo,
            >,
        > = __cordl_object.invoke("GetUserInfo", (ctx))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUserNamesForUserIds(
        &mut self,
        userIds: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::System::Collections::Generic::IReadOnlyList_1<
                    *mut quest_hook::libil2cpp::Il2CppString,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                *mut crate::System::Collections::Generic::IReadOnlyList_1<
                    *mut quest_hook::libil2cpp::Il2CppString,
                >,
            >,
        > = __cordl_object.invoke("GetUserNamesForUserIds", (userIds))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUsersSystemLanguage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::BGLib::Polyglot::Language> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::BGLib::Polyglot::Language = __cordl_object
            .invoke("GetUsersSystemLanguage", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RequestXPlatformAccessToken(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::XPlatformAccessTokenData,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                crate::GlobalNamespace::XPlatformAccessTokenData,
            >,
        > = __cordl_object.invoke("RequestXPlatformAccessToken", (cancellationToken))?;
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
    pub fn add_platformUserInfoDidChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::UserInfo>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_platformUserInfoDidChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CanXPlatformAccessTokenBeCached(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_CanXPlatformAccessTokenBeCached", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_platformUserInfoDidChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<*mut crate::GlobalNamespace::UserInfo>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_platformUserInfoDidChangeEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MockPlatformUserModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MockPlatformUserModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MockPlatformUserModel")]
impl AsRef<crate::GlobalNamespace::IPlatformUserModel>
for crate::GlobalNamespace::MockPlatformUserModel {
    fn as_ref(&self) -> &crate::GlobalNamespace::IPlatformUserModel {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "MockPlatformUserModel")]
impl AsMut<crate::GlobalNamespace::IPlatformUserModel>
for crate::GlobalNamespace::MockPlatformUserModel {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IPlatformUserModel {
        unsafe { std::mem::transmute(self) }
    }
}
