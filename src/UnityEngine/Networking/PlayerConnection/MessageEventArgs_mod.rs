#[cfg(feature = "UnityEngine+Networking+PlayerConnection+MessageEventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct MessageEventArgs {
    __cordl_parent: crate::System::Object,
    pub playerId: i32,
    pub data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "UnityEngine+Networking+PlayerConnection+MessageEventArgs")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Networking::PlayerConnection::MessageEventArgs =>
    "UnityEngine.Networking.PlayerConnection"."MessageEventArgs"
);
#[cfg(feature = "UnityEngine+Networking+PlayerConnection+MessageEventArgs")]
impl std::ops::Deref
for crate::UnityEngine::Networking::PlayerConnection::MessageEventArgs {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Networking+PlayerConnection+MessageEventArgs")]
impl std::ops::DerefMut
for crate::UnityEngine::Networking::PlayerConnection::MessageEventArgs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Networking+PlayerConnection+MessageEventArgs")]
impl crate::UnityEngine::Networking::PlayerConnection::MessageEventArgs {
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
#[cfg(feature = "UnityEngine+Networking+PlayerConnection+MessageEventArgs")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Networking::PlayerConnection::MessageEventArgs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}