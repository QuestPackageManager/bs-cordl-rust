#[cfg(feature = "System+Net+Mail+DotAtomReader")]
#[repr(C)]
#[derive(Debug)]
pub struct DotAtomReader {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Net+Mail+DotAtomReader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Mail::DotAtomReader =>
    "System.Net.Mail"."DotAtomReader"
);
#[cfg(feature = "System+Net+Mail+DotAtomReader")]
impl std::ops::Deref for crate::System::Net::Mail::DotAtomReader {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Mail+DotAtomReader")]
impl std::ops::DerefMut for crate::System::Net::Mail::DotAtomReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Mail+DotAtomReader")]
impl crate::System::Net::Mail::DotAtomReader {}
#[cfg(feature = "System+Net+Mail+DotAtomReader")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::Mail::DotAtomReader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
