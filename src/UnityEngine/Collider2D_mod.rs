#[cfg(feature = "UnityEngine+Collider2D")]
#[repr(C)]
#[derive(Debug)]
pub struct Collider2D {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::Behaviour>,
}
#[cfg(feature = "UnityEngine+Collider2D")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Collider2D => "UnityEngine"
    ."Collider2D"
);
#[cfg(feature = "UnityEngine+Collider2D")]
impl std::ops::Deref for crate::UnityEngine::Collider2D {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::Behaviour>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Collider2D")]
impl std::ops::DerefMut for crate::UnityEngine::Collider2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Collider2D")]
impl crate::UnityEngine::Collider2D {}
#[cfg(feature = "UnityEngine+Collider2D")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Collider2D {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
