#[cfg(feature = "System+Net+FtpControlStream")]
#[repr(C)]
#[derive(Debug)]
pub struct FtpControlStream {
    __cordl_parent: crate::System::Net::CommandStream,
    pub _dataSocket: quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::Socket>,
    pub _passiveEndPoint: quest_hook::libil2cpp::Gc<crate::System::Net::IPEndPoint>,
    pub _tlsStream: quest_hook::libil2cpp::Gc<crate::System::Net::TlsStream>,
    pub _bannerMessage: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    pub _welcomeMessage: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    pub _exitMessage: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    pub _credentials: quest_hook::libil2cpp::Gc<crate::System::WeakReference>,
    pub _currentTypeSetting: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _contentLength: i64,
    pub _lastModified: crate::System::DateTime,
    pub _dataHandshakeStarted: bool,
    pub _loginDirectory: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _establishedServerDirectory: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _requestedServerDirectory: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _responseUri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
    pub _loginState: crate::System::Net::FtpLoginState,
    pub StatusCode: crate::System::Net::FtpStatusCode,
    pub StatusLine: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "System+Net+FtpControlStream")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Net::FtpControlStream {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "FtpControlStream";
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
    pub fn AbortConnect(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AbortConnect", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AcceptCallback(
        asyncResult: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AcceptCallback", (asyncResult))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildCommandsList(
        &mut self,
        req: quest_hook::libil2cpp::Gc<crate::System::Net::WebRequest>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::System::Net::CommandStream_PipelineEntry,
                >,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::System::Net::CommandStream_PipelineEntry,
                >,
            >,
        > = __cordl_object.invoke("BuildCommandsList", (req))?;
        Ok(__cordl_ret.into())
    }
    pub fn CheckValid(
        &mut self,
        response: quest_hook::libil2cpp::Gc<crate::System::Net::ResponseDescription>,
        validThrough: quest_hook::libil2cpp::ByRefMut<i32>,
        completeLength: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("CheckValid", (response, validThrough, completeLength))?;
        Ok(__cordl_ret.into())
    }
    pub fn ClearState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ClearState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ConnectCallback(
        asyncResult: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConnectCallback", (asyncResult))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateFtpDataSocket(
        &mut self,
        request: quest_hook::libil2cpp::Gc<crate::System::Net::FtpWebRequest>,
        templateSocket: quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::Socket>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::Socket>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::Socket,
        > = __cordl_object.invoke("CreateFtpDataSocket", (request, templateSocket))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateFtpListenerSocket(
        &mut self,
        request: quest_hook::libil2cpp::Gc<crate::System::Net::FtpWebRequest>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateFtpListenerSocket", (request))?;
        Ok(__cordl_ret.into())
    }
    pub fn FormatAddress(
        &mut self,
        address: quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress>,
        Port: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("FormatAddress", (address, Port))?;
        Ok(__cordl_ret.into())
    }
    pub fn FormatAddressV6(
        &mut self,
        address: quest_hook::libil2cpp::Gc<crate::System::Net::IPAddress>,
        port: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("FormatAddressV6", (address, port))?;
        Ok(__cordl_ret.into())
    }
    pub fn FormatFtpCommand(
        &mut self,
        command: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        parameter: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("FormatFtpCommand", (command, parameter))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetContentLengthFrom213Response(
        &mut self,
        responseString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object
            .invoke("GetContentLengthFrom213Response", (responseString))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLastModifiedFrom213Response(
        &mut self,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke("GetLastModifiedFrom213Response", (str))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLoginDirectory(
        &mut self,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetLoginDirectory", (str))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPathInfo(
        pathOption: crate::System::Net::FtpControlStream_GetPathOption,
        uri: quest_hook::libil2cpp::Gc<crate::System::Uri>,
        path: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        directory: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
        filename: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPathInfo", (pathOption, uri, path, directory, filename))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPortCommandLine(
        &mut self,
        request: quest_hook::libil2cpp::Gc<crate::System::Net::FtpWebRequest>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("GetPortCommandLine", (request))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPortV4(
        &mut self,
        responseString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetPortV4", (responseString))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPortV6(
        &mut self,
        responseString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetPortV6", (responseString))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsFtpDataStreamWriteable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::TriState> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::TriState = __cordl_object
            .invoke("IsFtpDataStreamWriteable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        client: quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::TcpClient>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (client))?;
        Ok(__cordl_object.into())
    }
    pub fn PipelineCallback(
        &mut self,
        entry: quest_hook::libil2cpp::Gc<
            crate::System::Net::CommandStream_PipelineEntry,
        >,
        response: quest_hook::libil2cpp::Gc<crate::System::Net::ResponseDescription>,
        timeout: bool,
        stream: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Net::CommandStream_PipelineInstruction,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::CommandStream_PipelineInstruction = __cordl_object
            .invoke("PipelineCallback", (entry, response, timeout, stream))?;
        Ok(__cordl_ret.into())
    }
    pub fn QueueOrCreateDataConection(
        &mut self,
        entry: quest_hook::libil2cpp::Gc<
            crate::System::Net::CommandStream_PipelineEntry,
        >,
        response: quest_hook::libil2cpp::Gc<crate::System::Net::ResponseDescription>,
        timeout: bool,
        stream: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        >,
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
        Ok(__cordl_ret.into())
    }
    pub fn QueueOrCreateFtpDataStream(
        &mut self,
        stream: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Net::CommandStream_PipelineInstruction,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::CommandStream_PipelineInstruction = __cordl_object
            .invoke("QueueOrCreateFtpDataStream", (stream))?;
        Ok(__cordl_ret.into())
    }
    pub fn SSLHandshakeCallback(
        asyncResult: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SSLHandshakeCallback", (asyncResult))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryUpdateContentLength(
        &mut self,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TryUpdateContentLength", (str))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryUpdateResponseUri(
        &mut self,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        request: quest_hook::libil2cpp::Gc<crate::System::Net::FtpWebRequest>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("TryUpdateResponseUri", (str, request))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        client: quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::TcpClient>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (client))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_BannerMessage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_BannerMessage", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ContentLength(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_ContentLength", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Credentials(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::NetworkCredential>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::NetworkCredential,
        > = __cordl_object.invoke("get_Credentials", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ExitMessage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_ExitMessage", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LastModified(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke("get_LastModified", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ResponseUri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Uri>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Uri> = __cordl_object
            .invoke("get_ResponseUri", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_WelcomeMessage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_WelcomeMessage", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Credentials(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Net::NetworkCredential>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Credentials", (value))?;
        Ok(__cordl_ret.into())
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FtpControlStream_GetPathOption {
    #[default]
    AssumeFilename = 1i32,
    AssumeNoFilename = 2i32,
    Normal = 0i32,
}
#[cfg(feature = "System+Net+FtpControlStream+GetPathOption")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::FtpControlStream_GetPathOption {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "GetPathOption";
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
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Net::FtpControlStream_GetPathOption {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Net::FtpControlStream_GetPathOption {
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
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Net::FtpControlStream_GetPathOption {
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
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Net::FtpControlStream_GetPathOption {
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
