#[cfg(feature = "MultiplayerUnavailableReasonMethods")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerUnavailableReasonMethods {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "MultiplayerUnavailableReasonMethods")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerUnavailableReasonMethods => ""
    ."MultiplayerUnavailableReasonMethods"
);
#[cfg(feature = "MultiplayerUnavailableReasonMethods")]
impl std::ops::Deref for crate::GlobalNamespace::MultiplayerUnavailableReasonMethods {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerUnavailableReasonMethods")]
impl std::ops::DerefMut for crate::GlobalNamespace::MultiplayerUnavailableReasonMethods {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerUnavailableReasonMethods")]
impl crate::GlobalNamespace::MultiplayerUnavailableReasonMethods {
    pub const kMultiplayerUnavailableMaintenanceMode: &'static str = "MULTIPLAYER_UNAVAILABLE_MAINTENANCE_MODE";
    pub const kMultiplayerUnavailableServerOffline: &'static str = "MULTIPLAYER_UNAVAILABLE_SERVER_OFFLINE";
    pub const kMultiplayerUnavailableTryAgain: &'static str = "MULTIPLAYER_UNAVAILABLE_TRY_AGAIN";
    pub const kMultiplayerUnavailableUpdateRequired: &'static str = "MULTIPLAYER_UNAVAILABLE_UPDATE_REQUIRED";
    pub fn ErrorCode(
        multiplayerUnavailableReason: crate::GlobalNamespace::MultiplayerUnavailableReason,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ErrorCode", (multiplayerUnavailableReason))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLocalizedMessage(
        data: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MultiplayerStatusData>,
        language: crate::BGLib::Polyglot::Language,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLocalizedMessage", (data, language))?;
        Ok(__cordl_ret.into())
    }
    pub fn LocalizedKey(
        multiplayerUnavailableReason: crate::GlobalNamespace::MultiplayerUnavailableReason,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LocalizedKey", (multiplayerUnavailableReason))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetMultiplayerUnavailableReason(
        data: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MultiplayerStatusData>,
        reason: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::MultiplayerUnavailableReason,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetMultiplayerUnavailableReason", (data, reason))?;
        Ok(__cordl_ret.into())
    }
    pub fn VersionLessThan(
        currentVersion: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        minVersion: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("VersionLessThan", (currentVersion, minVersion))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MultiplayerUnavailableReasonMethods")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerUnavailableReasonMethods {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
