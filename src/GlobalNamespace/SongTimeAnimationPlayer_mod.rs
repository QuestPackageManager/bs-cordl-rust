#[cfg(feature = "SongTimeAnimationPlayer")]
#[repr(C)]
#[derive(Debug)]
pub struct SongTimeAnimationPlayer {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _animationClip: *mut crate::UnityEngine::AnimationClip,
    pub _audioTimeSyncController: *mut crate::GlobalNamespace::AudioTimeSyncController,
}
#[cfg(feature = "SongTimeAnimationPlayer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::SongTimeAnimationPlayer => ""
    ."SongTimeAnimationPlayer"
);
#[cfg(feature = "SongTimeAnimationPlayer")]
impl std::ops::Deref for crate::GlobalNamespace::SongTimeAnimationPlayer {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SongTimeAnimationPlayer")]
impl std::ops::DerefMut for crate::GlobalNamespace::SongTimeAnimationPlayer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SongTimeAnimationPlayer")]
impl crate::GlobalNamespace::SongTimeAnimationPlayer {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "SongTimeAnimationPlayer")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::SongTimeAnimationPlayer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
