#[cfg(feature = "Mono+Unity+UnityTlsConversions")]
#[repr(C)]
#[derive(Debug)]
pub struct UnityTlsConversions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Mono+Unity+UnityTlsConversions")]
unsafe impl quest_hook::libil2cpp::Type for crate::Mono::Unity::UnityTlsConversions {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Unity";
    const CLASS_NAME: &'static str = "UnityTlsConversions";
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
#[cfg(feature = "Mono+Unity+UnityTlsConversions")]
impl std::ops::Deref for crate::Mono::Unity::UnityTlsConversions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTlsConversions")]
impl std::ops::DerefMut for crate::Mono::Unity::UnityTlsConversions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Unity+UnityTlsConversions")]
impl crate::Mono::Unity::UnityTlsConversions {
    pub fn ConvertProtocolVersion(
        protocol: crate::Mono::Unity::UnityTls_unitytls_protocol,
    ) -> quest_hook::libil2cpp::Result<crate::Mono::Security::Interface::TlsProtocols> {
        let __cordl_ret: crate::Mono::Security::Interface::TlsProtocols = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertProtocolVersion", (protocol))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMaxProtocol(
        protocols: crate::System::Security::Authentication::SslProtocols,
    ) -> quest_hook::libil2cpp::Result<crate::Mono::Unity::UnityTls_unitytls_protocol> {
        let __cordl_ret: crate::Mono::Unity::UnityTls_unitytls_protocol = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMaxProtocol", (protocols))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMinProtocol(
        protocols: crate::System::Security::Authentication::SslProtocols,
    ) -> quest_hook::libil2cpp::Result<crate::Mono::Unity::UnityTls_unitytls_protocol> {
        let __cordl_ret: crate::Mono::Unity::UnityTls_unitytls_protocol = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMinProtocol", (protocols))?;
        Ok(__cordl_ret.into())
    }
    pub fn VerifyResultToAlertDescription(
        verifyResult: crate::Mono::Unity::UnityTls_unitytls_x509verify_result,
        defaultAlert: crate::Mono::Security::Interface::AlertDescription,
    ) -> quest_hook::libil2cpp::Result<
        crate::Mono::Security::Interface::AlertDescription,
    > {
        let __cordl_ret: crate::Mono::Security::Interface::AlertDescription = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("VerifyResultToAlertDescription", (verifyResult, defaultAlert))?;
        Ok(__cordl_ret.into())
    }
    pub fn VerifyResultToChainStatus(
        verifyResult: crate::Mono::Unity::UnityTls_unitytls_x509verify_result,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags,
    > {
        let __cordl_ret: crate::System::Security::Cryptography::X509Certificates::X509ChainStatusFlags = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("VerifyResultToChainStatus", (verifyResult))?;
        Ok(__cordl_ret.into())
    }
    pub fn VerifyResultToPolicyErrror(
        verifyResult: crate::Mono::Unity::UnityTls_unitytls_x509verify_result,
    ) -> quest_hook::libil2cpp::Result<crate::System::Net::Security::SslPolicyErrors> {
        let __cordl_ret: crate::System::Net::Security::SslPolicyErrors = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("VerifyResultToPolicyErrror", (verifyResult))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Unity+UnityTlsConversions")]
impl quest_hook::libil2cpp::ObjectType for crate::Mono::Unity::UnityTlsConversions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
