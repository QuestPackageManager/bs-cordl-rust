#[cfg(feature = "System+Net+HttpConnection")]
#[repr(C)]
#[derive(Debug)]
pub struct HttpConnection {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub sock: quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::Socket>,
    pub stream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    pub epl: quest_hook::libil2cpp::Gc<crate::System::Net::EndPointListener>,
    pub ms: quest_hook::libil2cpp::Gc<crate::System::IO::MemoryStream>,
    pub buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub context: quest_hook::libil2cpp::Gc<crate::System::Net::HttpListenerContext>,
    pub current_line: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    pub prefix: quest_hook::libil2cpp::Gc<crate::System::Net::ListenerPrefix>,
    pub i_stream: quest_hook::libil2cpp::Gc<crate::System::Net::RequestStream>,
    pub o_stream: quest_hook::libil2cpp::Gc<crate::System::Net::ResponseStream>,
    pub chunked: bool,
    pub reuses: i32,
    pub context_bound: bool,
    pub secure: bool,
    pub cert: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::X509Certificates::X509Certificate,
    >,
    pub s_timeout: i32,
    pub timer: quest_hook::libil2cpp::Gc<crate::System::Threading::Timer>,
    pub local_ep: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
    pub last_listener: quest_hook::libil2cpp::Gc<crate::System::Net::HttpListener>,
    pub client_cert_errors: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<i32>,
    >,
    pub client_cert: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
    >,
    pub ssl_stream: quest_hook::libil2cpp::Gc<crate::System::Net::Security::SslStream>,
    pub input_state: crate::System::Net::HttpConnection_InputState,
    pub line_state: crate::System::Net::HttpConnection_LineState,
    pub position: i32,
}
#[cfg(feature = "System+Net+HttpConnection")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Net::HttpConnection {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "HttpConnection";
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
#[cfg(feature = "System+Net+HttpConnection")]
impl std::ops::Deref for crate::System::Net::HttpConnection {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+HttpConnection")]
impl std::ops::DerefMut for crate::System::Net::HttpConnection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+HttpConnection")]
impl crate::System::Net::HttpConnection {
    #[cfg(feature = "System+Net+HttpConnection+InputState")]
    pub type InputState = crate::System::Net::HttpConnection_InputState;
    #[cfg(feature = "System+Net+HttpConnection+LineState")]
    pub type LineState = crate::System::Net::HttpConnection_LineState;
    pub fn BeginReadRequest(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BeginReadRequest", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Close(
        &mut self,
        force_close: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Close", (force_close))?;
        Ok(__cordl_ret.into())
    }
    pub fn CloseSocket(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CloseSocket", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRequestStream(
        &mut self,
        chunked: bool,
        contentlength: i64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::RequestStream>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::RequestStream> = __cordl_object
            .invoke("GetRequestStream", (chunked, contentlength))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetResponseStream(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::ResponseStream>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::ResponseStream> = __cordl_object
            .invoke("GetResponseStream", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        sock: quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::Socket>,
        epl: quest_hook::libil2cpp::Gc<crate::System::Net::EndPointListener>,
        secure: bool,
        cert: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sock, epl, secure, cert))?;
        Ok(__cordl_object.into())
    }
    pub fn OnRead(
        ares: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OnRead", (ares))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnReadInternal(
        &mut self,
        ares: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnReadInternal", (ares))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnTimeout(
        &mut self,
        unused: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnTimeout", (unused))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessInput(
        &mut self,
        ms: quest_hook::libil2cpp::Gc<crate::System::IO::MemoryStream>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ProcessInput", (ms))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadLine(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        len: i32,
        used: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ReadLine", (buffer, offset, len, used))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveConnection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveConnection", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SendError_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendError", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SendError_Il2CppString_i32_0(
        &mut self,
        msg: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        status: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendError", (msg, status))?;
        Ok(__cordl_ret.into())
    }
    pub fn Unbind(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Unbind", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn __ctor_b__24_0(
        &mut self,
        t: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        c: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        >,
        ch: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Chain,
        >,
        e: crate::System::Net::Security::SslPolicyErrors,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("<.ctor>b__24_0", (t, c, ch, e))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        sock: quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::Socket>,
        epl: quest_hook::libil2cpp::Gc<crate::System::Net::EndPointListener>,
        secure: bool,
        cert: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sock, epl, secure, cert))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsSecure(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsSecure", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LocalEndPoint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint> = __cordl_object
            .invoke("get_LocalEndPoint", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Reuses(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Reuses", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Prefix(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Net::ListenerPrefix>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Prefix", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Net+HttpConnection")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::HttpConnection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+HttpConnection+InputState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HttpConnection_InputState {
    #[default]
    Headers = 1i32,
    RequestLine = 0i32,
}
#[cfg(feature = "System+Net+HttpConnection+InputState")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::HttpConnection_InputState {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "InputState";
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
#[cfg(feature = "System+Net+HttpConnection+InputState")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Net::HttpConnection_InputState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Net+HttpConnection+InputState")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Net::HttpConnection_InputState {
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
#[cfg(feature = "System+Net+HttpConnection+InputState")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Net::HttpConnection_InputState {
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
#[cfg(feature = "System+Net+HttpConnection+InputState")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Net::HttpConnection_InputState {
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
#[cfg(feature = "System+Net+HttpConnection+LineState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HttpConnection_LineState {
    #[default]
    CR = 1i32,
    LF = 2i32,
    None = 0i32,
}
#[cfg(feature = "System+Net+HttpConnection+LineState")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::HttpConnection_LineState {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "LineState";
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
#[cfg(feature = "System+Net+HttpConnection+LineState")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Net::HttpConnection_LineState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Net+HttpConnection+LineState")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Net::HttpConnection_LineState {
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
#[cfg(feature = "System+Net+HttpConnection+LineState")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Net::HttpConnection_LineState {
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
#[cfg(feature = "System+Net+HttpConnection+LineState")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Net::HttpConnection_LineState {
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
