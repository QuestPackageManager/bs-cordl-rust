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
    pub fn CertificateCallback(
        &mut self,
        ctx: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        cn: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        cnLen: crate::System::IntPtr,
        caList: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        caListLen: crate::System::IntPtr,
        chain: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        errorState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "CertificateCallback",
                (ctx, cn, cnLen, caList, caListLen, chain, key, errorState),
            )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn FinishHandshake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinishHandshake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        parent: quest_hook::libil2cpp::Gc<
            crate::Mono::Net::Security::MobileAuthenticatedStream,
        >,
        options: quest_hook::libil2cpp::Gc<
            crate::Mono::Net::Security::MonoSslAuthenticationOptions,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parent, options))?;
        Ok(__cordl_object.into())
    }
    pub fn PendingRenegotiation(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("PendingRenegotiation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessHandshake(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ProcessHandshake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Read(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::ValueTuple_2<i32, bool>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::ValueTuple_2<i32, bool> = __cordl_object
            .invoke("Read", (buffer, offset, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadCallback(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bufferLen: crate::System::IntPtr,
        errorState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("ReadCallback", (buffer, bufferLen, errorState))?;
        Ok(__cordl_ret.into())
    }
    pub fn Renegotiate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Renegotiate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Shutdown(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Shutdown", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn StartHandshake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("StartHandshake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn VerifyCallback(
        &mut self,
        chain: crate::Mono::Unity::UnityTls_unitytls_x509list_ref,
        errorState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        crate::Mono::Unity::UnityTls_unitytls_x509verify_result,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Mono::Unity::UnityTls_unitytls_x509verify_result = __cordl_object
            .invoke("VerifyCallback", (chain, errorState))?;
        Ok(__cordl_ret.into())
    }
    pub fn Write(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::ValueTuple_2<i32, bool>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::ValueTuple_2<i32, bool> = __cordl_object
            .invoke("Write", (buffer, offset, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteCallback(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        bufferLen: crate::System::IntPtr,
        errorState: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::System::IntPtr> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::IntPtr = __cordl_object
            .invoke("WriteCallback", (data, bufferLen, errorState))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        parent: quest_hook::libil2cpp::Gc<
            crate::Mono::Net::Security::MobileAuthenticatedStream,
        >,
        options: quest_hook::libil2cpp::Gc<
            crate::Mono::Net::Security::MonoSslAuthenticationOptions,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (parent, options))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsAuthenticated(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsAuthenticated", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_LocalClientCertificate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        > = __cordl_object.invoke("get_LocalClientCertificate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RemoteCertificate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        > = __cordl_object.invoke("get_RemoteCertificate", ())?;
        Ok(__cordl_ret.into())
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
