#[cfg(feature = "Mono+Net+Security+MobileAuthenticatedStream")]
#[repr(C)]
#[derive(Debug)]
pub struct MobileAuthenticatedStream {
    __cordl_parent: crate::System::Net::Security::AuthenticatedStream,
    pub xobileTlsContext: *mut crate::Mono::Net::Security::MobileTlsContext,
    pub lastException: *mut crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo,
    pub asyncHandshakeRequest: *mut crate::Mono::Net::Security::AsyncProtocolRequest,
    pub asyncReadRequest: *mut crate::Mono::Net::Security::AsyncProtocolRequest,
    pub asyncWriteRequest: *mut crate::Mono::Net::Security::AsyncProtocolRequest,
    pub readBuffer: *mut crate::Mono::Net::Security::BufferOffsetSize2,
    pub writeBuffer: *mut crate::Mono::Net::Security::BufferOffsetSize2,
    pub ioLock: *mut crate::System::Object,
    pub closeRequested: i32,
    pub shutdown: bool,
    pub operation: crate::Mono::Net::Security::MobileAuthenticatedStream_Operation,
    pub _SslStream_k__BackingField: *mut crate::System::Net::Security::SslStream,
    pub _Settings_k__BackingField: *mut crate::Mono::Security::Interface::MonoTlsSettings,
    pub _Provider_k__BackingField: *mut crate::Mono::Net::Security::MobileTlsProvider,
    pub _TargetHost_k__BackingField: *mut crate::System::String,
    pub _cordl_ID: i32,
}
#[cfg(feature = "Mono+Net+Security+MobileAuthenticatedStream")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Net::Security::MobileAuthenticatedStream
    => "Mono.Net.Security"."MobileAuthenticatedStream"
);
#[cfg(feature = "Mono+Net+Security+MobileAuthenticatedStream")]
impl std::ops::Deref for crate::Mono::Net::Security::MobileAuthenticatedStream {
    type Target = crate::System::Net::Security::AuthenticatedStream;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Net+Security+MobileAuthenticatedStream")]
impl std::ops::DerefMut for crate::Mono::Net::Security::MobileAuthenticatedStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Net+Security+MobileAuthenticatedStream")]
impl crate::Mono::Net::Security::MobileAuthenticatedStream {
    #[cfg(feature = "Mono+Net+Security+MobileAuthenticatedStream+_InnerWrite_d__67")]
    pub type _InnerWrite_d__67 = crate::Mono::Net::Security::MobileAuthenticatedStream__InnerWrite_d__67;
    #[cfg(feature = "Mono+Net+Security+MobileAuthenticatedStream+_InnerRead_d__66")]
    pub type _InnerRead_d__66 = crate::Mono::Net::Security::MobileAuthenticatedStream__InnerRead_d__66;
    #[cfg(feature = "Mono+Net+Security+MobileAuthenticatedStream+_StartOperation_d__57")]
    pub type _StartOperation_d__57 = crate::Mono::Net::Security::MobileAuthenticatedStream__StartOperation_d__57;
    #[cfg(feature = "Mono+Net+Security+MobileAuthenticatedStream+Operation")]
    pub type Operation = crate::Mono::Net::Security::MobileAuthenticatedStream_Operation;
    #[cfg(feature = "Mono+Net+Security+MobileAuthenticatedStream+OperationType")]
    pub type OperationType = crate::Mono::Net::Security::MobileAuthenticatedStream_OperationType;
    #[cfg(
        feature = "Mono+Net+Security+MobileAuthenticatedStream+_ProcessAuthentication_d__48"
    )]
    pub type _ProcessAuthentication_d__48 = crate::Mono::Net::Security::MobileAuthenticatedStream__ProcessAuthentication_d__48;
    #[cfg(feature = "Mono+Net+Security+MobileAuthenticatedStream+__c__DisplayClass66_0")]
    pub type __c__DisplayClass66_0 = crate::Mono::Net::Security::MobileAuthenticatedStream___c__DisplayClass66_0;
    pub fn ProcessHandshake(
        &mut self,
        status: crate::Mono::Net::Security::AsyncOperationStatus,
        renegotiate: bool,
    ) -> quest_hook::libil2cpp::Result<
        crate::Mono::Net::Security::AsyncOperationStatus,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Mono::Net::Security::AsyncOperationStatus = __cordl_object
            .invoke("ProcessHandshake", (status, renegotiate))?;
        Ok(__cordl_ret)
    }
    pub fn set_TargetHost(
        &mut self,
        value: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_TargetHost", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_TargetHost(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_TargetHost", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetException(
        &mut self,
        e: *mut crate::System::Exception,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Runtime::ExceptionServices::ExceptionDispatchInfo = __cordl_object
            .invoke("SetException", (e))?;
        Ok(__cordl_ret)
    }
    pub fn WriteAsync(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        count: i32,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("WriteAsync", (buffer, offset, count, cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn StartOperation(
        &mut self,
        _cordl_type: crate::Mono::Net::Security::MobileAuthenticatedStream_OperationType,
        asyncRequest: *mut crate::Mono::Net::Security::AsyncProtocolRequest,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<i32>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<i32> = __cordl_object
            .invoke("StartOperation", (_cordl_type, asyncRequest, cancellationToken))?;
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
    pub fn get_InternalLocalCertificate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate = __cordl_object
            .invoke("get_InternalLocalCertificate", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CanTimeout(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CanTimeout", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Position(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Position", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_WriteTimeout(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_WriteTimeout", ())?;
        Ok(__cordl_ret)
    }
    pub fn AuthenticateAsServer(
        &mut self,
        serverCertificate: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        clientCertificateRequired: bool,
        enabledSslProtocols: crate::System::Security::Authentication::SslProtocols,
        checkCertificateRevocation: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AuthenticateAsServer",
                (
                    serverCertificate,
                    clientCertificateRequired,
                    enabledSslProtocols,
                    checkCertificateRevocation,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn AuthenticateAsClientAsync(
        &mut self,
        targetHost: *mut crate::System::String,
        clientCertificates: *mut crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
        enabledSslProtocols: crate::System::Security::Authentication::SslProtocols,
        checkCertificateRevocation: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke(
                "AuthenticateAsClientAsync",
                (
                    targetHost,
                    clientCertificates,
                    enabledSslProtocols,
                    checkCertificateRevocation,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn InnerRead(
        &mut self,
        sync: bool,
        requestedSize: i32,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<i32>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<i32> = __cordl_object
            .invoke("InnerRead", (sync, requestedSize, cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn ReadAsync(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        count: i32,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Threading::Tasks::Task_1<i32>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task_1<i32> = __cordl_object
            .invoke("ReadAsync", (buffer, offset, count, cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn get_IsAuthenticated(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsAuthenticated", ())?;
        Ok(__cordl_ret)
    }
    pub fn _InnerWrite_b__67_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("<InnerWrite>b__67_0", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        innerStream: *mut crate::System::IO::Stream,
        leaveInnerStreamOpen: bool,
        owner: *mut crate::System::Net::Security::SslStream,
        settings: *mut crate::Mono::Security::Interface::MonoTlsSettings,
        provider: *mut crate::Mono::Net::Security::MobileTlsProvider,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (innerStream, leaveInnerStreamOpen, owner, settings, provider),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_Settings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Mono::Security::Interface::MonoTlsSettings,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Security::Interface::MonoTlsSettings = __cordl_object
            .invoke("get_Settings", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CanSeek(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CanSeek", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_ReadTimeout(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_ReadTimeout", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_CanRead(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CanRead", ())?;
        Ok(__cordl_ret)
    }
    pub fn AuthenticateAsClient(
        &mut self,
        targetHost: *mut crate::System::String,
        clientCertificates: *mut crate::System::Security::Cryptography::X509Certificates::X509CertificateCollection,
        enabledSslProtocols: crate::System::Security::Authentication::SslProtocols,
        checkCertificateRevocation: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AuthenticateAsClient",
                (
                    targetHost,
                    clientCertificates,
                    enabledSslProtocols,
                    checkCertificateRevocation,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn Write(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Write", (buffer, offset, count))?;
        Ok(__cordl_ret)
    }
    pub fn SetLength(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLength", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Seek(
        &mut self,
        offset: i64,
        origin: crate::System::IO::SeekOrigin,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("Seek", (offset, origin))?;
        Ok(__cordl_ret)
    }
    pub fn InternalWrite_Il2CppArray_i32_i32_0(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("InternalWrite", (buffer, offset, _cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn InternalWrite_AsyncProtocolRequest_BufferOffsetSize2_Il2CppArray_i32_i32_1(
        &mut self,
        asyncRequest: *mut crate::Mono::Net::Security::AsyncProtocolRequest,
        internalBuffer: *mut crate::Mono::Net::Security::BufferOffsetSize2,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke(
                "InternalWrite",
                (asyncRequest, internalBuffer, buffer, offset, _cordl_size),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_Position(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_Position", ())?;
        Ok(__cordl_ret)
    }
    pub fn Read(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Read", (buffer, offset, count))?;
        Ok(__cordl_ret)
    }
    pub fn InnerWrite(
        &mut self,
        sync: bool,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke("InnerWrite", (sync, cancellationToken))?;
        Ok(__cordl_ret)
    }
    pub fn Flush(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Flush", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateContext(
        &mut self,
        options: *mut crate::Mono::Net::Security::MonoSslAuthenticationOptions,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Mono::Net::Security::MobileTlsContext,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Net::Security::MobileTlsContext = __cordl_object
            .invoke("CreateContext", (options))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessRead(
        &mut self,
        userBuffer: *mut crate::Mono::Net::Security::BufferOffsetSize,
    ) -> quest_hook::libil2cpp::Result<crate::System::ValueTuple_2<i32, bool>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::ValueTuple_2<i32, bool> = __cordl_object
            .invoke("ProcessRead", (userBuffer))?;
        Ok(__cordl_ret)
    }
    pub fn get_CanWrite(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_CanWrite", ())?;
        Ok(__cordl_ret)
    }
    pub fn CheckThrow(
        &mut self,
        authSuccessCheck: bool,
        shutdownCheck: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckThrow", (authSuccessCheck, shutdownCheck))?;
        Ok(__cordl_ret)
    }
    pub fn get_Length(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_Length", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProcessAuthentication(
        &mut self,
        runSynchronously: bool,
        options: *mut crate::Mono::Net::Security::MonoSslAuthenticationOptions,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Threading::Tasks::Task> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Threading::Tasks::Task = __cordl_object
            .invoke(
                "ProcessAuthentication",
                (runSynchronously, options, cancellationToken),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_ReadTimeout(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_ReadTimeout", ())?;
        Ok(__cordl_ret)
    }
    pub fn InternalRead_Il2CppArray_i32_i32_ByRefMut0(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        _cordl_size: i32,
        outWantMore: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("InternalRead", (buffer, offset, _cordl_size, outWantMore))?;
        Ok(__cordl_ret)
    }
    pub fn InternalRead_AsyncProtocolRequest_BufferOffsetSize_Il2CppArray_i32_i32_1(
        &mut self,
        asyncRequest: *mut crate::Mono::Net::Security::AsyncProtocolRequest,
        internalBuffer: *mut crate::Mono::Net::Security::BufferOffsetSize,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::ValueTuple_2<i32, bool>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::ValueTuple_2<i32, bool> = __cordl_object
            .invoke(
                "InternalRead",
                (asyncRequest, internalBuffer, buffer, offset, _cordl_size),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_LocalCertificate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate = __cordl_object
            .invoke("get_LocalCertificate", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SslStream(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Net::Security::SslStream> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Net::Security::SslStream = __cordl_object
            .invoke("get_SslStream", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProcessWrite(
        &mut self,
        userBuffer: *mut crate::Mono::Net::Security::BufferOffsetSize,
    ) -> quest_hook::libil2cpp::Result<crate::System::ValueTuple_2<i32, bool>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::ValueTuple_2<i32, bool> = __cordl_object
            .invoke("ProcessWrite", (userBuffer))?;
        Ok(__cordl_ret)
    }
    pub fn get_Provider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Mono::Net::Security::MobileTlsProvider,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Mono::Net::Security::MobileTlsProvider = __cordl_object
            .invoke("get_Provider", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_WriteTimeout(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_WriteTimeout", (value))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        innerStream: *mut crate::System::IO::Stream,
        leaveInnerStreamOpen: bool,
        owner: *mut crate::System::Net::Security::SslStream,
        settings: *mut crate::Mono::Security::Interface::MonoTlsSettings,
        provider: *mut crate::Mono::Net::Security::MobileTlsProvider,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (innerStream, leaveInnerStreamOpen, owner, settings, provider),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Mono+Net+Security+MobileAuthenticatedStream")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Net::Security::MobileAuthenticatedStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Mono+Net+Security+MobileAuthenticatedStream+Operation")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MobileAuthenticatedStream_Operation {
    Authenticated = 2i32,
    Close = 6i32,
    Handshake = 1i32,
    None = 0i32,
    Read = 4i32,
    Renegotiate = 3i32,
    Write = 5i32,
}
#[cfg(feature = "Mono+Net+Security+MobileAuthenticatedStream+Operation")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Net::Security::MobileAuthenticatedStream_Operation => "Mono.Net.Security"
    ."MobileAuthenticatedStream/Operation"
);
#[cfg(feature = "Mono+Net+Security+MobileAuthenticatedStream+OperationType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MobileAuthenticatedStream_OperationType {
    Read = 0i32,
    Renegotiate = 2i32,
    Shutdown = 3i32,
    Write = 1i32,
}
#[cfg(feature = "Mono+Net+Security+MobileAuthenticatedStream+OperationType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Net::Security::MobileAuthenticatedStream_OperationType => "Mono.Net.Security"
    ."MobileAuthenticatedStream/OperationType"
);
