#[cfg(feature = "MultiplayerUnavailableReasonMethods")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerUnavailableReasonMethods {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "MultiplayerUnavailableReasonMethods")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::MultiplayerUnavailableReasonMethods {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "MultiplayerUnavailableReasonMethods";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MultiplayerUnavailableReasonMethods as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::GlobalNamespace::MultiplayerUnavailableReason),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ErrorCode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::MultiplayerUnavailableReasonMethods as
                    quest_hook::libil2cpp::Type > ::class(), "ErrorCode", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (multiplayerUnavailableReason))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetLocalizedMessage(
        data: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MultiplayerStatusData>,
        language: crate::BGLib::Polyglot::Language,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MultiplayerUnavailableReasonMethods as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::MultiplayerStatusData,
                    >,
                    crate::BGLib::Polyglot::Language,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("GetLocalizedMessage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::MultiplayerUnavailableReasonMethods as
                    quest_hook::libil2cpp::Type > ::class(), "GetLocalizedMessage",
                    2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (data, language))? };
        Ok(__cordl_ret.into())
    }
    pub fn LocalizedKey(
        multiplayerUnavailableReason: crate::GlobalNamespace::MultiplayerUnavailableReason,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MultiplayerUnavailableReasonMethods as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::GlobalNamespace::MultiplayerUnavailableReason),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("LocalizedKey")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::MultiplayerUnavailableReasonMethods as
                    quest_hook::libil2cpp::Type > ::class(), "LocalizedKey", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked((), (multiplayerUnavailableReason))? };
        Ok(__cordl_ret.into())
    }
    pub fn TryGetMultiplayerUnavailableReason(
        data: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::MultiplayerStatusData>,
        reason: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::MultiplayerUnavailableReason,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MultiplayerUnavailableReasonMethods as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::MultiplayerStatusData,
                    >,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::GlobalNamespace::MultiplayerUnavailableReason,
                    >,
                ),
                bool,
                2usize,
            >("TryGetMultiplayerUnavailableReason")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::MultiplayerUnavailableReasonMethods as
                    quest_hook::libil2cpp::Type > ::class(),
                    "TryGetMultiplayerUnavailableReason", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (data, reason))? };
        Ok(__cordl_ret.into())
    }
    pub fn VersionLessThan(
        currentVersion: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        minVersion: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::MultiplayerUnavailableReasonMethods as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                bool,
                2usize,
            >("VersionLessThan")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::MultiplayerUnavailableReasonMethods as
                    quest_hook::libil2cpp::Type > ::class(), "VersionLessThan", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (currentVersion, minVersion))?
        };
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
