#[cfg(feature = "UnityEngine+Animations+NotKeyableAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct NotKeyableAttribute {
    __cordl_parent: crate::System::Attribute,
}
#[cfg(feature = "UnityEngine+Animations+NotKeyableAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Animations::NotKeyableAttribute =>
    "UnityEngine.Animations"."NotKeyableAttribute"
);
#[cfg(feature = "UnityEngine+Animations+NotKeyableAttribute")]
impl std::ops::Deref for crate::UnityEngine::Animations::NotKeyableAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Animations+NotKeyableAttribute")]
impl std::ops::DerefMut for crate::UnityEngine::Animations::NotKeyableAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Animations+NotKeyableAttribute")]
impl crate::UnityEngine::Animations::NotKeyableAttribute {}
#[cfg(feature = "UnityEngine+Animations+NotKeyableAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Animations::NotKeyableAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
