#[cfg(feature = "BeatSaber+AvatarCore+StaticAvatarVisualDataProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct StaticAvatarVisualDataProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _avatarsData: crate::GlobalNamespace::MultiplayerAvatarsData,
}
#[cfg(feature = "BeatSaber+AvatarCore+StaticAvatarVisualDataProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::BeatSaber::AvatarCore::StaticAvatarVisualDataProvider => "BeatSaber.AvatarCore"
    ."StaticAvatarVisualDataProvider"
);
#[cfg(feature = "BeatSaber+AvatarCore+StaticAvatarVisualDataProvider")]
impl std::ops::Deref for crate::BeatSaber::AvatarCore::StaticAvatarVisualDataProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+StaticAvatarVisualDataProvider")]
impl std::ops::DerefMut
for crate::BeatSaber::AvatarCore::StaticAvatarVisualDataProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+StaticAvatarVisualDataProvider")]
impl crate::BeatSaber::AvatarCore::StaticAvatarVisualDataProvider {
    pub fn New(
        avatarsData: crate::GlobalNamespace::MultiplayerAvatarsData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (avatarsData))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        avatarsData: crate::GlobalNamespace::MultiplayerAvatarsData,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (avatarsData))?;
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
#[cfg(feature = "BeatSaber+AvatarCore+StaticAvatarVisualDataProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::AvatarCore::StaticAvatarVisualDataProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+StaticAvatarVisualDataProvider")]
impl AsRef<crate::BeatSaber::AvatarCore::IAvatarVisualDataProvider>
for crate::BeatSaber::AvatarCore::StaticAvatarVisualDataProvider {
    fn as_ref(&self) -> &crate::BeatSaber::AvatarCore::IAvatarVisualDataProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+StaticAvatarVisualDataProvider")]
impl AsMut<crate::BeatSaber::AvatarCore::IAvatarVisualDataProvider>
for crate::BeatSaber::AvatarCore::StaticAvatarVisualDataProvider {
    fn as_mut(
        &mut self,
    ) -> &mut crate::BeatSaber::AvatarCore::IAvatarVisualDataProvider {
        unsafe { std::mem::transmute(self) }
    }
}