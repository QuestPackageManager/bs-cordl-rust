#[cfg(feature = "UnityEngine+Networking+PlayerConnection+PlayerConnection")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerConnection {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::ScriptableObject>,
    pub m_PlayerEditorConnectionEvents: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents,
    >,
    pub m_connectedPlayers: quest_hook::libil2cpp::Gc<i32>,
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
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::ScriptableObject>;
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
        Ok(__cordl_ret.into())
    }
    pub fn ConnectedCallbackInternal(
        playerId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConnectedCallbackInternal", (playerId))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateInstance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::PlayerConnection::PlayerConnection,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::PlayerConnection::PlayerConnection,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("CreateInstance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DisconnectAll(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DisconnectAll", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DisconnectedCallback(
        playerId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DisconnectedCallback", (playerId))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetConnectionNativeApi(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::IPlayerEditorConnectionNative>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::IPlayerEditorConnectionNative,
        > = __cordl_object.invoke("GetConnectionNativeApi", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MessageCallbackInternal(
        data: crate::System::IntPtr,
        _cordl_size: u64,
        guid: u64,
        messageId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("MessageCallbackInternal", (data, _cordl_size, guid, messageId))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Register(
        &mut self,
        messageId: crate::System::Guid,
        callback: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Networking::PlayerConnection::MessageEventArgs,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Register", (messageId, callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterConnection(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterConnection", (callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterDisconnection(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterDisconnection", (callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn Send(
        &mut self,
        messageId: crate::System::Guid,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Send", (messageId, data))?;
        Ok(__cordl_ret.into())
    }
    pub fn TrySend(
        &mut self,
        messageId: crate::System::Guid,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TrySend", (messageId, data))?;
        Ok(__cordl_ret.into())
    }
    pub fn Unregister(
        &mut self,
        messageId: crate::System::Guid,
        callback: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::Networking::PlayerConnection::MessageEventArgs,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Unregister", (messageId, callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterConnection(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterConnection", (callback))?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterDisconnection(
        &mut self,
        callback: quest_hook::libil2cpp::Gc<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterDisconnection", (callback))?;
        Ok(__cordl_ret.into())
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
    pub fn get_instance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::PlayerConnection::PlayerConnection,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::PlayerConnection::PlayerConnection,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_instance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_isConnected(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isConnected", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "UnityEngine+Networking+PlayerConnection+PlayerConnection")]
impl AsRef<
    quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Networking::PlayerConnection::IEditorPlayerConnection,
    >,
> for crate::UnityEngine::Networking::PlayerConnection::PlayerConnection {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Networking::PlayerConnection::IEditorPlayerConnection,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+Networking+PlayerConnection+PlayerConnection")]
impl AsMut<
    quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Networking::PlayerConnection::IEditorPlayerConnection,
    >,
> for crate::UnityEngine::Networking::PlayerConnection::PlayerConnection {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Networking::PlayerConnection::IEditorPlayerConnection,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
