#[cfg(feature = "DisconnectedReasonMethods")]
#[repr(C)]
#[derive(Debug)]
pub struct DisconnectedReasonMethods {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "DisconnectedReasonMethods")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::DisconnectedReasonMethods {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "DisconnectedReasonMethods";
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
#[cfg(feature = "DisconnectedReasonMethods")]
impl std::ops::Deref for crate::GlobalNamespace::DisconnectedReasonMethods {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn ErrorCode(
        disconnectedReason: crate::GlobalNamespace::DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::DisconnectedReasonMethods as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::GlobalNamespace::DisconnectedReason),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ErrorCode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::DisconnectedReasonMethods as
                    quest_hook::libil2cpp::Type > ::class(), "ErrorCode", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (disconnectedReason))? };
        Ok(__cordl_ret.into())
    }
    pub fn LocalizedKey(
        connectionFailedReason: crate::GlobalNamespace::DisconnectedReason,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::DisconnectedReasonMethods as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::GlobalNamespace::DisconnectedReason),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("LocalizedKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::DisconnectedReasonMethods as
                    quest_hook::libil2cpp::Type > ::class(), "LocalizedKey", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (connectionFailedReason))? };
        Ok(__cordl_ret.into())
    }
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
