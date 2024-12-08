#[cfg(feature = "System+Convert")]
#[repr(C)]
#[derive(Debug)]
pub struct Convert {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Convert")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Convert => "System"."Convert"
);
#[cfg(feature = "System+Convert")]
impl std::ops::Deref for crate::System::Convert {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Convert")]
impl std::ops::DerefMut for crate::System::Convert {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Convert")]
impl crate::System::Convert {}
#[cfg(feature = "System+Convert")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Convert {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
