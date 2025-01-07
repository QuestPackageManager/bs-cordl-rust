#[cfg(feature = "ConnectionFailedReasonMethods")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectionFailedReasonMethods {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ConnectionFailedReasonMethods")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ConnectionFailedReasonMethods {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ConnectionFailedReasonMethods";
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
#[cfg(feature = "ConnectionFailedReasonMethods")]
impl std::ops::Deref for crate::GlobalNamespace::ConnectionFailedReasonMethods {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectionFailedReasonMethods")]
impl std::ops::DerefMut for crate::GlobalNamespace::ConnectionFailedReasonMethods {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ConnectionFailedReasonMethods")]
impl crate::GlobalNamespace::ConnectionFailedReasonMethods {
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
    pub fn ErrorCode(
        connectionFailedReason: crate::GlobalNamespace::ConnectionFailedReason,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ErrorCode", (connectionFailedReason))?;
        Ok(__cordl_ret.into())
    }
    pub fn LocalizedKey(
        connectionFailedReason: crate::GlobalNamespace::ConnectionFailedReason,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LocalizedKey", (connectionFailedReason))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ConnectionFailedReasonMethods")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ConnectionFailedReasonMethods {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
