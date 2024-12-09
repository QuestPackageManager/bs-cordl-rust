#[cfg(feature = "System+Net+Mail+WhitespaceReader")]
#[repr(C)]
#[derive(Debug)]
pub struct WhitespaceReader {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+Mail+WhitespaceReader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Mail::WhitespaceReader =>
    "System.Net.Mail"."WhitespaceReader"
);
#[cfg(feature = "System+Net+Mail+WhitespaceReader")]
impl std::ops::Deref for crate::System::Net::Mail::WhitespaceReader {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Mail+WhitespaceReader")]
impl std::ops::DerefMut for crate::System::Net::Mail::WhitespaceReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Mail+WhitespaceReader")]
impl crate::System::Net::Mail::WhitespaceReader {}
#[cfg(feature = "System+Net+Mail+WhitespaceReader")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::Mail::WhitespaceReader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
