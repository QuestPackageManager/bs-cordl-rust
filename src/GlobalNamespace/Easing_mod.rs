#[cfg(feature = "Easing")]
#[repr(C)]
#[derive(Debug)]
pub struct Easing {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Easing")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::Easing => ""."Easing"
);
#[cfg(feature = "Easing")]
impl std::ops::Deref for crate::GlobalNamespace::Easing {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Easing")]
impl std::ops::DerefMut for crate::GlobalNamespace::Easing {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Easing")]
impl crate::GlobalNamespace::Easing {}
#[cfg(feature = "Easing")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::Easing {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
