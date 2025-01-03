#[cfg(feature = "PartyMessageHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct PartyMessageHandler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _serializer: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::NetworkPacketSerializer_2<
            crate::GlobalNamespace::PartyMessageHandler_MessageType,
            *mut crate::GlobalNamespace::IConnectedPlayer,
        >,
    >,
    pub _connectedPlayerManager: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::ConnectedPlayerManager,
    >,
    pub connectToMasterServerEvent: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::PartyMessageHandler_ConnectToMasterServerDelegate,
    >,
}
#[cfg(feature = "PartyMessageHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PartyMessageHandler => ""
    ."PartyMessageHandler"
);
#[cfg(feature = "PartyMessageHandler")]
impl std::ops::Deref for crate::GlobalNamespace::PartyMessageHandler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PartyMessageHandler")]
impl std::ops::DerefMut for crate::GlobalNamespace::PartyMessageHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PartyMessageHandler")]
impl crate::GlobalNamespace::PartyMessageHandler {
    #[cfg(feature = "PartyMessageHandler+ConnectToMasterServerDelegate")]
    pub type ConnectToMasterServerDelegate = crate::GlobalNamespace::PartyMessageHandler_ConnectToMasterServerDelegate;
    #[cfg(feature = "PartyMessageHandler+ConnectToMasterServerMessage")]
    pub type ConnectToMasterServerMessage = crate::GlobalNamespace::PartyMessageHandler_ConnectToMasterServerMessage;
    #[cfg(feature = "PartyMessageHandler+MessageType")]
    pub type MessageType = crate::GlobalNamespace::PartyMessageHandler_MessageType;
    #[cfg(feature = "PartyMessageHandler+ServerStatusUpdatedDelegate")]
    pub type ServerStatusUpdatedDelegate = crate::GlobalNamespace::PartyMessageHandler_ServerStatusUpdatedDelegate;
    pub fn ConnectToMasterServer(
        &mut self,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConnectToMasterServer", (secret))?;
        Ok(__cordl_ret.into())
    }
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
    pub fn HandleConnectToMasterServer(
        &mut self,
        packet: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PartyMessageHandler_ConnectToMasterServerMessage,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HandleConnectToMasterServer", (packet))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        connectedPlayerManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (connectedPlayerManager))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        connectedPlayerManager: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::ConnectedPlayerManager,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (connectedPlayerManager))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_connectToMasterServerEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PartyMessageHandler_ConnectToMasterServerDelegate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_connectToMasterServerEvent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_connectToMasterServerEvent(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PartyMessageHandler_ConnectToMasterServerDelegate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_connectToMasterServerEvent", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PartyMessageHandler")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::PartyMessageHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PartyMessageHandler")]
impl AsRef<crate::System::IDisposable> for crate::GlobalNamespace::PartyMessageHandler {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "PartyMessageHandler")]
impl AsMut<crate::System::IDisposable> for crate::GlobalNamespace::PartyMessageHandler {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "PartyMessageHandler+ConnectToMasterServerDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct PartyMessageHandler_ConnectToMasterServerDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "PartyMessageHandler+ConnectToMasterServerDelegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PartyMessageHandler_ConnectToMasterServerDelegate => ""
    ."PartyMessageHandler/ConnectToMasterServerDelegate"
);
#[cfg(feature = "PartyMessageHandler+ConnectToMasterServerDelegate")]
impl std::ops::Deref
for crate::GlobalNamespace::PartyMessageHandler_ConnectToMasterServerDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PartyMessageHandler+ConnectToMasterServerDelegate")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PartyMessageHandler_ConnectToMasterServerDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PartyMessageHandler+ConnectToMasterServerDelegate")]
impl crate::GlobalNamespace::PartyMessageHandler_ConnectToMasterServerDelegate {
    pub fn BeginInvoke(
        &mut self,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (secret, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (secret))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PartyMessageHandler+ConnectToMasterServerDelegate")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PartyMessageHandler_ConnectToMasterServerDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PartyMessageHandler+ConnectToMasterServerMessage")]
