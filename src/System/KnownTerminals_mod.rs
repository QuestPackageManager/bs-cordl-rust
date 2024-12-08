#[cfg(feature = "System+KnownTerminals")]
#[repr(C)]
#[derive(Debug)]
pub struct KnownTerminals {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+KnownTerminals")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::KnownTerminals => "System"
    ."KnownTerminals"
);
#[cfg(feature = "System+KnownTerminals")]
impl std::ops::Deref for crate::System::KnownTerminals {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+KnownTerminals")]
impl std::ops::DerefMut for crate::System::KnownTerminals {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+KnownTerminals")]
impl crate::System::KnownTerminals {}
#[cfg(feature = "System+KnownTerminals")]
impl quest_hook::libil2cpp::ObjectType for crate::System::KnownTerminals {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
