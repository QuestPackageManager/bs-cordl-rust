#[cfg(feature = "OculusPlatformUserModel")]
#[repr(C)]
#[derive(Debug)]
pub struct OculusPlatformUserModel {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _platformInit: quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::IPlatformInit>,
    pub _userInfoTask: quest_hook::libil2cpp::Gc<
        crate::System::Threading::Tasks::Task_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::UserInfo>,
        >,
    >,
    pub _userInfo: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::UserInfo>,
    pub _friendsUserIds: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub _lastXPlatformTokenStatusChange: f32,
    pub platformUserInfoDidChangeEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::UserInfo>,
        >,
    >,
}
#[cfg(feature = "OculusPlatformUserModel")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OculusPlatformUserModel {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OculusPlatformUserModel";
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
#[cfg(feature = "OculusPlatformUserModel")]
impl std::ops::Deref for crate::GlobalNamespace::OculusPlatformUserModel {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OculusPlatformUserModel")]
impl std::ops::DerefMut for crate::GlobalNamespace::OculusPlatformUserModel {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OculusPlatformUserModel")]
impl crate::GlobalNamespace::OculusPlatformUserModel {
    #[cfg(feature = "OculusPlatformUserModel+__GetUserNamesForUserIds_g__Fetch_16_0_d")]
    pub type __GetUserNamesForUserIds_g__Fetch_16_0_d = crate::GlobalNamespace::OculusPlatformUserModel___GetUserNamesForUserIds_g__Fetch_16_0_d;
    pub fn GetUserAuthToken(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::PlatformUserAuthTokenData,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::PlatformUserAuthTokenData,
                >,
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
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IReadOnlyList_1<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IReadOnlyList_1<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
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
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::UserInfo>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::UserInfo>,
            >,
        > = __cordl_object.invoke("GetUserInfo", (ctx))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUserInfoInternalAsync(
        &mut self,
        ctx: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::UserInfo>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::UserInfo>,
            >,
        > = __cordl_object.invoke("GetUserInfoInternalAsync", (ctx))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUserNamesForUserIds(
        &mut self,
        userIds: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IReadOnlyList_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IReadOnlyList_1<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::IReadOnlyList_1<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
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
    pub fn New(
        platformInit: quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::IPlatformInit>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (platformInit))?;
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
    pub fn _GetUserNamesForUserIds_g__Fetch_16_0(
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Message_1<
                        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::User>,
                    >,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Message_1<
                        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::User>,
                    >,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("<GetUserNamesForUserIds>g__Fetch|16_0", (userId))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        platformInit: quest_hook::libil2cpp::Gc<crate::BeatSaber::Init::IPlatformInit>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (platformInit))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_platformUserInfoDidChangeEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::UserInfo>,
            >,
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
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::UserInfo>,
            >,
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
#[cfg(feature = "OculusPlatformUserModel")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OculusPlatformUserModel {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OculusPlatformUserModel")]
impl AsRef<crate::GlobalNamespace::IPlatformUserModel>
for crate::GlobalNamespace::OculusPlatformUserModel {
    fn as_ref(&self) -> &crate::GlobalNamespace::IPlatformUserModel {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OculusPlatformUserModel")]
impl AsMut<crate::GlobalNamespace::IPlatformUserModel>
for crate::GlobalNamespace::OculusPlatformUserModel {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IPlatformUserModel {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OculusPlatformUserModel+__GetUserNamesForUserIds_g__Fetch_16_0_d")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OculusPlatformUserModel___GetUserNamesForUserIds_g__Fetch_16_0_d {
    pub __1__state: i32,
    pub __t__builder: crate::System::Runtime::CompilerServices::AsyncTaskMethodBuilder_1<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Message_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::User>,
            >,
        >,
    >,
    pub userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub __u__1: crate::System::Runtime::CompilerServices::TaskAwaiter_1<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Message_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::User>,
            >,
        >,
    >,
}
#[cfg(feature = "OculusPlatformUserModel+__GetUserNamesForUserIds_g__Fetch_16_0_d")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OculusPlatformUserModel___GetUserNamesForUserIds_g__Fetch_16_0_d {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "<<GetUserNamesForUserIds>g__Fetch|16_0>d";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "OculusPlatformUserModel+__GetUserNamesForUserIds_g__Fetch_16_0_d")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OculusPlatformUserModel___GetUserNamesForUserIds_g__Fetch_16_0_d {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OculusPlatformUserModel+__GetUserNamesForUserIds_g__Fetch_16_0_d")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OculusPlatformUserModel___GetUserNamesForUserIds_g__Fetch_16_0_d {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "OculusPlatformUserModel+__GetUserNamesForUserIds_g__Fetch_16_0_d")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OculusPlatformUserModel___GetUserNamesForUserIds_g__Fetch_16_0_d {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "OculusPlatformUserModel+__GetUserNamesForUserIds_g__Fetch_16_0_d")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OculusPlatformUserModel___GetUserNamesForUserIds_g__Fetch_16_0_d {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
    pub fn MoveNext(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveNext",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetStateMachine(
        &mut self,
        stateMachine: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::IAsyncStateMachine,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetStateMachine",
            (stateMachine),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OculusPlatformUserModel+__GetUserNamesForUserIds_g__Fetch_16_0_d")]
impl AsRef<crate::System::Runtime::CompilerServices::IAsyncStateMachine>
for crate::GlobalNamespace::OculusPlatformUserModel___GetUserNamesForUserIds_g__Fetch_16_0_d {
    fn as_ref(&self) -> &crate::System::Runtime::CompilerServices::IAsyncStateMachine {
        todo!()
    }
}
#[cfg(feature = "OculusPlatformUserModel+__GetUserNamesForUserIds_g__Fetch_16_0_d")]
impl AsMut<crate::System::Runtime::CompilerServices::IAsyncStateMachine>
for crate::GlobalNamespace::OculusPlatformUserModel___GetUserNamesForUserIds_g__Fetch_16_0_d {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::CompilerServices::IAsyncStateMachine {
        todo!()
    }
}
