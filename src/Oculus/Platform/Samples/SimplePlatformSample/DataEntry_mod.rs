#[cfg(feature = "Oculus+Platform+Samples+SimplePlatformSample+DataEntry")]
#[repr(C)]
#[derive(Debug)]
pub struct DataEntry {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub dataOutput: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Text>,
}
#[cfg(feature = "Oculus+Platform+Samples+SimplePlatformSample+DataEntry")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Oculus::Platform::Samples::SimplePlatformSample::DataEntry {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Oculus.Platform.Samples.SimplePlatformSample";
    const CLASS_NAME: &'static str = "DataEntry";
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Start")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as
                    quest_hook::libil2cpp::Type > ::class(), "Start", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SubmitCommand(
        &mut self,
        command: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SubmitCommand")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as
                    quest_hook::libil2cpp::Type > ::class(), "SubmitCommand", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (command))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Update")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as
                    quest_hook::libil2cpp::Type > ::class(), "Update", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn achievementCountCallback(
        &mut self,
        msg: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("achievementCountCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as
                    quest_hook::libil2cpp::Type > ::class(), "achievementCountCallback",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (msg))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn achievementDefinitionCallback(
        &mut self,
        msg: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Message_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::AchievementDefinitionList,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Message_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::AchievementDefinitionList,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("achievementDefinitionCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as
                    quest_hook::libil2cpp::Type > ::class(),
                    "achievementDefinitionCallback", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (msg))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn achievementFieldsCallback(
        &mut self,
        msg: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("achievementFieldsCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as
                    quest_hook::libil2cpp::Type > ::class(), "achievementFieldsCallback",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (msg))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn achievementProgressCallback(
        &mut self,
        msg: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Message_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::AchievementProgressList,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Message_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::AchievementProgressList,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("achievementProgressCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as
                    quest_hook::libil2cpp::Type > ::class(),
                    "achievementProgressCallback", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (msg))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn achievementUnlockCallback(
        &mut self,
        msg: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("achievementUnlockCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as
                    quest_hook::libil2cpp::Type > ::class(), "achievementUnlockCallback",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (msg))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn addCountAchievement(
        &mut self,
        achievementName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        count: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("addCountAchievement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as
                    quest_hook::libil2cpp::Type > ::class(), "addCountAchievement",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (achievementName, count))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn addFieldsAchievement(
        &mut self,
        achievementName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        fields: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("addFieldsAchievement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as
                    quest_hook::libil2cpp::Type > ::class(), "addFieldsAchievement",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (achievementName, fields))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn checkEntitlement(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("checkEntitlement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as
                    quest_hook::libil2cpp::Type > ::class(), "checkEntitlement", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn getAchievementDefinition(
        &mut self,
        achievementName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("getAchievementDefinition")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as
                    quest_hook::libil2cpp::Type > ::class(), "getAchievementDefinition",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (achievementName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn getAchievementProgress(
        &mut self,
        achievementName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("getAchievementProgress")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as
                    quest_hook::libil2cpp::Type > ::class(), "getAchievementProgress",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (achievementName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn getEntitlementCallback(
        &mut self,
        msg: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("getEntitlementCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as
                    quest_hook::libil2cpp::Type > ::class(), "getEntitlementCallback",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (msg))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn getFriendsCallback(
        &mut self,
        msg: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Message_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::UserList>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Message_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::UserList,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("getFriendsCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as
                    quest_hook::libil2cpp::Type > ::class(), "getFriendsCallback", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (msg))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn getLeaderboardEntries(
        &mut self,
        leaderboardName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("getLeaderboardEntries")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as
                    quest_hook::libil2cpp::Type > ::class(), "getLeaderboardEntries",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (leaderboardName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn getLoggedInFriends(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("getLoggedInFriends")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as
                    quest_hook::libil2cpp::Type > ::class(), "getLoggedInFriends", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn getLoggedInUser(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("getLoggedInUser")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as
                    quest_hook::libil2cpp::Type > ::class(), "getLoggedInUser", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn getUser(
        &mut self,
        userID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("getUser")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as
                    quest_hook::libil2cpp::Type > ::class(), "getUser", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (userID))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn getUserCallback(
        &mut self,
        msg: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Message_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::User>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Message_1<
                        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::User>,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("getUserCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as
                    quest_hook::libil2cpp::Type > ::class(), "getUserCallback", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (msg))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn getUserNonce(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("getUserNonce")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as
                    quest_hook::libil2cpp::Type > ::class(), "getUserNonce", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn leaderboardGetCallback(
        &mut self,
        msg: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Message_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::LeaderboardEntryList,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Message_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::LeaderboardEntryList,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("leaderboardGetCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as
                    quest_hook::libil2cpp::Type > ::class(), "leaderboardGetCallback",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (msg))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn leaderboardWriteCallback(
        &mut self,
        msg: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("leaderboardWriteCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as
                    quest_hook::libil2cpp::Type > ::class(), "leaderboardWriteCallback",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (msg))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn outputUserArray(
        &mut self,
        users: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::UserList>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::UserList>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("outputUserArray")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as
                    quest_hook::libil2cpp::Type > ::class(), "outputUserArray", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (users))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn printOutputLine(
        &mut self,
        newLine: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("printOutputLine")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as
                    quest_hook::libil2cpp::Type > ::class(), "printOutputLine", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (newLine))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn unlockAchievement(
        &mut self,
        achievementName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("unlockAchievement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as
                    quest_hook::libil2cpp::Type > ::class(), "unlockAchievement", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (achievementName))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn userProofCallback(
        &mut self,
        msg: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Message_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::UserProof>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Message_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::UserProof,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("userProofCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as
                    quest_hook::libil2cpp::Type > ::class(), "userProofCallback", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (msg))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn writeLeaderboardEntry(
        &mut self,
        leaderboardName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("writeLeaderboardEntry")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Oculus::Platform::Samples::SimplePlatformSample::DataEntry as
                    quest_hook::libil2cpp::Type > ::class(), "writeLeaderboardEntry",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (leaderboardName, value))?
        };
        Ok(__cordl_ret.into())
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
