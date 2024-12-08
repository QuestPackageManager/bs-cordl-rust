#[cfg(feature = "System+Net+CommandStream")]
#[repr(C)]
#[derive(Debug)]
pub struct CommandStream {
    __cordl_parent: crate::System::Net::NetworkStreamWrapper,
    pub _recoverableFailure: bool,
    pub _request: *mut crate::System::Net::WebRequest,
    pub _isAsync: bool,
    pub _aborted: bool,
    pub _commands: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Net::CommandStream_PipelineEntry,
    >,
    pub _index: i32,
    pub _doRead: bool,
    pub _doSend: bool,
    pub _currentResponseDescription: *mut crate::System::Net::ResponseDescription,
    pub _abortReason: *mut crate::System::String,
    pub _buffer: *mut crate::System::String,
    pub _encoding: *mut crate::System::Text::Encoding,
    pub _decoder: *mut crate::System::Text::Decoder,
}
#[cfg(feature = "System+Net+CommandStream")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::CommandStream => "System.Net"
    ."CommandStream"
);
#[cfg(feature = "System+Net+CommandStream")]
impl std::ops::Deref for crate::System::Net::CommandStream {
    type Target = crate::System::Net::NetworkStreamWrapper;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+CommandStream")]
impl std::ops::DerefMut for crate::System::Net::CommandStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+CommandStream")]
impl crate::System::Net::CommandStream {
    #[cfg(feature = "System+Net+CommandStream+PipelineEntry")]
    pub type PipelineEntry = crate::System::Net::CommandStream_PipelineEntry;
    #[cfg(feature = "System+Net+CommandStream+PipelineEntryFlags")]
    pub type PipelineEntryFlags = crate::System::Net::CommandStream_PipelineEntryFlags;
    #[cfg(feature = "System+Net+CommandStream+PipelineInstruction")]
    pub type PipelineInstruction = crate::System::Net::CommandStream_PipelineInstruction;
    pub fn Abort(
        &mut self,
        e: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Abort", (e))?;
        Ok(__cordl_ret)
    }
    pub fn BuildCommandsList(
        &mut self,
        request: *mut crate::System::Net::WebRequest,
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
        > = __cordl_object.invoke("BuildCommandsList", (request))?;
        Ok(__cordl_ret)
    }
    pub fn CheckContinuePipeline(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckContinuePipeline", ())?;
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
    pub fn ContinueCommandPipeline(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("ContinueCommandPipeline", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateException_FtpStatusCode_String1(
        &mut self,
        code: crate::System::Net::FtpStatusCode,
        statusDescription: *mut crate::System::String,
        innerException: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke("GenerateException", (code, statusDescription, innerException))?;
        Ok(__cordl_ret)
    }
    pub fn GenerateException_String_WebExceptionStatus0(
        &mut self,
        message: *mut crate::System::String,
        status: crate::System::Net::WebExceptionStatus,
        innerException: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Exception> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Exception = __cordl_object
            .invoke("GenerateException", (message, status, innerException))?;
        Ok(__cordl_ret)
    }
    pub fn InitCommandPipeline(
        &mut self,
        request: *mut crate::System::Net::WebRequest,
        commands: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Net::CommandStream_PipelineEntry,
        >,
        isAsync: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitCommandPipeline", (request, commands, isAsync))?;
        Ok(__cordl_ret)
    }
    pub fn InvokeRequestCallback(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InvokeRequestCallback", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn MarkAsRecoverableFailure(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkAsRecoverableFailure", ())?;
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
    pub fn PostReadCommandProcessing(
        &mut self,
        stream: quest_hook::libil2cpp::ByRefMut<*mut crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("PostReadCommandProcessing", (stream))?;
        Ok(__cordl_ret)
    }
    pub fn PostSendCommandProcessing(
        &mut self,
        stream: quest_hook::libil2cpp::ByRefMut<*mut crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("PostSendCommandProcessing", (stream))?;
        Ok(__cordl_ret)
    }
    pub fn ReceiveCommandResponse(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::ResponseDescription> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::ResponseDescription = __cordl_object
            .invoke("ReceiveCommandResponse", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReceiveCommandResponseCallback(
        &mut self,
        state: *mut crate::System::Net::ReceiveState,
        bytesRead: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReceiveCommandResponseCallback", (state, bytesRead))?;
        Ok(__cordl_ret)
    }
    pub fn SubmitRequest(
        &mut self,
        request: *mut crate::System::Net::WebRequest,
        isAsync: bool,
        readInitalResponseOnConnect: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::IO::Stream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::IO::Stream = __cordl_object
            .invoke("SubmitRequest", (request, isAsync, readInitalResponseOnConnect))?;
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
    pub fn get_Encoding(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::Encoding> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::Encoding = __cordl_object
            .invoke("get_Encoding", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_RecoverableFailure(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_RecoverableFailure", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Encoding(
        &mut self,
        value: *mut crate::System::Text::Encoding,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Encoding", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+CommandStream")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Net::CommandStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+CommandStream+PipelineEntry")]
#[repr(C)]
#[derive(Debug)]
pub struct CommandStream_PipelineEntry {
    __cordl_parent: crate::System::Object,
    pub Command: *mut crate::System::String,
    pub Flags: crate::System::Net::CommandStream_PipelineEntryFlags,
}
#[cfg(feature = "System+Net+CommandStream+PipelineEntry")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Net::CommandStream_PipelineEntry =>
    "System.Net"."CommandStream/PipelineEntry"
);
#[cfg(feature = "System+Net+CommandStream+PipelineEntry")]
impl std::ops::Deref for crate::System::Net::CommandStream_PipelineEntry {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+CommandStream+PipelineEntry")]
impl std::ops::DerefMut for crate::System::Net::CommandStream_PipelineEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+CommandStream+PipelineEntry")]
impl crate::System::Net::CommandStream_PipelineEntry {
    pub fn HasFlag(
        &mut self,
        flags: crate::System::Net::CommandStream_PipelineEntryFlags,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasFlag", (flags))?;
        Ok(__cordl_ret)
    }
    pub fn New_CommandStream_PipelineEntryFlags1(
        command: *mut crate::System::String,
        flags: crate::System::Net::CommandStream_PipelineEntryFlags,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (command, flags))?;
        Ok(__cordl_object)
    }
    pub fn New_String0(
        command: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (command))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_CommandStream_PipelineEntryFlags1(
        &mut self,
        command: *mut crate::System::String,
        flags: crate::System::Net::CommandStream_PipelineEntryFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (command, flags))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String0(
        &mut self,
        command: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (command))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Net+CommandStream+PipelineEntry")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Net::CommandStream_PipelineEntry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Net+CommandStream+PipelineEntryFlags")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandStream_PipelineEntryFlags {
    CreateDataConnection = 4i32,
    DontLogParameter = 8i32,
    GiveDataStream = 2i32,
    UserCommand = 1i32,
}
#[cfg(feature = "System+Net+CommandStream+PipelineEntryFlags")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::CommandStream_PipelineEntryFlags =>
    "System.Net"."CommandStream/PipelineEntryFlags"
);
#[cfg(feature = "System+Net+CommandStream+PipelineInstruction")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandStream_PipelineInstruction {
    Abort = 0i32,
    Advance = 1i32,
    GiveStream = 4i32,
    Pause = 2i32,
    Reread = 3i32,
}
#[cfg(feature = "System+Net+CommandStream+PipelineInstruction")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Net::CommandStream_PipelineInstruction
    => "System.Net"."CommandStream/PipelineInstruction"
);
