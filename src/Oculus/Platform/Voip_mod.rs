#[cfg(feature = "Oculus+Platform+Voip")]
#[repr(C)]
#[derive(Debug)]
pub struct Voip {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Oculus+Platform+Voip")]
unsafe impl quest_hook::libil2cpp::Type for crate::Oculus::Platform::Voip {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Oculus.Platform";
    const CLASS_NAME: &'static str = "Voip";
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
#[cfg(feature = "Oculus+Platform+Voip")]
impl std::ops::Deref for crate::Oculus::Platform::Voip {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Voip")]
impl std::ops::DerefMut for crate::Oculus::Platform::Voip {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Oculus+Platform+Voip")]
impl crate::Oculus::Platform::Voip {
    pub fn Accept(
        userID: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), quest_hook::libil2cpp::Void, 1usize>("Accept")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Accept", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (userID))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetIsConnectionUsingDtx(
        peerID: u64,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::VoipDtxState> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64),
                crate::Oculus::Platform::VoipDtxState,
                1usize,
            >("GetIsConnectionUsingDtx")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetIsConnectionUsingDtx", 1usize
                )
            });
        let __cordl_ret: crate::Oculus::Platform::VoipDtxState = unsafe {
            method.invoke_unchecked((), (peerID))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetLocalBitrate(
        peerID: u64,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::VoipBitrate> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64),
                crate::Oculus::Platform::VoipBitrate,
                1usize,
            >("GetLocalBitrate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetLocalBitrate", 1usize
                )
            });
        let __cordl_ret: crate::Oculus::Platform::VoipBitrate = unsafe {
            method.invoke_unchecked((), (peerID))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetMicrophoneAvailability() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::MicrophoneAvailabilityState,
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
                            crate::Oculus::Platform::Models::MicrophoneAvailabilityState,
                        >,
                    >,
                >,
                0usize,
            >("GetMicrophoneAvailability")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetMicrophoneAvailability", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::MicrophoneAvailabilityState,
                >,
            >,
        > = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn GetRemoteBitrate(
        peerID: u64,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::VoipBitrate> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64),
                crate::Oculus::Platform::VoipBitrate,
                1usize,
            >("GetRemoteBitrate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetRemoteBitrate", 1usize
                )
            });
        let __cordl_ret: crate::Oculus::Platform::VoipBitrate = unsafe {
            method.invoke_unchecked((), (peerID))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetSystemVoipMicrophoneMuted() -> quest_hook::libil2cpp::Result<
        crate::Oculus::Platform::VoipMuteState,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::Oculus::Platform::VoipMuteState,
                0usize,
            >("GetSystemVoipMicrophoneMuted")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetSystemVoipMicrophoneMuted", 0usize
                )
            });
        let __cordl_ret: crate::Oculus::Platform::VoipMuteState = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetSystemVoipStatus() -> quest_hook::libil2cpp::Result<
        crate::Oculus::Platform::SystemVoipStatus,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::Oculus::Platform::SystemVoipStatus,
                0usize,
            >("GetSystemVoipStatus")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetSystemVoipStatus", 0usize
                )
            });
        let __cordl_ret: crate::Oculus::Platform::SystemVoipStatus = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetMicrophoneAvailabilityStateUpdateNotificationCallback(
        callback: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Message_1_Callback<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Message_1_Callback<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetMicrophoneAvailabilityStateUpdateNotificationCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetMicrophoneAvailabilityStateUpdateNotificationCallback",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (callback))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetMicrophoneFilterCallback(
        callback: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::CAPI_FilterCallback>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::CAPI_FilterCallback,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetMicrophoneFilterCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetMicrophoneFilterCallback", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (callback))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetMicrophoneMuted(
        state: crate::Oculus::Platform::VoipMuteState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Oculus::Platform::VoipMuteState),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetMicrophoneMuted")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetMicrophoneMuted", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (state))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetNewConnectionOptions(
        voipOptions: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::VoipOptions>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::Oculus::Platform::VoipOptions>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetNewConnectionOptions")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetNewConnectionOptions", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (voipOptions))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetSystemVoipStateNotificationCallback(
        callback: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Message_1_Callback<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::SystemVoipState,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Message_1_Callback<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::SystemVoipState,
                        >,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetSystemVoipStateNotificationCallback")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetSystemVoipStateNotificationCallback", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (callback))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetSystemVoipSuppressed(
        suppressed: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::SystemVoipState,
                >,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (bool),
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Request_1<
                        quest_hook::libil2cpp::Gc<
                            crate::Oculus::Platform::Models::SystemVoipState,
                        >,
                    >,
                >,
                1usize,
            >("SetSystemVoipSuppressed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetSystemVoipSuppressed", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::SystemVoipState,
                >,
            >,
        > = unsafe { method.invoke_unchecked((), (suppressed)) };
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        userID: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), quest_hook::libil2cpp::Void, 1usize>("Start")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Start", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (userID))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Stop(
        userID: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), quest_hook::libil2cpp::Void, 1usize>("Stop")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Stop", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (userID))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Oculus+Platform+Voip")]
impl quest_hook::libil2cpp::ObjectType for crate::Oculus::Platform::Voip {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
