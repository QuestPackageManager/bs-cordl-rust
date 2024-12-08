#[cfg(feature = "UnityEngine+Physics2D")]
#[repr(C)]
#[derive(Debug)]
pub struct Physics2D {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+Physics2D")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Physics2D => "UnityEngine"
    ."Physics2D"
);
#[cfg(feature = "UnityEngine+Physics2D")]
impl std::ops::Deref for crate::UnityEngine::Physics2D {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Physics2D")]
impl std::ops::DerefMut for crate::UnityEngine::Physics2D {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Physics2D")]
impl crate::UnityEngine::Physics2D {}
#[cfg(feature = "UnityEngine+Physics2D")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::Physics2D {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
