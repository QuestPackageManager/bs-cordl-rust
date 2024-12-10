#[cfg(feature = "UnityEngine+Playables+IPlayableOutput")]
#[repr(C)]
#[derive(Debug)]
pub struct IPlayableOutput {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Playables+IPlayableOutput")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Playables::IPlayableOutput =>
    "UnityEngine.Playables"."IPlayableOutput"
);
#[cfg(feature = "UnityEngine+Playables+IPlayableOutput")]
impl std::ops::Deref for crate::UnityEngine::Playables::IPlayableOutput {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Playables+IPlayableOutput")]
impl std::ops::DerefMut for crate::UnityEngine::Playables::IPlayableOutput {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Playables+IPlayableOutput")]
impl crate::UnityEngine::Playables::IPlayableOutput {
    pub fn GetHandle(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Playables::PlayableOutputHandle,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Playables::PlayableOutputHandle = __cordl_object
            .invoke("GetHandle", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+Playables+IPlayableOutput")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Playables::IPlayableOutput {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
