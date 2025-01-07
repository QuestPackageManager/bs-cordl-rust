#[cfg(feature = "Internal+Cryptography+Pal+CertificateData")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CertificateData {
    pub RawData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub SubjectPublicKeyInfo: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<u8>,
    >,
    pub Version: i32,
    pub SerialNumber: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub TbsSignature: crate::Internal::Cryptography::Pal::CertificateData_AlgorithmIdentifier,
    pub Issuer: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::X509Certificates::X500DistinguishedName,
    >,
    pub NotBefore: crate::System::DateTime,
    pub NotAfter: crate::System::DateTime,
    pub Subject: quest_hook::libil2cpp::Gc<
        crate::System::Security::Cryptography::X509Certificates::X500DistinguishedName,
    >,
    pub PublicKeyAlgorithm: crate::Internal::Cryptography::Pal::CertificateData_AlgorithmIdentifier,
    pub PublicKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub IssuerUniqueId: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<u8>,
    >,
    pub SubjectUniqueId: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<u8>,
    >,
    pub Extensions: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::System::Security::Cryptography::X509Certificates::X509Extension,
            >,
        >,
    >,
    pub SignatureAlgorithm: crate::Internal::Cryptography::Pal::CertificateData_AlgorithmIdentifier,
    pub SignatureValue: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<u8>,
    >,
}
#[cfg(feature = "Internal+Cryptography+Pal+CertificateData")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Internal::Cryptography::Pal::CertificateData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Internal.Cryptography.Pal";
    const CLASS_NAME: &'static str = "CertificateData";
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
#[cfg(feature = "Internal+Cryptography+Pal+CertificateData")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Internal::Cryptography::Pal::CertificateData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Internal+Cryptography+Pal+CertificateData")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Internal::Cryptography::Pal::CertificateData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "Internal+Cryptography+Pal+CertificateData")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Internal::Cryptography::Pal::CertificateData {
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
#[cfg(feature = "Internal+Cryptography+Pal+CertificateData")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Internal::Cryptography::Pal::CertificateData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
    pub fn FindAltNameMatch(
        extensionBytes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
        matchType: crate::Internal::Cryptography::Pal::GeneralNameType,
        otherOid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FindAltNameMatch", (extensionBytes, matchType, otherOid))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNameInfo(
        &mut self,
        nameType: crate::System::Security::Cryptography::X509Certificates::X509NameType,
        forIssuer: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetNameInfo",
            (nameType, forIssuer),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSimpleNameInfo(
        name: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X500DistinguishedName,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSimpleNameInfo", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadReverseRdns(
        name: quest_hook::libil2cpp::Gc<
            crate::System::Security::Cryptography::X509Certificates::X500DistinguishedName,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::System::Collections::Generic::KeyValuePair_2<
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::System::Collections::Generic::KeyValuePair_2<
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ReadReverseRdns", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        rawData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (rawData),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Internal+Cryptography+Pal+CertificateData+AlgorithmIdentifier")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CertificateData_AlgorithmIdentifier {
    pub AlgorithmId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub Parameters: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
}
#[cfg(feature = "Internal+Cryptography+Pal+CertificateData+AlgorithmIdentifier")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Internal::Cryptography::Pal::CertificateData_AlgorithmIdentifier {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Internal.Cryptography.Pal";
    const CLASS_NAME: &'static str = "AlgorithmIdentifier";
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
#[cfg(feature = "Internal+Cryptography+Pal+CertificateData+AlgorithmIdentifier")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Internal::Cryptography::Pal::CertificateData_AlgorithmIdentifier {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Internal+Cryptography+Pal+CertificateData+AlgorithmIdentifier")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Internal::Cryptography::Pal::CertificateData_AlgorithmIdentifier {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "Internal+Cryptography+Pal+CertificateData+AlgorithmIdentifier")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Internal::Cryptography::Pal::CertificateData_AlgorithmIdentifier {
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
#[cfg(feature = "Internal+Cryptography+Pal+CertificateData+AlgorithmIdentifier")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Internal::Cryptography::Pal::CertificateData_AlgorithmIdentifier {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
