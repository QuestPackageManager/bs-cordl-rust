#[cfg(feature = "System+Net+FtpControlStream")]
#[repr(C)]
#[derive(Debug)]
pub struct FtpControlStream {
    __cordl_parent: crate::System::Net::CommandStream,
    pub _dataSocket: *mut crate::System::Net::Sockets::Socket,
    pub _passiveEndPoint: *mut crate::System::Net::IPEndPoint,
    pub _tlsStream: *mut crate::System::Net::TlsStream,
    pub _bannerMessage: *mut crate::System::Text::StringBuilder,
    pub _welcomeMessage: *mut crate::System::Text::StringBuilder,
    pub _exitMessage: *mut crate::System::Text::StringBuilder,
    pub _credentials: *mut crate::System::WeakReference,
    pub _currentTypeSetting: *mut crate::System::String,
    pub _contentLength: i64,
    pub _lastModified: crate::System::DateTime,
    pub _dataHandshakeStarted: bool,
    pub _loginDirectory: *mut crate::System::String,
    pub _establishedServerDirectory: *mut crate::System::String,
    pub _requestedServerDirectory: *mut crate::System::String,
    pub _responseUri: *mut crate::System::Uri,
    pub _loginState: crate::System::Net::FtpLoginState,
    pub StatusCode: crate::System::Net::FtpStatusCode,
    pub StatusLine: *mut crate::System::String,
}
#[cfg(feature = "System+Net+FtpControlStream")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::FtpControlStream => "System.Net"
    ."FtpControlStream"
);
#[cfg(feature = "System+Net+FtpControlStream")]
impl std::ops::Deref for crate::System::Net::FtpControlStream {
    type Target = crate::System::Net::CommandStream;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+FtpControlStream")]
impl std::ops::DerefMut for crate::System::Net::FtpControlStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+FtpControlStream")]
impl crate::System::Net::FtpControlStream {
    #[cfg(feature = "System+Net+FtpControlStream+GetPathOption")]
    pub type GetPathOption = crate::System::Net::FtpControlStream_GetPathOption;
    #[cfg(feature = "System+Net+FtpControlStream+__c__DisplayClass31_0")]
    pub type __c__DisplayClass31_0 = crate::System::Net::FtpControlStream___c__DisplayClass31_0;
    pub fn AbortConnect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AbortConnect", ())?;
        Ok(__cordl_ret)
    }
    pub fn BuildCommandsList(
        &mut self,
        req: *mut crate::System::Net::WebRequest,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Net::CommandStream_PipelineEntry,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Net::CommandStream_PipelineEntry,
        > = __cordl_object.invoke("BuildCommandsList", (req))?;
        Ok(__cordl_ret)
    }
    pub fn CheckValid(
        &mut self,
        response: *mut crate::System::Net::ResponseDescription,
        validThrough: quest_hook::libil2cpp::ByRefMut<i32>,
        completeLength: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CheckValid", (response, validThrough, completeLength))?;
        Ok(__cordl_ret)
    }
    pub fn ClearState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearState", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateFtpDataSocket(
        &mut self,
        request: *mut crate::System::Net::FtpWebRequest,
        templateSocket: *mut crate::System::Net::Sockets::Socket,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::Sockets::Socket> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::Sockets::Socket = __cordl_object
            .invoke("CreateFtpDataSocket", (request, templateSocket))?;
        Ok(__cordl_ret)
    }
    pub fn CreateFtpListenerSocket(
        &mut self,
        request: *mut crate::System::Net::FtpWebRequest,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateFtpListenerSocket", (request))?;
        Ok(__cordl_ret)
    }
    pub fn FormatAddress(
        &mut self,
        address: *mut crate::System::Net::IPAddress,
        Port: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("FormatAddress", (address, Port))?;
        Ok(__cordl_ret)
    }
    pub fn FormatAddressV6(
        &mut self,
        address: *mut crate::System::Net::IPAddress,
        port: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("FormatAddressV6", (address, port))?;
        Ok(__cordl_ret)
    }
    pub fn FormatFtpCommand(
        &mut self,
        command: *mut crate::System::String,
        parameter: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("FormatFtpCommand", (command, parameter))?;
        Ok(__cordl_ret)
    }
    pub fn GetContentLengthFrom213Response(
        &mut self,
        responseString: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object
            .invoke("GetContentLengthFrom213Response", (responseString))?;
        Ok(__cordl_ret)
    }
    pub fn GetLastModifiedFrom213Response(
        &mut self,
        str: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke("GetLastModifiedFrom213Response", (str))?;
        Ok(__cordl_ret)
    }
    pub fn GetLoginDirectory(
        &mut self,
        str: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetLoginDirectory", (str))?;
        Ok(__cordl_ret)
    }
    pub fn GetPortCommandLine(
        &mut self,
        request: *mut crate::System::Net::FtpWebRequest,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("GetPortCommandLine", (request))?;
        Ok(__cordl_ret)
    }
    pub fn GetPortV4(
        &mut self,
        responseString: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetPortV4", (responseString))?;
        Ok(__cordl_ret)
    }
    pub fn GetPortV6(
        &mut self,
        responseString: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetPortV6", (responseString))?;
        Ok(__cordl_ret)
    }
    pub fn IsFtpDataStreamWriteable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::TriState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::TriState = __cordl_object
            .invoke("IsFtpDataStreamWriteable", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        client: *mut crate::System::Net::Sockets::TcpClient,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (client))?;
        Ok(__cordl_object)
    }
    pub fn PipelineCallback(
        &mut self,
        entry: *mut crate::System::Net::CommandStream_PipelineEntry,
        response: *mut crate::System::Net::ResponseDescription,
        timeout: bool,
        stream: quest_hook::libil2cpp::ByRefMut<*mut crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Net::CommandStream_PipelineInstruction,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::CommandStream_PipelineInstruction = __cordl_object
            .invoke("PipelineCallback", (entry, response, timeout, stream))?;
        Ok(__cordl_ret)
    }
    pub fn QueueOrCreateDataConection(
        &mut self,
        entry: *mut crate::System::Net::CommandStream_PipelineEntry,
        response: *mut crate::System::Net::ResponseDescription,
        timeout: bool,
        stream: quest_hook::libil2cpp::ByRefMut<*mut crate::System::IO::Stream>,
        isSocketReady: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Net::CommandStream_PipelineInstruction,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::CommandStream_PipelineInstruction = __cordl_object
            .invoke(
                "QueueOrCreateDataConection",
                (entry, response, timeout, stream, isSocketReady),
            )?;
        Ok(__cordl_ret)
    }
    pub fn QueueOrCreateFtpDataStream(
        &mut self,
        stream: quest_hook::libil2cpp::ByRefMut<*mut crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Net::CommandStream_PipelineInstruction,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::CommandStream_PipelineInstruction = __cordl_object
            .invoke("QueueOrCreateFtpDataStream", (stream))?;
        Ok(__cordl_ret)
    }
    pub fn TryUpdateContentLength(
        &mut self,
        str: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TryUpdateContentLength", (str))?;
        Ok(__cordl_ret)
    }
    pub fn TryUpdateResponseUri(
        &mut self,
        str: *mut crate::System::String,
        request: *mut crate::System::Net::FtpWebRequest,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TryUpdateResponseUri", (str, request))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        client: *mut crate::System::Net::Sockets::TcpClient,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (client))?;
        Ok(__cordl_ret)
    }
    pub fn get_BannerMessage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_BannerMessage", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ContentLength(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_ContentLength", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Credentials(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::NetworkCredential> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::NetworkCredential = __cordl_object
            .invoke("get_Credentials", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ExitMessage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_ExitMessage", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_LastModified(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke("get_LastModified", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ResponseUri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Uri> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Uri = __cordl_object
            .invoke("get_ResponseUri", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_WelcomeMessage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_WelcomeMessage", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Credentials(
        &mut self,
        value: *mut crate::System::Net::NetworkCredential,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Credentials", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+FtpControlStream")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::FtpControlStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+FtpControlStream+GetPathOption")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FtpControlStream_GetPathOption {
    AssumeFilename = 1i32,
    AssumeNoFilename = 2i32,
    Normal = 0i32,
}
#[cfg(feature = "System+Net+FtpControlStream+GetPathOption")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::FtpControlStream_GetPathOption =>
    "System.Net"."FtpControlStream/GetPathOption"
);