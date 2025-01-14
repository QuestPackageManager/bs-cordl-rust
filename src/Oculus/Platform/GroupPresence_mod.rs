#[cfg(feature = "Oculus+Platform+GroupPresence")]
#[repr(C)]
#[derive(Debug)]
pub struct GroupPresence {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Oculus+Platform+GroupPresence")]
unsafe impl quest_hook::libil2cpp::Type for crate::Oculus::Platform::GroupPresence {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Oculus.Platform";
    const CLASS_NAME: &'static str = "GroupPresence";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
                0usize,
            >("Clear")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Clear", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request> = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetInvitableUsers(
        options: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::InviteOptions>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::UserList>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::Oculus::Platform::InviteOptions>),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::UserList,
                        >,
                    >,
                >,
                1usize,
            >("GetInvitableUsers")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetInvitableUsers", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Models::UserList>,
            >,
        > = unsafe { method.invoke_unchecked((), (options)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetNextApplicationInviteListPage(
        list: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Models::ApplicationInviteList,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::ApplicationInviteList,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::ApplicationInviteList,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::ApplicationInviteList,
                        >,
                    >,
                >,
                1usize,
            >("GetNextApplicationInviteListPage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetNextApplicationInviteListPage", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::ApplicationInviteList,
                >,
            >,
        > = unsafe { method.invoke_unchecked((), (list)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetSentInvites() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::ApplicationInviteList,
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
                            crate::Oculus::Platform::Models::ApplicationInviteList,
                        >,
                    >,
                >,
                0usize,
            >("GetSentInvites")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetSentInvites", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::ApplicationInviteList,
                >,
            >,
        > = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn LaunchInvitePanel(
        options: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::InviteOptions>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::InvitePanelResultInfo,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::Oculus::Platform::InviteOptions>),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::InvitePanelResultInfo,
                        >,
                    >,
                >,
                1usize,
            >("LaunchInvitePanel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LaunchInvitePanel", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::InvitePanelResultInfo,
                >,
            >,
        > = unsafe { method.invoke_unchecked((), (options)) };
        Ok(__cordl_ret.into())
    }
    pub fn LaunchMultiplayerErrorDialog(
        options: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::MultiplayerErrorOptions,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::MultiplayerErrorOptions,
                >),
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
                1usize,
            >("LaunchMultiplayerErrorDialog")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LaunchMultiplayerErrorDialog", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request> = unsafe {
            method.invoke_unchecked((), (options))
        };
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
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::RejoinDialogResult,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::RejoinDialogResult,
                        >,
                    >,
                >,
                3usize,
            >("LaunchRejoinDialog")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LaunchRejoinDialog", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::RejoinDialogResult,
                >,
            >,
        > = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (lobby_session_id, match_session_id, destination_api_name),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn LaunchRosterPanel(
        options: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::RosterOptions>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::Oculus::Platform::RosterOptions>),
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
                1usize,
            >("LaunchRosterPanel")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LaunchRosterPanel", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request> = unsafe {
            method.invoke_unchecked((), (options))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SendInvites(
        userIDs: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::SendInvitesResult,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::SendInvitesResult,
                        >,
                    >,
                >,
                1usize,
            >("SendInvites")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SendInvites", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::SendInvitesResult,
                >,
            >,
        > = unsafe { method.invoke_unchecked((), (userIDs)) };
        Ok(__cordl_ret.into())
    }
    pub fn Set(
        groupPresenceOptions: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::GroupPresenceOptions,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::GroupPresenceOptions,
                >),
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
                1usize,
            >("Set")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Set", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request> = unsafe {
            method.invoke_unchecked((), (groupPresenceOptions))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetDeeplinkMessageOverride(
        deeplink_message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
                1usize,
            >("SetDeeplinkMessageOverride")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetDeeplinkMessageOverride", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request> = unsafe {
            method.invoke_unchecked((), (deeplink_message))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetDestination(
        api_name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
                1usize,
            >("SetDestination")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetDestination", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request> = unsafe {
            method.invoke_unchecked((), (api_name))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetInvitationsSentNotificationCallback(
        callback: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Message_1_Callback<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::LaunchInvitePanelFlowResult,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Message_1_Callback<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::LaunchInvitePanelFlowResult,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetInvitationsSentNotificationCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetInvitationsSentNotificationCallback", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (callback))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetIsJoinable(
        is_joinable: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (bool),
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
                1usize,
            >("SetIsJoinable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetIsJoinable", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request> = unsafe {
            method.invoke_unchecked((), (is_joinable))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetJoinIntentReceivedNotificationCallback(
        callback: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Message_1_Callback<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::GroupPresenceJoinIntent,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Message_1_Callback<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::GroupPresenceJoinIntent,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetJoinIntentReceivedNotificationCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetJoinIntentReceivedNotificationCallback", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (callback))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetLeaveIntentReceivedNotificationCallback(
        callback: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Message_1_Callback<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::GroupPresenceLeaveIntent,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Message_1_Callback<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::GroupPresenceLeaveIntent,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetLeaveIntentReceivedNotificationCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetLeaveIntentReceivedNotificationCallback", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (callback))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetLobbySession(
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
                1usize,
            >("SetLobbySession")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetLobbySession", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request> = unsafe {
            method.invoke_unchecked((), (id))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetMatchSession(
        id: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request>,
                1usize,
            >("SetMatchSession")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetMatchSession", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::Request> = unsafe {
            method.invoke_unchecked((), (id))
        };
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
