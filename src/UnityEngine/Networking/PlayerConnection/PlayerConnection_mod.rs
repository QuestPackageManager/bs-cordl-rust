#[cfg(feature = "UnityEngine+Networking+PlayerConnection+PlayerConnection")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerConnection {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub m_PlayerEditorConnectionEvents: *mut crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents,
    pub m_connectedPlayers: *mut crate::System::Collections::Generic::List_1<i32>,
    pub m_IsInitilized: bool,
}
#[cfg(feature = "UnityEngine+Networking+PlayerConnection+PlayerConnection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Networking::PlayerConnection::PlayerConnection =>
    "UnityEngine.Networking.PlayerConnection"."PlayerConnection"
);
#[cfg(feature = "UnityEngine+Networking+PlayerConnection+PlayerConnection")]
impl std::ops::Deref
for crate::UnityEngine::Networking::PlayerConnection::PlayerConnection {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Networking+PlayerConnection+PlayerConnection")]
impl std::ops::DerefMut
for crate::UnityEngine::Networking::PlayerConnection::PlayerConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Networking+PlayerConnection+PlayerConnection")]
impl crate::UnityEngine::Networking::PlayerConnection::PlayerConnection {
    #[cfg(
        feature = "UnityEngine+Networking+PlayerConnection+PlayerConnection+__c__DisplayClass12_0"
    )]
    pub type __c__DisplayClass12_0 = crate::UnityEngine::Networking::PlayerConnection::PlayerConnection___c__DisplayClass12_0;
    #[cfg(
        feature = "UnityEngine+Networking+PlayerConnection+PlayerConnection+__c__DisplayClass20_0"
    )]
    pub type __c__DisplayClass20_0 = crate::UnityEngine::Networking::PlayerConnection::PlayerConnection___c__DisplayClass20_0;
    #[cfg(
        feature = "UnityEngine+Networking+PlayerConnection+PlayerConnection+__c__DisplayClass13_0"
    )]
    pub type __c__DisplayClass13_0 = crate::UnityEngine::Networking::PlayerConnection::PlayerConnection___c__DisplayClass13_0;
    pub fn DisconnectAll(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisconnectAll", ())?;
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
    pub fn UnregisterConnection(
        &mut self,
        callback: *mut crate::UnityEngine::Events::UnityAction_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterConnection", (callback))?;
        Ok(__cordl_ret)
    }
    pub fn Unregister(
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
            .invoke("Unregister", (messageId, callback))?;
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
    pub fn get_isConnected(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isConnected", ())?;
        Ok(__cordl_ret)
    }
    pub fn UnregisterDisconnection(
        &mut self,
        callback: *mut crate::UnityEngine::Events::UnityAction_1<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterDisconnection", (callback))?;
        Ok(__cordl_ret)
    }
    pub fn TrySend(
        &mut self,
        messageId: crate::System::Guid,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TrySend", (messageId, data))?;
        Ok(__cordl_ret)
    }
    pub fn GetConnectionNativeApi(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::IPlayerEditorConnectionNative,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::IPlayerEditorConnectionNative = __cordl_object
            .invoke("GetConnectionNativeApi", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret)
    }
    pub fn BlockUntilRecvMsg(
        &mut self,
        messageId: crate::System::Guid,
        timeout: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("BlockUntilRecvMsg", (messageId, timeout))?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+Networking+PlayerConnection+PlayerConnection")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Networking::PlayerConnection::PlayerConnection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
