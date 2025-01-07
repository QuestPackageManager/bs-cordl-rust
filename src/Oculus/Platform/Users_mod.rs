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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::User>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Get", (userID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAccessToken() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetAccessToken", ())?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::BlockedUserList,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBlockedUsers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLoggedInUser() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::User>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::User>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLoggedInUser", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLoggedInUserFriends() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::UserList>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::UserList>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLoggedInUserFriends", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLoggedInUserLocale() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLoggedInUserLocale", ())?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::BlockedUserList,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNextBlockedUserListPage", (list))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::UserCapabilityList,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNextUserCapabilityListPage", (list))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::UserList>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNextUserListPage", (list))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::OrgScopedID>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetOrgScopedID", (userID))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::SdkAccountList,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetSdkAccounts", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUserProof() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::UserProof>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::UserProof>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetUserProof", ())?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::LaunchBlockFlowResult,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LaunchBlockFlow", (userID))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::LaunchFriendRequestFlowResult,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LaunchFriendRequestFlow", (userID))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::LaunchUnblockFlowResult,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LaunchUnblockFlow", (userID))?;
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
