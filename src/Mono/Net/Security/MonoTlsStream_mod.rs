#[cfg(feature = "Mono+Net+Security+MonoTlsStream")]
#[repr(C)]
#[derive(Debug)]
pub struct MonoTlsStream {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub provider: *mut crate::Mono::Net::Security::MobileTlsProvider,
    pub networkStream: *mut crate::System::Net::Sockets::NetworkStream,
    pub request: *mut crate::System::Net::HttpWebRequest,
    pub settings: *mut crate::Mono::Security::Interface::MonoTlsSettings,
    pub sslStream: *mut crate::System::Net::Security::SslStream,
    pub sslStreamLock: *mut quest_hook::libil2cpp::Il2CppObject,
    pub status: crate::System::Net::WebExceptionStatus,
    pub _CertificateValidationFailed_k__BackingField: bool,
}
#[cfg(feature = "Mono+Net+Security+MonoTlsStream")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Net::Security::MonoTlsStream =>
    "Mono.Net.Security"."MonoTlsStream"
);
#[cfg(feature = "Mono+Net+Security+MonoTlsStream")]
impl std::ops::Deref for crate::Mono::Net::Security::MonoTlsStream {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Net+Security+MonoTlsStream")]
impl std::ops::DerefMut for crate::Mono::Net::Security::MonoTlsStream {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Net+Security+MonoTlsStream")]
impl crate::Mono::Net::Security::MonoTlsStream {
    #[cfg(feature = "Mono+Net+Security+MonoTlsStream+_CreateStream_d__18")]
    pub type _CreateStream_d__18 = crate::Mono::Net::Security::MonoTlsStream__CreateStream_d__18;
    pub fn CloseSslStream(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CloseSslStream", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateStream(
        &mut self,
        tunnel: quest_hook::libil2cpp::Gc<crate::System::Net::WebConnectionTunnel>,
        cancellationToken: crate::System::Threading::CancellationToken,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<*mut crate::System::IO::Stream>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Threading::Tasks::Task_1<*mut crate::System::IO::Stream>,
        > = __cordl_object.invoke("CreateStream", (tunnel, cancellationToken))?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        request: quest_hook::libil2cpp::Gc<crate::System::Net::HttpWebRequest>,
        networkStream: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::NetworkStream,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (request, networkStream))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        request: quest_hook::libil2cpp::Gc<crate::System::Net::HttpWebRequest>,
        networkStream: quest_hook::libil2cpp::Gc<
            crate::System::Net::Sockets::NetworkStream,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (request, networkStream))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CertificateValidationFailed(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_CertificateValidationFailed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ExceptionStatus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::WebExceptionStatus> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Net::WebExceptionStatus = __cordl_object
            .invoke("get_ExceptionStatus", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Request(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Net::HttpWebRequest>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Net::HttpWebRequest> = __cordl_object
            .invoke("get_Request", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_CertificateValidationFailed(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_CertificateValidationFailed", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Net+Security+MonoTlsStream")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Net::Security::MonoTlsStream {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
