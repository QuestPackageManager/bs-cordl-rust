#[cfg(feature = "UnityEngine+InputSystem+RemoteInputPlayerConnection")]
#[repr(C)]
#[derive(Debug)]
pub struct RemoteInputPlayerConnection {
    __cordl_parent: crate::UnityEngine::ScriptableObject,
    pub m_Connection: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Networking::PlayerConnection::IEditorPlayerConnection,
    >,
    pub m_Subscribers: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<
                crate::UnityEngine::InputSystem::RemoteInputPlayerConnection_Subscriber,
            >,
        >,
    >,
    pub m_ConnectedIds: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<i32>,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+RemoteInputPlayerConnection")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::RemoteInputPlayerConnection {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "RemoteInputPlayerConnection";
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
        connection: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::PlayerConnection::IEditorPlayerConnection,
        >,
        isConnected: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Bind", (connection, isConnected))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnChangeUsages(
        &mut self,
        args: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::PlayerConnection::MessageEventArgs,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnChangeUsages", (args))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn OnNewDevice(
        &mut self,
        args: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::PlayerConnection::MessageEventArgs,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnNewDevice", (args))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnNewEvents(
        &mut self,
        args: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::PlayerConnection::MessageEventArgs,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnNewEvents", (args))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnNewLayout(
        &mut self,
        args: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::PlayerConnection::MessageEventArgs,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnNewLayout", (args))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnRemoveDevice(
        &mut self,
        args: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::PlayerConnection::MessageEventArgs,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnRemoveDevice", (args))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnStartSending(
        &mut self,
        args: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::PlayerConnection::MessageEventArgs,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnStartSending", (args))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnStopSending(
        &mut self,
        args: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::PlayerConnection::MessageEventArgs,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnStopSending", (args))?;
        Ok(__cordl_ret.into())
    }
    pub fn SendToSubscribers(
        &mut self,
        _cordl_type: crate::UnityEngine::InputSystem::InputRemoting_MessageType,
        args: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Networking::PlayerConnection::MessageEventArgs,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendToSubscribers", (_cordl_type, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn Subscribe(
        &mut self,
        observer: quest_hook::libil2cpp::Gc<
            crate::System::IObserver_1<
                crate::UnityEngine::InputSystem::InputRemoting_Message,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IDisposable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IDisposable> = __cordl_object
            .invoke("Subscribe", (observer))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn System_IObserver_UnityEngine_InputSystem_InputRemoting_Message__OnError(
        &mut self,
        error: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.IObserver<UnityEngine.InputSystem.InputRemoting.Message>.OnError",
                (error),
            )?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "UnityEngine+InputSystem+RemoteInputPlayerConnection")]
impl AsRef<
    crate::System::IObservable_1<crate::UnityEngine::InputSystem::InputRemoting_Message>,
> for crate::UnityEngine::InputSystem::RemoteInputPlayerConnection {
    fn as_ref(
        &self,
    ) -> &crate::System::IObservable_1<
        crate::UnityEngine::InputSystem::InputRemoting_Message,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+RemoteInputPlayerConnection")]
impl AsMut<
    crate::System::IObservable_1<crate::UnityEngine::InputSystem::InputRemoting_Message>,
> for crate::UnityEngine::InputSystem::RemoteInputPlayerConnection {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IObservable_1<
        crate::UnityEngine::InputSystem::InputRemoting_Message,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+RemoteInputPlayerConnection")]
impl AsRef<
    crate::System::IObserver_1<crate::UnityEngine::InputSystem::InputRemoting_Message>,
> for crate::UnityEngine::InputSystem::RemoteInputPlayerConnection {
    fn as_ref(
        &self,
    ) -> &crate::System::IObserver_1<
        crate::UnityEngine::InputSystem::InputRemoting_Message,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+RemoteInputPlayerConnection")]
impl AsMut<
    crate::System::IObserver_1<crate::UnityEngine::InputSystem::InputRemoting_Message>,
> for crate::UnityEngine::InputSystem::RemoteInputPlayerConnection {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IObserver_1<
        crate::UnityEngine::InputSystem::InputRemoting_Message,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+RemoteInputPlayerConnection+Subscriber")]
#[repr(C)]
#[derive(Debug)]
pub struct RemoteInputPlayerConnection_Subscriber {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub owner: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::RemoteInputPlayerConnection,
    >,
    pub observer: quest_hook::libil2cpp::Gc<
        crate::System::IObserver_1<
            crate::UnityEngine::InputSystem::InputRemoting_Message,
        >,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+RemoteInputPlayerConnection+Subscriber")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::RemoteInputPlayerConnection_Subscriber {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "RemoteInputPlayerConnection/Subscriber";
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
        Ok(__cordl_ret.into())
    }
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
#[cfg(feature = "UnityEngine+InputSystem+RemoteInputPlayerConnection+Subscriber")]
impl AsRef<crate::System::IDisposable>
for crate::UnityEngine::InputSystem::RemoteInputPlayerConnection_Subscriber {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+RemoteInputPlayerConnection+Subscriber")]
impl AsMut<crate::System::IDisposable>
for crate::UnityEngine::InputSystem::RemoteInputPlayerConnection_Subscriber {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
