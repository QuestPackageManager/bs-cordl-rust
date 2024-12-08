#[cfg(feature = "System+Net+HttpConnection")]
#[repr(C)]
#[derive(Debug)]
pub struct HttpConnection {
    __cordl_parent: crate::System::Object,
    pub sock: *mut crate::System::Net::Sockets::Socket,
    pub stream: *mut crate::System::IO::Stream,
    pub epl: *mut crate::System::Net::EndPointListener,
    pub ms: *mut crate::System::IO::MemoryStream,
    pub buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub context: *mut crate::System::Net::HttpListenerContext,
    pub current_line: *mut crate::System::Text::StringBuilder,
    pub prefix: *mut crate::System::Net::ListenerPrefix,
    pub i_stream: *mut crate::System::Net::RequestStream,
    pub o_stream: *mut crate::System::Net::ResponseStream,
    pub chunked: bool,
    pub reuses: i32,
    pub context_bound: bool,
    pub secure: bool,
    pub cert: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate,
    pub s_timeout: i32,
    pub timer: *mut crate::System::Threading::Timer,
    pub local_ep: *mut crate::System::Net::IPEndPoint,
    pub last_listener: *mut crate::System::Net::HttpListener,
    pub client_cert_errors: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub client_cert: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
    pub ssl_stream: *mut crate::System::Net::Security::SslStream,
    pub input_state: crate::System::Net::HttpConnection_InputState,
    pub line_state: crate::System::Net::HttpConnection_LineState,
    pub position: i32,
}
#[cfg(feature = "System+Net+HttpConnection")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::HttpConnection => "System.Net"
    ."HttpConnection"
);
#[cfg(feature = "System+Net+HttpConnection")]
impl std::ops::Deref for crate::System::Net::HttpConnection {
    type Target = crate::System::Object;
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
    #[cfg(feature = "System+Net+HttpConnection+LineState")]
    pub type LineState = crate::System::Net::HttpConnection_LineState;
    #[cfg(feature = "System+Net+HttpConnection+InputState")]
    pub type InputState = crate::System::Net::HttpConnection_InputState;
    pub fn CloseSocket(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CloseSocket", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        sock: *mut crate::System::Net::Sockets::Socket,
        epl: *mut crate::System::Net::EndPointListener,
        secure: bool,
        cert: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sock, epl, secure, cert))?;
        Ok(__cordl_ret)
    }
    pub fn BeginReadRequest(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BeginReadRequest", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Reuses(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Reuses", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnTimeout(
        &mut self,
        unused: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnTimeout", (unused))?;
        Ok(__cordl_ret)
    }
    pub fn OnReadInternal(
        &mut self,
        ares: *mut crate::System::IAsyncResult,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnReadInternal", (ares))?;
        Ok(__cordl_ret)
    }
    pub fn __ctor_b__24_0(
        &mut self,
        t: *mut crate::System::Object,
        c: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        ch: *mut crate::System::Security::Cryptography::X509Certificates::X509Chain,
        e: crate::System::Net::Security::SslPolicyErrors,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("<.ctor>b__24_0", (t, c, ch, e))?;
        Ok(__cordl_ret)
    }
    pub fn GetRequestStream(
        &mut self,
        chunked: bool,
        contentlength: i64,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::RequestStream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::RequestStream = __cordl_object
            .invoke("GetRequestStream", (chunked, contentlength))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LocalEndPoint(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::IPEndPoint> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::IPEndPoint = __cordl_object
            .invoke("get_LocalEndPoint", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsSecure(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsSecure", ())?;
        Ok(__cordl_ret)
    }
    pub fn SendError_String_i32_0(
        &mut self,
        msg: *mut crate::System::String,
        status: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendError", (msg, status))?;
        Ok(__cordl_ret)
    }
    pub fn SendError_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SendError", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Prefix(
        &mut self,
        value: *mut crate::System::Net::ListenerPrefix,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Prefix", (value))?;
        Ok(__cordl_ret)
    }
    pub fn GetResponseStream(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::ResponseStream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::ResponseStream = __cordl_object
            .invoke("GetResponseStream", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadLine(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        len: i32,
        used: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ReadLine", (buffer, offset, len, used))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveConnection(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveConnection", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProcessInput(
        &mut self,
        ms: *mut crate::System::IO::MemoryStream,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ProcessInput", (ms))?;
        Ok(__cordl_ret)
    }
    pub fn Unbind(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Unbind", ())?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn New(
        sock: *mut crate::System::Net::Sockets::Socket,
        epl: *mut crate::System::Net::EndPointListener,
        secure: bool,
        cert: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sock, epl, secure, cert))?;
        Ok(__cordl_object)
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpConnection_InputState {
    Headers = 1i32,
    RequestLine = 0i32,
}
#[cfg(feature = "System+Net+HttpConnection+InputState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::HttpConnection_InputState =>
    "System.Net"."HttpConnection/InputState"
);
#[cfg(feature = "System+Net+HttpConnection+LineState")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpConnection_LineState {
    CR = 1i32,
    LF = 2i32,
    None = 0i32,
}
#[cfg(feature = "System+Net+HttpConnection+LineState")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::HttpConnection_LineState =>
    "System.Net"."HttpConnection/LineState"
);
