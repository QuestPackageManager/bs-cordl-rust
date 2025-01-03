#[cfg(feature = "System+Net+Sockets+SendPacketsElement")]
#[repr(C)]
#[derive(Debug)]
pub struct SendPacketsElement {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_FilePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_Buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub m_Offset: i32,
    pub m_Count: i32,
    pub m_endOfPacket: bool,
}
#[cfg(feature = "System+Net+Sockets+SendPacketsElement")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::Sockets::SendPacketsElement =>
    "System.Net.Sockets"."SendPacketsElement"
);
#[cfg(feature = "System+Net+Sockets+SendPacketsElement")]
impl std::ops::Deref for crate::System::Net::Sockets::SendPacketsElement {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Sockets+SendPacketsElement")]
impl std::ops::DerefMut for crate::System::Net::Sockets::SendPacketsElement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+Sockets+SendPacketsElement")]
impl crate::System::Net::Sockets::SendPacketsElement {}
#[cfg(feature = "System+Net+Sockets+SendPacketsElement")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::Sockets::SendPacketsElement {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
