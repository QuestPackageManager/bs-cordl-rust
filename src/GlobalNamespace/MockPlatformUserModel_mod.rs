#[cfg(feature = "MockPlatformUserModel")]
#[repr(C)]
#[derive(Debug)]
pub struct MockPlatformUserModel {
    __cordl_parent: crate::System::Object,
    pub platformUserInfoDidChangeEvent: *mut crate::System::Action_1<*mut UserInfo>,
}
#[cfg(feature = "MockPlatformUserModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MockPlatformUserModel => ""."MockPlatformUserModel"
);
#[cfg(feature = "MockPlatformUserModel")]
impl std::ops::Deref for MockPlatformUserModel {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlatformUserModel")]
impl std::ops::DerefMut for MockPlatformUserModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MockPlatformUserModel")]
impl MockPlatformUserModel {
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
    pub fn GetUserAuthToken(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut PlatformUserAuthTokenData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut PlatformUserAuthTokenData,
        > = __cordl_object.invoke("GetUserAuthToken", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CanXPlatformAccessTokenBeCached(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_CanXPlatformAccessTokenBeCached", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetUserFriendsUserIds(
        &mut self,
        cached: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::System::String,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::System::String,
            >,
        > = __cordl_object.invoke("GetUserFriendsUserIds", (cached))?;
        Ok(__cordl_ret)
    }
    pub fn GetUserNamesForUserIds(
        &mut self,
        userIds: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::System::String,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            *mut crate::System::Collections::Generic::IReadOnlyList_1<
                *mut crate::System::String,
            >,
        > = __cordl_object.invoke("GetUserNamesForUserIds", (userIds))?;
        Ok(__cordl_ret)
    }
    pub fn remove_platformUserInfoDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut UserInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_platformUserInfoDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn RequestXPlatformAccessToken(
        &mut self,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<XPlatformAccessTokenData>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<
            XPlatformAccessTokenData,
        > = __cordl_object.invoke("RequestXPlatformAccessToken", (cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn add_platformUserInfoDidChangeEvent(
        &mut self,
        value: *mut crate::System::Action_1<*mut UserInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_platformUserInfoDidChangeEvent", (value))?;
        Ok(__cordl_ret)
    }
    pub fn GetUserInfo(
        &mut self,
        ctx: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut UserInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<*mut UserInfo> = __cordl_object
            .invoke("GetUserInfo", (ctx))?;
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
#[cfg(feature = "MockPlatformUserModel")]
impl quest_hook::libil2cpp::ObjectType for MockPlatformUserModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
