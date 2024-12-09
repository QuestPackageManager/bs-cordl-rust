#[cfg(feature = "OVRNetwork")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRNetwork {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "OVRNetwork")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRNetwork => ""."OVRNetwork"
);
#[cfg(feature = "OVRNetwork")]
impl std::ops::Deref for crate::GlobalNamespace::OVRNetwork {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRNetwork")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRNetwork {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRNetwork")]
impl crate::GlobalNamespace::OVRNetwork {
    pub const FrameHeaderMagicIdentifier: u32 = 2208787440u32;
    pub const MaxBufferLength: i32 = 65536i32;
    pub const MaxPayloadLength: i32 = 65524i32;
    #[cfg(feature = "OVRNetwork+FrameHeader")]
    pub type FrameHeader = crate::GlobalNamespace::OVRNetwork_FrameHeader;
    #[cfg(feature = "OVRNetwork+OVRNetworkTcpClient")]
    pub type OVRNetworkTcpClient = crate::GlobalNamespace::OVRNetwork_OVRNetworkTcpClient;
    #[cfg(feature = "OVRNetwork+OVRNetworkTcpServer")]
    pub type OVRNetworkTcpServer = crate::GlobalNamespace::OVRNetwork_OVRNetworkTcpServer;
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
#[cfg(feature = "OVRNetwork")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRNetwork {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRNetwork+OVRNetworkTcpClient+ConnectionState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRNetworkTcpClient_OVRNetwork_ConnectionState {
    Connected = 1i32,
    Connecting = 2i32,
    Disconnected = 0i32,
}
#[cfg(feature = "OVRNetwork+OVRNetworkTcpClient+ConnectionState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRNetworkTcpClient_OVRNetwork_ConnectionState => ""
    ."OVRNetwork/OVRNetworkTcpClient/ConnectionState"
);
#[cfg(feature = "OVRNetwork+FrameHeader")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRNetwork_FrameHeader {
    pub protocolIdentifier: u32,
    pub payloadType: i32,
    pub payloadLength: i32,
}
#[cfg(feature = "OVRNetwork+FrameHeader")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRNetwork_FrameHeader => ""
    ."OVRNetwork/FrameHeader"
);
#[cfg(feature = "OVRNetwork+FrameHeader")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRNetwork_FrameHeader {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRNetwork+FrameHeader")]
impl crate::GlobalNamespace::OVRNetwork_FrameHeader {
    pub const StructSize: i32 = 12i32;
    pub fn ToBytes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToBytes",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRNetwork+OVRNetworkTcpClient")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRNetwork_OVRNetworkTcpClient {
    __cordl_parent: crate::System::Object,
    pub connectionStateChangedCallback: *mut crate::System::Action,
    pub payloadReceivedCallback: *mut crate::System::Action_4<
        i32,
        *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        i32,
        i32,
    >,
    pub tcpClient: *mut crate::System::Net::Sockets::TcpClient,
    pub receivedBuffers: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    >,
    pub receivedBufferIndex: i32,
    pub receivedBufferDataSize: i32,
    pub readyReceiveDataEvent: *mut crate::System::Threading::ManualResetEvent,
}
#[cfg(feature = "OVRNetwork+OVRNetworkTcpClient")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRNetwork_OVRNetworkTcpClient
    => ""."OVRNetwork/OVRNetworkTcpClient"
);
#[cfg(feature = "OVRNetwork+OVRNetworkTcpClient")]
impl std::ops::Deref for crate::GlobalNamespace::OVRNetwork_OVRNetworkTcpClient {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRNetwork+OVRNetworkTcpClient")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRNetwork_OVRNetworkTcpClient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRNetwork+OVRNetworkTcpClient")]
impl crate::GlobalNamespace::OVRNetwork_OVRNetworkTcpClient {
    #[cfg(feature = "OVRNetwork+OVRNetworkTcpClient+ConnectionState")]
    pub type ConnectionState = crate::GlobalNamespace::OVRNetworkTcpClient_OVRNetwork_ConnectionState;
    pub fn Connect(
        &mut self,
        listeningPort: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Connect", (listeningPort))?;
        Ok(__cordl_ret)
    }
    pub fn ConnectCallback(
        &mut self,
        ar: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConnectCallback", (ar))?;
        Ok(__cordl_ret)
    }
    pub fn Disconnect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Disconnect", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnReadDataCallback(
        &mut self,
        ar: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnReadDataCallback", (ar))?;
        Ok(__cordl_ret)
    }
    pub fn Tick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Tick", ())?;
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
    pub fn get_Connected(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Connected", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_connectionState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRNetworkTcpClient_OVRNetwork_ConnectionState,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRNetworkTcpClient_OVRNetwork_ConnectionState = __cordl_object
            .invoke("get_connectionState", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRNetwork+OVRNetworkTcpClient")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRNetwork_OVRNetworkTcpClient {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRNetwork+OVRNetworkTcpServer")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRNetwork_OVRNetworkTcpServer {
    __cordl_parent: crate::System::Object,
    pub tcpListener: *mut crate::System::Net::Sockets::TcpListener,
    pub clientsLock: *mut crate::System::Object,
    pub clients: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Net::Sockets::TcpClient,
    >,
}
#[cfg(feature = "OVRNetwork+OVRNetworkTcpServer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRNetwork_OVRNetworkTcpServer
    => ""."OVRNetwork/OVRNetworkTcpServer"
);
#[cfg(feature = "OVRNetwork+OVRNetworkTcpServer")]
impl std::ops::Deref for crate::GlobalNamespace::OVRNetwork_OVRNetworkTcpServer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRNetwork+OVRNetworkTcpServer")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRNetwork_OVRNetworkTcpServer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRNetwork+OVRNetworkTcpServer")]
impl crate::GlobalNamespace::OVRNetwork_OVRNetworkTcpServer {
    pub fn Broadcast(
        &mut self,
        payloadType: i32,
        payload: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Broadcast", (payloadType, payload))?;
        Ok(__cordl_ret)
    }
    pub fn DoAcceptTcpClientCallback(
        &mut self,
        ar: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoAcceptTcpClientCallback", (ar))?;
        Ok(__cordl_ret)
    }
    pub fn DoWriteDataCallback(
        &mut self,
        ar: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoWriteDataCallback", (ar))?;
        Ok(__cordl_ret)
    }
    pub fn HasConnectedClient(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasConnectedClient", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn StartListening(
        &mut self,
        listeningPort: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartListening", (listeningPort))?;
        Ok(__cordl_ret)
    }
    pub fn StopListening(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopListening", ())?;
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
#[cfg(feature = "OVRNetwork+OVRNetworkTcpServer")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRNetwork_OVRNetworkTcpServer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
