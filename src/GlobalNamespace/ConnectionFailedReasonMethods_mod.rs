#[cfg(feature = "ConnectionFailedReasonMethods")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectionFailedReasonMethods {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "ConnectionFailedReasonMethods")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for ConnectionFailedReasonMethods => ""
    ."ConnectionFailedReasonMethods"
);
#[cfg(feature = "ConnectionFailedReasonMethods")]
impl std::ops::Deref for ConnectionFailedReasonMethods {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectionFailedReasonMethods")]
impl std::ops::DerefMut for ConnectionFailedReasonMethods {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectionFailedReasonMethods")]
impl ConnectionFailedReasonMethods {
    pub const kConnectionFailedConnectionCanceled: &'static str = "SERVER_CONNECTION_FAILED_TRY_AGAIN";
    pub const kConnectionFailedFailedToFindMatch: &'static str = "CONNECTION_FAILED_FAILED_TO_FIND_MATCH";
    pub const kConnectionFailedInvalidPassword: &'static str = "CONNECTION_FAILED_INVALID_PASSWORD";
    pub const kConnectionFailedMasterServerCertificateValidationFailed: &'static str = "CONNECTION_FAILED_NETWORK_NOT_CONNECTED";
    pub const kConnectionFailedMasterServerNotAuthenticated: &'static str = "SERVER_CONNECTION_FAILED_TRY_AGAIN";
    pub const kConnectionFailedMasterServerUnreachable: &'static str = "SERVER_CONNECTION_FAILED_TRY_AGAIN";
    pub const kConnectionFailedNetworkNotConnected: &'static str = "CONNECTION_FAILED_NETWORK_NOT_CONNECTED";
    pub const kConnectionFailedServerAtCapacity: &'static str = "CONNECTION_FAILED_SERVER_AT_CAPACITY";
    pub const kConnectionFailedServerDoesNotExist: &'static str = "CONNECTION_FAILED_SERVER_DOES_NOT_EXIST";
    pub const kConnectionFailedServerIsTerminating: &'static str = "CONNECTION_FAILED_SERVER_DOES_NOT_EXIST";
    pub const kConnectionFailedServerUnreachable: &'static str = "SERVER_CONNECTION_FAILED_TRY_AGAIN";
    pub const kConnectionFailedTimeout: &'static str = "CONNECTION_FAILED_TIMEOUT";
    pub const kConnectionFailedUnknown: &'static str = "SERVER_CONNECTION_FAILED_TRY_AGAIN";
    pub const kConnectionFailedVersionMismatch: &'static str = "CONNECTION_FAILED_VERSION_MISMATCH";
}
#[cfg(feature = "ConnectionFailedReasonMethods")]
impl quest_hook::libil2cpp::ObjectType for ConnectionFailedReasonMethods {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
