#[cfg(feature = "cordl_class_IBeatSaberMultiplayerSessionManager")]
#[derive(Debug)]
#[repr(C)]
pub struct IBeatSaberMultiplayerSessionManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_IBeatSaberMultiplayerSessionManager")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::IBeatSaberMultiplayerSessionManager
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "IBeatSaberMultiplayerSessionManager";
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
#[cfg(feature = "IBeatSaberMultiplayerSessionManager")]
impl std::ops::Deref for crate::GlobalNamespace::IBeatSaberMultiplayerSessionManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IBeatSaberMultiplayerSessionManager")]
impl std::ops::DerefMut for crate::GlobalNamespace::IBeatSaberMultiplayerSessionManager {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IBeatSaberMultiplayerSessionManager")]
impl crate::GlobalNamespace::IBeatSaberMultiplayerSessionManager {
    pub fn add_playerAvatarChangedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Action_1<
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IBeatSaberConnectedPlayer,
                            >,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "add_playerAvatarChangedEvent"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "add_playerAvatarChangedEvent",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(object_param: *mut quest_hook::libil2cpp::Il2CppObject) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn remove_playerAvatarChangedEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(quest_hook::libil2cpp::Gc<
                        crate::System::Action_1<
                            quest_hook::libil2cpp::Gc<
                                crate::GlobalNamespace::IBeatSaberConnectedPlayer,
                            >,
                        >,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "remove_playerAvatarChangedEvent"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "remove_playerAvatarChangedEvent",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_IBeatSaberMultiplayerSessionManager")]
impl quest_hook::libil2cpp::ObjectType
    for crate::GlobalNamespace::IBeatSaberMultiplayerSessionManager
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "IBeatSaberMultiplayerSessionManager")]
impl
    AsRef<
        crate::GlobalNamespace::IMultiplayerSessionManager_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
        >,
    > for crate::GlobalNamespace::IBeatSaberMultiplayerSessionManager
{
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::IMultiplayerSessionManager_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IBeatSaberMultiplayerSessionManager")]
impl
    AsMut<
        crate::GlobalNamespace::IMultiplayerSessionManager_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
        >,
    > for crate::GlobalNamespace::IBeatSaberMultiplayerSessionManager
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::IMultiplayerSessionManager_1<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IBeatSaberMultiplayerSessionManager")]
impl
    AsRef<
        crate::GlobalNamespace::IMultiplayerSessionManager_4<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatSaberConnectedPlayerManager>,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatSaberConnectedPlayer>,
            crate::GlobalNamespace::BeatSaberPlayerIdentityPacketData,
        >,
    > for crate::GlobalNamespace::IBeatSaberMultiplayerSessionManager
{
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::IMultiplayerSessionManager_4<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatSaberConnectedPlayerManager>,
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatSaberConnectedPlayer>,
        crate::GlobalNamespace::BeatSaberPlayerIdentityPacketData,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IBeatSaberMultiplayerSessionManager")]
impl
    AsMut<
        crate::GlobalNamespace::IMultiplayerSessionManager_4<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatSaberConnectedPlayerManager>,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatSaberConnectedPlayer>,
            crate::GlobalNamespace::BeatSaberPlayerIdentityPacketData,
        >,
    > for crate::GlobalNamespace::IBeatSaberMultiplayerSessionManager
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::IMultiplayerSessionManager_4<
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatSaberConnectedPlayerManager>,
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BeatSaberConnectedPlayer>,
        crate::GlobalNamespace::BeatSaberPlayerIdentityPacketData,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IBeatSaberMultiplayerSessionManager")]
impl
    AsRef<
        crate::GlobalNamespace::IMultiplayerSessionMessageProcessor_2<
            crate::GlobalNamespace::NetworkMessageType,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
        >,
    > for crate::GlobalNamespace::IBeatSaberMultiplayerSessionManager
{
    fn as_ref(
        &self,
    ) -> &crate::GlobalNamespace::IMultiplayerSessionMessageProcessor_2<
        crate::GlobalNamespace::NetworkMessageType,
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "IBeatSaberMultiplayerSessionManager")]
impl
    AsMut<
        crate::GlobalNamespace::IMultiplayerSessionMessageProcessor_2<
            crate::GlobalNamespace::NetworkMessageType,
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
        >,
    > for crate::GlobalNamespace::IBeatSaberMultiplayerSessionManager
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::IMultiplayerSessionMessageProcessor_2<
        crate::GlobalNamespace::NetworkMessageType,
        quest_hook::libil2cpp::Gc<crate::GlobalNamespace::IBeatSaberConnectedPlayer>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
