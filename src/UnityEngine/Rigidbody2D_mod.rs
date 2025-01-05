#[cfg(feature = "UnityEngine+Rigidbody2D")]
#[repr(C)]
#[derive(Debug)]
pub struct Rigidbody2D {
    __cordl_parent: crate::UnityEngine::Component,
}
#[cfg(feature = "UnityEngine+Rigidbody2D")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Rigidbody2D => "UnityEngine"
    ."Rigidbody2D"
);
#[cfg(feature = "UnityEngine+Rigidbody2D")]
impl std::ops::Deref for crate::UnityEngine::Rigidbody2D {
    type Target = crate::UnityEngine::Component;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rigidbody2D")]
impl std::ops::DerefMut for crate::UnityEngine::Rigidbody2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Rigidbody2D")]
impl crate::UnityEngine::Rigidbody2D {}
#[cfg(feature = "UnityEngine+Rigidbody2D")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Rigidbody2D {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
