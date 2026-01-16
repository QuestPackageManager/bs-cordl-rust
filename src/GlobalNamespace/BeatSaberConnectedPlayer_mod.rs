#[cfg(feature = "cordl_class_BeatSaberConnectedPlayer")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatSaberConnectedPlayer {
    __cordl_parent: crate::GlobalNamespace::ConnectedPlayer_3<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatSaberConnectedPlayer>,
        crate::GlobalNamespace::BeatSaberPlayerIdentityPacketData,
    >,
    pub _playerAvatars: crate::GlobalNamespace::MultiplayerAvatarsData,
}
#[cfg(feature = "cordl_class_BeatSaberConnectedPlayer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatSaberConnectedPlayer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatSaberConnectedPlayer";
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
#[cfg(feature = "BeatSaberConnectedPlayer")]
impl std::ops::Deref for crate::GlobalNamespace::BeatSaberConnectedPlayer {
    type Target = crate::GlobalNamespace::ConnectedPlayer_3<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatSaberConnectedPlayer>,
        crate::GlobalNamespace::BeatSaberPlayerIdentityPacketData,
    >;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaberConnectedPlayer")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatSaberConnectedPlayer {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaberConnectedPlayer")]
impl crate::GlobalNamespace::BeatSaberConnectedPlayer {
    pub fn GetGameSpecificPlayerIdentityData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::BeatSaberPlayerIdentityPacketData,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::GlobalNamespace::BeatSaberPlayerIdentityPacketData,
                        0usize,
                    >("GetGameSpecificPlayerIdentityData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetGameSpecificPlayerIdentityData", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::BeatSaberPlayerIdentityPacketData = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPlayerAvatarPacket(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerAvatarPacket>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::PlayerAvatarPacket,
                        >,
                        0usize,
                    >("GetPlayerAvatarPacket")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetPlayerAvatarPacket", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PlayerAvatarPacket,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        manager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_3<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::IBeatSaberConnectedPlayer,
                >,
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatSaberConnectedPlayer,
                >,
                crate::GlobalNamespace::BeatSaberPlayerIdentityPacketData,
            >,
        >,
        connectionId: u8,
        remoteConnectionId: u8,
        connection: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
        parent: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatSaberConnectedPlayer,
        >,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        userName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isConnectionOwner: bool,
        isMe: bool,
        publicEncryptionKey: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        random: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        compatibilityVersion: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    manager,
                    connectionId,
                    remoteConnectionId,
                    connection,
                    parent,
                    userId,
                    userName,
                    isConnectionOwner,
                    isMe,
                    publicEncryptionKey,
                    random,
                    compatibilityVersion,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn SetMultiplayerAvatarsData(
        &mut self,
        playerAvatars: crate::GlobalNamespace::MultiplayerAvatarsData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::MultiplayerAvatarsData),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SetMultiplayerAvatarsData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetMultiplayerAvatarsData", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (playerAvatars))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateAvatar(
        &mut self,
        packet: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerAvatarPacket>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::PlayerAvatarPacket,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UpdateAvatar")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UpdateAvatar", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (packet))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateIdentity(
        &mut self,
        identityData: crate::GlobalNamespace::BeatSaberPlayerIdentityPacketData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::GlobalNamespace::BeatSaberPlayerIdentityPacketData),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("UpdateIdentity")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UpdateIdentity", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (identityData))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        manager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_3<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::IBeatSaberConnectedPlayer,
                >,
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BeatSaberConnectedPlayer,
                >,
                crate::GlobalNamespace::BeatSaberPlayerIdentityPacketData,
            >,
        >,
        connectionId: u8,
        remoteConnectionId: u8,
        connection: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
        parent: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatSaberConnectedPlayer,
        >,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        userName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isConnectionOwner: bool,
        isMe: bool,
        publicEncryptionKey: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        random: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        compatibilityVersion: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::ConnectedPlayerManager_3<
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::IBeatSaberConnectedPlayer,
                                    >,
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::BeatSaberConnectedPlayer,
                                    >,
                                    crate::GlobalNamespace::BeatSaberPlayerIdentityPacketData,
                                >,
                            >,
                            u8,
                            u8,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IConnection,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatSaberConnectedPlayer,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            bool,
                            bool,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        12usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            12usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (
                        manager,
                        connectionId,
                        remoteConnectionId,
                        connection,
                        parent,
                        userId,
                        userName,
                        isConnectionOwner,
                        isMe,
                        publicEncryptionKey,
                        random,
                        compatibilityVersion,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_multiplayerAvatarsData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::MultiplayerAvatarsData> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        crate::GlobalNamespace::MultiplayerAvatarsData,
                        0usize,
                    >("get_multiplayerAvatarsData")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_multiplayerAvatarsData", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::MultiplayerAvatarsData = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BeatSaberConnectedPlayer")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatSaberConnectedPlayer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatSaberConnectedPlayer")]
impl AsRef<crate::GlobalNamespace::IBeatSaberConnectedPlayer>
for crate::GlobalNamespace::BeatSaberConnectedPlayer {
    fn as_ref(&self) -> &crate::GlobalNamespace::IBeatSaberConnectedPlayer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaberConnectedPlayer")]
impl AsMut<crate::GlobalNamespace::IBeatSaberConnectedPlayer>
for crate::GlobalNamespace::BeatSaberConnectedPlayer {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IBeatSaberConnectedPlayer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaberConnectedPlayer")]
impl AsRef<crate::GlobalNamespace::IConnectedPlayer>
for crate::GlobalNamespace::BeatSaberConnectedPlayer {
    fn as_ref(&self) -> &crate::GlobalNamespace::IConnectedPlayer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaberConnectedPlayer")]
impl AsMut<crate::GlobalNamespace::IConnectedPlayer>
for crate::GlobalNamespace::BeatSaberConnectedPlayer {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IConnectedPlayer {
        unsafe { std::mem::transmute(self) }
    }
}
