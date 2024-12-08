#[cfg(feature = "System+Net+Mail+QuotedPairReader")]
#[repr(C)]
#[derive(Debug)]
pub struct QuotedPairReader {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Net+Mail+QuotedPairReader")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Mail::QuotedPairReader =>
    "System.Net.Mail"."QuotedPairReader"
);
#[cfg(feature = "System+Net+Mail+QuotedPairReader")]
impl std::ops::Deref for crate::System::Net::Mail::QuotedPairReader {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Mail+QuotedPairReader")]
impl std::ops::DerefMut for crate::System::Net::Mail::QuotedPairReader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Mail+QuotedPairReader")]
impl crate::System::Net::Mail::QuotedPairReader {}
#[cfg(feature = "System+Net+Mail+QuotedPairReader")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::Mail::QuotedPairReader {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
