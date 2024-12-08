#[cfg(feature = "UnityEngine+ProBuilder+ShapeFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct ShapeFactory {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+ProBuilder+ShapeFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::ShapeFactory =>
    "UnityEngine.ProBuilder"."ShapeFactory"
);
#[cfg(feature = "UnityEngine+ProBuilder+ShapeFactory")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::ShapeFactory {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ShapeFactory")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::ShapeFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+ShapeFactory")]
impl crate::UnityEngine::ProBuilder::ShapeFactory {}
#[cfg(feature = "UnityEngine+ProBuilder+ShapeFactory")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::ShapeFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
