#[cfg(feature = "Oculus+Platform+Samples+SimplePlatformSample+DataEntry")]
#[repr(C)]
#[derive(Debug)]
pub struct DataEntry {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub dataOutput: *mut crate::UnityEngine::UI::Text,
}
#[cfg(feature = "Oculus+Platform+Samples+SimplePlatformSample+DataEntry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Oculus::Platform::Samples::SimplePlatformSample::DataEntry =>
    "Oculus.Platform.Samples.SimplePlatformSample"."DataEntry"
);
#[cfg(feature = "Oculus+Platform+Samples+SimplePlatformSample+DataEntry")]
impl std::ops::Deref
for crate::Oculus::Platform::Samples::SimplePlatformSample::DataEntry {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Samples+SimplePlatformSample+DataEntry")]
impl std::ops::DerefMut
for crate::Oculus::Platform::Samples::SimplePlatformSample::DataEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Samples+SimplePlatformSample+DataEntry")]
impl crate::Oculus::Platform::Samples::SimplePlatformSample::DataEntry {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn SubmitCommand(
        &mut self,
        command: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SubmitCommand", (command))?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret)
    }
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
    pub fn achievementCountCallback(
        &mut self,
        msg: *mut crate::Oculus::Platform::Message,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("achievementCountCallback", (msg))?;
        Ok(__cordl_ret)
    }
    pub fn achievementDefinitionCallback(
        &mut self,
        msg: *mut crate::Oculus::Platform::Message_1<
            *mut crate::Oculus::Platform::Models::AchievementDefinitionList,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("achievementDefinitionCallback", (msg))?;
        Ok(__cordl_ret)
    }
    pub fn achievementFieldsCallback(
        &mut self,
        msg: *mut crate::Oculus::Platform::Message,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("achievementFieldsCallback", (msg))?;
        Ok(__cordl_ret)
    }
    pub fn achievementProgressCallback(
        &mut self,
        msg: *mut crate::Oculus::Platform::Message_1<
            *mut crate::Oculus::Platform::Models::AchievementProgressList,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("achievementProgressCallback", (msg))?;
        Ok(__cordl_ret)
    }
    pub fn achievementUnlockCallback(
        &mut self,
        msg: *mut crate::Oculus::Platform::Message,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("achievementUnlockCallback", (msg))?;
        Ok(__cordl_ret)
    }
    pub fn addCountAchievement(
        &mut self,
        achievementName: *mut quest_hook::libil2cpp::Il2CppString,
        count: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("addCountAchievement", (achievementName, count))?;
        Ok(__cordl_ret)
    }
    pub fn addFieldsAchievement(
        &mut self,
        achievementName: *mut quest_hook::libil2cpp::Il2CppString,
        fields: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("addFieldsAchievement", (achievementName, fields))?;
        Ok(__cordl_ret)
    }
    pub fn checkEntitlement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("checkEntitlement", ())?;
        Ok(__cordl_ret)
    }
    pub fn getAchievementDefinition(
        &mut self,
        achievementName: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("getAchievementDefinition", (achievementName))?;
        Ok(__cordl_ret)
    }
    pub fn getAchievementProgress(
        &mut self,
        achievementName: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("getAchievementProgress", (achievementName))?;
        Ok(__cordl_ret)
    }
    pub fn getEntitlementCallback(
        &mut self,
        msg: *mut crate::Oculus::Platform::Message,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("getEntitlementCallback", (msg))?;
        Ok(__cordl_ret)
    }
    pub fn getFriendsCallback(
        &mut self,
        msg: *mut crate::Oculus::Platform::Message_1<
            *mut crate::Oculus::Platform::Models::UserList,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("getFriendsCallback", (msg))?;
        Ok(__cordl_ret)
    }
    pub fn getLeaderboardEntries(
        &mut self,
        leaderboardName: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("getLeaderboardEntries", (leaderboardName))?;
        Ok(__cordl_ret)
    }
    pub fn getLoggedInFriends(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("getLoggedInFriends", ())?;
        Ok(__cordl_ret)
    }
    pub fn getLoggedInUser(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("getLoggedInUser", ())?;
        Ok(__cordl_ret)
    }
    pub fn getUser(
        &mut self,
        userID: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("getUser", (userID))?;
        Ok(__cordl_ret)
    }
    pub fn getUserCallback(
        &mut self,
        msg: *mut crate::Oculus::Platform::Message_1<
            *mut crate::Oculus::Platform::Models::User,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("getUserCallback", (msg))?;
        Ok(__cordl_ret)
    }
    pub fn getUserNonce(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("getUserNonce", ())?;
        Ok(__cordl_ret)
    }
    pub fn leaderboardGetCallback(
        &mut self,
        msg: *mut crate::Oculus::Platform::Message_1<
            *mut crate::Oculus::Platform::Models::LeaderboardEntryList,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("leaderboardGetCallback", (msg))?;
        Ok(__cordl_ret)
    }
    pub fn leaderboardWriteCallback(
        &mut self,
        msg: *mut crate::Oculus::Platform::Message,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("leaderboardWriteCallback", (msg))?;
        Ok(__cordl_ret)
    }
    pub fn outputUserArray(
        &mut self,
        users: *mut crate::Oculus::Platform::Models::UserList,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("outputUserArray", (users))?;
        Ok(__cordl_ret)
    }
    pub fn printOutputLine(
        &mut self,
        newLine: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("printOutputLine", (newLine))?;
        Ok(__cordl_ret)
    }
    pub fn unlockAchievement(
        &mut self,
        achievementName: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("unlockAchievement", (achievementName))?;
        Ok(__cordl_ret)
    }
    pub fn userProofCallback(
        &mut self,
        msg: *mut crate::Oculus::Platform::Message_1<
            *mut crate::Oculus::Platform::Models::UserProof,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("userProofCallback", (msg))?;
        Ok(__cordl_ret)
    }
    pub fn writeLeaderboardEntry(
        &mut self,
        leaderboardName: *mut quest_hook::libil2cpp::Il2CppString,
        value: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("writeLeaderboardEntry", (leaderboardName, value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Oculus+Platform+Samples+SimplePlatformSample+DataEntry")]
impl quest_hook::libil2cpp::ObjectType
for crate::Oculus::Platform::Samples::SimplePlatformSample::DataEntry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
