#[cfg(feature = "System+Net+NetworkInformation+LinuxIPInterfaceProperties")]
#[repr(C)]
#[derive(Debug)]
pub struct LinuxIPInterfaceProperties {
    __cordl_parent: crate::System::Net::NetworkInformation::UnixIPInterfaceProperties,
}
#[cfg(feature = "System+Net+NetworkInformation+LinuxIPInterfaceProperties")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::NetworkInformation::LinuxIPInterfaceProperties =>
    "System.Net.NetworkInformation"."LinuxIPInterfaceProperties"
);
#[cfg(feature = "System+Net+NetworkInformation+LinuxIPInterfaceProperties")]
impl std::ops::Deref
for crate::System::Net::NetworkInformation::LinuxIPInterfaceProperties {
    type Target = crate::System::Net::NetworkInformation::UnixIPInterfaceProperties;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+LinuxIPInterfaceProperties")]
impl std::ops::DerefMut
for crate::System::Net::NetworkInformation::LinuxIPInterfaceProperties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+LinuxIPInterfaceProperties")]
impl crate::System::Net::NetworkInformation::LinuxIPInterfaceProperties {
    pub fn New(
        iface: *mut crate::System::Net::NetworkInformation::LinuxNetworkInterface,
        addresses: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Net::IPAddress,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (iface, addresses))?;
        Ok(__cordl_object)
    }
    pub fn ParseRouteInfo(
        &mut self,
        iface: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Net::NetworkInformation::IPAddressCollection,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::NetworkInformation::IPAddressCollection = __cordl_object
            .invoke("ParseRouteInfo", (iface))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        iface: *mut crate::System::Net::NetworkInformation::LinuxNetworkInterface,
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
}
#[cfg(feature = "System+Net+NetworkInformation+LinuxIPInterfaceProperties")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::NetworkInformation::LinuxIPInterfaceProperties {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
