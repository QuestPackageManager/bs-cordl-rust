#[cfg(feature = "UnityEngine+Gizmos")]
#[repr(C)]
#[derive(Debug)]
pub struct Gizmos {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Gizmos")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Gizmos => "UnityEngine"."Gizmos"
);
#[cfg(feature = "UnityEngine+Gizmos")]
impl std::ops::Deref for crate::UnityEngine::Gizmos {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Gizmos")]
impl std::ops::DerefMut for crate::UnityEngine::Gizmos {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Gizmos")]
impl crate::UnityEngine::Gizmos {}
#[cfg(feature = "UnityEngine+Gizmos")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Gizmos {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
