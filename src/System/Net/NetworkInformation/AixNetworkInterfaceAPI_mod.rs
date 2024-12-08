#[cfg(feature = "System+Net+NetworkInformation+AixNetworkInterfaceAPI")]
#[repr(C)]
#[derive(Debug)]
pub struct AixNetworkInterfaceAPI {
    __cordl_parent: crate::System::Net::NetworkInformation::UnixNetworkInterfaceAPI,
}
#[cfg(feature = "System+Net+NetworkInformation+AixNetworkInterfaceAPI")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::NetworkInformation::AixNetworkInterfaceAPI =>
    "System.Net.NetworkInformation"."AixNetworkInterfaceAPI"
);
#[cfg(feature = "System+Net+NetworkInformation+AixNetworkInterfaceAPI")]
impl std::ops::Deref for crate::System::Net::NetworkInformation::AixNetworkInterfaceAPI {
    type Target = crate::System::Net::NetworkInformation::UnixNetworkInterfaceAPI;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+AixNetworkInterfaceAPI")]
impl std::ops::DerefMut
for crate::System::Net::NetworkInformation::AixNetworkInterfaceAPI {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+AixNetworkInterfaceAPI")]
impl crate::System::Net::NetworkInformation::AixNetworkInterfaceAPI {
    pub fn GetAllNetworkInterfaces(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Net::NetworkInformation::NetworkInterface,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Net::NetworkInformation::NetworkInterface,
        > = __cordl_object.invoke("GetAllNetworkInterfaces", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+NetworkInformation+AixNetworkInterfaceAPI")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::NetworkInformation::AixNetworkInterfaceAPI {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
