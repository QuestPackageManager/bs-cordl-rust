#[cfg(feature = "System+Net+NetworkInformation+MacOsIPInterfaceProperties")]
#[repr(C)]
#[derive(Debug)]
pub struct MacOsIPInterfaceProperties {
    __cordl_parent: crate::System::Net::NetworkInformation::UnixIPInterfaceProperties,
}
#[cfg(feature = "System+Net+NetworkInformation+MacOsIPInterfaceProperties")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::NetworkInformation::MacOsIPInterfaceProperties =>
    "System.Net.NetworkInformation"."MacOsIPInterfaceProperties"
);
#[cfg(feature = "System+Net+NetworkInformation+MacOsIPInterfaceProperties")]
impl std::ops::Deref
for crate::System::Net::NetworkInformation::MacOsIPInterfaceProperties {
    type Target = crate::System::Net::NetworkInformation::UnixIPInterfaceProperties;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+MacOsIPInterfaceProperties")]
impl std::ops::DerefMut
for crate::System::Net::NetworkInformation::MacOsIPInterfaceProperties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+MacOsIPInterfaceProperties")]
impl crate::System::Net::NetworkInformation::MacOsIPInterfaceProperties {
    pub fn _ctor(
        &mut self,
        iface: *mut crate::System::Net::NetworkInformation::MacOsNetworkInterface,
        addresses: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Net::IPAddress,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (iface, addresses))?;
        Ok(__cordl_ret)
    }
    pub fn get_GatewayAddresses(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Net::NetworkInformation::GatewayIPAddressInformationCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::NetworkInformation::GatewayIPAddressInformationCollection = __cordl_object
            .invoke("get_GatewayAddresses", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        iface: *mut crate::System::Net::NetworkInformation::MacOsNetworkInterface,
        addresses: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Net::IPAddress,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (iface, addresses))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Net+NetworkInformation+MacOsIPInterfaceProperties")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::NetworkInformation::MacOsIPInterfaceProperties {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
