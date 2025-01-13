#[cfg(feature = "Oculus+Platform+Message")]
#[repr(C)]
#[derive(Debug)]
pub struct Message {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _cordl_type: crate::Oculus::Platform::Message_MessageType,
    pub requestID: u64,
    pub error: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Error>,
}
#[cfg(feature = "Oculus+Platform+Message")]
unsafe impl quest_hook::libil2cpp::Type for crate::Oculus::Platform::Message {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Oculus.Platform";
    const CLASS_NAME: &'static str = "Message";
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
#[cfg(feature = "Oculus+Platform+Message")]
impl std::ops::Deref for crate::Oculus::Platform::Message {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Message")]
impl std::ops::DerefMut for crate::Oculus::Platform::Message {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Message")]
impl crate::Oculus::Platform::Message {
    #[cfg(feature = "Oculus+Platform+Message+Callback")]
    pub type Callback = crate::Oculus::Platform::Message_Callback;
    #[cfg(feature = "Oculus+Platform+Message+ExtraMessageTypesHandler")]
    pub type ExtraMessageTypesHandler = crate::Oculus::Platform::Message_ExtraMessageTypesHandler;
    #[cfg(feature = "Oculus+Platform+Message+MessageType")]
    pub type MessageType = crate::Oculus::Platform::Message_MessageType;
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAbuseReportRecording(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::AbuseReportRecording>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::AbuseReportRecording,
        > = __cordl_object.invoke("GetAbuseReportRecording", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAchievementDefinitions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::AchievementDefinitionList,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::AchievementDefinitionList,
        > = __cordl_object.invoke("GetAchievementDefinitions", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAchievementProgressList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::AchievementProgressList,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::AchievementProgressList,
        > = __cordl_object.invoke("GetAchievementProgressList", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAchievementUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::AchievementUpdate>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::AchievementUpdate,
        > = __cordl_object.invoke("GetAchievementUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAppDownloadProgressResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::AppDownloadProgressResult,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::AppDownloadProgressResult,
        > = __cordl_object.invoke("GetAppDownloadProgressResult", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAppDownloadResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::AppDownloadResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::AppDownloadResult,
        > = __cordl_object.invoke("GetAppDownloadResult", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetApplicationInviteList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::ApplicationInviteList>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::ApplicationInviteList,
        > = __cordl_object.invoke("GetApplicationInviteList", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetApplicationVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::ApplicationVersion>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::ApplicationVersion,
        > = __cordl_object.invoke("GetApplicationVersion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssetDetails(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::AssetDetails>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::AssetDetails,
        > = __cordl_object.invoke("GetAssetDetails", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssetDetailsList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::AssetDetailsList>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::AssetDetailsList,
        > = __cordl_object.invoke("GetAssetDetailsList", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssetFileDeleteResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::AssetFileDeleteResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::AssetFileDeleteResult,
        > = __cordl_object.invoke("GetAssetFileDeleteResult", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssetFileDownloadCancelResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::AssetFileDownloadCancelResult,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::AssetFileDownloadCancelResult,
        > = __cordl_object.invoke("GetAssetFileDownloadCancelResult", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssetFileDownloadResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::AssetFileDownloadResult,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::AssetFileDownloadResult,
        > = __cordl_object.invoke("GetAssetFileDownloadResult", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssetFileDownloadUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::AssetFileDownloadUpdate,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::AssetFileDownloadUpdate,
        > = __cordl_object.invoke("GetAssetFileDownloadUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAvatarEditorResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::AvatarEditorResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::AvatarEditorResult,
        > = __cordl_object.invoke("GetAvatarEditorResult", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBlockedUserList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::BlockedUserList>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::BlockedUserList,
        > = __cordl_object.invoke("GetBlockedUserList", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetChallenge(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Challenge>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::Challenge,
        > = __cordl_object.invoke("GetChallenge", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetChallengeEntryList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::ChallengeEntryList>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::ChallengeEntryList,
        > = __cordl_object.invoke("GetChallengeEntryList", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetChallengeList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::ChallengeList>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::ChallengeList,
        > = __cordl_object.invoke("GetChallengeList", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDataStore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = __cordl_object.invoke("GetDataStore", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDestinationList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::DestinationList>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::DestinationList,
        > = __cordl_object.invoke("GetDestinationList", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetError(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Error>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::Error,
        > = __cordl_object.invoke("GetError", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGroupPresenceJoinIntent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::GroupPresenceJoinIntent,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::GroupPresenceJoinIntent,
        > = __cordl_object.invoke("GetGroupPresenceJoinIntent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetGroupPresenceLeaveIntent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::GroupPresenceLeaveIntent,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::GroupPresenceLeaveIntent,
        > = __cordl_object.invoke("GetGroupPresenceLeaveIntent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHttpTransferUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::HttpTransferUpdate>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::HttpTransferUpdate,
        > = __cordl_object.invoke("GetHttpTransferUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInstalledApplicationList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::InstalledApplicationList,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::InstalledApplicationList,
        > = __cordl_object.invoke("GetInstalledApplicationList", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInvitePanelResultInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::InvitePanelResultInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::InvitePanelResultInfo,
        > = __cordl_object.invoke("GetInvitePanelResultInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLaunchBlockFlowResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::LaunchBlockFlowResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::LaunchBlockFlowResult,
        > = __cordl_object.invoke("GetLaunchBlockFlowResult", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLaunchFriendRequestFlowResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::LaunchFriendRequestFlowResult,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::LaunchFriendRequestFlowResult,
        > = __cordl_object.invoke("GetLaunchFriendRequestFlowResult", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLaunchInvitePanelFlowResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::LaunchInvitePanelFlowResult,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::LaunchInvitePanelFlowResult,
        > = __cordl_object.invoke("GetLaunchInvitePanelFlowResult", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLaunchReportFlowResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::LaunchReportFlowResult,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::LaunchReportFlowResult,
        > = __cordl_object.invoke("GetLaunchReportFlowResult", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLaunchUnblockFlowResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::LaunchUnblockFlowResult,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::LaunchUnblockFlowResult,
        > = __cordl_object.invoke("GetLaunchUnblockFlowResult", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLeaderboardDidUpdate(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetLeaderboardDidUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLeaderboardEntryList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::LeaderboardEntryList>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::LeaderboardEntryList,
        > = __cordl_object.invoke("GetLeaderboardEntryList", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLeaderboardList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::LeaderboardList>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::LeaderboardList,
        > = __cordl_object.invoke("GetLeaderboardList", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLinkedAccountList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::LinkedAccountList>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::LinkedAccountList,
        > = __cordl_object.invoke("GetLinkedAccountList", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLivestreamingApplicationStatus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::LivestreamingApplicationStatus,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::LivestreamingApplicationStatus,
        > = __cordl_object.invoke("GetLivestreamingApplicationStatus", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLivestreamingStartResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::LivestreamingStartResult,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::LivestreamingStartResult,
        > = __cordl_object.invoke("GetLivestreamingStartResult", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLivestreamingStatus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::LivestreamingStatus>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::LivestreamingStatus,
        > = __cordl_object.invoke("GetLivestreamingStatus", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLivestreamingVideoStats(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::LivestreamingVideoStats,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::LivestreamingVideoStats,
        > = __cordl_object.invoke("GetLivestreamingVideoStats", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMicrophoneAvailabilityState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::MicrophoneAvailabilityState,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::MicrophoneAvailabilityState,
        > = __cordl_object.invoke("GetMicrophoneAvailabilityState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNetSyncConnection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::NetSyncConnection>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::NetSyncConnection,
        > = __cordl_object.invoke("GetNetSyncConnection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNetSyncSessionList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::NetSyncSessionList>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::NetSyncSessionList,
        > = __cordl_object.invoke("GetNetSyncSessionList", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNetSyncSessionsChangedNotification(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::NetSyncSessionsChangedNotification,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::NetSyncSessionsChangedNotification,
        > = __cordl_object.invoke("GetNetSyncSessionsChangedNotification", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNetSyncSetSessionPropertyResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::NetSyncSetSessionPropertyResult,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::NetSyncSetSessionPropertyResult,
        > = __cordl_object.invoke("GetNetSyncSetSessionPropertyResult", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNetSyncVoipAttenuationValueList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::NetSyncVoipAttenuationValueList,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::NetSyncVoipAttenuationValueList,
        > = __cordl_object.invoke("GetNetSyncVoipAttenuationValueList", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOrgScopedID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::OrgScopedID>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::OrgScopedID,
        > = __cordl_object.invoke("GetOrgScopedID", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetParty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Party>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::Party,
        > = __cordl_object.invoke("GetParty", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPartyID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::PartyID>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::PartyID,
        > = __cordl_object.invoke("GetPartyID", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPartyUpdateNotification(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::PartyUpdateNotification,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::PartyUpdateNotification,
        > = __cordl_object.invoke("GetPartyUpdateNotification", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPidList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::PidList>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::PidList,
        > = __cordl_object.invoke("GetPidList", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPlatformInitialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::PlatformInitialize>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::PlatformInitialize,
        > = __cordl_object.invoke("GetPlatformInitialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetProductList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::ProductList>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::ProductList,
        > = __cordl_object.invoke("GetProductList", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPurchase(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::Purchase>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::Purchase,
        > = __cordl_object.invoke("GetPurchase", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPurchaseList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::PurchaseList>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::PurchaseList,
        > = __cordl_object.invoke("GetPurchaseList", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRejoinDialogResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::RejoinDialogResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::RejoinDialogResult,
        > = __cordl_object.invoke("GetRejoinDialogResult", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSdkAccountList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::SdkAccountList>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::SdkAccountList,
        > = __cordl_object.invoke("GetSdkAccountList", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSendInvitesResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::SendInvitesResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::SendInvitesResult,
        > = __cordl_object.invoke("GetSendInvitesResult", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetShareMediaResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::ShareMediaResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::ShareMediaResult,
        > = __cordl_object.invoke("GetShareMediaResult", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSystemVoipState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::SystemVoipState>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::SystemVoipState,
        > = __cordl_object.invoke("GetSystemVoipState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUser(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::User>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::User,
        > = __cordl_object.invoke("GetUser", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUserAccountAgeCategory(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::UserAccountAgeCategory,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::UserAccountAgeCategory,
        > = __cordl_object.invoke("GetUserAccountAgeCategory", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUserCapabilityList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::UserCapabilityList>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::UserCapabilityList,
        > = __cordl_object.invoke("GetUserCapabilityList", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUserDataStoreUpdateResponse(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::UserDataStoreUpdateResponse,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::UserDataStoreUpdateResponse,
        > = __cordl_object.invoke("GetUserDataStoreUpdateResponse", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUserList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::UserList>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::UserList,
        > = __cordl_object.invoke("GetUserList", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUserProof(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::UserProof>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::UserProof,
        > = __cordl_object.invoke("GetUserProof", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetUserReportID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::UserReportID>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::UserReportID,
        > = __cordl_object.invoke("GetUserReportID", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        c_message: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (c_message))?;
        Ok(__cordl_object.into())
    }
    pub fn ParseMessageHandle(
        messageHandle: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseMessageHandle", (messageHandle))?;
        Ok(__cordl_ret.into())
    }
    pub fn PopMessage() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PopMessage", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        c_message: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (c_message))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_HandleExtraMessageTypes() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Message_ExtraMessageTypesHandler,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Message_ExtraMessageTypesHandler,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_HandleExtraMessageTypes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsError(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsError", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RequestID(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object.invoke("get_RequestID", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::Message_MessageType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Oculus::Platform::Message_MessageType = __cordl_object
            .invoke("get_Type", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_HandleExtraMessageTypes(
        value: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Message_ExtraMessageTypesHandler,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_HandleExtraMessageTypes", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+Message")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::Message {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Oculus+Platform+Message+Callback")]
#[repr(C)]
#[derive(Debug)]
pub struct Message_Callback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "Oculus+Platform+Message+Callback")]
unsafe impl quest_hook::libil2cpp::Type for crate::Oculus::Platform::Message_Callback {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Oculus.Platform";
    const CLASS_NAME: &'static str = "Message/Callback";
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
#[cfg(feature = "Oculus+Platform+Message+Callback")]
impl std::ops::Deref for crate::Oculus::Platform::Message_Callback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Message+Callback")]
impl std::ops::DerefMut for crate::Oculus::Platform::Message_Callback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Message+Callback")]
impl crate::Oculus::Platform::Message_Callback {
    pub fn BeginInvoke(
        &mut self,
        message: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (message, callback, object))?;
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
        message: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (message))?;
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
#[cfg(feature = "Oculus+Platform+Message+Callback")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::Message_Callback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Oculus+Platform+Message+ExtraMessageTypesHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct Message_ExtraMessageTypesHandler {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "Oculus+Platform+Message+ExtraMessageTypesHandler")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Oculus::Platform::Message_ExtraMessageTypesHandler {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Oculus.Platform";
    const CLASS_NAME: &'static str = "Message/ExtraMessageTypesHandler";
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
#[cfg(feature = "Oculus+Platform+Message+ExtraMessageTypesHandler")]
impl std::ops::Deref for crate::Oculus::Platform::Message_ExtraMessageTypesHandler {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Message+ExtraMessageTypesHandler")]
impl std::ops::DerefMut for crate::Oculus::Platform::Message_ExtraMessageTypesHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Message+ExtraMessageTypesHandler")]
impl crate::Oculus::Platform::Message_ExtraMessageTypesHandler {
    pub fn BeginInvoke(
        &mut self,
        messageHandle: crate::System::IntPtr,
        message_type: crate::Oculus::Platform::Message_MessageType,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (messageHandle, message_type, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message> = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        messageHandle: crate::System::IntPtr,
        message_type: crate::Oculus::Platform::Message_MessageType,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Message> = __cordl_object
            .invoke("Invoke", (messageHandle, message_type))?;
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
#[cfg(feature = "Oculus+Platform+Message+ExtraMessageTypesHandler")]
impl quest_hook::libil2cpp::ObjectType
for crate::Oculus::Platform::Message_ExtraMessageTypesHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Oculus+Platform+Message+MessageType")]
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Message_MessageType {
    #[default]
    AbuseReport_ReportRequestHandled = 1267661958u32,
    Achievements_AddCount = 65495601u32,
    Achievements_AddFields = 346693929u32,
    Achievements_GetAllDefinitions = 64177549u32,
    Achievements_GetAllProgress = 1335877149u32,
    Achievements_GetDefinitionsByName = 1653670332u32,
    Achievements_GetNextAchievementDefinitionArrayPage = 712888917u32,
    Achievements_GetNextAchievementProgressArrayPage = 792913703u32,
    Achievements_GetProgressByName = 354837425u32,
    Achievements_Unlock = 1497156573u32,
    ApplicationLifecycle_GetRegisteredPIDs = 82169698u32,
    ApplicationLifecycle_GetSessionKey = 984570141u32,
    ApplicationLifecycle_RegisterSessionKey = 1303818232u32,
    Application_CancelAppDownload = 2082496734u32,
    Application_CheckAppDownloadProgress = 1429514532u32,
    Application_GetVersion = 1751583246u32,
    Application_InstallAppUpdateAndRelaunch = 343960453u32,
    Application_LaunchOtherApp = 1424151032u32,
    Application_StartAppDownload = 1157365870u32,
    AssetFile_Delete = 1834842246u32,
    AssetFile_DeleteById = 1525206354u32,
    AssetFile_DeleteByName = 1108001231u32,
    AssetFile_Download = 289710021u32,
    AssetFile_DownloadById = 755009938u32,
    AssetFile_DownloadByName = 1664536314u32,
    AssetFile_DownloadCancel = 134927303u32,
    AssetFile_DownloadCancelById = 1365611796u32,
    AssetFile_DownloadCancelByName = 1147858170u32,
    AssetFile_GetList = 1258057588u32,
    AssetFile_Status = 47394656u32,
    AssetFile_StatusById = 1570069816u32,
    AssetFile_StatusByName = 1104140880u32,
    Avatar_LaunchAvatarEditor = 99737939u32,
    Challenges_Create = 1750718017u32,
    Challenges_DeclineInvite = 1452177088u32,
    Challenges_Delete = 642287050u32,
    Challenges_Get = 2002276083u32,
    Challenges_GetEntries = 303739999u32,
    Challenges_GetEntriesAfterRank = 143202943u32,
    Challenges_GetEntriesByIds = 828705244u32,
    Challenges_GetList = 1126581078u32,
    Challenges_GetNextChallenges = 1534894518u32,
    Challenges_GetNextEntries = 2135728326u32,
    Challenges_GetPreviousChallenges = 246678541u32,
    Challenges_GetPreviousEntries = 2026439792u32,
    Challenges_Join = 556040297u32,
    Challenges_Leave = 694228709u32,
    Challenges_UpdateInfo = 292929120u32,
    DeviceApplicationIntegrity_GetIntegrityToken = 846310362u32,
    Entitlement_GetIsViewerEntitled = 409688241u32,
    GroupPresence_Clear = 1839897795u32,
    GroupPresence_GetInvitableUsers = 592167921u32,
    GroupPresence_GetNextApplicationInviteArrayPage = 83411186u32,
    GroupPresence_GetSentInvites = 136710833u32,
    GroupPresence_LaunchInvitePanel = 262066079u32,
    GroupPresence_LaunchMultiplayerErrorDialog = 693481252u32,
    GroupPresence_LaunchRejoinDialog = 360121199u32,
    GroupPresence_LaunchRosterPanel = 896698498u32,
    GroupPresence_SendInvites = 231461732u32,
    GroupPresence_Set = 1734302756u32,
    GroupPresence_SetDeeplinkMessageOverride = 1377492749u32,
    GroupPresence_SetDestination = 1281042058u32,
    GroupPresence_SetIsJoinable = 714018901u32,
    GroupPresence_SetLobbySession = 1224693182u32,
    GroupPresence_SetMatchSession = 827098296u32,
    IAP_ConsumePurchase = 532378329u32,
    IAP_GetNextProductArrayPage = 467225263u32,
    IAP_GetNextPurchaseArrayPage = 1196886677u32,
    IAP_GetProductsBySKU = 2124073717u32,
    IAP_GetViewerPurchases = 974095385u32,
    IAP_GetViewerPurchasesDurableCache = 1666817579u32,
    IAP_LaunchCheckoutFlow = 1067126029u32,
    LanguagePack_GetCurrent = 529592533u32,
    LanguagePack_SetCurrent = 1531952096u32,
    Leaderboard_Get = 1792298744u32,
    Leaderboard_GetEntries = 1572030284u32,
    Leaderboard_GetEntriesAfterRank = 406293487u32,
    Leaderboard_GetEntriesByIds = 962624508u32,
    Leaderboard_GetNextEntries = 1310751961u32,
    Leaderboard_GetNextLeaderboardArrayPage = 905344667u32,
    Leaderboard_GetPreviousEntries = 1224858304u32,
    Leaderboard_WriteEntry = 293587198u32,
    Leaderboard_WriteEntryWithSupplementaryMetric = 1925616378u32,
    Media_ShareToFacebook = 14912239u32,
    Notification_AbuseReport_ReportButtonPressed = 608644972u32,
    Notification_ApplicationLifecycle_LaunchIntentChanged = 78859427u32,
    Notification_AssetFile_DownloadUpdate = 803015885u32,
    Notification_GroupPresence_InvitationsSent = 1738179766u32,
    Notification_GroupPresence_JoinIntentReceived = 2000194038u32,
    Notification_GroupPresence_LeaveIntentReceived = 1194846749u32,
    Notification_HTTP_Transfer = 2111073839u32,
    Notification_Livestreaming_StatusChange = 575101294u32,
    Notification_MarkAsRead = 1903319523u32,
    Notification_NetSync_ConnectionStatusChanged = 120882378u32,
    Notification_NetSync_SessionsChanged = 947814198u32,
    Notification_Party_PartyUpdate = 487688882u32,
    Notification_Voip_MicrophoneAvailabilityStateUpdate = 1042336599u32,
    Notification_Voip_SystemVoipState = 1490179237u32,
    Notification_Vrcamera_GetDataChannelMessageUpdate = 1860498236u32,
    Notification_Vrcamera_GetSurfaceUpdate = 938610820u32,
    Party_GetCurrent = 1200830304u32,
    Platform_InitializeAndroidAsynchronous = 450037684u32,
    Platform_InitializeStandaloneOculus = 1375260172u32,
    Platform_InitializeWindowsAsynchronous = 1839708815u32,
    Platform_InitializeWithAccessToken = 896085803u32,
    RichPresence_Clear = 1471632051u32,
    RichPresence_GetDestinations = 1483681044u32,
    RichPresence_GetNextDestinationArrayPage = 1731624773u32,
    RichPresence_Set = 1007973641u32,
    Unknown = 0u32,
    UserAgeCategory_Get = 567009472u32,
    UserAgeCategory_Report = 776853718u32,
    UserDataStore_PrivateDeleteEntryByKey = 1552510782u32,
    UserDataStore_PrivateGetEntries = 1821016616u32,
    UserDataStore_PrivateGetEntryByKey = 470188825u32,
    UserDataStore_PrivateWriteEntry = 1104315019u32,
    UserDataStore_PublicDeleteEntryByKey = 500557307u32,
    UserDataStore_PublicGetEntries = 377310146u32,
    UserDataStore_PublicGetEntryByKey = 425486022u32,
    UserDataStore_PublicWriteEntry = 875973130u32,
    User_Get = 1808768583u32,
    User_GetAccessToken = 111696574u32,
    User_GetBlockedUsers = 2099254614u32,
    User_GetLoggedInUser = 1131361373u32,
    User_GetLoggedInUserFriends = 1484532365u32,
    User_GetNextBlockedUserArrayPage = 2083192267u32,
    User_GetNextUserArrayPage = 645723971u32,
    User_GetNextUserCapabilityArrayPage = 587854745u32,
    User_GetOrgScopedID = 418426907u32,
    User_GetSdkAccounts = 1733454467u32,
    User_GetUserProof = 578880643u32,
    User_LaunchBlockFlow = 1876305192u32,
    User_LaunchFriendRequestFlow = 151303576u32,
    User_LaunchUnblockFlow = 346172055u32,
    Voip_GetMicrophoneAvailability = 1951195973u32,
    Voip_SetSystemVoipSuppressed = 1161808298u32,
}
#[cfg(feature = "Oculus+Platform+Message+MessageType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Oculus::Platform::Message_MessageType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Oculus.Platform";
    const CLASS_NAME: &'static str = "MessageType";
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
#[cfg(feature = "Oculus+Platform+Message+MessageType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Oculus::Platform::Message_MessageType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Oculus+Platform+Message+MessageType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Oculus::Platform::Message_MessageType {
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
#[cfg(feature = "Oculus+Platform+Message+MessageType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Oculus::Platform::Message_MessageType {
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
#[cfg(feature = "Oculus+Platform+Message+MessageType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Oculus::Platform::Message_MessageType {
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
