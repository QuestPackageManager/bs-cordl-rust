#[cfg(feature = "UnityEngine+Networking+PlayerConnection+IEditorPlayerConnection")]
#[repr(C)]
#[derive(Debug)]
pub struct IEditorPlayerConnection {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+Networking+PlayerConnection+IEditorPlayerConnection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Networking::PlayerConnection::IEditorPlayerConnection =>
    "UnityEngine.Networking.PlayerConnection"."IEditorPlayerConnection"
);
#[cfg(feature = "UnityEngine+Networking+PlayerConnection+IEditorPlayerConnection")]
impl std::ops::Deref
for crate::UnityEngine::Networking::PlayerConnection::IEditorPlayerConnection {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Networking+PlayerConnection+IEditorPlayerConnection")]
impl std::ops::DerefMut
for crate::UnityEngine::Networking::PlayerConnection::IEditorPlayerConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Networking+PlayerConnection+IEditorPlayerConnection")]
impl crate::UnityEngine::Networking::PlayerConnection::IEditorPlayerConnection {
    pub fn Register(
        &mut self,
        messageId: crate::System::Guid,
        callback: *mut crate::UnityEngine::Events::UnityAction_1<
            *mut crate::UnityEngine::Networking::PlayerConnection::MessageEventArgs,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Register", (messageId, callback))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterConnection(
        &mut self,
        callback: *mut crate::UnityEngine::Events::UnityAction_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterConnection", (callback))?;
        Ok(__cordl_ret)
    }
    pub fn RegisterDisconnection(
        &mut self,
        callback: *mut crate::UnityEngine::Events::UnityAction_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterDisconnection", (callback))?;
        Ok(__cordl_ret)
    }
    pub fn Send(
        &mut self,
        messageId: crate::System::Guid,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Send", (messageId, data))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+Networking+PlayerConnection+IEditorPlayerConnection")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Networking::PlayerConnection::IEditorPlayerConnection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
