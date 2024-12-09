#[cfg(feature = "UnityEngine+WWWTranscoder")]
#[repr(C)]
#[derive(Debug)]
pub struct WWWTranscoder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+WWWTranscoder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::WWWTranscoder => "UnityEngine"
    ."WWWTranscoder"
);
#[cfg(feature = "UnityEngine+WWWTranscoder")]
impl std::ops::Deref for crate::UnityEngine::WWWTranscoder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+WWWTranscoder")]
impl std::ops::DerefMut for crate::UnityEngine::WWWTranscoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+WWWTranscoder")]
impl crate::UnityEngine::WWWTranscoder {}
#[cfg(feature = "UnityEngine+WWWTranscoder")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::WWWTranscoder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
