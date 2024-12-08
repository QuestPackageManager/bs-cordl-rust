#[cfg(feature = "MultiplayerUnavailableReasonMethods")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerUnavailableReasonMethods {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "MultiplayerUnavailableReasonMethods")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for MultiplayerUnavailableReasonMethods => ""
    ."MultiplayerUnavailableReasonMethods"
);
#[cfg(feature = "MultiplayerUnavailableReasonMethods")]
impl std::ops::Deref for MultiplayerUnavailableReasonMethods {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerUnavailableReasonMethods")]
impl std::ops::DerefMut for MultiplayerUnavailableReasonMethods {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerUnavailableReasonMethods")]
impl MultiplayerUnavailableReasonMethods {
    pub const kMultiplayerUnavailableMaintenanceMode: &'static str = "MULTIPLAYER_UNAVAILABLE_MAINTENANCE_MODE";
    pub const kMultiplayerUnavailableServerOffline: &'static str = "MULTIPLAYER_UNAVAILABLE_SERVER_OFFLINE";
    pub const kMultiplayerUnavailableTryAgain: &'static str = "MULTIPLAYER_UNAVAILABLE_TRY_AGAIN";
    pub const kMultiplayerUnavailableUpdateRequired: &'static str = "MULTIPLAYER_UNAVAILABLE_UPDATE_REQUIRED";
}
#[cfg(feature = "MultiplayerUnavailableReasonMethods")]
impl quest_hook::libil2cpp::ObjectType for MultiplayerUnavailableReasonMethods {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
