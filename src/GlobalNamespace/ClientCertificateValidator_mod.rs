#[cfg(feature = "ClientCertificateValidator")]
#[repr(C)]
#[derive(Debug)]
pub struct ClientCertificateValidator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ClientCertificateValidator")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ClientCertificateValidator {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ClientCertificateValidator";
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
#[cfg(feature = "ClientCertificateValidator")]
impl std::ops::Deref for crate::GlobalNamespace::ClientCertificateValidator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ClientCertificateValidator")]
impl std::ops::DerefMut for crate::GlobalNamespace::ClientCertificateValidator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ClientCertificateValidator")]
impl crate::GlobalNamespace::ClientCertificateValidator {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ValidateCertificateChain(
        &mut self,
        endPoint: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::DnsEndPoint>,
        certificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
        certificateChain: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ValidateCertificateChain",
                (endPoint, certificate, certificateChain),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateCertificateChainInternal(
        &mut self,
        endPoint: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::DnsEndPoint>,
        certificate: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X509Certificate2,
        >,
        certificateChain: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ValidateCertificateChainInternal",
                (endPoint, certificate, certificateChain),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ClientCertificateValidator")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ClientCertificateValidator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ClientCertificateValidator")]
impl AsRef<crate::GlobalNamespace::ICertificateValidator>
for crate::GlobalNamespace::ClientCertificateValidator {
    fn as_ref(&self) -> &crate::GlobalNamespace::ICertificateValidator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "ClientCertificateValidator")]
impl AsMut<crate::GlobalNamespace::ICertificateValidator>
for crate::GlobalNamespace::ClientCertificateValidator {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::ICertificateValidator {
        unsafe { std::mem::transmute(self) }
    }
}
