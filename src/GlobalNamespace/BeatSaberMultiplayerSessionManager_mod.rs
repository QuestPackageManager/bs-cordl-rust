#[cfg(feature = "cordl_class_BeatSaberMultiplayerSessionManager")]
#[repr(C)]
#[derive(Debug)]
pub struct BeatSaberMultiplayerSessionManager {
    __cordl_parent: crate::GlobalNamespace::MultiplayerSessionManager_5<
        crate::GlobalNamespace::NetworkMessageType,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatSaberConnectedPlayerManager,
        >,
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatSaberConnectedPlayer>,
        crate::GlobalNamespace::BeatSaberPlayerIdentityPacketData,
    >,
    pub playerAvatarChangedEvent: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
        >,
    >,
}
#[cfg(feature = "cordl_class_BeatSaberMultiplayerSessionManager")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::BeatSaberMultiplayerSessionManager {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatSaberMultiplayerSessionManager";
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
#[cfg(feature = "BeatSaberMultiplayerSessionManager")]
impl std::ops::Deref for crate::GlobalNamespace::BeatSaberMultiplayerSessionManager {
    type Target = crate::GlobalNamespace::MultiplayerSessionManager_5<
        crate::GlobalNamespace::NetworkMessageType,
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatSaberConnectedPlayerManager,
        >,
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatSaberConnectedPlayer>,
        crate::GlobalNamespace::BeatSaberPlayerIdentityPacketData,
    >;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaberMultiplayerSessionManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatSaberMultiplayerSessionManager {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaberMultiplayerSessionManager")]
impl crate::GlobalNamespace::BeatSaberMultiplayerSessionManager {
    pub fn HandlePlayerAvatarChanged(
        &mut self,
        player: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IBeatSaberConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::IBeatSaberConnectedPlayer,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("HandlePlayerAvatarChanged")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "HandlePlayerAvatarChanged", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (player))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IMultiplayerSessionManager_BeatSaberConnectedPlayerManager_IBeatSaberConnectedPlayer_BeatSaberConnectedPlayer_BeatSaberPlayerIdentityPacketData__StartSession(
        &mut self,
        connectedPlayerManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatSaberConnectedPlayerManager,
        >,
        multiplayerSessionInitializer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IMultiplayerSessionManager_1_IMultiplayerSessionInitializer<
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
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::BeatSaberConnectedPlayerManager,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IMultiplayerSessionManager_1_IMultiplayerSessionInitializer<
                                    quest_hook::libil2cpp::Gc<
                                        crate::GlobalNamespace::IBeatSaberConnectedPlayer,
                                    >,
                                >,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(
                        "IMultiplayerSessionManager<BeatSaberConnectedPlayerManager,IBeatSaberConnectedPlayer,BeatSaberConnectedPlayer,BeatSaberPlayerIdentityPacketData>.StartSession",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IMultiplayerSessionManager<BeatSaberConnectedPlayerManager,IBeatSaberConnectedPlayer,BeatSaberConnectedPlayer,BeatSaberPlayerIdentityPacketData>.StartSession",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info
                .invoke_unchecked(
                    self,
                    (connectedPlayerManager, multiplayerSessionInitializer),
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
    pub fn RegisterGameSpecificEventHandlers(
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
                    >("RegisterGameSpecificEventHandlers")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "RegisterGameSpecificEventHandlers", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterGameSpecificEventHandlers(
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
                    >("UnregisterGameSpecificEventHandlers")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "UnregisterGameSpecificEventHandlers", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
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
#[cfg(feature = "cordl_class_BeatSaberMultiplayerSessionManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BeatSaberMultiplayerSessionManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatSaberMultiplayerSessionManager")]
impl AsRef<crate::GlobalNamespace::IBeatSaberMultiplayerSessionManager>
for crate::GlobalNamespace::BeatSaberMultiplayerSessionManager {
    fn as_ref(&self) -> &crate::GlobalNamespace::IBeatSaberMultiplayerSessionManager {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaberMultiplayerSessionManager")]
impl AsMut<crate::GlobalNamespace::IBeatSaberMultiplayerSessionManager>
for crate::GlobalNamespace::BeatSaberMultiplayerSessionManager {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::IBeatSaberMultiplayerSessionManager {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaberMultiplayerSessionManager")]
impl AsRef<
    crate::GlobalNamespace::IMultiplayerSessionManager_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
    >,
> for crate::GlobalNamespace::BeatSaberMultiplayerSessionManager {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::IMultiplayerSessionManager_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaberMultiplayerSessionManager")]
impl AsMut<
    crate::GlobalNamespace::IMultiplayerSessionManager_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
    >,
> for crate::GlobalNamespace::BeatSaberMultiplayerSessionManager {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::IMultiplayerSessionManager_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaberMultiplayerSessionManager")]
impl AsRef<
    crate::GlobalNamespace::IMultiplayerSessionManager_4<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatSaberConnectedPlayerManager,
        >,
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatSaberConnectedPlayer>,
        crate::GlobalNamespace::BeatSaberPlayerIdentityPacketData,
    >,
> for crate::GlobalNamespace::BeatSaberMultiplayerSessionManager {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::IMultiplayerSessionManager_4<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatSaberConnectedPlayerManager,
        >,
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatSaberConnectedPlayer>,
        crate::GlobalNamespace::BeatSaberPlayerIdentityPacketData,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaberMultiplayerSessionManager")]
impl AsMut<
    crate::GlobalNamespace::IMultiplayerSessionManager_4<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatSaberConnectedPlayerManager,
        >,
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatSaberConnectedPlayer>,
        crate::GlobalNamespace::BeatSaberPlayerIdentityPacketData,
    >,
> for crate::GlobalNamespace::BeatSaberMultiplayerSessionManager {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::IMultiplayerSessionManager_4<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BeatSaberConnectedPlayerManager,
        >,
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatSaberConnectedPlayer>,
        crate::GlobalNamespace::BeatSaberPlayerIdentityPacketData,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaberMultiplayerSessionManager")]
impl AsRef<
    crate::GlobalNamespace::IMultiplayerSessionMessageProcessor_2<
        crate::GlobalNamespace::NetworkMessageType,
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
    >,
> for crate::GlobalNamespace::BeatSaberMultiplayerSessionManager {
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::IMultiplayerSessionMessageProcessor_2<
        crate::GlobalNamespace::NetworkMessageType,
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaberMultiplayerSessionManager")]
impl AsMut<
    crate::GlobalNamespace::IMultiplayerSessionMessageProcessor_2<
        crate::GlobalNamespace::NetworkMessageType,
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
    >,
> for crate::GlobalNamespace::BeatSaberMultiplayerSessionManager {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::IMultiplayerSessionMessageProcessor_2<
        crate::GlobalNamespace::NetworkMessageType,
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
