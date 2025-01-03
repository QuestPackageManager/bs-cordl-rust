#[cfg(feature = "MultiplayerLocalActivePlayerIntroAnimator")]
#[repr(C)]
#[derive(Debug)]
pub struct MultiplayerLocalActivePlayerIntroAnimator {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _introPlayableDirector: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Playables::PlayableDirector,
    >,
}
#[cfg(feature = "MultiplayerLocalActivePlayerIntroAnimator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MultiplayerLocalActivePlayerIntroAnimator => ""
    ."MultiplayerLocalActivePlayerIntroAnimator"
);
#[cfg(feature = "MultiplayerLocalActivePlayerIntroAnimator")]
impl std::ops::Deref
for crate::GlobalNamespace::MultiplayerLocalActivePlayerIntroAnimator {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLocalActivePlayerIntroAnimator")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MultiplayerLocalActivePlayerIntroAnimator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MultiplayerLocalActivePlayerIntroAnimator")]
impl crate::GlobalNamespace::MultiplayerLocalActivePlayerIntroAnimator {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetAfterIntroValues(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetAfterIntroValues", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetBeforeIntroValues(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetBeforeIntroValues", ())?;
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
#[cfg(feature = "MultiplayerLocalActivePlayerIntroAnimator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MultiplayerLocalActivePlayerIntroAnimator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
