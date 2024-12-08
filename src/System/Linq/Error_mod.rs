#[cfg(feature = "System+Linq+Error")]
#[repr(C)]
#[derive(Debug)]
pub struct Error {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Linq+Error")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Linq::Error => "System.Linq"."Error"
);
#[cfg(feature = "System+Linq+Error")]
impl std::ops::Deref for crate::System::Linq::Error {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Error")]
impl std::ops::DerefMut for crate::System::Linq::Error {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+Error")]
impl crate::System::Linq::Error {}
#[cfg(feature = "System+Linq+Error")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Linq::Error {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
