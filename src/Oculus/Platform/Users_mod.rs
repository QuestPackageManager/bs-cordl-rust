#[cfg(feature = "Oculus+Platform+Users")]
#[repr(C)]
#[derive(Debug)]
pub struct Users {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Oculus+Platform+Users")]
unsafe impl quest_hook::libil2cpp::Type for crate::Oculus::Platform::Users {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Oculus.Platform";
    const CLASS_NAME: &'static str = "Users";
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
#[cfg(feature = "Oculus+Platform+Users")]
impl std::ops::Deref for crate::Oculus::Platform::Users {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Users")]
impl std::ops::DerefMut for crate::Oculus::Platform::Users {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Users")]
impl crate::Oculus::Platform::Users {
    pub fn Get(
        userID: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::User>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::User>,
                    >,
                >,
                1usize,
            >("Get")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Get", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::User>,
            >,
        > = unsafe { method.invoke_unchecked((), (userID)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetAccessToken() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >,
                0usize,
            >("GetAccessToken")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetAccessToken", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn GetBlockedUsers() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::BlockedUserList,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::BlockedUserList,
                        >,
                    >,
                >,
                0usize,
            >("GetBlockedUsers")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetBlockedUsers", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::BlockedUserList,
                >,
            >,
        > = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn GetLoggedInUser() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::User>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::User>,
                    >,
                >,
                0usize,
            >("GetLoggedInUser")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetLoggedInUser", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::User>,
            >,
        > = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn GetLoggedInUserFriends() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::UserList>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::UserList,
                        >,
                    >,
                >,
                0usize,
            >("GetLoggedInUserFriends")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetLoggedInUserFriends", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::UserList>,
            >,
        > = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn GetLoggedInUserLocale() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("GetLoggedInUserLocale")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetLoggedInUserLocale", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn GetNextBlockedUserListPage(
        list: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::BlockedUserList>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::BlockedUserList,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::BlockedUserList,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::BlockedUserList,
                        >,
                    >,
                >,
                1usize,
            >("GetNextBlockedUserListPage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetNextBlockedUserListPage", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::BlockedUserList,
                >,
            >,
        > = unsafe { method.invoke_unchecked((), (list)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetNextUserCapabilityListPage(
        list: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::UserCapabilityList,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::UserCapabilityList,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::UserCapabilityList,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::UserCapabilityList,
                        >,
                    >,
                >,
                1usize,
            >("GetNextUserCapabilityListPage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetNextUserCapabilityListPage", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::UserCapabilityList,
                >,
            >,
        > = unsafe { method.invoke_unchecked((), (list)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetNextUserListPage(
        list: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::UserList>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::UserList>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::UserList>),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::UserList,
                        >,
                    >,
                >,
                1usize,
            >("GetNextUserListPage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetNextUserListPage", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::UserList>,
            >,
        > = unsafe { method.invoke_unchecked((), (list)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetOrgScopedID(
        userID: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::OrgScopedID>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::OrgScopedID,
                        >,
                    >,
                >,
                1usize,
            >("GetOrgScopedID")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetOrgScopedID", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::OrgScopedID>,
            >,
        > = unsafe { method.invoke_unchecked((), (userID)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetSdkAccounts() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::SdkAccountList,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::SdkAccountList,
                        >,
                    >,
                >,
                0usize,
            >("GetSdkAccounts")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetSdkAccounts", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::SdkAccountList,
                >,
            >,
        > = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn GetUserProof() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::UserProof>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::UserProof,
                        >,
                    >,
                >,
                0usize,
            >("GetUserProof")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetUserProof", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::UserProof>,
            >,
        > = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn LaunchBlockFlow(
        userID: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::LaunchBlockFlowResult,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::LaunchBlockFlowResult,
                        >,
                    >,
                >,
                1usize,
            >("LaunchBlockFlow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LaunchBlockFlow", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::LaunchBlockFlowResult,
                >,
            >,
        > = unsafe { method.invoke_unchecked((), (userID)) };
        Ok(__cordl_ret.into())
    }
    pub fn LaunchFriendRequestFlow(
        userID: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::LaunchFriendRequestFlowResult,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::LaunchFriendRequestFlowResult,
                        >,
                    >,
                >,
                1usize,
            >("LaunchFriendRequestFlow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LaunchFriendRequestFlow", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::LaunchFriendRequestFlowResult,
                >,
            >,
        > = unsafe { method.invoke_unchecked((), (userID)) };
        Ok(__cordl_ret.into())
    }
    pub fn LaunchUnblockFlow(
        userID: u64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::LaunchUnblockFlowResult,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::LaunchUnblockFlowResult,
                        >,
                    >,
                >,
                1usize,
            >("LaunchUnblockFlow")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LaunchUnblockFlow", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::LaunchUnblockFlowResult,
                >,
            >,
        > = unsafe { method.invoke_unchecked((), (userID)) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+Users")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::Users {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
