#[cfg(feature = "System+Net+ReceiveState")]
#[repr(C)]
#[derive(Debug)]
pub struct ReceiveState {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub Resp: quest_hook::libil2cpp::Gc<crate::System::Net::ResponseDescription>,
    pub ValidThrough: i32,
    pub Buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub Connection: quest_hook::libil2cpp::Gc<crate::System::Net::CommandStream>,
}
#[cfg(feature = "System+Net+ReceiveState")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::ReceiveState => "System.Net"
    ."ReceiveState"
);
#[cfg(feature = "System+Net+ReceiveState")]
impl std::ops::Deref for crate::System::Net::ReceiveState {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ReceiveState")]
impl std::ops::DerefMut for crate::System::Net::ReceiveState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+ReceiveState")]
impl crate::System::Net::ReceiveState {
    pub fn New(
        connection: quest_hook::libil2cpp::Gc<crate::System::Net::CommandStream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (connection))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        connection: quest_hook::libil2cpp::Gc<crate::System::Net::CommandStream>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (connection))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+ReceiveState")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::ReceiveState {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
