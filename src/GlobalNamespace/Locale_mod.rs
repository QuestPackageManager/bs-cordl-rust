#[cfg(feature = "Locale")]
#[repr(C)]
#[derive(Debug)]
pub struct Locale {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Locale")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::Locale => ""."Locale"
);
#[cfg(feature = "Locale")]
impl std::ops::Deref for crate::GlobalNamespace::Locale {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Locale")]
impl std::ops::DerefMut for crate::GlobalNamespace::Locale {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Locale")]
impl crate::GlobalNamespace::Locale {}
#[cfg(feature = "Locale")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::Locale {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
