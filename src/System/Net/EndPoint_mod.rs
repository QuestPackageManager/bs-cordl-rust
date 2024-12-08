#[cfg(feature = "System+Net+EndPoint")]
#[repr(C)]
#[derive(Debug)]
pub struct EndPoint {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Net+EndPoint")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::EndPoint => "System.Net"."EndPoint"
);
#[cfg(feature = "System+Net+EndPoint")]
impl std::ops::Deref for crate::System::Net::EndPoint {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+EndPoint")]
impl std::ops::DerefMut for crate::System::Net::EndPoint {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+EndPoint")]
impl crate::System::Net::EndPoint {
    pub fn Create(
        &mut self,
        socketAddress: *mut crate::System::Net::SocketAddress,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::EndPoint> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::EndPoint = __cordl_object
            .invoke("Create", (socketAddress))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Serialize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::SocketAddress> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::SocketAddress = __cordl_object
            .invoke("Serialize", ())?;
        Ok(__cordl_ret)
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
    pub fn get_AddressFamily(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::Sockets::AddressFamily> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::Sockets::AddressFamily = __cordl_object
            .invoke("get_AddressFamily", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+EndPoint")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::EndPoint {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
