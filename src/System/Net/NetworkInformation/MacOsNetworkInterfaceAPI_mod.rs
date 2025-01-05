#[cfg(feature = "System+Net+NetworkInformation+MacOsNetworkInterfaceAPI")]
#[repr(C)]
#[derive(Debug)]
pub struct MacOsNetworkInterfaceAPI {
    __cordl_parent: crate::System::Net::NetworkInformation::UnixNetworkInterfaceAPI,
    pub AF_INET6: i32,
}
#[cfg(feature = "System+Net+NetworkInformation+MacOsNetworkInterfaceAPI")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::NetworkInformation::MacOsNetworkInterfaceAPI =>
    "System.Net.NetworkInformation"."MacOsNetworkInterfaceAPI"
);
#[cfg(feature = "System+Net+NetworkInformation+MacOsNetworkInterfaceAPI")]
impl std::ops::Deref
for crate::System::Net::NetworkInformation::MacOsNetworkInterfaceAPI {
    type Target = crate::System::Net::NetworkInformation::UnixNetworkInterfaceAPI;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+MacOsNetworkInterfaceAPI")]
impl std::ops::DerefMut
for crate::System::Net::NetworkInformation::MacOsNetworkInterfaceAPI {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+MacOsNetworkInterfaceAPI")]
impl crate::System::Net::NetworkInformation::MacOsNetworkInterfaceAPI {
    pub fn GetAllNetworkInterfaces(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Net::NetworkInformation::NetworkInterface,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Net::NetworkInformation::NetworkInterface,
            >,
        > = __cordl_object.invoke("GetAllNetworkInterfaces", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_1(
        AF_INET6: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (AF_INET6))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_1(
        &mut self,
        AF_INET6: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (AF_INET6))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+NetworkInformation+MacOsNetworkInterfaceAPI")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::NetworkInformation::MacOsNetworkInterfaceAPI {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
