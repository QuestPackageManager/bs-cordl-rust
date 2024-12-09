#[cfg(feature = "Oculus+Platform+Message")]
#[repr(C)]
#[derive(Debug)]
pub struct Message {
    __cordl_parent: crate::System::Object,
    pub _cordl_type: crate::Oculus::Platform::Message_MessageType,
    pub requestID: u64,
    pub error: *mut crate::Oculus::Platform::Models::Error,
}
#[cfg(feature = "Oculus+Platform+Message")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::Message => "Oculus.Platform"
    ."Message"
);
#[cfg(feature = "Oculus+Platform+Message")]
impl std::ops::Deref for crate::Oculus::Platform::Message {
    type Target = crate::System::Object;
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
        Ok(__cordl_ret)
    }
    pub fn GetAbuseReportRecording(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::AbuseReportRecording,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::AbuseReportRecording = __cordl_object
            .invoke("GetAbuseReportRecording", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetAchievementDefinitions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::AchievementDefinitionList,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::AchievementDefinitionList = __cordl_object
            .invoke("GetAchievementDefinitions", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetAchievementProgressList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::AchievementProgressList,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::AchievementProgressList = __cordl_object
            .invoke("GetAchievementProgressList", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetAchievementUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::AchievementUpdate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::AchievementUpdate = __cordl_object
            .invoke("GetAchievementUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetAppDownloadProgressResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::AppDownloadProgressResult,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::AppDownloadProgressResult = __cordl_object
            .invoke("GetAppDownloadProgressResult", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetAppDownloadResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::AppDownloadResult,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::AppDownloadResult = __cordl_object
            .invoke("GetAppDownloadResult", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetApplicationInviteList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::ApplicationInviteList,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::ApplicationInviteList = __cordl_object
            .invoke("GetApplicationInviteList", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetApplicationVersion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::ApplicationVersion,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::ApplicationVersion = __cordl_object
            .invoke("GetApplicationVersion", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetAssetDetails(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::AssetDetails,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::AssetDetails = __cordl_object
            .invoke("GetAssetDetails", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetAssetDetailsList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::AssetDetailsList,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::AssetDetailsList = __cordl_object
            .invoke("GetAssetDetailsList", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetAssetFileDeleteResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::AssetFileDeleteResult,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::AssetFileDeleteResult = __cordl_object
            .invoke("GetAssetFileDeleteResult", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetAssetFileDownloadCancelResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::AssetFileDownloadCancelResult,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::AssetFileDownloadCancelResult = __cordl_object
            .invoke("GetAssetFileDownloadCancelResult", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetAssetFileDownloadResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::AssetFileDownloadResult,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::AssetFileDownloadResult = __cordl_object
            .invoke("GetAssetFileDownloadResult", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetAssetFileDownloadUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::AssetFileDownloadUpdate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::AssetFileDownloadUpdate = __cordl_object
            .invoke("GetAssetFileDownloadUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetAvatarEditorResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::AvatarEditorResult,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::AvatarEditorResult = __cordl_object
            .invoke("GetAvatarEditorResult", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetBlockedUserList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::BlockedUserList,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::BlockedUserList = __cordl_object
            .invoke("GetBlockedUserList", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetChallenge(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Oculus::Platform::Models::Challenge> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::Challenge = __cordl_object
            .invoke("GetChallenge", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetChallengeEntryList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::ChallengeEntryList,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::ChallengeEntryList = __cordl_object
            .invoke("GetChallengeEntryList", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetChallengeList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::ChallengeList,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::ChallengeList = __cordl_object
            .invoke("GetChallengeList", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetDataStore(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut crate::System::String,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::Dictionary_2<
            *mut crate::System::String,
            *mut crate::System::String,
        > = __cordl_object.invoke("GetDataStore", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetDestinationList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::DestinationList,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::DestinationList = __cordl_object
            .invoke("GetDestinationList", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetError(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Oculus::Platform::Models::Error> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::Error = __cordl_object
            .invoke("GetError", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetGroupPresenceJoinIntent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::GroupPresenceJoinIntent,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::GroupPresenceJoinIntent = __cordl_object
            .invoke("GetGroupPresenceJoinIntent", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetGroupPresenceLeaveIntent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::GroupPresenceLeaveIntent,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::GroupPresenceLeaveIntent = __cordl_object
            .invoke("GetGroupPresenceLeaveIntent", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetHttpTransferUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::HttpTransferUpdate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::HttpTransferUpdate = __cordl_object
            .invoke("GetHttpTransferUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetInstalledApplicationList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::InstalledApplicationList,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::InstalledApplicationList = __cordl_object
            .invoke("GetInstalledApplicationList", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetInvitePanelResultInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::InvitePanelResultInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::InvitePanelResultInfo = __cordl_object
            .invoke("GetInvitePanelResultInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetLaunchBlockFlowResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::LaunchBlockFlowResult,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::LaunchBlockFlowResult = __cordl_object
            .invoke("GetLaunchBlockFlowResult", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetLaunchFriendRequestFlowResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::LaunchFriendRequestFlowResult,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::LaunchFriendRequestFlowResult = __cordl_object
            .invoke("GetLaunchFriendRequestFlowResult", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetLaunchInvitePanelFlowResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::LaunchInvitePanelFlowResult,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::LaunchInvitePanelFlowResult = __cordl_object
            .invoke("GetLaunchInvitePanelFlowResult", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetLaunchReportFlowResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::LaunchReportFlowResult,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::LaunchReportFlowResult = __cordl_object
            .invoke("GetLaunchReportFlowResult", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetLaunchUnblockFlowResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::LaunchUnblockFlowResult,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::LaunchUnblockFlowResult = __cordl_object
            .invoke("GetLaunchUnblockFlowResult", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetLeaderboardDidUpdate(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("GetLeaderboardDidUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetLeaderboardEntryList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::LeaderboardEntryList,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::LeaderboardEntryList = __cordl_object
            .invoke("GetLeaderboardEntryList", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetLeaderboardList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::LeaderboardList,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::LeaderboardList = __cordl_object
            .invoke("GetLeaderboardList", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetLinkedAccountList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::LinkedAccountList,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::LinkedAccountList = __cordl_object
            .invoke("GetLinkedAccountList", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetLivestreamingApplicationStatus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::LivestreamingApplicationStatus,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::LivestreamingApplicationStatus = __cordl_object
            .invoke("GetLivestreamingApplicationStatus", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetLivestreamingStartResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::LivestreamingStartResult,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::LivestreamingStartResult = __cordl_object
            .invoke("GetLivestreamingStartResult", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetLivestreamingStatus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::LivestreamingStatus,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::LivestreamingStatus = __cordl_object
            .invoke("GetLivestreamingStatus", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetLivestreamingVideoStats(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::LivestreamingVideoStats,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::LivestreamingVideoStats = __cordl_object
            .invoke("GetLivestreamingVideoStats", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetMicrophoneAvailabilityState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::MicrophoneAvailabilityState,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::MicrophoneAvailabilityState = __cordl_object
            .invoke("GetMicrophoneAvailabilityState", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetNetSyncConnection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::NetSyncConnection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::NetSyncConnection = __cordl_object
            .invoke("GetNetSyncConnection", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetNetSyncSessionList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::NetSyncSessionList,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::NetSyncSessionList = __cordl_object
            .invoke("GetNetSyncSessionList", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetNetSyncSessionsChangedNotification(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::NetSyncSessionsChangedNotification,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::NetSyncSessionsChangedNotification = __cordl_object
            .invoke("GetNetSyncSessionsChangedNotification", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetNetSyncSetSessionPropertyResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::NetSyncSetSessionPropertyResult,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::NetSyncSetSessionPropertyResult = __cordl_object
            .invoke("GetNetSyncSetSessionPropertyResult", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetNetSyncVoipAttenuationValueList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::NetSyncVoipAttenuationValueList,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::NetSyncVoipAttenuationValueList = __cordl_object
            .invoke("GetNetSyncVoipAttenuationValueList", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetOrgScopedID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::OrgScopedID,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::OrgScopedID = __cordl_object
            .invoke("GetOrgScopedID", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetParty(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Oculus::Platform::Models::Party> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::Party = __cordl_object
            .invoke("GetParty", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetPartyID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Oculus::Platform::Models::PartyID> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::PartyID = __cordl_object
            .invoke("GetPartyID", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetPartyUpdateNotification(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::PartyUpdateNotification,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::PartyUpdateNotification = __cordl_object
            .invoke("GetPartyUpdateNotification", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetPidList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Oculus::Platform::Models::PidList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::PidList = __cordl_object
            .invoke("GetPidList", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetPlatformInitialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::PlatformInitialize,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::PlatformInitialize = __cordl_object
            .invoke("GetPlatformInitialize", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetProductList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::ProductList,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::ProductList = __cordl_object
            .invoke("GetProductList", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetPurchase(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Oculus::Platform::Models::Purchase> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::Purchase = __cordl_object
            .invoke("GetPurchase", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetPurchaseList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::PurchaseList,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::PurchaseList = __cordl_object
            .invoke("GetPurchaseList", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetRejoinDialogResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::RejoinDialogResult,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::RejoinDialogResult = __cordl_object
            .invoke("GetRejoinDialogResult", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSdkAccountList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::SdkAccountList,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::SdkAccountList = __cordl_object
            .invoke("GetSdkAccountList", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSendInvitesResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::SendInvitesResult,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::SendInvitesResult = __cordl_object
            .invoke("GetSendInvitesResult", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetShareMediaResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::ShareMediaResult,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::ShareMediaResult = __cordl_object
            .invoke("GetShareMediaResult", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetString", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetSystemVoipState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::SystemVoipState,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::SystemVoipState = __cordl_object
            .invoke("GetSystemVoipState", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetUser(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Oculus::Platform::Models::User> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::User = __cordl_object
            .invoke("GetUser", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetUserAccountAgeCategory(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::UserAccountAgeCategory,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::UserAccountAgeCategory = __cordl_object
            .invoke("GetUserAccountAgeCategory", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetUserCapabilityList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::UserCapabilityList,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::UserCapabilityList = __cordl_object
            .invoke("GetUserCapabilityList", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetUserDataStoreUpdateResponse(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::UserDataStoreUpdateResponse,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::UserDataStoreUpdateResponse = __cordl_object
            .invoke("GetUserDataStoreUpdateResponse", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetUserList(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Oculus::Platform::Models::UserList> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::UserList = __cordl_object
            .invoke("GetUserList", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetUserProof(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Oculus::Platform::Models::UserProof> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::UserProof = __cordl_object
            .invoke("GetUserProof", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetUserReportID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Oculus::Platform::Models::UserReportID,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Models::UserReportID = __cordl_object
            .invoke("GetUserReportID", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        c_message: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (c_message))?;
        Ok(__cordl_object)
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
        Ok(__cordl_ret)
    }
    pub fn get_IsError(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsError", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_RequestID(&mut self) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object.invoke("get_RequestID", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::Message_MessageType> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Oculus::Platform::Message_MessageType = __cordl_object
            .invoke("get_Type", ())?;
        Ok(__cordl_ret)
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::Message_Callback =>
    "Oculus.Platform"."Message/Callback"
);
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
        message: *mut crate::Oculus::Platform::Message,
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (message, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        message: *mut crate::Oculus::Platform::Message,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (message))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
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
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Oculus::Platform::Message_ExtraMessageTypesHandler => "Oculus.Platform"
    ."Message/ExtraMessageTypesHandler"
);
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
        callback: *mut crate::System::AsyncCallback,
        object: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IAsyncResult> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IAsyncResult = __cordl_object
            .invoke("BeginInvoke", (messageHandle, message_type, callback, object))?;
        Ok(__cordl_ret)
    }
    pub fn EndInvoke(
        &mut self,
        result: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Oculus::Platform::Message> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Message = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        messageHandle: crate::System::IntPtr,
        message_type: crate::Oculus::Platform::Message_MessageType,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Oculus::Platform::Message> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Oculus::Platform::Message = __cordl_object
            .invoke("Invoke", (messageHandle, message_type))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Message_MessageType {
    AbuseReport_ReportRequestHandled = 2398914288u32,
    Achievements_AddCount = 828565443u32,
    Achievements_AddFields = 690072276u32,
    Achievements_GetAllDefinitions = 2370163651u32,
    Achievements_GetAllProgress = 2682134000u32,
    Achievements_GetDefinitionsByName = 2432810224u32,
    Achievements_GetNextAchievementDefinitionArrayPage = 2110936560u32,
    Achievements_GetNextAchievementProgressArrayPage = 1122445296u32,
    Achievements_GetProgressByName = 2976065237u32,
    Achievements_Unlock = 1019993584u32,
    ApplicationLifecycle_GetRegisteredPIDs = 1657791940u32,
    ApplicationLifecycle_GetSessionKey = 2941853168u32,
    ApplicationLifecycle_RegisterSessionKey = 3064985840u32,
    Application_CancelAppDownload = 543219440u32,
    Application_CheckAppDownloadProgress = 883500272u32,
    Application_GetVersion = 1728712432u32,
    Application_InstallAppUpdateAndRelaunch = 2238415060u32,
    Application_LaunchOtherApp = 3805411568u32,
    Application_StartAppDownload = 4227886832u32,
    AssetFile_Delete = 1568179952u32,
    AssetFile_DeleteById = 3905770224u32,
    AssetFile_DeleteByName = 180473840u32,
    AssetFile_Download = 3315549393u32,
    AssetFile_DownloadById = 9016048u32,
    AssetFile_DownloadByName = 919534320u32,
    AssetFile_DownloadCancel = 3352496840u32,
    AssetFile_DownloadCancelById = 1704269040u32,
    AssetFile_DownloadCancelByName = 1793915632u32,
    AssetFile_GetList = 4235162864u32,
    AssetFile_Status = 1613747138u32,
    AssetFile_StatusById = 2505914608u32,
    AssetFile_StatusByName = 3487191280u32,
    Avatar_LaunchAvatarEditor = 1407316421u32,
    Challenges_Create = 1507213808u32,
    Challenges_DeclineInvite = 2390147312u32,
    Challenges_Delete = 1216727792u32,
    Challenges_Get = 1481569264u32,
    Challenges_GetEntries = 1605638866u32,
    Challenges_GetEntriesAfterRank = 2132445640u32,
    Challenges_GetEntriesByIds = 1695145200u32,
    Challenges_GetList = 641947376u32,
    Challenges_GetNextChallenges = 2090972912u32,
    Challenges_GetNextEntries = 1285605104u32,
    Challenges_GetPreviousChallenges = 218412238u32,
    Challenges_GetPreviousEntries = 3372511472u32,
    Challenges_Join = 612395504u32,
    Challenges_Leave = 1628890608u32,
    Challenges_UpdateInfo = 1623094737u32,
    DeviceApplicationIntegrity_GetIntegrityToken = 1907088112u32,
    Entitlement_GetIsViewerEntitled = 2975362008u32,
    GroupPresence_Clear = 2862400496u32,
    GroupPresence_GetInvitableUsers = 1271132656u32,
    GroupPresence_GetNextApplicationInviteArrayPage = 4072732868u32,
    GroupPresence_GetSentInvites = 2970232520u32,
    GroupPresence_LaunchInvitePanel = 2681183951u32,
    GroupPresence_LaunchMultiplayerErrorDialog = 1437541616u32,
    GroupPresence_LaunchRejoinDialog = 1862498261u32,
    GroupPresence_LaunchRosterPanel = 1921549040u32,
    GroupPresence_SendInvites = 1691601869u32,
    GroupPresence_Set = 1599874288u32,
    GroupPresence_SetDeeplinkMessageOverride = 450825712u32,
    GroupPresence_SetDestination = 1529252592u32,
    GroupPresence_SetIsJoinable = 2400212464u32,
    GroupPresence_SetLobbySession = 4283809520u32,
    GroupPresence_SetMatchSession = 1283766512u32,
    IAP_ConsumePurchase = 3648175071u32,
    IAP_GetNextProductArrayPage = 2940918235u32,
    IAP_GetNextPurchaseArrayPage = 1460311536u32,
    IAP_GetProductsBySKU = 2596992496u32,
    IAP_GetViewerPurchases = 260315632u32,
    IAP_GetViewerPurchasesDurableCache = 1503538160u32,
    IAP_LaunchCheckoutFlow = 2601324016u32,
    LanguagePack_GetCurrent = 3589312735u32,
    LanguagePack_SetCurrent = 1337712880u32,
    Leaderboard_Get = 3561945328u32,
    Leaderboard_GetEntries = 3007794416u32,
    Leaderboard_GetEntriesAfterRank = 4018878424u32,
    Leaderboard_GetEntriesByIds = 1618738416u32,
    Leaderboard_GetNextEntries = 545053168u32,
    Leaderboard_GetNextLeaderboardArrayPage = 4134968304u32,
    Leaderboard_GetPreviousEntries = 31113456u32,
    Leaderboard_WriteEntry = 4274552785u32,
    Leaderboard_WriteEntryWithSupplementaryMetric = 3331521264u32,
    Media_ShareToFacebook = 4018856896u32,
    Notification_AbuseReport_ReportButtonPressed = 1194290416u32,
    Notification_ApplicationLifecycle_LaunchIntentChanged = 2739712964u32,
    Notification_AssetFile_DownloadUpdate = 3708603888u32,
    Notification_GroupPresence_InvitationsSent = 2592388848u32,
    Notification_GroupPresence_JoinIntentReceived = 948565744u32,
    Notification_GroupPresence_LeaveIntentReceived = 938089968u32,
    Notification_HTTP_Transfer = 3563991024u32,
    Notification_Livestreaming_StatusChange = 1197043440u32,
    Notification_MarkAsRead = 1918493680u32,
    Notification_NetSync_ConnectionStatusChanged = 3397661895u32,
    Notification_NetSync_SessionsChanged = 2122266352u32,
    Notification_Party_PartyUpdate = 2995392989u32,
    Notification_Voip_MicrophoneAvailabilityStateUpdate = 550197232u32,
    Notification_Voip_SystemVoipState = 3528762864u32,
    Notification_Vrcamera_GetDataChannelMessageUpdate = 3841146096u32,
    Notification_Vrcamera_GetSurfaceUpdate = 4061168880u32,
    Party_GetCurrent = 2469880048u32,
    Platform_InitializeAndroidAsynchronous = 3020411866u32,
    Platform_InitializeStandaloneOculus = 4174253296u32,
    Platform_InitializeWindowsAsynchronous = 2814021616u32,
    Platform_InitializeWithAccessToken = 1764699120u32,
    RichPresence_Clear = 3075650544u32,
    RichPresence_GetDestinations = 1865225456u32,
    RichPresence_GetNextDestinationArrayPage = 914310640u32,
    RichPresence_Set = 343214576u32,
    Unknown = 4236701696u32,
    UserAgeCategory_Get = 3420504304u32,
    UserAgeCategory_Report = 1306056432u32,
    UserDataStore_PrivateDeleteEntryByKey = 2305769200u32,
    UserDataStore_PrivateGetEntries = 2323785968u32,
    UserDataStore_PrivateGetEntryByKey = 428017372u32,
    UserDataStore_PrivateWriteEntry = 3531770864u32,
    UserDataStore_PublicDeleteEntryByKey = 4226143709u32,
    UserDataStore_PublicGetEntries = 3259727318u32,
    UserDataStore_PublicGetEntryByKey = 3328597209u32,
    UserDataStore_PublicWriteEntry = 910822128u32,
    User_Get = 3483256816u32,
    User_GetAccessToken = 3193612486u32,
    User_GetBlockedUsers = 538269424u32,
    User_GetLoggedInUser = 1865702896u32,
    User_GetLoggedInUserFriends = 2083163632u32,
    User_GetNextBlockedUserArrayPage = 721275888u32,
    User_GetNextUserArrayPage = 2096579568u32,
    User_GetNextUserCapabilityArrayPage = 166959600u32,
    User_GetOrgScopedID = 464580824u32,
    User_GetSdkAccounts = 1382712304u32,
    User_GetUserProof = 2164556784u32,
    User_LaunchBlockFlow = 3592759536u32,
    User_LaunchFriendRequestFlow = 2562000073u32,
    User_LaunchUnblockFlow = 2536153812u32,
    Voip_GetMicrophoneAvailability = 1289962992u32,
    Voip_SetSystemVoipSuppressed = 1070181104u32,
}
#[cfg(feature = "Oculus+Platform+Message+MessageType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::Message_MessageType =>
    "Oculus.Platform"."Message/MessageType"
);
