#[cfg(feature = "BeatSaber+AvatarCore+IAvatarVisualDataProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct IAvatarVisualDataProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "BeatSaber+AvatarCore+IAvatarVisualDataProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::AvatarCore::IAvatarVisualDataProvider
    => "BeatSaber.AvatarCore"."IAvatarVisualDataProvider"
);
#[cfg(feature = "BeatSaber+AvatarCore+IAvatarVisualDataProvider")]
impl std::ops::Deref for crate::BeatSaber::AvatarCore::IAvatarVisualDataProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+IAvatarVisualDataProvider")]
impl std::ops::DerefMut for crate::BeatSaber::AvatarCore::IAvatarVisualDataProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+IAvatarVisualDataProvider")]
impl crate::BeatSaber::AvatarCore::IAvatarVisualDataProvider {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_avatarsData(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::MultiplayerAvatarsData> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::MultiplayerAvatarsData = __cordl_object
            .invoke("get_avatarsData", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+IAvatarVisualDataProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::AvatarCore::IAvatarVisualDataProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
