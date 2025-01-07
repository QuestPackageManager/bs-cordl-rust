#[cfg(feature = "System+Net+NetworkInformation+UnixNetworkInterfaceFactoryPal")]
#[repr(C)]
#[derive(Debug)]
pub struct UnixNetworkInterfaceFactoryPal {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Net+NetworkInformation+UnixNetworkInterfaceFactoryPal")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::NetworkInformation::UnixNetworkInterfaceFactoryPal {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net.NetworkInformation";
    const CLASS_NAME: &'static str = "UnixNetworkInterfaceFactoryPal";
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
#[cfg(feature = "System+Net+NetworkInformation+UnixNetworkInterfaceFactoryPal")]
impl std::ops::Deref
for crate::System::Net::NetworkInformation::UnixNetworkInterfaceFactoryPal {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
impl crate::System::Net::NetworkInformation::UnixNetworkInterfaceFactoryPal {
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
}
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
