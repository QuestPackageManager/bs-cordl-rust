#[cfg(feature = "SR")]
#[repr(C)]
#[derive(Debug)]
pub struct SR {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "SR")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SR => ""."SR"
);
#[cfg(feature = "SR")]
impl std::ops::Deref for SR {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SR")]
impl std::ops::DerefMut for SR {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SR")]
impl SR {}
#[cfg(feature = "SR")]
impl quest_hook::libil2cpp::ObjectType for SR {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