#[repr(C)]
#[derive(Debug)]
pub struct PartyMessageHandler_ConnectToMasterServerMessage {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "PartyMessageHandler+ConnectToMasterServerMessage")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PartyMessageHandler_ConnectToMasterServerMessage => ""
    ."PartyMessageHandler/ConnectToMasterServerMessage"
);
#[cfg(feature = "PartyMessageHandler+ConnectToMasterServerMessage")]
impl std::ops::Deref
for crate::GlobalNamespace::PartyMessageHandler_ConnectToMasterServerMessage {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PartyMessageHandler+ConnectToMasterServerMessage")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PartyMessageHandler_ConnectToMasterServerMessage {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PartyMessageHandler+ConnectToMasterServerMessage")]
impl crate::GlobalNamespace::PartyMessageHandler_ConnectToMasterServerMessage {
    pub fn Deserialize(
        &mut self,
        reader: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataReader>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Deserialize", (reader))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        secret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PartyMessageHandler_ConnectToMasterServerMessage,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PartyMessageHandler_ConnectToMasterServerMessage,
        > = __cordl_object.invoke("Init", (secret))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Release(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Serialize(
        &mut self,
        writer: quest_hook::libil2cpp::Gc<crate::LiteNetLib::Utils::NetDataWriter>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Serialize", (writer))?;
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
    pub fn get_pool() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PacketPool_1<
                *mut crate::GlobalNamespace::PartyMessageHandler_ConnectToMasterServerMessage,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::PacketPool_1<
                *mut crate::GlobalNamespace::PartyMessageHandler_ConnectToMasterServerMessage,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_pool", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PartyMessageHandler+ConnectToMasterServerMessage")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PartyMessageHandler_ConnectToMasterServerMessage {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "PartyMessageHandler+ConnectToMasterServerMessage")]
impl AsRef<crate::GlobalNamespace::IPoolablePacket>
for crate::GlobalNamespace::PartyMessageHandler_ConnectToMasterServerMessage {
    fn as_ref(&self) -> &crate::GlobalNamespace::IPoolablePacket {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "PartyMessageHandler+ConnectToMasterServerMessage")]
impl AsMut<crate::GlobalNamespace::IPoolablePacket>
for crate::GlobalNamespace::PartyMessageHandler_ConnectToMasterServerMessage {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::IPoolablePacket {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "PartyMessageHandler+ConnectToMasterServerMessage")]
impl AsRef<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::PartyMessageHandler_ConnectToMasterServerMessage {
    fn as_ref(&self) -> &crate::LiteNetLib::Utils::INetSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "PartyMessageHandler+ConnectToMasterServerMessage")]
impl AsMut<crate::LiteNetLib::Utils::INetSerializable>
for crate::GlobalNamespace::PartyMessageHandler_ConnectToMasterServerMessage {
    fn as_mut(&mut self) -> &mut crate::LiteNetLib::Utils::INetSerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "PartyMessageHandler+MessageType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartyMessageHandler_MessageType {
    ConnectToMasterServer = 0i32,
}
#[cfg(feature = "PartyMessageHandler+MessageType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::PartyMessageHandler_MessageType
    => ""."PartyMessageHandler/MessageType"
);
#[cfg(feature = "PartyMessageHandler+ServerStatusUpdatedDelegate")]
#[repr(C)]
#[derive(Debug)]
pub struct PartyMessageHandler_ServerStatusUpdatedDelegate {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "PartyMessageHandler+ServerStatusUpdatedDelegate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::PartyMessageHandler_ServerStatusUpdatedDelegate => ""
    ."PartyMessageHandler/ServerStatusUpdatedDelegate"
);
#[cfg(feature = "PartyMessageHandler+ServerStatusUpdatedDelegate")]
impl std::ops::Deref
for crate::GlobalNamespace::PartyMessageHandler_ServerStatusUpdatedDelegate {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "PartyMessageHandler+ServerStatusUpdatedDelegate")]
impl std::ops::DerefMut
for crate::GlobalNamespace::PartyMessageHandler_ServerStatusUpdatedDelegate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "PartyMessageHandler+ServerStatusUpdatedDelegate")]
impl crate::GlobalNamespace::PartyMessageHandler_ServerStatusUpdatedDelegate {
    pub fn BeginInvoke(
        &mut self,
        selectionMask: crate::GlobalNamespace::BeatmapLevelSelectionMask,
        configuration: crate::GlobalNamespace::GameplayServerConfiguration,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (selectionMask, configuration, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        selectionMask: crate::GlobalNamespace::BeatmapLevelSelectionMask,
        configuration: crate::GlobalNamespace::GameplayServerConfiguration,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", (selectionMask, configuration))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "PartyMessageHandler+ServerStatusUpdatedDelegate")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::PartyMessageHandler_ServerStatusUpdatedDelegate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
