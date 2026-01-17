#[cfg(feature = "cordl_class_BeatSaberDisconnectedPlayer")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct BeatSaberDisconnectedPlayer {
    __cordl_parent: crate::GlobalNamespace::DisconnectedPlayer,
    pub _multiplayerAvatarsData_k__BackingField: crate::GlobalNamespace::MultiplayerAvatarsData,
}
#[cfg(feature = "cordl_class_BeatSaberDisconnectedPlayer")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::BeatSaberDisconnectedPlayer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "BeatSaberDisconnectedPlayer";
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
#[cfg(feature = "BeatSaberDisconnectedPlayer")]
impl std::ops::Deref for crate::GlobalNamespace::BeatSaberDisconnectedPlayer {
    type Target = crate::GlobalNamespace::DisconnectedPlayer;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaberDisconnectedPlayer")]
impl std::ops::DerefMut for crate::GlobalNamespace::BeatSaberDisconnectedPlayer {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaberDisconnectedPlayer")]
impl crate::GlobalNamespace::BeatSaberDisconnectedPlayer {
    pub fn New(
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        userName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sortIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (userId, userName, sortIndex))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        userId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        userName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        sortIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        i32,
                    ), quest_hook::libil2cpp::Void, 3usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (userId, userName, sortIndex))? };
        Ok(__cordl_ret.into())
    }
    pub fn get_multiplayerAvatarsData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::MultiplayerAvatarsData> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), crate::GlobalNamespace::MultiplayerAvatarsData, 0usize>(
                        "get_multiplayerAvatarsData",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_multiplayerAvatarsData",
                            0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::MultiplayerAvatarsData =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_BeatSaberDisconnectedPlayer")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BeatSaberDisconnectedPlayer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatSaberDisconnectedPlayer")]
impl AsRef<crate::GlobalNamespace::IBeatSaberConnectedPlayer>
    for crate::GlobalNamespace::BeatSaberDisconnectedPlayer
{
    fn as_ref(&self) -> &crate::GlobalNamespace::IBeatSaberConnectedPlayer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaberDisconnectedPlayer")]
impl AsMut<crate::GlobalNamespace::IBeatSaberConnectedPlayer>
    for crate::GlobalNamespace::BeatSaberDisconnectedPlayer
{
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IBeatSaberConnectedPlayer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaberDisconnectedPlayer")]
impl AsRef<crate::GlobalNamespace::IConnectedPlayer>
    for crate::GlobalNamespace::BeatSaberDisconnectedPlayer
{
    fn as_ref(&self) -> &crate::GlobalNamespace::IConnectedPlayer {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaberDisconnectedPlayer")]
impl AsMut<crate::GlobalNamespace::IConnectedPlayer>
    for crate::GlobalNamespace::BeatSaberDisconnectedPlayer
{
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IConnectedPlayer {
        unsafe { std::mem::transmute(self) }
    }
}
