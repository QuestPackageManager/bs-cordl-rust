#[cfg(feature = "System+Net+NetworkInformation+CommonUnixIPGlobalProperties")]
#[repr(C)]
#[derive(Debug)]
pub struct CommonUnixIPGlobalProperties {
    __cordl_parent: quest_hook::libil2cpp::Gc<
        crate::System::Net::NetworkInformation::IPGlobalProperties,
    >,
}
#[cfg(feature = "System+Net+NetworkInformation+CommonUnixIPGlobalProperties")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Net::NetworkInformation::CommonUnixIPGlobalProperties =>
    "System.Net.NetworkInformation"."CommonUnixIPGlobalProperties"
);
#[cfg(feature = "System+Net+NetworkInformation+CommonUnixIPGlobalProperties")]
impl std::ops::Deref
for crate::System::Net::NetworkInformation::CommonUnixIPGlobalProperties {
    type Target = quest_hook::libil2cpp::Gc<
        crate::System::Net::NetworkInformation::IPGlobalProperties,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+CommonUnixIPGlobalProperties")]
impl std::ops::DerefMut
for crate::System::Net::NetworkInformation::CommonUnixIPGlobalProperties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+NetworkInformation+CommonUnixIPGlobalProperties")]
impl crate::System::Net::NetworkInformation::CommonUnixIPGlobalProperties {
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
    pub fn get_DomainName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_DomainName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn getdomainname(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("getdomainname", (name, len))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+NetworkInformation+CommonUnixIPGlobalProperties")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::NetworkInformation::CommonUnixIPGlobalProperties {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
