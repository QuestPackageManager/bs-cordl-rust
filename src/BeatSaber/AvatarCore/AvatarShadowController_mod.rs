#[cfg(feature = "BeatSaber+AvatarCore+AvatarShadowController")]
#[repr(C)]
#[derive(Debug)]
pub struct AvatarShadowController {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _spriteRenderer: quest_hook::libil2cpp::Gc<crate::UnityEngine::SpriteRenderer>,
    pub _distanceScale: f32,
    pub _scale: f32,
    pub _alpha: f32,
    pub _floorYPos: f32,
    pub _floorYPosLocal: bool,
    pub _avatarController: quest_hook::libil2cpp::Gc<
        crate::BeatSaber::AvatarCore::AvatarController,
    >,
    pub _transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
}
#[cfg(feature = "BeatSaber+AvatarCore+AvatarShadowController")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::BeatSaber::AvatarCore::AvatarShadowController =>
    "BeatSaber.AvatarCore"."AvatarShadowController"
);
#[cfg(feature = "BeatSaber+AvatarCore+AvatarShadowController")]
impl std::ops::Deref for crate::BeatSaber::AvatarCore::AvatarShadowController {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+AvatarShadowController")]
impl std::ops::DerefMut for crate::BeatSaber::AvatarCore::AvatarShadowController {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+AvatarShadowController")]
impl crate::BeatSaber::AvatarCore::AvatarShadowController {
    pub fn FollowPosition(
        &mut self,
        followPosition: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FollowPosition", (followPosition))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BeatSaber+AvatarCore+AvatarShadowController")]
impl quest_hook::libil2cpp::ObjectType
for crate::BeatSaber::AvatarCore::AvatarShadowController {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
