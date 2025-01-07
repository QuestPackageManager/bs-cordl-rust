#[cfg(feature = "Org+BouncyCastle+Cmp+CertificateStatus")]
#[repr(C)]
#[derive(Debug)]
pub struct CertificateStatus {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub digestAlgFinder: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Cms::DefaultDigestAlgorithmIdentifierFinder,
    >,
    pub certStatus: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Cmp::CertStatus,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Cmp+CertificateStatus")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Cmp::CertificateStatus {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Cmp";
    const CLASS_NAME: &'static str = "CertificateStatus";
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
#[cfg(feature = "Org+BouncyCastle+Cmp+CertificateStatus")]
impl std::ops::Deref for crate::Org::BouncyCastle::Cmp::CertificateStatus {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cmp+CertificateStatus")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Cmp::CertificateStatus {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Cmp+CertificateStatus")]
impl crate::Org::BouncyCastle::Cmp::CertificateStatus {
    pub fn IsVerified(
        &mut self,
        cert: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::X509::X509Certificate>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsVerified", (cert))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        digestAlgFinder: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::DefaultDigestAlgorithmIdentifierFinder,
        >,
        certStatus: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::CertStatus,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (digestAlgFinder, certStatus))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        digestAlgFinder: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Cms::DefaultDigestAlgorithmIdentifierFinder,
        >,
        certStatus: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::CertStatus,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (digestAlgFinder, certStatus))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_CertRequestId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("get_CertRequestId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PkiStatusInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Asn1::Cmp::PkiStatusInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Cmp::PkiStatusInfo,
        > = __cordl_object.invoke("get_PkiStatusInfo", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Cmp+CertificateStatus")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Cmp::CertificateStatus {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
