#[cfg(feature = "System+Net+NetworkInformation+UnixNetworkInterfaceFactoryPal")]
#[repr(C)]
#[derive(Debug)]
pub struct UnixNetworkInterfaceFactoryPal {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Net+NetworkInformation+UnixNetworkInterfaceFactoryPal")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::NetworkInformation::UnixNetworkInterfaceFactoryPal =>
    "System.Net.NetworkInformation"."UnixNetworkInterfaceFactoryPal"
);
#[cfg(feature = "System+Net+NetworkInformation+UnixNetworkInterfaceFactoryPal")]
impl std::ops::Deref
for crate::System::Net::NetworkInformation::UnixNetworkInterfaceFactoryPal {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+UnixNetworkInterfaceFactoryPal")]
impl std::ops::DerefMut
for crate::System::Net::NetworkInformation::UnixNetworkInterfaceFactoryPal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+UnixNetworkInterfaceFactoryPal")]
impl crate::System::Net::NetworkInformation::UnixNetworkInterfaceFactoryPal {}
#[cfg(feature = "System+Net+NetworkInformation+UnixNetworkInterfaceFactoryPal")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::NetworkInformation::UnixNetworkInterfaceFactoryPal {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
