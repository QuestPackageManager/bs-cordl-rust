#[cfg(feature = "MathfExtra")]
#[repr(C)]
#[derive(Debug)]
pub struct MathfExtra {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "MathfExtra")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MathfExtra => ""."MathfExtra"
);
#[cfg(feature = "MathfExtra")]
impl std::ops::Deref for crate::GlobalNamespace::MathfExtra {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MathfExtra")]
impl std::ops::DerefMut for crate::GlobalNamespace::MathfExtra {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MathfExtra")]
impl crate::GlobalNamespace::MathfExtra {}
#[cfg(feature = "MathfExtra")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::MathfExtra {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
