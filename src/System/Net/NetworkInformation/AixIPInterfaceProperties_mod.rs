#[cfg(feature = "System+Net+NetworkInformation+AixIPInterfaceProperties")]
#[repr(C)]
#[derive(Debug)]
pub struct AixIPInterfaceProperties {
    __cordl_parent: crate::System::Net::NetworkInformation::UnixIPInterfaceProperties,
    pub _mtu: i32,
}
#[cfg(feature = "System+Net+NetworkInformation+AixIPInterfaceProperties")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::NetworkInformation::AixIPInterfaceProperties =>
    "System.Net.NetworkInformation"."AixIPInterfaceProperties"
);
#[cfg(feature = "System+Net+NetworkInformation+AixIPInterfaceProperties")]
impl std::ops::Deref
for crate::System::Net::NetworkInformation::AixIPInterfaceProperties {
    type Target = crate::System::Net::NetworkInformation::UnixIPInterfaceProperties;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+AixIPInterfaceProperties")]
impl std::ops::DerefMut
for crate::System::Net::NetworkInformation::AixIPInterfaceProperties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+AixIPInterfaceProperties")]
impl crate::System::Net::NetworkInformation::AixIPInterfaceProperties {
    pub fn New(
        iface: quest_hook::libil2cpp::Gc<
            crate::System::Net::NetworkInformation::AixNetworkInterface,
        >,
        addresses: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::System::Net::IPAddress,
            >,
        >,
        mtu: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (iface, addresses, mtu))?;
        Ok(__cordl_object.into())
    }
    pub fn ParseRouteInfo_icall(
        iface: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        gw_addr_list: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ParseRouteInfo_icall", (iface, gw_addr_list))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        iface: quest_hook::libil2cpp::Gc<
            crate::System::Net::NetworkInformation::AixNetworkInterface,
        >,
        addresses: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::System::Net::IPAddress,
            >,
        >,
        mtu: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (iface, addresses, mtu))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_GatewayAddresses(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Net::NetworkInformation::GatewayIPAddressInformationCollection,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::NetworkInformation::GatewayIPAddressInformationCollection,
        > = __cordl_object.invoke("get_GatewayAddresses", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+NetworkInformation+AixIPInterfaceProperties")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::NetworkInformation::AixIPInterfaceProperties {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
