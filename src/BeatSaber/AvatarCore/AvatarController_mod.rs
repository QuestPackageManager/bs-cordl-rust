#[cfg(feature = "BeatSaber+AvatarCore+AvatarController")]
#[repr(C)]
#[derive(Debug)]
pub struct AvatarController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _parentingTransform: *mut crate::UnityEngine::Transform,
    pub _container: *mut crate::Zenject::DiContainer,
    pub _avatarSystemCollection: *mut crate::BeatSaber::AvatarCore::AvatarSystemCollection,
    pub _avatarDisplayContext: crate::BeatSaber::AvatarCore::AvatarDisplayContext,
    pub _visualDataProvider: *mut crate::BeatSaber::AvatarCore::IAvatarVisualDataProvider,
    pub _poseDataProvider: *mut crate::BeatSaber::AvatarCore::IAvatarPoseDataProvider,
    pub _optionalDataProvider: *mut crate::BeatSaber::AvatarCore::IOptionalAvatarDataProvider,
    pub _avatar: *mut crate::BeatSaber::AvatarCore::Avatar,
}
#[cfg(feature = "BeatSaber+AvatarCore+AvatarController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::AvatarCore::AvatarController =>
    "BeatSaber.AvatarCore"."AvatarController"
);
#[cfg(feature = "BeatSaber+AvatarCore+AvatarController")]
impl std::ops::Deref for crate::BeatSaber::AvatarCore::AvatarController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+AvatarController")]
impl std::ops::DerefMut for crate::BeatSaber::AvatarCore::AvatarController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+AvatarController")]
impl crate::BeatSaber::AvatarCore::AvatarController {
    #[cfg(feature = "BeatSaber+AvatarCore+AvatarController+_LoadAndDisplayAvatar_d__11")]
    pub type _LoadAndDisplayAvatar_d__11 = crate::BeatSaber::AvatarCore::AvatarController__LoadAndDisplayAvatar_d__11;
    pub fn LoadAndDisplayAvatar(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LoadAndDisplayAvatar", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_avatar(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::BeatSaber::AvatarCore::Avatar> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::BeatSaber::AvatarCore::Avatar = __cordl_object
            .invoke("get_avatar", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+AvatarController")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::AvatarCore::AvatarController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
