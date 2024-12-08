#[cfg(feature = "Mono+Security+Interface+MonoRemoteCertificateValidationCallback")]
#[repr(C)]
#[derive(Debug)]
pub struct MonoRemoteCertificateValidationCallback {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "Mono+Security+Interface+MonoRemoteCertificateValidationCallback")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Mono::Security::Interface::MonoRemoteCertificateValidationCallback =>
    "Mono.Security.Interface"."MonoRemoteCertificateValidationCallback"
);
#[cfg(feature = "Mono+Security+Interface+MonoRemoteCertificateValidationCallback")]
impl std::ops::Deref
for crate::Mono::Security::Interface::MonoRemoteCertificateValidationCallback {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Interface+MonoRemoteCertificateValidationCallback")]
impl std::ops::DerefMut
for crate::Mono::Security::Interface::MonoRemoteCertificateValidationCallback {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Security+Interface+MonoRemoteCertificateValidationCallback")]
impl crate::Mono::Security::Interface::MonoRemoteCertificateValidationCallback {
    pub fn _ctor(
        &mut self,
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
    pub fn Invoke(
        &mut self,
        targetHost: *mut crate::System::String,
        certificate: *mut crate::System::Security::Cryptography::X509Certificates::X509Certificate,
        chain: *mut crate::System::Security::Cryptography::X509Certificates::X509Chain,
        sslPolicyErrors: crate::Mono::Security::Interface::MonoSslPolicyErrors,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Invoke", (targetHost, certificate, chain, sslPolicyErrors))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut crate::System::Object,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Mono+Security+Interface+MonoRemoteCertificateValidationCallback")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Security::Interface::MonoRemoteCertificateValidationCallback {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
