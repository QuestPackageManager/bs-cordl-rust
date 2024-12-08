#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiFailureInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct PkiFailureInfo {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::DerBitString,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiFailureInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Cmp::PkiFailureInfo =>
    "Org.BouncyCastle.Asn1.Cmp"."PkiFailureInfo"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiFailureInfo")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Cmp::PkiFailureInfo {
    type Target = crate::Org::BouncyCastle::Asn1::DerBitString;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiFailureInfo")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Cmp::PkiFailureInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiFailureInfo")]
impl crate::Org::BouncyCastle::Asn1::Cmp::PkiFailureInfo {
    pub const AddInfoNotAvailable: i32 = 4194304i32;
    pub const BadAlg: i32 = 128i32;
    pub const BadCertId: i32 = 8i32;
    pub const BadCertTemplate: i32 = 1048576i32;
    pub const BadDataFormat: i32 = 4i32;
    pub const BadMessageCheck: i32 = 64i32;
    pub const BadPop: i32 = 16384i32;
    pub const BadRecipientNonce: i32 = 1024i32;
    pub const BadRequest: i32 = 32i32;
    pub const BadSenderNonce: i32 = 2097152i32;
    pub const BadTime: i32 = 16i32;
    pub const CertConfirmed: i32 = 4096i32;
    pub const CertRevoked: i32 = 8192i32;
    pub const DuplicateCertReq: i32 = 536870912i32;
    pub const IncorrectData: i32 = 1i32;
    pub const MissingTimeStamp: i32 = 32768i32;
    pub const NotAuthorized: i32 = 65536i32;
    pub const SignerNotTrusted: i32 = 524288i32;
    pub const SystemFailure: i32 = 1073741824i32;
    pub const SystemUnavail: i32 = -2147483648i32;
    pub const TimeNotAvailable: i32 = 512i32;
    pub const TransactionIdInUse: i32 = 262144i32;
    pub const UnacceptedExtension: i32 = 8388608i32;
    pub const UnacceptedPolicy: i32 = 256i32;
    pub const UnsupportedVersion: i32 = 131072i32;
    pub const WrongAuthority: i32 = 2i32;
    pub const WrongIntegrity: i32 = 2048i32;
    pub fn New_DerBitString1(
        info: *mut crate::Org::BouncyCastle::Asn1::DerBitString,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_0(info: i32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info))?;
        Ok(__cordl_object)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DerBitString1(
        &mut self,
        info: *mut crate::Org::BouncyCastle::Asn1::DerBitString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (info))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_0(
        &mut self,
        info: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (info))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+PkiFailureInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Cmp::PkiFailureInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
