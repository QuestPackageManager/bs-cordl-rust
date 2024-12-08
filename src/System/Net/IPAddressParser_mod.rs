#[cfg(feature = "System+Net+IPAddressParser")]
#[repr(C)]
#[derive(Debug)]
pub struct IPAddressParser {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Net+IPAddressParser")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::IPAddressParser => "System.Net"
    ."IPAddressParser"
);
#[cfg(feature = "System+Net+IPAddressParser")]
impl std::ops::Deref for crate::System::Net::IPAddressParser {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+IPAddressParser")]
impl std::ops::DerefMut for crate::System::Net::IPAddressParser {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+IPAddressParser")]
impl crate::System::Net::IPAddressParser {}
#[cfg(feature = "System+Net+IPAddressParser")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::IPAddressParser {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}