#[cfg(feature = "System+Net+NetworkInformation+LinuxUnicastIPAddressInformation")]
#[repr(C)]
#[derive(Debug)]
pub struct LinuxUnicastIPAddressInformation {
    __cordl_parent: crate::System::Net::NetworkInformation::UnicastIPAddressInformation,
    pub address: quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress>,
}
#[cfg(feature = "System+Net+NetworkInformation+LinuxUnicastIPAddressInformation")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::NetworkInformation::LinuxUnicastIPAddressInformation {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net.NetworkInformation";
    const CLASS_NAME: &'static str = "LinuxUnicastIPAddressInformation";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+Net+NetworkInformation+LinuxUnicastIPAddressInformation")]
impl std::ops::Deref
for crate::System::Net::NetworkInformation::LinuxUnicastIPAddressInformation {
    type Target = crate::System::Net::NetworkInformation::UnicastIPAddressInformation;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+LinuxUnicastIPAddressInformation")]
impl std::ops::DerefMut
for crate::System::Net::NetworkInformation::LinuxUnicastIPAddressInformation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+LinuxUnicastIPAddressInformation")]
impl crate::System::Net::NetworkInformation::LinuxUnicastIPAddressInformation {
    pub fn New(
        address: quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (address))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        address: quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (address))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Address(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress> = __cordl_object
            .invoke("get_Address", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+NetworkInformation+LinuxUnicastIPAddressInformation")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::NetworkInformation::LinuxUnicastIPAddressInformation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
