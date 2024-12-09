#[cfg(feature = "System+Net+Mail+QuotedStringFormatReader")]
#[repr(C)]
#[derive(Debug)]
pub struct QuotedStringFormatReader {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+Mail+QuotedStringFormatReader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Mail::QuotedStringFormatReader =>
    "System.Net.Mail"."QuotedStringFormatReader"
);
#[cfg(feature = "System+Net+Mail+QuotedStringFormatReader")]
impl std::ops::Deref for crate::System::Net::Mail::QuotedStringFormatReader {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Mail+QuotedStringFormatReader")]
impl std::ops::DerefMut for crate::System::Net::Mail::QuotedStringFormatReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Mail+QuotedStringFormatReader")]
impl crate::System::Net::Mail::QuotedStringFormatReader {}
#[cfg(feature = "System+Net+Mail+QuotedStringFormatReader")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Mail::QuotedStringFormatReader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
