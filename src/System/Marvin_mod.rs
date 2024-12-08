#[cfg(feature = "System+Marvin")]
#[repr(C)]
#[derive(Debug)]
pub struct Marvin {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Marvin")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Marvin => "System"."Marvin"
);
#[cfg(feature = "System+Marvin")]
impl std::ops::Deref for crate::System::Marvin {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Marvin")]
impl std::ops::DerefMut for crate::System::Marvin {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Marvin")]
impl crate::System::Marvin {}
#[cfg(feature = "System+Marvin")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Marvin {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}