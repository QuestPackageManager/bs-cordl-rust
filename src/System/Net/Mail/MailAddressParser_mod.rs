#[cfg(feature = "System+Net+Mail+MailAddressParser")]
#[repr(C)]
#[derive(Debug)]
pub struct MailAddressParser {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Net+Mail+MailAddressParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Mail::MailAddressParser =>
    "System.Net.Mail"."MailAddressParser"
);
#[cfg(feature = "System+Net+Mail+MailAddressParser")]
impl std::ops::Deref for crate::System::Net::Mail::MailAddressParser {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Mail+MailAddressParser")]
impl std::ops::DerefMut for crate::System::Net::Mail::MailAddressParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Mail+MailAddressParser")]
impl crate::System::Net::Mail::MailAddressParser {}
#[cfg(feature = "System+Net+Mail+MailAddressParser")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::Mail::MailAddressParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
