#[cfg(feature = "System+Net+CommandStream")]
#[repr(C)]
#[derive(Debug)]
pub struct CommandStream {
    __cordl_parent: crate::System::Net::NetworkStreamWrapper,
    pub _recoverableFailure: bool,
    pub _request: quest_hook::libil2cpp::Gc<crate::System::Net::WebRequest>,
    pub _isAsync: bool,
    pub _aborted: bool,
    pub _commands: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::System::Net::CommandStream_PipelineEntry>,
        >,
    >,
    pub _index: i32,
    pub _doRead: bool,
    pub _doSend: bool,
    pub _currentResponseDescription: quest_hook::libil2cpp::Gc<
        crate::System::Net::ResponseDescription,
    >,
    pub _abortReason: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _encoding: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
    pub _decoder: quest_hook::libil2cpp::Gc<crate::System::Text::Decoder>,
}
#[cfg(feature = "System+Net+CommandStream")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Net::CommandStream {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "CommandStream";
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
#[cfg(feature = "System+Net+CommandStream")]
impl std::ops::Deref for crate::System::Net::CommandStream {
    type Target = crate::System::Net::NetworkStreamWrapper;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+CommandStream")]
impl std::ops::DerefMut for crate::System::Net::CommandStream {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        e: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Exception>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Abort")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Abort",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (e))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn BuildCommandsList(
        &mut self,
        request: quest_hook::libil2cpp::Gc<crate::System::Net::WebRequest>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::System::Net::CommandStream_PipelineEntry,
                >,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Net::WebRequest>),
                        quest_hook::libil2cpp::Gc<
                            quest_hook::libil2cpp::Il2CppArray<
                                quest_hook::libil2cpp::Gc<
                                    crate::System::Net::CommandStream_PipelineEntry,
                                >,
                            >,
                        >,
                        1usize,
                    >("BuildCommandsList")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "BuildCommandsList", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::System::Net::CommandStream_PipelineEntry,
                >,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (request))? };
        Ok(__cordl_ret.into())
    }
    pub fn CheckContinuePipeline(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("CheckContinuePipeline")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CheckContinuePipeline", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn CheckValid(
        &mut self,
        response: quest_hook::libil2cpp::Gc<crate::System::Net::ResponseDescription>,
        validThrough: quest_hook::libil2cpp::ByRefMut<i32>,
        completeLength: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Net::ResponseDescription,
                            >,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                            quest_hook::libil2cpp::ByRefMut<i32>,
                        ),
                        bool,
                        3usize,
                    >("CheckValid")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "CheckValid", 3usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (response, validThrough, completeLength))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClearState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("ClearState")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ClearState", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ContinueCommandPipeline(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                        0usize,
                    >("ContinueCommandPipeline")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ContinueCommandPipeline", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (bool),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Dispose")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Dispose",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (disposing))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateException_FtpStatusCode_Il2CppString1(
        &mut self,
        code: crate::System::Net::FtpStatusCode,
        statusDescription: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        innerException: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            crate::System::Net::FtpStatusCode,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            quest_hook::libil2cpp::Gc<crate::System::Exception>,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::Exception>,
                        3usize,
                    >("GenerateException")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GenerateException", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (code, statusDescription, innerException))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateException_Il2CppString_WebExceptionStatus0(
        &mut self,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        status: crate::System::Net::WebExceptionStatus,
        innerException: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::System::Net::WebExceptionStatus,
                            quest_hook::libil2cpp::Gc<crate::System::Exception>,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::Exception>,
                        3usize,
                    >("GenerateException")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GenerateException", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = unsafe {
            cordl_method_info.invoke_unchecked(self, (message, status, innerException))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InitCommandPipeline(
        &mut self,
        request: quest_hook::libil2cpp::Gc<crate::System::Net::WebRequest>,
        commands: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<
                    crate::System::Net::CommandStream_PipelineEntry,
                >,
            >,
        >,
        isAsync: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Net::WebRequest>,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<
                                    quest_hook::libil2cpp::Gc<
                                        crate::System::Net::CommandStream_PipelineEntry,
                                    >,
                                >,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Void,
                        3usize,
                    >("InitCommandPipeline")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InitCommandPipeline", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (request, commands, isAsync))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn InvokeRequestCallback(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("InvokeRequestCallback")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "InvokeRequestCallback", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (obj))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MarkAsRecoverableFailure(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Void,
                        0usize,
                    >("MarkAsRecoverableFailure")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "MarkAsRecoverableFailure", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Net::CommandStream_PipelineEntry,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Net::ResponseDescription,
                            >,
                            bool,
                            quest_hook::libil2cpp::ByRefMut<
                                quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                            >,
                        ),
                        crate::System::Net::CommandStream_PipelineInstruction,
                        4usize,
                    >("PipelineCallback")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PipelineCallback", 4usize
                        )
                    })
            });
        let __cordl_ret: crate::System::Net::CommandStream_PipelineInstruction = unsafe {
            cordl_method_info.invoke_unchecked(self, (entry, response, timeout, stream))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PostReadCommandProcessing(
        &mut self,
        stream: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                        >),
                        bool,
                        1usize,
                    >("PostReadCommandProcessing")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PostReadCommandProcessing", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (stream))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn PostSendCommandProcessing(
        &mut self,
        stream: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                        >),
                        bool,
                        1usize,
                    >("PostSendCommandProcessing")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "PostSendCommandProcessing", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (stream))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReadCallback(
        asyncResult: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ReadCallback")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReadCallback", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (asyncResult))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ReceiveCommandResponse(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::ResponseDescription>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Net::ResponseDescription,
                        >,
                        0usize,
                    >("ReceiveCommandResponse")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReceiveCommandResponse", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Net::ResponseDescription,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn ReceiveCommandResponseCallback(
        &mut self,
        state: quest_hook::libil2cpp::Gc<crate::System::Net::ReceiveState>,
        bytesRead: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Net::ReceiveState>,
                            i32,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("ReceiveCommandResponseCallback")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ReceiveCommandResponseCallback", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (state, bytesRead))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn SubmitRequest(
        &mut self,
        request: quest_hook::libil2cpp::Gc<crate::System::Net::WebRequest>,
        isAsync: bool,
        readInitalResponseOnConnect: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<crate::System::Net::WebRequest>,
                            bool,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
                        3usize,
                    >("SubmitRequest")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SubmitRequest", 3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IO::Stream> = unsafe {
            cordl_method_info
                .invoke_unchecked(self, (request, isAsync, readInitalResponseOnConnect))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn WriteCallback(
        asyncResult: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("WriteCallback")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "WriteCallback", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked((), (asyncResult))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        client: quest_hook::libil2cpp::Gc<crate::System::Net::Sockets::TcpClient>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Net::Sockets::TcpClient,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (client))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Encoding(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
                        0usize,
                    >("get_Encoding")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Encoding", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding> = unsafe {
            cordl_method_info.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_RecoverableFailure(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), bool, 0usize>("get_RecoverableFailure")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_RecoverableFailure", 0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_Encoding(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Text::Encoding>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_Encoding")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_Encoding", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
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
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Command: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub Flags: crate::System::Net::CommandStream_PipelineEntryFlags,
}
#[cfg(feature = "System+Net+CommandStream+PipelineEntry")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::CommandStream_PipelineEntry {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "CommandStream/PipelineEntry";
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
#[cfg(feature = "System+Net+CommandStream+PipelineEntry")]
impl std::ops::Deref for crate::System::Net::CommandStream_PipelineEntry {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+CommandStream+PipelineEntry")]
impl std::ops::DerefMut for crate::System::Net::CommandStream_PipelineEntry {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Net+CommandStream+PipelineEntry")]
impl crate::System::Net::CommandStream_PipelineEntry {
    pub fn HasFlag(
        &mut self,
        flags: crate::System::Net::CommandStream_PipelineEntryFlags,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::System::Net::CommandStream_PipelineEntryFlags),
                        bool,
                        1usize,
                    >("HasFlag")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "HasFlag",
                            1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe {
            cordl_method_info.invoke_unchecked(self, (flags))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New_CommandStream_PipelineEntryFlags1(
        command: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        flags: crate::System::Net::CommandStream_PipelineEntryFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (command, flags))?;
        Ok(__cordl_object.into())
    }
    pub fn New_Il2CppString0(
        command: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (command))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_CommandStream_PipelineEntryFlags1(
        &mut self,
        command: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        flags: crate::System::Net::CommandStream_PipelineEntryFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppString,
                            >,
                            crate::System::Net::CommandStream_PipelineEntryFlags,
                        ),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (command, flags))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString0(
        &mut self,
        command: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), ".ctor",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (command))?
        };
        Ok(__cordl_ret.into())
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CommandStream_PipelineEntryFlags {
    #[default]
    CreateDataConnection = 4i32,
    DontLogParameter = 8i32,
    GiveDataStream = 2i32,
    UserCommand = 1i32,
}
#[cfg(feature = "System+Net+CommandStream+PipelineEntryFlags")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::CommandStream_PipelineEntryFlags {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "CommandStream/PipelineEntryFlags";
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
#[cfg(feature = "System+Net+CommandStream+PipelineEntryFlags")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Net::CommandStream_PipelineEntryFlags {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Net+CommandStream+PipelineEntryFlags")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Net::CommandStream_PipelineEntryFlags {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "System+Net+CommandStream+PipelineEntryFlags")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Net::CommandStream_PipelineEntryFlags {
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
#[cfg(feature = "System+Net+CommandStream+PipelineEntryFlags")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Net::CommandStream_PipelineEntryFlags {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "System+Net+CommandStream+PipelineInstruction")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CommandStream_PipelineInstruction {
    #[default]
    Abort = 0i32,
    Advance = 1i32,
    GiveStream = 4i32,
    Pause = 2i32,
    Reread = 3i32,
}
#[cfg(feature = "System+Net+CommandStream+PipelineInstruction")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Net::CommandStream_PipelineInstruction {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Net";
    const CLASS_NAME: &'static str = "CommandStream/PipelineInstruction";
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
#[cfg(feature = "System+Net+CommandStream+PipelineInstruction")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Net::CommandStream_PipelineInstruction {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Net+CommandStream+PipelineInstruction")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Net::CommandStream_PipelineInstruction {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "System+Net+CommandStream+PipelineInstruction")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Net::CommandStream_PipelineInstruction {
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
#[cfg(feature = "System+Net+CommandStream+PipelineInstruction")]
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Net::CommandStream_PipelineInstruction {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
