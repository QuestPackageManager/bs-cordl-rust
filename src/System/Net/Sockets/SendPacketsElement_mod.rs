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
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::Sockets::SendPacketsElement {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net.Sockets";
    const CLASS_NAME: &'static str = "SendPacketsElement";
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
