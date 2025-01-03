#[cfg(feature = "Oculus+Platform+GroupPresence")]
#[repr(C)]
#[derive(Debug)]
pub struct GroupPresence {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Oculus+Platform+GroupPresence")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::GroupPresence =>
    "Oculus.Platform"."GroupPresence"
);
#[cfg(feature = "Oculus+Platform+GroupPresence")]
impl std::ops::Deref for crate::Oculus::Platform::GroupPresence {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+GroupPresence")]
impl std::ops::DerefMut for crate::Oculus::Platform::GroupPresence {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+GroupPresence")]
impl crate::Oculus::Platform::GroupPresence {
    pub fn Clear() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Clear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInvitableUsers(
        options: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::InviteOptions>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::UserList,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::UserList,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInvitableUsers", (options))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNextApplicationInviteListPage(
        list: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::ApplicationInviteList,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::ApplicationInviteList,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::ApplicationInviteList,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetNextApplicationInviteListPage", (list))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSentInvites() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::ApplicationInviteList,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::ApplicationInviteList,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetSentInvites", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LaunchInvitePanel(
        options: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::InviteOptions>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::InvitePanelResultInfo,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::InvitePanelResultInfo,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LaunchInvitePanel", (options))?;
        Ok(__cordl_ret.into())
    }
    pub fn LaunchMultiplayerErrorDialog(
        options: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::MultiplayerErrorOptions,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LaunchMultiplayerErrorDialog", (options))?;
        Ok(__cordl_ret.into())
    }
    pub fn LaunchRejoinDialog(
        lobby_session_id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        match_session_id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        destination_api_name: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::RejoinDialogResult,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::RejoinDialogResult,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "LaunchRejoinDialog",
                (lobby_session_id, match_session_id, destination_api_name),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn LaunchRosterPanel(
        options: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::RosterOptions>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LaunchRosterPanel", (options))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendInvites(
        userIDs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::SendInvitesResult,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                *mut crate::Oculus::Platform::Models::SendInvitesResult,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SendInvites", (userIDs))?;
        Ok(__cordl_ret.into())
    }
    pub fn Set(
        groupPresenceOptions: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::GroupPresenceOptions,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Set", (groupPresenceOptions))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDeeplinkMessageOverride(
        deeplink_message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetDeeplinkMessageOverride", (deeplink_message))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetDestination(
        api_name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetDestination", (api_name))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetInvitationsSentNotificationCallback(
        callback: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Message_1_Callback<
                *mut crate::Oculus::Platform::Models::LaunchInvitePanelFlowResult,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetInvitationsSentNotificationCallback", (callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetIsJoinable(
        is_joinable: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetIsJoinable", (is_joinable))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetJoinIntentReceivedNotificationCallback(
        callback: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Message_1_Callback<
                *mut crate::Oculus::Platform::Models::GroupPresenceJoinIntent,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetJoinIntentReceivedNotificationCallback", (callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLeaveIntentReceivedNotificationCallback(
        callback: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Message_1_Callback<
                *mut crate::Oculus::Platform::Models::GroupPresenceLeaveIntent,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetLeaveIntentReceivedNotificationCallback", (callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLobbySession(
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetLobbySession", (id))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMatchSession(
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetMatchSession", (id))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+GroupPresence")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::GroupPresence {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
