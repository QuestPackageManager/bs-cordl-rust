#[cfg(feature = "System+Net+IPAddress+ReadOnlyIPAddress")]
#[repr(C)]
#[derive(Debug)]
pub struct IPAddress_ReadOnlyIPAddress {
    __cordl_parent: crate::System::Net::IPAddress,
}
#[cfg(feature = "System+Net+IPAddress+ReadOnlyIPAddress")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IPAddress_ReadOnlyIPAddress =>
    "System.Net"."IPAddress/ReadOnlyIPAddress"
);
#[cfg(feature = "System+Net+IPAddress+ReadOnlyIPAddress")]
impl std::ops::Deref for crate::GlobalNamespace::IPAddress_ReadOnlyIPAddress {
    type Target = crate::System::Net::IPAddress;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+IPAddress+ReadOnlyIPAddress")]
impl std::ops::DerefMut for crate::GlobalNamespace::IPAddress_ReadOnlyIPAddress {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+IPAddress+ReadOnlyIPAddress")]
impl crate::GlobalNamespace::IPAddress_ReadOnlyIPAddress {
    pub fn New(newAddress: i64) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (newAddress))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        newAddress: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (newAddress))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+IPAddress+ReadOnlyIPAddress")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IPAddress_ReadOnlyIPAddress {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
