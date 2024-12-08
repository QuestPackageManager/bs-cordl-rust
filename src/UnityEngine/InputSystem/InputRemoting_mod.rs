#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+ChangeUsageMsg")]
#[repr(C)]
#[derive(Debug)]
pub struct InputRemoting_ChangeUsageMsg {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+ChangeUsageMsg")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputRemoting_ChangeUsageMsg => "UnityEngine.InputSystem"
    ."InputRemoting/ChangeUsageMsg"
);
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+ChangeUsageMsg")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::InputRemoting_ChangeUsageMsg {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+ChangeUsageMsg")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::InputRemoting_ChangeUsageMsg {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+ChangeUsageMsg")]
impl crate::UnityEngine::InputSystem::InputRemoting_ChangeUsageMsg {
    #[cfg(feature = "UnityEngine+InputSystem+InputRemoting+ChangeUsageMsg+Data")]
    pub type Data = crate::UnityEngine::InputSystem::ChangeUsageMsg_Data;
    #[cfg(feature = "UnityEngine+InputSystem+InputRemoting+ChangeUsageMsg+__c")]
    pub type __c = crate::UnityEngine::InputSystem::ChangeUsageMsg___c;
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+ChangeUsageMsg")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::InputRemoting_ChangeUsageMsg {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+ConnectMsg")]
#[repr(C)]
#[derive(Debug)]
pub struct InputRemoting_ConnectMsg {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+ConnectMsg")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputRemoting_ConnectMsg => "UnityEngine.InputSystem"
    ."InputRemoting/ConnectMsg"
);
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+ConnectMsg")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::InputRemoting_ConnectMsg {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+ConnectMsg")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::InputRemoting_ConnectMsg {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+ConnectMsg")]
impl crate::UnityEngine::InputSystem::InputRemoting_ConnectMsg {}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+ConnectMsg")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::InputRemoting_ConnectMsg {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+NewLayoutMsg+Data")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct NewLayoutMsg_Data {
    pub name: *mut crate::System::String,
    pub layoutJson: *mut crate::System::String,
    pub isOverride: bool,
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+NewLayoutMsg+Data")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::NewLayoutMsg_Data =>
    "UnityEngine.InputSystem"."InputRemoting/NewLayoutMsg/Data"
);
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+NewLayoutMsg+Data")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::NewLayoutMsg_Data {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+NewLayoutMsg+Data")]
impl crate::UnityEngine::InputSystem::NewLayoutMsg_Data {}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+NewDeviceMsg+Data")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct NewDeviceMsg_Data {
    pub name: *mut crate::System::String,
    pub layout: *mut crate::System::String,
    pub deviceId: i32,
    pub usages: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
    pub description: crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+NewDeviceMsg+Data")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::NewDeviceMsg_Data =>
    "UnityEngine.InputSystem"."InputRemoting/NewDeviceMsg/Data"
);
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+NewDeviceMsg+Data")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::NewDeviceMsg_Data {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+NewDeviceMsg+Data")]
impl crate::UnityEngine::InputSystem::NewDeviceMsg_Data {}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+ChangeUsageMsg+Data")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ChangeUsageMsg_Data {
    pub deviceId: i32,
    pub usages: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::String>,
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+ChangeUsageMsg+Data")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::ChangeUsageMsg_Data =>
    "UnityEngine.InputSystem"."InputRemoting/ChangeUsageMsg/Data"
);
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+ChangeUsageMsg+Data")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::ChangeUsageMsg_Data {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+ChangeUsageMsg+Data")]
impl crate::UnityEngine::InputSystem::ChangeUsageMsg_Data {}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+DisconnectMsg")]
#[repr(C)]
#[derive(Debug)]
pub struct InputRemoting_DisconnectMsg {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+DisconnectMsg")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputRemoting_DisconnectMsg => "UnityEngine.InputSystem"
    ."InputRemoting/DisconnectMsg"
);
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+DisconnectMsg")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::InputRemoting_DisconnectMsg {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+DisconnectMsg")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::InputRemoting_DisconnectMsg {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+DisconnectMsg")]
impl crate::UnityEngine::InputSystem::InputRemoting_DisconnectMsg {}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+DisconnectMsg")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::InputRemoting_DisconnectMsg {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+Flags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputRemoting_Flags {
    Sending = 1i32,
    StartSendingOnConnect = 2i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+Flags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::InputRemoting_Flags =>
    "UnityEngine.InputSystem"."InputRemoting/Flags"
);
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting")]
#[repr(C)]
#[derive(Debug)]
pub struct InputRemoting {
    __cordl_parent: crate::System::Object,
    pub m_Flags: crate::UnityEngine::InputSystem::InputRemoting_Flags,
    pub m_LocalManager: *mut crate::UnityEngine::InputSystem::InputManager,
    pub m_Subscribers: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::InputSystem::InputRemoting_Subscriber,
    >,
    pub m_Senders: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::InputSystem::InputRemoting_RemoteSender,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::InputRemoting =>
    "UnityEngine.InputSystem"."InputRemoting"
);
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::InputRemoting {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::InputRemoting {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting")]
impl crate::UnityEngine::InputSystem::InputRemoting {
    #[cfg(feature = "UnityEngine+InputSystem+InputRemoting+MessageType")]
    pub type MessageType = crate::UnityEngine::InputSystem::InputRemoting_MessageType;
    #[cfg(feature = "UnityEngine+InputSystem+InputRemoting+Subscriber")]
    pub type Subscriber = crate::UnityEngine::InputSystem::InputRemoting_Subscriber;
    #[cfg(feature = "UnityEngine+InputSystem+InputRemoting+ConnectMsg")]
    pub type ConnectMsg = crate::UnityEngine::InputSystem::InputRemoting_ConnectMsg;
    #[cfg(feature = "UnityEngine+InputSystem+InputRemoting+NewEventsMsg")]
    pub type NewEventsMsg = crate::UnityEngine::InputSystem::InputRemoting_NewEventsMsg;
    #[cfg(feature = "UnityEngine+InputSystem+InputRemoting+RemoteSender")]
    pub type RemoteSender = crate::UnityEngine::InputSystem::InputRemoting_RemoteSender;
    #[cfg(feature = "UnityEngine+InputSystem+InputRemoting+StartSendingMsg")]
    pub type StartSendingMsg = crate::UnityEngine::InputSystem::InputRemoting_StartSendingMsg;
    #[cfg(feature = "UnityEngine+InputSystem+InputRemoting+DisconnectMsg")]
    pub type DisconnectMsg = crate::UnityEngine::InputSystem::InputRemoting_DisconnectMsg;
    #[cfg(feature = "UnityEngine+InputSystem+InputRemoting+RemoveDeviceMsg")]
    pub type RemoveDeviceMsg = crate::UnityEngine::InputSystem::InputRemoting_RemoveDeviceMsg;
    #[cfg(feature = "UnityEngine+InputSystem+InputRemoting+NewLayoutMsg")]
    pub type NewLayoutMsg = crate::UnityEngine::InputSystem::InputRemoting_NewLayoutMsg;
    #[cfg(feature = "UnityEngine+InputSystem+InputRemoting+StopSendingMsg")]
    pub type StopSendingMsg = crate::UnityEngine::InputSystem::InputRemoting_StopSendingMsg;
    #[cfg(feature = "UnityEngine+InputSystem+InputRemoting+NewDeviceMsg")]
    pub type NewDeviceMsg = crate::UnityEngine::InputSystem::InputRemoting_NewDeviceMsg;
    #[cfg(feature = "UnityEngine+InputSystem+InputRemoting+ChangeUsageMsg")]
    pub type ChangeUsageMsg = crate::UnityEngine::InputSystem::InputRemoting_ChangeUsageMsg;
    #[cfg(feature = "UnityEngine+InputSystem+InputRemoting+Flags")]
    pub type Flags = crate::UnityEngine::InputSystem::InputRemoting_Flags;
    #[cfg(feature = "UnityEngine+InputSystem+InputRemoting+Message")]
    pub type Message = crate::UnityEngine::InputSystem::InputRemoting_Message;
    #[cfg(feature = "UnityEngine+InputSystem+InputRemoting+RemoteInputDevice")]
    pub type RemoteInputDevice = crate::UnityEngine::InputSystem::InputRemoting_RemoteInputDevice;
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
    pub fn SendDevice(
        &mut self,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendDevice", (device))?;
        Ok(__cordl_ret)
    }
    pub fn SendEvent(
        &mut self,
        eventPtr: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendEvent", (eventPtr, device))?;
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
    pub fn FindOrCreateSenderRecord(
        &mut self,
        senderId: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("FindOrCreateSenderRecord", (senderId))?;
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
    pub fn SendLayout(
        &mut self,
        layoutName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendLayout", (layoutName))?;
        Ok(__cordl_ret)
    }
    pub fn SendAllDevices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendAllDevices", ())?;
        Ok(__cordl_ret)
    }
    pub fn SendDeviceChange(
        &mut self,
        device: *mut crate::UnityEngine::InputSystem::InputDevice,
        change: crate::UnityEngine::InputSystem::InputDeviceChange,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendDeviceChange", (device, change))?;
        Ok(__cordl_ret)
    }
    pub fn set_sending(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_sending", (value))?;
        Ok(__cordl_ret)
    }
    pub fn FindLocalDeviceId(
        &mut self,
        remoteDeviceId: i32,
        senderIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("FindLocalDeviceId", (remoteDeviceId, senderIndex))?;
        Ok(__cordl_ret)
    }
    pub fn SendAllGeneratedLayouts(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendAllGeneratedLayouts", ())?;
        Ok(__cordl_ret)
    }
    pub fn SendLayoutChange(
        &mut self,
        layout: *mut crate::System::String,
        change: crate::UnityEngine::InputSystem::InputControlLayoutChange,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendLayoutChange", (layout, change))?;
        Ok(__cordl_ret)
    }
    pub fn get_sending(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_sending", ())?;
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
    pub fn _ctor(
        &mut self,
        manager: *mut crate::UnityEngine::InputSystem::InputManager,
        startSendingOnConnect: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (manager, startSendingOnConnect))?;
        Ok(__cordl_ret)
    }
    pub fn StartSending(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartSending", ())?;
        Ok(__cordl_ret)
    }
    pub fn Send(
        &mut self,
        msg: crate::UnityEngine::InputSystem::InputRemoting_Message,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Send", (msg))?;
        Ok(__cordl_ret)
    }
    pub fn TryGetDeviceByRemoteId(
        &mut self,
        remoteDeviceId: i32,
        senderIndex: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputDevice,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputDevice = __cordl_object
            .invoke("TryGetDeviceByRemoteId", (remoteDeviceId, senderIndex))?;
        Ok(__cordl_ret)
    }
    pub fn StopSending(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopSending", ())?;
        Ok(__cordl_ret)
    }
    pub fn RemoveRemoteDevices(
        &mut self,
        participantId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveRemoteDevices", (participantId))?;
        Ok(__cordl_ret)
    }
    pub fn get_manager(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputManager,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputManager = __cordl_object
            .invoke("get_manager", ())?;
        Ok(__cordl_ret)
    }
    pub fn SendInitialMessages(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendInitialMessages", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        manager: *mut crate::UnityEngine::InputSystem::InputManager,
        startSendingOnConnect: bool,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (manager, startSendingOnConnect))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::InputRemoting {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+Message")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputRemoting_Message {
    pub participantId: i32,
    pub _cordl_type: crate::UnityEngine::InputSystem::InputRemoting_MessageType,
    pub data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+Message")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::InputRemoting_Message
    => "UnityEngine.InputSystem"."InputRemoting/Message"
);
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+Message")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputRemoting_Message {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+Message")]
impl crate::UnityEngine::InputSystem::InputRemoting_Message {}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+MessageType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputRemoting_MessageType {
    ChangeUsages = 7i32,
    Connect = 0i32,
    Disconnect = 1i32,
    NewDevice = 3i32,
    NewEvents = 4i32,
    NewLayout = 2i32,
    RemoveDevice = 5i32,
    RemoveLayout = 6i32,
    StartSending = 8i32,
    StopSending = 9i32,
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+MessageType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputRemoting_MessageType => "UnityEngine.InputSystem"
    ."InputRemoting/MessageType"
);
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+NewDeviceMsg")]
#[repr(C)]
#[derive(Debug)]
pub struct InputRemoting_NewDeviceMsg {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+NewDeviceMsg")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputRemoting_NewDeviceMsg => "UnityEngine.InputSystem"
    ."InputRemoting/NewDeviceMsg"
);
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+NewDeviceMsg")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::InputRemoting_NewDeviceMsg {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+NewDeviceMsg")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::InputRemoting_NewDeviceMsg {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+NewDeviceMsg")]
impl crate::UnityEngine::InputSystem::InputRemoting_NewDeviceMsg {
    #[cfg(feature = "UnityEngine+InputSystem+InputRemoting+NewDeviceMsg+Data")]
    pub type Data = crate::UnityEngine::InputSystem::NewDeviceMsg_Data;
    #[cfg(feature = "UnityEngine+InputSystem+InputRemoting+NewDeviceMsg+__c")]
    pub type __c = crate::UnityEngine::InputSystem::NewDeviceMsg___c;
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+NewDeviceMsg")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::InputRemoting_NewDeviceMsg {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+NewEventsMsg")]
#[repr(C)]
#[derive(Debug)]
pub struct InputRemoting_NewEventsMsg {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+NewEventsMsg")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputRemoting_NewEventsMsg => "UnityEngine.InputSystem"
    ."InputRemoting/NewEventsMsg"
);
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+NewEventsMsg")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::InputRemoting_NewEventsMsg {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+NewEventsMsg")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::InputRemoting_NewEventsMsg {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+NewEventsMsg")]
impl crate::UnityEngine::InputSystem::InputRemoting_NewEventsMsg {}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+NewEventsMsg")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::InputRemoting_NewEventsMsg {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+NewLayoutMsg")]
#[repr(C)]
#[derive(Debug)]
pub struct InputRemoting_NewLayoutMsg {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+NewLayoutMsg")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputRemoting_NewLayoutMsg => "UnityEngine.InputSystem"
    ."InputRemoting/NewLayoutMsg"
);
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+NewLayoutMsg")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::InputRemoting_NewLayoutMsg {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+NewLayoutMsg")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::InputRemoting_NewLayoutMsg {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+NewLayoutMsg")]
impl crate::UnityEngine::InputSystem::InputRemoting_NewLayoutMsg {
    #[cfg(feature = "UnityEngine+InputSystem+InputRemoting+NewLayoutMsg+Data")]
    pub type Data = crate::UnityEngine::InputSystem::NewLayoutMsg_Data;
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+NewLayoutMsg")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::InputRemoting_NewLayoutMsg {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+RemoteInputDevice")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputRemoting_RemoteInputDevice {
    pub remoteId: i32,
    pub localId: i32,
    pub description: crate::UnityEngine::InputSystem::Layouts::InputDeviceDescription,
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+RemoteInputDevice")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputRemoting_RemoteInputDevice =>
    "UnityEngine.InputSystem"."InputRemoting/RemoteInputDevice"
);
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+RemoteInputDevice")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputRemoting_RemoteInputDevice {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+RemoteInputDevice")]
impl crate::UnityEngine::InputSystem::InputRemoting_RemoteInputDevice {}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+RemoteSender")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputRemoting_RemoteSender {
    pub senderId: i32,
    pub layouts: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::InputSystem::Utilities::InternedString,
    >,
    pub devices: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::InputSystem::InputRemoting_RemoteInputDevice,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+RemoteSender")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputRemoting_RemoteSender => "UnityEngine.InputSystem"
    ."InputRemoting/RemoteSender"
);
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+RemoteSender")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputRemoting_RemoteSender {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+RemoteSender")]
impl crate::UnityEngine::InputSystem::InputRemoting_RemoteSender {}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+RemoveDeviceMsg")]
#[repr(C)]
#[derive(Debug)]
pub struct InputRemoting_RemoveDeviceMsg {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+RemoveDeviceMsg")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputRemoting_RemoveDeviceMsg =>
    "UnityEngine.InputSystem"."InputRemoting/RemoveDeviceMsg"
);
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+RemoveDeviceMsg")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::InputRemoting_RemoveDeviceMsg {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+RemoveDeviceMsg")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::InputRemoting_RemoveDeviceMsg {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+RemoveDeviceMsg")]
impl crate::UnityEngine::InputSystem::InputRemoting_RemoveDeviceMsg {}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+RemoveDeviceMsg")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::InputRemoting_RemoveDeviceMsg {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+StartSendingMsg")]
#[repr(C)]
#[derive(Debug)]
pub struct InputRemoting_StartSendingMsg {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+StartSendingMsg")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputRemoting_StartSendingMsg =>
    "UnityEngine.InputSystem"."InputRemoting/StartSendingMsg"
);
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+StartSendingMsg")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::InputRemoting_StartSendingMsg {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+StartSendingMsg")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::InputRemoting_StartSendingMsg {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+StartSendingMsg")]
impl crate::UnityEngine::InputSystem::InputRemoting_StartSendingMsg {}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+StartSendingMsg")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::InputRemoting_StartSendingMsg {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+StopSendingMsg")]
#[repr(C)]
#[derive(Debug)]
pub struct InputRemoting_StopSendingMsg {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+StopSendingMsg")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputRemoting_StopSendingMsg => "UnityEngine.InputSystem"
    ."InputRemoting/StopSendingMsg"
);
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+StopSendingMsg")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::InputRemoting_StopSendingMsg {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+StopSendingMsg")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::InputRemoting_StopSendingMsg {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+StopSendingMsg")]
impl crate::UnityEngine::InputSystem::InputRemoting_StopSendingMsg {}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+StopSendingMsg")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::InputRemoting_StopSendingMsg {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+Subscriber")]
#[repr(C)]
#[derive(Debug)]
pub struct InputRemoting_Subscriber {
    __cordl_parent: crate::System::Object,
    pub owner: *mut crate::UnityEngine::InputSystem::InputRemoting,
    pub observer: *mut crate::System::IObserver_1<
        crate::UnityEngine::InputSystem::InputRemoting_Message,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+Subscriber")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::InputRemoting_Subscriber => "UnityEngine.InputSystem"
    ."InputRemoting/Subscriber"
);
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+Subscriber")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::InputRemoting_Subscriber {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+Subscriber")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::InputRemoting_Subscriber {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+Subscriber")]
impl crate::UnityEngine::InputSystem::InputRemoting_Subscriber {
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputRemoting+Subscriber")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::InputRemoting_Subscriber {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
