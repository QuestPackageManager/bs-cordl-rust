#[cfg(feature = "UnityEngine+InputSystem+RemoteInputPlayerConnection")]
#[repr(C)]
#[derive(Debug)]
pub struct RemoteInputPlayerConnection {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub m_Connection: *mut crate::UnityEngine::Networking::PlayerConnection::IEditorPlayerConnection,
    pub m_Subscribers: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::InputSystem::RemoteInputPlayerConnection_Subscriber,
    >,
    pub m_ConnectedIds: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
}
#[cfg(feature = "UnityEngine+InputSystem+RemoteInputPlayerConnection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::RemoteInputPlayerConnection => "UnityEngine.InputSystem"
    ."RemoteInputPlayerConnection"
);
#[cfg(feature = "UnityEngine+InputSystem+RemoteInputPlayerConnection")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::RemoteInputPlayerConnection {
    type Target = crate::UnityEngine::ScriptableObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+RemoteInputPlayerConnection")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::RemoteInputPlayerConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+RemoteInputPlayerConnection")]
impl crate::UnityEngine::InputSystem::RemoteInputPlayerConnection {
    #[cfg(feature = "UnityEngine+InputSystem+RemoteInputPlayerConnection+Subscriber")]
    pub type Subscriber = crate::UnityEngine::InputSystem::RemoteInputPlayerConnection_Subscriber;
    pub fn Bind(
        &mut self,
        connection: *mut crate::UnityEngine::Networking::PlayerConnection::IEditorPlayerConnection,
        isConnected: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Bind", (connection, isConnected))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnChangeUsages(
        &mut self,
        args: *mut crate::UnityEngine::Networking::PlayerConnection::MessageEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnChangeUsages", (args))?;
        Ok(__cordl_ret)
    }
    pub fn OnConnected(
        &mut self,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnConnected", (id))?;
        Ok(__cordl_ret)
    }
    pub fn OnDisconnected(
        &mut self,
        id: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisconnected", (id))?;
        Ok(__cordl_ret)
    }
    pub fn OnNewDevice(
        &mut self,
        args: *mut crate::UnityEngine::Networking::PlayerConnection::MessageEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnNewDevice", (args))?;
        Ok(__cordl_ret)
    }
    pub fn OnNewEvents(
        &mut self,
        args: *mut crate::UnityEngine::Networking::PlayerConnection::MessageEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnNewEvents", (args))?;
        Ok(__cordl_ret)
    }
    pub fn OnNewLayout(
        &mut self,
        args: *mut crate::UnityEngine::Networking::PlayerConnection::MessageEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnNewLayout", (args))?;
        Ok(__cordl_ret)
    }
    pub fn OnRemoveDevice(
        &mut self,
        args: *mut crate::UnityEngine::Networking::PlayerConnection::MessageEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnRemoveDevice", (args))?;
        Ok(__cordl_ret)
    }
    pub fn OnStartSending(
        &mut self,
        args: *mut crate::UnityEngine::Networking::PlayerConnection::MessageEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnStartSending", (args))?;
        Ok(__cordl_ret)
    }
    pub fn OnStopSending(
        &mut self,
        args: *mut crate::UnityEngine::Networking::PlayerConnection::MessageEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnStopSending", (args))?;
        Ok(__cordl_ret)
    }
    pub fn SendToSubscribers(
        &mut self,
        _cordl_type: crate::UnityEngine::InputSystem::InputRemoting_MessageType,
        args: *mut crate::UnityEngine::Networking::PlayerConnection::MessageEventArgs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendToSubscribers", (_cordl_type, args))?;
        Ok(__cordl_ret)
    }
    pub fn Subscribe(
        &mut self,
        observer: *mut crate::System::IObserver_1<
            crate::UnityEngine::InputSystem::InputRemoting_Message,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IDisposable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IDisposable = __cordl_object
            .invoke("Subscribe", (observer))?;
        Ok(__cordl_ret)
    }
    pub fn System_IObserver_UnityEngine_InputSystem_InputRemoting_Message__OnCompleted(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.IObserver<UnityEngine.InputSystem.InputRemoting.Message>.OnCompleted",
                (),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_IObserver_UnityEngine_InputSystem_InputRemoting_Message__OnError(
        &mut self,
        error: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.IObserver<UnityEngine.InputSystem.InputRemoting.Message>.OnError",
                (error),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_IObserver_UnityEngine_InputSystem_InputRemoting_Message__OnNext(
        &mut self,
        msg: crate::UnityEngine::InputSystem::InputRemoting_Message,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.IObserver<UnityEngine.InputSystem.InputRemoting.Message>.OnNext",
                (msg),
            )?;
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
}
#[cfg(feature = "UnityEngine+InputSystem+RemoteInputPlayerConnection")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::RemoteInputPlayerConnection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+RemoteInputPlayerConnection+Subscriber")]
#[repr(C)]
#[derive(Debug)]
pub struct RemoteInputPlayerConnection_Subscriber {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub owner: *mut crate::UnityEngine::InputSystem::RemoteInputPlayerConnection,
    pub observer: *mut crate::System::IObserver_1<
        crate::UnityEngine::InputSystem::InputRemoting_Message,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+RemoteInputPlayerConnection+Subscriber")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::RemoteInputPlayerConnection_Subscriber =>
    "UnityEngine.InputSystem"."RemoteInputPlayerConnection/Subscriber"
);
#[cfg(feature = "UnityEngine+InputSystem+RemoteInputPlayerConnection+Subscriber")]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::RemoteInputPlayerConnection_Subscriber {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+RemoteInputPlayerConnection+Subscriber")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::RemoteInputPlayerConnection_Subscriber {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+RemoteInputPlayerConnection+Subscriber")]
impl crate::UnityEngine::InputSystem::RemoteInputPlayerConnection_Subscriber {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
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
#[cfg(feature = "UnityEngine+InputSystem+RemoteInputPlayerConnection+Subscriber")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::RemoteInputPlayerConnection_Subscriber {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
