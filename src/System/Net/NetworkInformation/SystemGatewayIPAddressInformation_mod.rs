#[cfg(feature = "System+Net+NetworkInformation+SystemGatewayIPAddressInformation")]
#[repr(C)]
#[derive(Debug)]
pub struct SystemGatewayIPAddressInformation {
    __cordl_parent: crate::System::Net::NetworkInformation::GatewayIPAddressInformation,
    pub address: *mut crate::System::Net::IPAddress,
}
#[cfg(feature = "System+Net+NetworkInformation+SystemGatewayIPAddressInformation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::NetworkInformation::SystemGatewayIPAddressInformation =>
    "System.Net.NetworkInformation"."SystemGatewayIPAddressInformation"
);
#[cfg(feature = "System+Net+NetworkInformation+SystemGatewayIPAddressInformation")]
impl std::ops::Deref
for crate::System::Net::NetworkInformation::SystemGatewayIPAddressInformation {
    type Target = crate::System::Net::NetworkInformation::GatewayIPAddressInformation;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+SystemGatewayIPAddressInformation")]
impl std::ops::DerefMut
for crate::System::Net::NetworkInformation::SystemGatewayIPAddressInformation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+SystemGatewayIPAddressInformation")]
impl crate::System::Net::NetworkInformation::SystemGatewayIPAddressInformation {
    pub fn _ctor(
        &mut self,
        address: *mut crate::System::Net::IPAddress,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (address))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        address: *mut crate::System::Net::IPAddress,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (address))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Net+NetworkInformation+SystemGatewayIPAddressInformation")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::NetworkInformation::SystemGatewayIPAddressInformation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
