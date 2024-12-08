#[cfg(feature = "BeatSaber+AvatarCore+ConnectedPlayerAvatarVisualDataProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct ConnectedPlayerAvatarVisualDataProvider {
    __cordl_parent: crate::System::Object,
    pub _connectedPlayer: *mut IConnectedPlayer,
}
#[cfg(feature = "BeatSaber+AvatarCore+ConnectedPlayerAvatarVisualDataProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::AvatarCore::ConnectedPlayerAvatarVisualDataProvider =>
    "BeatSaber.AvatarCore"."ConnectedPlayerAvatarVisualDataProvider"
);
#[cfg(feature = "BeatSaber+AvatarCore+ConnectedPlayerAvatarVisualDataProvider")]
impl std::ops::Deref
for crate::BeatSaber::AvatarCore::ConnectedPlayerAvatarVisualDataProvider {
    type Target = crate::System::Object;
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
    pub fn get_avatarsData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<MultiplayerAvatarsData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: MultiplayerAvatarsData = __cordl_object
            .invoke("get_avatarsData", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        connectedPlayer: *mut IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (connectedPlayer))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        connectedPlayer: *mut IConnectedPlayer,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (connectedPlayer))?;
        Ok(__cordl_object)
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
