#[cfg(feature = "BeatSaber+AvatarCore+ConnectedPlayerAvatarVisualDataProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectedPlayerAvatarVisualDataProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _connectedPlayer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::IConnectedPlayer,
    >,
}
#[cfg(feature = "BeatSaber+AvatarCore+ConnectedPlayerAvatarVisualDataProvider")]
unsafe impl quest_hook::libil2cpp::Type
for crate::BeatSaber::AvatarCore::ConnectedPlayerAvatarVisualDataProvider {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "BeatSaber.AvatarCore";
    const CLASS_NAME: &'static str = "ConnectedPlayerAvatarVisualDataProvider";
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
#[cfg(feature = "BeatSaber+AvatarCore+ConnectedPlayerAvatarVisualDataProvider")]
impl std::ops::Deref
for crate::BeatSaber::AvatarCore::ConnectedPlayerAvatarVisualDataProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+ConnectedPlayerAvatarVisualDataProvider")]
impl std::ops::DerefMut
for crate::BeatSaber::AvatarCore::ConnectedPlayerAvatarVisualDataProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+ConnectedPlayerAvatarVisualDataProvider")]
impl crate::BeatSaber::AvatarCore::ConnectedPlayerAvatarVisualDataProvider {
    pub fn New(
        connectedPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (connectedPlayer))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        connectedPlayer: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IConnectedPlayer,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (connectedPlayer))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_avatarsData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::MultiplayerAvatarsData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::MultiplayerAvatarsData = __cordl_object
            .invoke("get_avatarsData", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+ConnectedPlayerAvatarVisualDataProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::AvatarCore::ConnectedPlayerAvatarVisualDataProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+ConnectedPlayerAvatarVisualDataProvider")]
impl AsRef<crate::BeatSaber::AvatarCore::IAvatarVisualDataProvider>
for crate::BeatSaber::AvatarCore::ConnectedPlayerAvatarVisualDataProvider {
    fn as_ref(&self) -> &crate::BeatSaber::AvatarCore::IAvatarVisualDataProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+ConnectedPlayerAvatarVisualDataProvider")]
impl AsMut<crate::BeatSaber::AvatarCore::IAvatarVisualDataProvider>
for crate::BeatSaber::AvatarCore::ConnectedPlayerAvatarVisualDataProvider {
    fn as_mut(
        &mut self,
    ) -> &mut crate::BeatSaber::AvatarCore::IAvatarVisualDataProvider {
        unsafe { std::mem::transmute(self) }
    }
}
