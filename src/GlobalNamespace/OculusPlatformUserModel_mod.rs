#[cfg(feature = "OculusPlatformUserModel+__GetUserNamesForUserIds_g__Fetch_16_0_d")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OculusPlatformUserModel___GetUserNamesForUserIds_g__Fetch_16_0_d {
    pub __1__state: i32,
    pub __t__builder: crate::System::Runtime::CompilerServices::AsyncTaskMethodBuilder_1<
        *mut crate::Oculus::Platform::Message_1<
            *mut crate::Oculus::Platform::Models::User,
        >,
    >,
    pub userId: *mut crate::System::String,
    pub __u__1: crate::System::Runtime::CompilerServices::TaskAwaiter_1<
        *mut crate::Oculus::Platform::Message_1<
            *mut crate::Oculus::Platform::Models::User,
        >,
    >,
}
#[cfg(feature = "OculusPlatformUserModel+__GetUserNamesForUserIds_g__Fetch_16_0_d")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OculusPlatformUserModel___GetUserNamesForUserIds_g__Fetch_16_0_d
    => ""."OculusPlatformUserModel/<<GetUserNamesForUserIds>g__Fetch|16_0>d"
);
#[cfg(feature = "OculusPlatformUserModel+__GetUserNamesForUserIds_g__Fetch_16_0_d")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OculusPlatformUserModel___GetUserNamesForUserIds_g__Fetch_16_0_d {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OculusPlatformUserModel+__GetUserNamesForUserIds_g__Fetch_16_0_d")]
impl crate::GlobalNamespace::OculusPlatformUserModel___GetUserNamesForUserIds_g__Fetch_16_0_d {
    pub fn SetStateMachine(
        &mut self,
        stateMachine: *mut crate::System::Runtime::CompilerServices::IAsyncStateMachine,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetStateMachine",
            (stateMachine),
        )?;
        Ok(__cordl_ret)
    }
    pub fn MoveNext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveNext",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OculusPlatformUserModel")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusPlatformUserModel {
    __cordl_parent: crate::System::Object,
    pub _platformInit: *mut IPlatformInit,
    pub _userInfoTask: *mut crate::System::Threading::Tasks::Task_1<*mut UserInfo>,
    pub _userInfo: *mut UserInfo,
    pub _friendsUserIds: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::String,
    >,
    pub _lastXPlatformTokenStatusChange: f32,
    pub platformUserInfoDidChangeEvent: *mut crate::System::Action_1<*mut UserInfo>,
}
#[cfg(feature = "OculusPlatformUserModel")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for OculusPlatformUserModel => ""."OculusPlatformUserModel"
);
#[cfg(feature = "OculusPlatformUserModel")]
impl std::ops::Deref for OculusPlatformUserModel {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusPlatformUserModel")]
impl std::ops::DerefMut for OculusPlatformUserModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusPlatformUserModel")]
impl OculusPlatformUserModel {
    #[cfg(feature = "OculusPlatformUserModel+__c")]
    pub type __c = crate::GlobalNamespace::OculusPlatformUserModel___c;
    #[cfg(feature = "OculusPlatformUserModel+_GetUserInfo_d__12")]
    pub type _GetUserInfo_d__12 = crate::GlobalNamespace::OculusPlatformUserModel__GetUserInfo_d__12;
    #[cfg(feature = "OculusPlatformUserModel+_RequestXPlatformAccessToken_d__17")]
    pub type _RequestXPlatformAccessToken_d__17 = crate::GlobalNamespace::OculusPlatformUserModel__RequestXPlatformAccessToken_d__17;
    #[cfg(feature = "OculusPlatformUserModel+__GetUserNamesForUserIds_g__Fetch_16_0_d")]
    pub type __GetUserNamesForUserIds_g__Fetch_16_0_d = crate::GlobalNamespace::OculusPlatformUserModel___GetUserNamesForUserIds_g__Fetch_16_0_d;
    #[cfg(feature = "OculusPlatformUserModel+_GetUserFriendsUserIds_d__14")]
    pub type _GetUserFriendsUserIds_d__14 = crate::GlobalNamespace::OculusPlatformUserModel__GetUserFriendsUserIds_d__14;
    #[cfg(feature = "OculusPlatformUserModel+_GetUserInfoInternalAsync_d__13")]
    pub type _GetUserInfoInternalAsync_d__13 = crate::GlobalNamespace::OculusPlatformUserModel__GetUserInfoInternalAsync_d__13;
    #[cfg(feature = "OculusPlatformUserModel+_GetUserAuthToken_d__15")]
    pub type _GetUserAuthToken_d__15 = crate::GlobalNamespace::OculusPlatformUserModel__GetUserAuthToken_d__15;
    #[cfg(feature = "OculusPlatformUserModel+_GetUserNamesForUserIds_d__16")]
    pub type _GetUserNamesForUserIds_d__16 = crate::GlobalNamespace::OculusPlatformUserModel__GetUserNamesForUserIds_d__16;
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
    pub fn _ctor(
        &mut self,
        platformInit: *mut IPlatformInit,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (platformInit))?;
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
    pub fn GetUserInfoInternalAsync(
        &mut self,
        ctx: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<*mut UserInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<*mut UserInfo> = __cordl_object
            .invoke("GetUserInfoInternalAsync", (ctx))?;
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
    pub fn New(
        platformInit: *mut IPlatformInit,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (platformInit))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "OculusPlatformUserModel")]
impl quest_hook::libil2cpp::ObjectType for OculusPlatformUserModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
