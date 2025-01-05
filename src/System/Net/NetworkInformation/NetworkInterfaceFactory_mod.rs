#[cfg(feature = "System+Net+NetworkInformation+NetworkInterfaceFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct NetworkInterfaceFactory {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+NetworkInformation+NetworkInterfaceFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::NetworkInformation::NetworkInterfaceFactory =>
    "System.Net.NetworkInformation"."NetworkInterfaceFactory"
);
#[cfg(feature = "System+Net+NetworkInformation+NetworkInterfaceFactory")]
impl std::ops::Deref
for crate::System::Net::NetworkInformation::NetworkInterfaceFactory {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+NetworkInterfaceFactory")]
impl std::ops::DerefMut
for crate::System::Net::NetworkInformation::NetworkInterfaceFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+NetworkInterfaceFactory")]
impl crate::System::Net::NetworkInformation::NetworkInterfaceFactory {
    pub fn Create() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Net::NetworkInformation::NetworkInterfaceFactory,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::NetworkInformation::NetworkInterfaceFactory,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Create", ())?;
        Ok(__cordl_ret.into())
    }
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+NetworkInformation+NetworkInterfaceFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::NetworkInformation::NetworkInterfaceFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
