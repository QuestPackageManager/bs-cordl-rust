#[cfg(feature = "Oculus+Platform+Voip")]
#[repr(C)]
#[derive(Debug)]
pub struct Voip {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Oculus+Platform+Voip")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Oculus::Platform::Voip => "Oculus.Platform"
    ."Voip"
);
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
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Accept", (userID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetIsConnectionUsingDtx(
        peerID: u64,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::VoipDtxState> {
        let __cordl_ret: crate::Oculus::Platform::VoipDtxState = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetIsConnectionUsingDtx", (peerID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLocalBitrate(
        peerID: u64,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::VoipBitrate> {
        let __cordl_ret: crate::Oculus::Platform::VoipBitrate = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLocalBitrate", (peerID))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::MicrophoneAvailabilityState,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMicrophoneAvailability", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRemoteBitrate(
        peerID: u64,
    ) -> quest_hook::libil2cpp::Result<crate::Oculus::Platform::VoipBitrate> {
        let __cordl_ret: crate::Oculus::Platform::VoipBitrate = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRemoteBitrate", (peerID))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSystemVoipMicrophoneMuted() -> quest_hook::libil2cpp::Result<
        crate::Oculus::Platform::VoipMuteState,
    > {
        let __cordl_ret: crate::Oculus::Platform::VoipMuteState = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSystemVoipMicrophoneMuted", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSystemVoipStatus() -> quest_hook::libil2cpp::Result<
        crate::Oculus::Platform::SystemVoipStatus,
    > {
        let __cordl_ret: crate::Oculus::Platform::SystemVoipStatus = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSystemVoipStatus", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMicrophoneAvailabilityStateUpdateNotificationCallback(
        callback: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Message_1_Callback<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "SetMicrophoneAvailabilityStateUpdateNotificationCallback",
                (callback),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMicrophoneFilterCallback(
        callback: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::CAPI_FilterCallback>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetMicrophoneFilterCallback", (callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMicrophoneMuted(
        state: crate::Oculus::Platform::VoipMuteState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetMicrophoneMuted", (state))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetNewConnectionOptions(
        voipOptions: quest_hook::libil2cpp::Gc<crate::Oculus::Platform::VoipOptions>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetNewConnectionOptions", (voipOptions))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetSystemVoipStateNotificationCallback", (callback))?;
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Oculus::Platform::Request_1<
                quest_hook::libil2cpp::Gc<
                    crate::Oculus::Platform::Models::SystemVoipState,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetSystemVoipSuppressed", (suppressed))?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        userID: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Start", (userID))?;
        Ok(__cordl_ret.into())
    }
    pub fn Stop(
        userID: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Stop", (userID))?;
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
