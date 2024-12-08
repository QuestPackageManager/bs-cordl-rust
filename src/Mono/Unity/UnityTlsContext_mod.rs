#[cfg(feature = "Mono+Unity+UnityTlsContext")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityTlsContext {
    __cordl_parent: crate::Mono::Net::Security::MobileTlsContext,
    pub tlsContext: *mut quest_hook::libil2cpp::Il2CppObject,
    pub requestedClientCertChain: *mut quest_hook::libil2cpp::Il2CppObject,
    pub requestedClientKey: *mut quest_hook::libil2cpp::Il2CppObject,
    pub readCallback: *mut crate::Mono::Unity::UnityTls_unitytls_tlsctx_read_callback,
    pub writeCallback: *mut crate::Mono::Unity::UnityTls_unitytls_tlsctx_write_callback,
    pub certificateCallback: *mut crate::Mono::Unity::UnityTls_unitytls_tlsctx_certificate_callback,
    pub verifyCallback: *mut crate::Mono::Unity::UnityTls_unitytls_tlsctx_x509verify_callback,
    pub localClientCertificate: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate,
    pub remoteCertificate: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
    pub connectioninfo: *mut crate::Mono::Security::Interface::MonoTlsConnectionInfo,
    pub isAuthenticated: bool,
    pub hasContext: bool,
    pub closedGraceful: bool,
    pub writeBuffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub readBuffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub handle: crate::System::Runtime::InteropServices::GCHandle,
    pub lastException: *mut crate::System::Exception,
}
#[cfg(feature = "Mono+Unity+UnityTlsContext")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Unity::UnityTlsContext => "Mono.Unity"
    ."UnityTlsContext"
);
#[cfg(feature = "Mono+Unity+UnityTlsContext")]
impl std::ops::Deref for crate::Mono::Unity::UnityTlsContext {
    type Target = crate::Mono::Net::Security::MobileTlsContext;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTlsContext")]
impl std::ops::DerefMut for crate::Mono::Unity::UnityTlsContext {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTlsContext")]
impl crate::Mono::Unity::UnityTlsContext {
    pub fn get_LocalClientCertificate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate = __cordl_object
            .invoke("get_LocalClientCertificate", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        parent: *mut crate::Mono::Net::Security::MobileAuthenticatedStream,
        options: *mut crate::Mono::Net::Security::MonoSslAuthenticationOptions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (parent, options))?;
        Ok(__cordl_ret)
    }
    pub fn WriteCallback(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppObject,
        bufferLen: crate::System::IntPtr,
        errorState: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("WriteCallback", (data, bufferLen, errorState))?;
        Ok(__cordl_ret)
    }
    pub fn get_RemoteCertificate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate2 = __cordl_object
            .invoke("get_RemoteCertificate", ())?;
        Ok(__cordl_ret)
    }
    pub fn ReadCallback(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppObject,
        bufferLen: crate::System::IntPtr,
        errorState: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("ReadCallback", (buffer, bufferLen, errorState))?;
        Ok(__cordl_ret)
    }
    pub fn CertificateCallback(
        &mut self,
        ctx: *mut quest_hook::libil2cpp::Il2CppObject,
        cn: *mut quest_hook::libil2cpp::Il2CppObject,
        cnLen: crate::System::IntPtr,
        caList: *mut quest_hook::libil2cpp::Il2CppObject,
        caListLen: crate::System::IntPtr,
        chain: *mut quest_hook::libil2cpp::Il2CppObject,
        key: *mut quest_hook::libil2cpp::Il2CppObject,
        errorState: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CertificateCallback",
                (ctx, cn, cnLen, caList, caListLen, chain, key, errorState),
            )?;
        Ok(__cordl_ret)
    }
    pub fn StartHandshake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartHandshake", ())?;
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
    pub fn PendingRenegotiation(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("PendingRenegotiation", ())?;
        Ok(__cordl_ret)
    }
    pub fn Shutdown(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Shutdown", ())?;
        Ok(__cordl_ret)
    }
    pub fn VerifyCallback(
        &mut self,
        chain: crate::Mono::Unity::UnityTls_unitytls_x509list_ref,
        errorState: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<
        crate::Mono::Unity::UnityTls_unitytls_x509verify_result,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Mono::Unity::UnityTls_unitytls_x509verify_result = __cordl_object
            .invoke("VerifyCallback", (chain, errorState))?;
        Ok(__cordl_ret)
    }
    pub fn FinishHandshake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinishHandshake", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsAuthenticated(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsAuthenticated", ())?;
        Ok(__cordl_ret)
    }
    pub fn Read(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::ValueTuple_2<i32, bool>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::ValueTuple_2<i32, bool> = __cordl_object
            .invoke("Read", (buffer, offset, count))?;
        Ok(__cordl_ret)
    }
    pub fn Write(
        &mut self,
        buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::ValueTuple_2<i32, bool>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::ValueTuple_2<i32, bool> = __cordl_object
            .invoke("Write", (buffer, offset, count))?;
        Ok(__cordl_ret)
    }
    pub fn Renegotiate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Renegotiate", ())?;
        Ok(__cordl_ret)
    }
    pub fn ProcessHandshake(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ProcessHandshake", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        parent: *mut crate::Mono::Net::Security::MobileAuthenticatedStream,
        options: *mut crate::Mono::Net::Security::MonoSslAuthenticationOptions,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parent, options))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Mono+Unity+UnityTlsContext")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Unity::UnityTlsContext {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
