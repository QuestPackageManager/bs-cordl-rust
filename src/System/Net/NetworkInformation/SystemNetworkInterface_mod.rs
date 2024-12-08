#[cfg(feature = "System+Net+NetworkInformation+SystemNetworkInterface")]
#[repr(C)]
#[derive(Debug)]
pub struct SystemNetworkInterface {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Net+NetworkInformation+SystemNetworkInterface")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::NetworkInformation::SystemNetworkInterface =>
    "System.Net.NetworkInformation"."SystemNetworkInterface"
);
#[cfg(feature = "System+Net+NetworkInformation+SystemNetworkInterface")]
impl std::ops::Deref for crate::System::Net::NetworkInformation::SystemNetworkInterface {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+SystemNetworkInterface")]
impl std::ops::DerefMut
for crate::System::Net::NetworkInformation::SystemNetworkInterface {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+SystemNetworkInterface")]
impl crate::System::Net::NetworkInformation::SystemNetworkInterface {}
#[cfg(feature = "System+Net+NetworkInformation+SystemNetworkInterface")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::NetworkInformation::SystemNetworkInterface {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}