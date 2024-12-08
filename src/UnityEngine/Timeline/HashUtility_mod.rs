#[cfg(feature = "UnityEngine+Timeline+HashUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct HashUtility {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+Timeline+HashUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Timeline::HashUtility =>
    "UnityEngine.Timeline"."HashUtility"
);
#[cfg(feature = "UnityEngine+Timeline+HashUtility")]
impl std::ops::Deref for crate::UnityEngine::Timeline::HashUtility {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+HashUtility")]
impl std::ops::DerefMut for crate::UnityEngine::Timeline::HashUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Timeline+HashUtility")]
impl crate::UnityEngine::Timeline::HashUtility {}
#[cfg(feature = "UnityEngine+Timeline+HashUtility")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Timeline::HashUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
