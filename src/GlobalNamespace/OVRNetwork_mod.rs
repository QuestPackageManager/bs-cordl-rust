#[cfg(feature = "OVRNetwork")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRNetwork {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRNetwork")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRNetwork {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRNetwork";
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
#[cfg(feature = "OVRNetwork")]
impl std::ops::Deref for crate::GlobalNamespace::OVRNetwork {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub const FrameHeaderMagicIdentifier: u32 = 1384359787u32;
    pub const MaxBufferLength: i32 = 65536i32;
    pub const MaxPayloadLength: i32 = 65524i32;
    #[cfg(feature = "OVRNetwork+FrameHeader")]
    pub type FrameHeader = crate::GlobalNamespace::OVRNetwork_FrameHeader;
    #[cfg(feature = "OVRNetwork+OVRNetworkTcpClient")]
    pub type OVRNetworkTcpClient = crate::GlobalNamespace::OVRNetwork_OVRNetworkTcpClient;
    #[cfg(feature = "OVRNetwork+OVRNetworkTcpServer")]
    pub type OVRNetworkTcpServer = crate::GlobalNamespace::OVRNetwork_OVRNetworkTcpServer;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRNetworkTcpClient_OVRNetwork_ConnectionState {
    #[default]
    Connected = 1i32,
    Connecting = 2i32,
    Disconnected = 0i32,
}
#[cfg(feature = "OVRNetwork+OVRNetworkTcpClient+ConnectionState")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRNetworkTcpClient_OVRNetwork_ConnectionState {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRNetwork/OVRNetworkTcpClient/ConnectionState";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "OVRNetwork+OVRNetworkTcpClient+ConnectionState")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRNetworkTcpClient_OVRNetwork_ConnectionState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVRNetwork+OVRNetworkTcpClient+ConnectionState")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRNetworkTcpClient_OVRNetwork_ConnectionState {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "OVRNetwork+OVRNetworkTcpClient+ConnectionState")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRNetworkTcpClient_OVRNetwork_ConnectionState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "OVRNetwork+OVRNetworkTcpClient+ConnectionState")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRNetworkTcpClient_OVRNetwork_ConnectionState {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "OVRNetwork+FrameHeader")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRNetwork_FrameHeader {
    pub protocolIdentifier: u32,
    pub payloadType: i32,
    pub payloadLength: i32,
}
#[cfg(feature = "OVRNetwork+FrameHeader")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRNetwork_FrameHeader {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRNetwork/FrameHeader";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "OVRNetwork+FrameHeader")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRNetwork_FrameHeader {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVRNetwork+FrameHeader")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRNetwork_FrameHeader {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "OVRNetwork+FrameHeader")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRNetwork_FrameHeader {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "OVRNetwork+FrameHeader")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRNetwork_FrameHeader {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
    pub fn FromBytes(
        arr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRNetwork_FrameHeader> {
        let __cordl_ret: crate::GlobalNamespace::OVRNetwork_FrameHeader = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromBytes", (arr))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToBytes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToBytes", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRNetwork+OVRNetworkTcpClient")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRNetwork_OVRNetworkTcpClient {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub connectionStateChangedCallback: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub payloadReceivedCallback: quest_hook::libil2cpp::Gc<
        crate::System::Action_4<
            i32,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
            i32,
            i32,
        >,
    >,
    pub tcpClient: quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::TcpClient>,
    pub receivedBuffers: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        >,
    >,
    pub receivedBufferIndex: i32,
    pub receivedBufferDataSize: i32,
    pub readyReceiveDataEvent: quest_hook::libil2cpp::Gc<
        crate::System::Threading::ManualResetEvent,
    >,
}
#[cfg(feature = "OVRNetwork+OVRNetworkTcpClient")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRNetwork_OVRNetworkTcpClient {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRNetwork/OVRNetworkTcpClient";
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
#[cfg(feature = "OVRNetwork+OVRNetworkTcpClient")]
impl std::ops::Deref for crate::GlobalNamespace::OVRNetwork_OVRNetworkTcpClient {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        Ok(__cordl_ret.into())
    }
    pub fn ConnectCallback(
        &mut self,
        ar: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ConnectCallback", (ar))?;
        Ok(__cordl_ret.into())
    }
    pub fn Disconnect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Disconnect", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnReadDataCallback(
        &mut self,
        ar: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnReadDataCallback", (ar))?;
        Ok(__cordl_ret.into())
    }
    pub fn Tick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Tick", ())?;
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
    pub fn get_Connected(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Connected", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub tcpListener: quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::TcpListener>,
    pub clientsLock: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub clients: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::TcpClient>,
        >,
    >,
}
#[cfg(feature = "OVRNetwork+OVRNetworkTcpServer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRNetwork_OVRNetworkTcpServer {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRNetwork/OVRNetworkTcpServer";
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
#[cfg(feature = "OVRNetwork+OVRNetworkTcpServer")]
impl std::ops::Deref for crate::GlobalNamespace::OVRNetwork_OVRNetworkTcpServer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        payload: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Broadcast", (payloadType, payload))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoAcceptTcpClientCallback(
        &mut self,
        ar: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoAcceptTcpClientCallback", (ar))?;
        Ok(__cordl_ret.into())
    }
    pub fn DoWriteDataCallback(
        &mut self,
        ar: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoWriteDataCallback", (ar))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasConnectedClient(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasConnectedClient", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn StopListening(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StopListening", ())?;
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
