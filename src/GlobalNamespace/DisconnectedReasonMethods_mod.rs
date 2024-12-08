#[cfg(feature = "DisconnectedReasonMethods")]
#[repr(C)]
#[derive(Debug)]
pub struct DisconnectedReasonMethods {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "DisconnectedReasonMethods")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::DisconnectedReasonMethods => ""
    ."DisconnectedReasonMethods"
);
#[cfg(feature = "DisconnectedReasonMethods")]
impl std::ops::Deref for crate::GlobalNamespace::DisconnectedReasonMethods {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DisconnectedReasonMethods")]
impl std::ops::DerefMut for crate::GlobalNamespace::DisconnectedReasonMethods {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DisconnectedReasonMethods")]
impl crate::GlobalNamespace::DisconnectedReasonMethods {
    pub const kDisconnectedKicked: &'static str = "DISCONNECTED_KICKED";
    pub const kDisconnectedMasterServerUnreachable: &'static str = "DISCONNECTED_MASTER_SERVER_UNREACHABLE";
    pub const kDisconnectedServerAtCapacity: &'static str = "DISCONNECTED_SERVER_AT_CAPACITY";
    pub const kDisconnectedServerConnectionClosed: &'static str = "DISCONNECTED_SERVER_SHUT_DOWN";
    pub const kDisconnectedServerTerminated: &'static str = "DISCONNECTED_SERVER_SHUT_DOWN";
    pub const kDisconnectedTimeout: &'static str = "DISCONNECTED_TIMEOUT";
    pub const kDisconnectedUnknown: &'static str = "DISCONNECTED_UNKNOWN";
    pub const kDisconnectedUserInitiated: &'static str = "DISCONNECTED_USER_INITIATED";
}
#[cfg(feature = "DisconnectedReasonMethods")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::DisconnectedReasonMethods {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
