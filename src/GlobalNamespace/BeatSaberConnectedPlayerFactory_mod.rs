#[cfg(feature = "cordl_class_BeatSaberConnectedPlayerFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatSaberConnectedPlayerFactory {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_BeatSaberConnectedPlayerFactory")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::BeatSaberConnectedPlayerFactory
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatSaberConnectedPlayerFactory";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "BeatSaberConnectedPlayerFactory")]
impl std::ops::Deref for crate::GlobalNamespace::BeatSaberConnectedPlayerFactory {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaberConnectedPlayerFactory")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatSaberConnectedPlayerFactory {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaberConnectedPlayerFactory")]
impl crate::GlobalNamespace::BeatSaberConnectedPlayerFactory {
    pub fn CreateDirectlyConnectedPlayer(
        &mut self,
        manager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_3<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatSaberConnectedPlayer>,
                crate::GlobalNamespace::BeatSaberPlayerIdentityPacketData,
            >,
        >,
        connectionId: u8,
        connection: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IConnection>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatSaberConnectedPlayer>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
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
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IConnection,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatSaberConnectedPlayer,
                        >,
                        3usize,
                    >("CreateDirectlyConnectedPlayer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateDirectlyConnectedPlayer", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatSaberConnectedPlayer,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (manager, connectionId, connection))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateLocalPlayer(
        &mut self,
        manager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_3<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatSaberConnectedPlayer>,
                crate::GlobalNamespace::BeatSaberPlayerIdentityPacketData,
            >,
        >,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        userName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        isConnectionOwner: bool,
        publicEncryptionKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        random: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        compatibilityVersion: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatSaberConnectedPlayer>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
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
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
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
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatSaberConnectedPlayer,
                        >,
                        7usize,
                    >("CreateLocalPlayer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateLocalPlayer", 7usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatSaberConnectedPlayer,
        > = unsafe {
            cordl_method_info.invoke_unchecked(
                self,
                (
                    manager,
                    userId,
                    userName,
                    isConnectionOwner,
                    publicEncryptionKey,
                    random,
                    compatibilityVersion,
                ),
            )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CreateRemoteConnectedPlayer(
        &mut self,
        manager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager_3<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatSaberConnectedPlayer>,
                crate::GlobalNamespace::BeatSaberPlayerIdentityPacketData,
            >,
        >,
        connectionId: u8,
        packet: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::PlayerConnectedPacket>,
        parent: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatSaberConnectedPlayer>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatSaberConnectedPlayer>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
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
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::PlayerConnectedPacket,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatSaberConnectedPlayer,
                            >,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BeatSaberConnectedPlayer,
                        >,
                        4usize,
                    >("CreateRemoteConnectedPlayer")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CreateRemoteConnectedPlayer", 4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatSaberConnectedPlayer,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (manager, connectionId, packet, parent))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BeatSaberConnectedPlayerFactory")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BeatSaberConnectedPlayerFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatSaberConnectedPlayerFactory")]
impl
    AsRef<
        crate::GlobalNamespace::IConnectedPlayerFactory_3<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatSaberConnectedPlayer>,
            crate::GlobalNamespace::BeatSaberPlayerIdentityPacketData,
        >,
    > for crate::GlobalNamespace::BeatSaberConnectedPlayerFactory
{
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::IConnectedPlayerFactory_3<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatSaberConnectedPlayer>,
        crate::GlobalNamespace::BeatSaberPlayerIdentityPacketData,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaberConnectedPlayerFactory")]
impl
    AsMut<
        crate::GlobalNamespace::IConnectedPlayerFactory_3<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatSaberConnectedPlayer>,
            crate::GlobalNamespace::BeatSaberPlayerIdentityPacketData,
        >,
    > for crate::GlobalNamespace::BeatSaberConnectedPlayerFactory
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::IConnectedPlayerFactory_3<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatSaberConnectedPlayer>,
        crate::GlobalNamespace::BeatSaberPlayerIdentityPacketData,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
