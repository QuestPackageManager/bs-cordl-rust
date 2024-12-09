#[cfg(feature = "Internal+Cryptography+Pal+CertificateData")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CertificateData {
    pub RawData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub SubjectPublicKeyInfo: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub Version: i32,
    pub SerialNumber: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub TbsSignature: crate::Internal::Cryptography::Pal::CertificateData_AlgorithmIdentifier,
    pub Issuer: *mut crate::System::Security::Cryptography::X509Certificates::X500DistinguishedName,
    pub NotBefore: crate::System::DateTime,
    pub NotAfter: crate::System::DateTime,
    pub Subject: *mut crate::System::Security::Cryptography::X509Certificates::X500DistinguishedName,
    pub PublicKeyAlgorithm: crate::Internal::Cryptography::Pal::CertificateData_AlgorithmIdentifier,
    pub PublicKey: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub IssuerUniqueId: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub SubjectUniqueId: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub Extensions: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Security::Cryptography::X509Certificates::X509Extension,
    >,
    pub SignatureAlgorithm: crate::Internal::Cryptography::Pal::CertificateData_AlgorithmIdentifier,
    pub SignatureValue: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "Internal+Cryptography+Pal+CertificateData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Internal::Cryptography::Pal::CertificateData =>
    "Internal.Cryptography.Pal"."CertificateData"
);
#[cfg(feature = "Internal+Cryptography+Pal+CertificateData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Internal::Cryptography::Pal::CertificateData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Internal+Cryptography+Pal+CertificateData")]
impl crate::Internal::Cryptography::Pal::CertificateData {
    #[cfg(feature = "Internal+Cryptography+Pal+CertificateData+AlgorithmIdentifier")]
    pub type AlgorithmIdentifier = crate::Internal::Cryptography::Pal::CertificateData_AlgorithmIdentifier;
    #[cfg(feature = "Internal+Cryptography+Pal+CertificateData+_ReadReverseRdns_d__21")]
    pub type _ReadReverseRdns_d__21 = crate::Internal::Cryptography::Pal::CertificateData__ReadReverseRdns_d__21;
    pub fn GetNameInfo(
        &mut self,
        nameType: crate::System::Security::Cryptography::X509Certificates::X509NameType,
        forIssuer: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetNameInfo",
            (nameType, forIssuer),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        rawData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (rawData),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Internal+Cryptography+Pal+CertificateData+AlgorithmIdentifier")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct CertificateData_AlgorithmIdentifier {
    pub AlgorithmId: *mut crate::System::String,
    pub Parameters: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "Internal+Cryptography+Pal+CertificateData+AlgorithmIdentifier")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Internal::Cryptography::Pal::CertificateData_AlgorithmIdentifier =>
    "Internal.Cryptography.Pal"."CertificateData/AlgorithmIdentifier"
);
#[cfg(feature = "Internal+Cryptography+Pal+CertificateData+AlgorithmIdentifier")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Internal::Cryptography::Pal::CertificateData_AlgorithmIdentifier {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Internal+Cryptography+Pal+CertificateData+AlgorithmIdentifier")]
impl crate::Internal::Cryptography::Pal::CertificateData_AlgorithmIdentifier {}
