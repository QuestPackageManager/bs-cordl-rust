#[cfg(feature = "PlayerLobbyPermissionConfigurationNetSerializable")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerLobbyPermissionConfigurationNetSerializable {
    __cordl_parent: crate::GlobalNamespace::PoolableSerializable,
    pub _userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _isServerOwner: bool,
    pub _hasRecommendBeatmapsPermission: bool,
    pub _hasRecommendGameplayModifiersPermission: bool,
    pub _hasKickVotePermission: bool,
    pub _hasInvitePermission: bool,
}
#[cfg(feature = "PlayerLobbyPermissionConfigurationNetSerializable")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::PlayerLobbyPermissionConfigurationNetSerializable {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "PlayerLobbyPermissionConfigurationNetSerializable";
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
#[cfg(feature = "PlayerLobbyPermissionConfigurationNetSerializable")]
impl std::ops::Deref
for crate::GlobalNamespace::PlayerLobbyPermissionConfigurationNetSerializable {
    type Target = crate::GlobalNamespace::PoolableSerializable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerLobbyPermissionConfigurationNetSerializable")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PlayerLobbyPermissionConfigurationNetSerializable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PlayerLobbyPermissionConfigurationNetSerializable")]
impl crate::GlobalNamespace::PlayerLobbyPermissionConfigurationNetSerializable {
    pub fn Deserialize(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PlayerLobbyPermissionConfigurationNetSerializable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Deserialize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::GlobalNamespace::PlayerLobbyPermissionConfigurationNetSerializable
                    as quest_hook::libil2cpp::Type > ::class(), "Deserialize", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (reader))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isServerOwner: bool,
        hasRecommendBeatmapsPermission: bool,
        hasRecommendGameplayModifiersPermission: bool,
        hasKickVotePermission: bool,
        hasInvitePermission: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerLobbyPermissionConfigurationNetSerializable,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PlayerLobbyPermissionConfigurationNetSerializable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    bool,
                    bool,
                    bool,
                    bool,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::PlayerLobbyPermissionConfigurationNetSerializable,
                >,
                6usize,
            >("Init")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::GlobalNamespace::PlayerLobbyPermissionConfigurationNetSerializable
                    as quest_hook::libil2cpp::Type > ::class(), "Init", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerLobbyPermissionConfigurationNetSerializable,
        > = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        userId,
                        isServerOwner,
                        hasRecommendBeatmapsPermission,
                        hasRecommendGameplayModifiersPermission,
                        hasKickVotePermission,
                        hasInvitePermission,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Obtain() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerLobbyPermissionConfigurationNetSerializable,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PlayerLobbyPermissionConfigurationNetSerializable as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::PlayerLobbyPermissionConfigurationNetSerializable,
                >,
                0usize,
            >("Obtain")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::GlobalNamespace::PlayerLobbyPermissionConfigurationNetSerializable
                    as quest_hook::libil2cpp::Type > ::class(), "Obtain", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerLobbyPermissionConfigurationNetSerializable,
        > = unsafe { method.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Serialize(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PlayerLobbyPermissionConfigurationNetSerializable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Serialize")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::GlobalNamespace::PlayerLobbyPermissionConfigurationNetSerializable
                    as quest_hook::libil2cpp::Type > ::class(), "Serialize", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (writer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PlayerLobbyPermissionConfigurationNetSerializable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::GlobalNamespace::PlayerLobbyPermissionConfigurationNetSerializable
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_hasInvitePermission(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PlayerLobbyPermissionConfigurationNetSerializable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_hasInvitePermission")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::GlobalNamespace::PlayerLobbyPermissionConfigurationNetSerializable
                    as quest_hook::libil2cpp::Type > ::class(),
                    "get_hasInvitePermission", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_hasKickVotePermission(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PlayerLobbyPermissionConfigurationNetSerializable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_hasKickVotePermission")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::GlobalNamespace::PlayerLobbyPermissionConfigurationNetSerializable
                    as quest_hook::libil2cpp::Type > ::class(),
                    "get_hasKickVotePermission", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_hasRecommendBeatmapsPermission(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PlayerLobbyPermissionConfigurationNetSerializable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_hasRecommendBeatmapsPermission")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::GlobalNamespace::PlayerLobbyPermissionConfigurationNetSerializable
                    as quest_hook::libil2cpp::Type > ::class(),
                    "get_hasRecommendBeatmapsPermission", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_hasRecommendGameplayModifiersPermission(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PlayerLobbyPermissionConfigurationNetSerializable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                bool,
                0usize,
            >("get_hasRecommendGameplayModifiersPermission")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::GlobalNamespace::PlayerLobbyPermissionConfigurationNetSerializable
                    as quest_hook::libil2cpp::Type > ::class(),
                    "get_hasRecommendGameplayModifiersPermission", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_isServerOwner(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PlayerLobbyPermissionConfigurationNetSerializable as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_isServerOwner")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::GlobalNamespace::PlayerLobbyPermissionConfigurationNetSerializable
                    as quest_hook::libil2cpp::Type > ::class(), "get_isServerOwner",
                    0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_userId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::PlayerLobbyPermissionConfigurationNetSerializable as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_userId")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::GlobalNamespace::PlayerLobbyPermissionConfigurationNetSerializable
                    as quest_hook::libil2cpp::Type > ::class(), "get_userId", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PlayerLobbyPermissionConfigurationNetSerializable")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PlayerLobbyPermissionConfigurationNetSerializable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
