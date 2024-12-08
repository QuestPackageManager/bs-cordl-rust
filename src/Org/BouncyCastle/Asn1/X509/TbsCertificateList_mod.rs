#[cfg(
    feature = "Org+BouncyCastle+Asn1+X509+TbsCertificateList+RevokedCertificatesEnumeration"
)]
#[repr(C)]
#[derive(Debug)]
pub struct TbsCertificateList_RevokedCertificatesEnumeration {
    __cordl_parent: crate::System::Object,
    pub en: *mut crate::System::Collections::IEnumerable,
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+X509+TbsCertificateList+RevokedCertificatesEnumeration"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::X509::TbsCertificateList_RevokedCertificatesEnumeration =>
    "Org.BouncyCastle.Asn1.X509"."TbsCertificateList/RevokedCertificatesEnumeration"
);
#[cfg(
    feature = "Org+BouncyCastle+Asn1+X509+TbsCertificateList+RevokedCertificatesEnumeration"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::X509::TbsCertificateList_RevokedCertificatesEnumeration {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+X509+TbsCertificateList+RevokedCertificatesEnumeration"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::X509::TbsCertificateList_RevokedCertificatesEnumeration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+X509+TbsCertificateList+RevokedCertificatesEnumeration"
)]
impl crate::Org::BouncyCastle::Asn1::X509::TbsCertificateList_RevokedCertificatesEnumeration {
    #[cfg(
        feature = "Org+BouncyCastle+Asn1+X509+TbsCertificateList+RevokedCertificatesEnumeration+RevokedCertificatesEnumerator"
    )]
    pub type RevokedCertificatesEnumerator = crate::Org::BouncyCastle::Asn1::X509::RevokedCertificatesEnumeration_RevokedCertificatesEnumerator;
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("GetEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        en: *mut crate::System::Collections::IEnumerable,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (en))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        en: *mut crate::System::Collections::IEnumerable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (en))?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+X509+TbsCertificateList+RevokedCertificatesEnumeration"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::X509::TbsCertificateList_RevokedCertificatesEnumeration {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+X509+TbsCertificateList+RevokedCertificatesEnumeration+RevokedCertificatesEnumerator"
)]
#[repr(C)]
#[derive(Debug)]
pub struct RevokedCertificatesEnumeration_RevokedCertificatesEnumerator {
    __cordl_parent: crate::System::Object,
    pub e: *mut crate::System::Collections::IEnumerator,
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+X509+TbsCertificateList+RevokedCertificatesEnumeration+RevokedCertificatesEnumerator"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::X509::RevokedCertificatesEnumeration_RevokedCertificatesEnumerator
    => "Org.BouncyCastle.Asn1.X509"
    ."TbsCertificateList/RevokedCertificatesEnumeration/RevokedCertificatesEnumerator"
);
#[cfg(
    feature = "Org+BouncyCastle+Asn1+X509+TbsCertificateList+RevokedCertificatesEnumeration+RevokedCertificatesEnumerator"
)]
impl std::ops::Deref
for crate::Org::BouncyCastle::Asn1::X509::RevokedCertificatesEnumeration_RevokedCertificatesEnumerator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+X509+TbsCertificateList+RevokedCertificatesEnumeration+RevokedCertificatesEnumerator"
)]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::X509::RevokedCertificatesEnumeration_RevokedCertificatesEnumerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+X509+TbsCertificateList+RevokedCertificatesEnumeration+RevokedCertificatesEnumerator"
)]
impl crate::Org::BouncyCastle::Asn1::X509::RevokedCertificatesEnumeration_RevokedCertificatesEnumerator {
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveNext", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        e: *mut crate::System::Collections::IEnumerator,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (e))?;
        Ok(__cordl_object)
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        e: *mut crate::System::Collections::IEnumerator,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (e))?;
        Ok(__cordl_ret)
    }
    pub fn get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("get_Current", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(
    feature = "Org+BouncyCastle+Asn1+X509+TbsCertificateList+RevokedCertificatesEnumeration+RevokedCertificatesEnumerator"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::X509::RevokedCertificatesEnumeration_RevokedCertificatesEnumerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+TbsCertificateList")]
#[repr(C)]
#[derive(Debug)]
pub struct TbsCertificateList {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    pub version: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
    pub signature: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    pub issuer: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
    pub thisUpdate: *mut crate::Org::BouncyCastle::Asn1::X509::Time,
    pub nextUpdate: *mut crate::Org::BouncyCastle::Asn1::X509::Time,
    pub revokedCertificates: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    pub crlExtensions: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+TbsCertificateList")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::X509::TbsCertificateList => "Org.BouncyCastle.Asn1.X509"
    ."TbsCertificateList"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+TbsCertificateList")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::X509::TbsCertificateList {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+TbsCertificateList")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::X509::TbsCertificateList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+TbsCertificateList")]
impl crate::Org::BouncyCastle::Asn1::X509::TbsCertificateList {
    #[cfg(
        feature = "Org+BouncyCastle+Asn1+X509+TbsCertificateList+RevokedCertificatesEnumeration"
    )]
    pub type RevokedCertificatesEnumeration = crate::Org::BouncyCastle::Asn1::X509::TbsCertificateList_RevokedCertificatesEnumeration;
    pub fn GetRevokedCertificateEnumeration(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerable> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerable = __cordl_object
            .invoke("GetRevokedCertificateEnumeration", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetRevokedCertificates(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Asn1::X509::CrlEntry,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Asn1::X509::CrlEntry,
        > = __cordl_object.invoke("GetRevokedCertificates", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
    }
    pub fn ToAsn1Object(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::Asn1Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Object = __cordl_object
            .invoke("ToAsn1Object", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (seq))?;
        Ok(__cordl_ret)
    }
    pub fn get_Extensions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions = __cordl_object
            .invoke("get_Extensions", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Issuer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::X509Name,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::X509Name = __cordl_object
            .invoke("get_Issuer", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_NextUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::X509::Time> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::Time = __cordl_object
            .invoke("get_NextUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Signature(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier = __cordl_object
            .invoke("get_Signature", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ThisUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::X509::Time> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::Time = __cordl_object
            .invoke("get_ThisUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Version(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Version", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_VersionNumber(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::DerInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerInteger = __cordl_object
            .invoke("get_VersionNumber", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+X509+TbsCertificateList")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::X509::TbsCertificateList {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
