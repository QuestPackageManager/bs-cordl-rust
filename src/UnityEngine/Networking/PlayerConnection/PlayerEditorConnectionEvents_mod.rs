#[cfg(feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents")]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerEditorConnectionEvents {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub messageTypeSubscribers: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_MessageTypeSubscribers,
        >,
    >,
    pub connectionEvent: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_ConnectionChangeEvent,
    >,
    pub disconnectionEvent: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_ConnectionChangeEvent,
    >,
}
#[cfg(feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents =>
    "UnityEngine.Networking.PlayerConnection"."PlayerEditorConnectionEvents"
);
#[cfg(feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents")]
impl std::ops::Deref
for crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents")]
impl std::ops::DerefMut
for crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents")]
impl crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents {
    #[cfg(
        feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+ConnectionChangeEvent"
    )]
    pub type ConnectionChangeEvent = crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_ConnectionChangeEvent;
    #[cfg(
        feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+MessageEvent"
    )]
    pub type MessageEvent = crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_MessageEvent;
    #[cfg(
        feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+MessageTypeSubscribers"
    )]
    pub type MessageTypeSubscribers = crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_MessageTypeSubscribers;
    pub fn AddAndCreate(
        &mut self,
        messageId: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityEvent_1<
                *mut crate::UnityEngine::Networking::PlayerConnection::MessageEventArgs,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityEvent_1<
                *mut crate::UnityEngine::Networking::PlayerConnection::MessageEventArgs,
            >,
        > = __cordl_object.invoke("AddAndCreate", (messageId))?;
        Ok(__cordl_ret.into())
    }
    pub fn InvokeMessageIdSubscribers(
        &mut self,
        messageId: crate::System::Guid,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        playerId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeMessageIdSubscribers", (messageId, data, playerId))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UnregisterManagedCallback(
        &mut self,
        messageId: crate::System::Guid,
        callback: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Events::UnityAction_1<
                *mut crate::UnityEngine::Networking::PlayerConnection::MessageEventArgs,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterManagedCallback", (messageId, callback))?;
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
#[cfg(feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+ConnectionChangeEvent"
)]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerEditorConnectionEvents_ConnectionChangeEvent {
    __cordl_parent: crate::UnityEngine::Events::UnityEvent_1<i32>,
}
#[cfg(
    feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+ConnectionChangeEvent"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_ConnectionChangeEvent
    => "UnityEngine.Networking.PlayerConnection"
    ."PlayerEditorConnectionEvents/ConnectionChangeEvent"
);
#[cfg(
    feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+ConnectionChangeEvent"
)]
impl std::ops::Deref
for crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_ConnectionChangeEvent {
    type Target = crate::UnityEngine::Events::UnityEvent_1<i32>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+ConnectionChangeEvent"
)]
impl std::ops::DerefMut
for crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_ConnectionChangeEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+ConnectionChangeEvent"
)]
impl crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_ConnectionChangeEvent {
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
#[cfg(
    feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+ConnectionChangeEvent"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_ConnectionChangeEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+MessageEvent"
)]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerEditorConnectionEvents_MessageEvent {
    __cordl_parent: crate::UnityEngine::Events::UnityEvent_1<
        *mut crate::UnityEngine::Networking::PlayerConnection::MessageEventArgs,
    >,
}
#[cfg(
    feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+MessageEvent"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_MessageEvent
    => "UnityEngine.Networking.PlayerConnection"
    ."PlayerEditorConnectionEvents/MessageEvent"
);
#[cfg(
    feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+MessageEvent"
)]
impl std::ops::Deref
for crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_MessageEvent {
    type Target = crate::UnityEngine::Events::UnityEvent_1<
        *mut crate::UnityEngine::Networking::PlayerConnection::MessageEventArgs,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+MessageEvent"
)]
impl std::ops::DerefMut
for crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_MessageEvent {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+MessageEvent"
)]
impl crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_MessageEvent {
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
#[cfg(
    feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+MessageEvent"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_MessageEvent {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+MessageTypeSubscribers"
)]
#[repr(C)]
#[derive(Debug)]
pub struct PlayerEditorConnectionEvents_MessageTypeSubscribers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_messageTypeId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub subscriberCount: i32,
    pub messageCallback: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_MessageEvent,
    >,
}
#[cfg(
    feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+MessageTypeSubscribers"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_MessageTypeSubscribers
    => "UnityEngine.Networking.PlayerConnection"
    ."PlayerEditorConnectionEvents/MessageTypeSubscribers"
);
#[cfg(
    feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+MessageTypeSubscribers"
)]
impl std::ops::Deref
for crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_MessageTypeSubscribers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+MessageTypeSubscribers"
)]
impl std::ops::DerefMut
for crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_MessageTypeSubscribers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+MessageTypeSubscribers"
)]
impl crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_MessageTypeSubscribers {
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
    pub fn get_MessageTypeId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Guid> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Guid = __cordl_object
            .invoke("get_MessageTypeId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_MessageTypeId(
        &mut self,
        value: crate::System::Guid,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MessageTypeId", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+Networking+PlayerConnection+PlayerEditorConnectionEvents+MessageTypeSubscribers"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::Networking::PlayerConnection::PlayerEditorConnectionEvents_MessageTypeSubscribers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
