#[cfg(feature = "cordl_class_BeatSaberConnectedPlayerManager")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatSaberConnectedPlayerManager {
    __cordl_parent: crate::GlobalNamespace::ConnectedPlayerManager_3<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatSaberConnectedPlayer>,
        crate::GlobalNamespace::BeatSaberPlayerIdentityPacketData,
    >,
    pub playerAvatarChangedEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
        >,
    >,
    pub _localPlayerAvatars: crate::GlobalNamespace::MultiplayerAvatarsData,
    pub _beatSaberMessageSerializer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NetworkPacketSerializer_2<
            crate::GlobalNamespace::BeatSaberConnectedPlayerManager_BeatSaberMessageType,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
        >,
    >,
}
#[cfg(feature = "cordl_class_BeatSaberConnectedPlayerManager")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatSaberConnectedPlayerManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatSaberConnectedPlayerManager";
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
#[cfg(feature = "BeatSaberConnectedPlayerManager")]
impl std::ops::Deref for crate::GlobalNamespace::BeatSaberConnectedPlayerManager {
    type Target = crate::GlobalNamespace::ConnectedPlayerManager_3<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatSaberConnectedPlayer>,
        crate::GlobalNamespace::BeatSaberPlayerIdentityPacketData,
    >;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaberConnectedPlayerManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatSaberConnectedPlayerManager {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaberConnectedPlayerManager")]
impl crate::GlobalNamespace::BeatSaberConnectedPlayerManager {
    #[cfg(feature = "BeatSaberConnectedPlayerManager+BeatSaberMessageType")]
    pub type BeatSaberMessageType = crate::GlobalNamespace::BeatSaberConnectedPlayerManager_BeatSaberMessageType;
    pub fn DisposeGameSpecificResources(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("DisposeGameSpecificResources")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "DisposeGameSpecificResources", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleGameSpecificConnected(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("HandleGameSpecificConnected")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleGameSpecificConnected", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleGameSpecificPlayerAdded(
        &mut self,
        player: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatSaberConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatSaberConnectedPlayer,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("HandleGameSpecificPlayerAdded")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleGameSpecificPlayerAdded", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (player))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandleGameSpecificPlayerIdentityUpdate(
        &mut self,
        identityData: crate::GlobalNamespace::BeatSaberPlayerIdentityPacketData,
        iPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatSaberConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::GlobalNamespace::BeatSaberPlayerIdentityPacketData,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IBeatSaberConnectedPlayer,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("HandleGameSpecificPlayerIdentityUpdate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandleGameSpecificPlayerIdentityUpdate", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (identityData, iPlayer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn HandlePlayerAvatarUpdate(
        &mut self,
        packet: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerAvatarPacket>,
        iPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatSaberConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::PlayerAvatarPacket,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IBeatSaberConnectedPlayer,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("HandlePlayerAvatarUpdate")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandlePlayerAvatarUpdate", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (packet, iPlayer))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New_IConnectionManager_IConnectedPlayerFactory_3_0(
        connectionManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectionManager,
        >,
        connectedPlayerFactory: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayerFactory_3<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::IBeatSaberConnectedPlayer,
                >,
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatSaberConnectedPlayer,
                >,
                crate::GlobalNamespace::BeatSaberPlayerIdentityPacketData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (connectionManager, connectedPlayerFactory))?;
        Ok(__cordl_object.into())
    }
    pub fn New_ITimeProvider_ITaskUtility_IConnectionManager_IConnectedPlayerFactory_3_1(
        timeProvider: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITimeProvider>,
        taskUtility: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITaskUtility>,
        connectionManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectionManager,
        >,
        connectedPlayerFactory: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayerFactory_3<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::IBeatSaberConnectedPlayer,
                >,
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatSaberConnectedPlayer,
                >,
                crate::GlobalNamespace::BeatSaberPlayerIdentityPacketData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (timeProvider, taskUtility, connectionManager, connectedPlayerFactory),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn SetLocalPlayerAvatar(
        &mut self,
        multiplayerAvatarsData: crate::GlobalNamespace::MultiplayerAvatarsData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::MultiplayerAvatarsData),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetLocalPlayerAvatar")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetLocalPlayerAvatar", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (multiplayerAvatarsData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IConnectionManager_IConnectedPlayerFactory_3_0(
        &mut self,
        connectionManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectionManager,
        >,
        connectedPlayerFactory: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayerFactory_3<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::IBeatSaberConnectedPlayer,
                >,
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatSaberConnectedPlayer,
                >,
                crate::GlobalNamespace::BeatSaberPlayerIdentityPacketData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IConnectionManager,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IConnectedPlayerFactory_3<
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::IBeatSaberConnectedPlayer,
                                    >,
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::BeatSaberConnectedPlayer,
                                    >,
                                    crate::GlobalNamespace::BeatSaberPlayerIdentityPacketData,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (connectionManager, connectedPlayerFactory))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ITimeProvider_ITaskUtility_IConnectionManager_IConnectedPlayerFactory_3_1(
        &mut self,
        timeProvider: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITimeProvider>,
        taskUtility: quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITaskUtility>,
        connectionManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectionManager,
        >,
        connectedPlayerFactory: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayerFactory_3<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::IBeatSaberConnectedPlayer,
                >,
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatSaberConnectedPlayer,
                >,
                crate::GlobalNamespace::BeatSaberPlayerIdentityPacketData,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITimeProvider>,
                            quest_hook::libil2cpp::Gc<crate::BGNet::Core::ITaskUtility>,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IConnectionManager,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IConnectedPlayerFactory_3<
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::IBeatSaberConnectedPlayer,
                                    >,
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::BeatSaberConnectedPlayer,
                                    >,
                                    crate::GlobalNamespace::BeatSaberPlayerIdentityPacketData,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        4usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        timeProvider,
                        taskUtility,
                        connectionManager,
                        connectedPlayerFactory,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_playerAvatarChangedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::IBeatSaberConnectedPlayer,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::IBeatSaberConnectedPlayer,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("add_playerAvatarChangedEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "add_playerAvatarChangedEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_playerAvatarChangedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::IBeatSaberConnectedPlayer,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Action_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::GlobalNamespace::IBeatSaberConnectedPlayer,
                                >,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("remove_playerAvatarChangedEvent")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "remove_playerAvatarChangedEvent", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BeatSaberConnectedPlayerManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatSaberConnectedPlayerManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_BeatSaberConnectedPlayerManager+BeatSaberMessageType")]
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BeatSaberConnectedPlayerManager_BeatSaberMessageType {
    #[default]
    PlayerAvatarUpdate = 0u8,
}
#[cfg(feature = "cordl_class_BeatSaberConnectedPlayerManager+BeatSaberMessageType")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatSaberConnectedPlayerManager_BeatSaberMessageType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatSaberConnectedPlayerManager/BeatSaberMessageType";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_BeatSaberConnectedPlayerManager+BeatSaberMessageType")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::BeatSaberConnectedPlayerManager_BeatSaberMessageType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_BeatSaberConnectedPlayerManager+BeatSaberMessageType")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::BeatSaberConnectedPlayerManager_BeatSaberMessageType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_BeatSaberConnectedPlayerManager+BeatSaberMessageType")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::BeatSaberConnectedPlayerManager_BeatSaberMessageType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "cordl_class_BeatSaberConnectedPlayerManager+BeatSaberMessageType")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::BeatSaberConnectedPlayerManager_BeatSaberMessageType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
