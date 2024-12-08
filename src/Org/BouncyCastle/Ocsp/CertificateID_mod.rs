#[cfg(feature = "Org+BouncyCastle+Ocsp+CertificateID")]
#[repr(C)]
#[derive(Debug)]
pub struct CertificateID {
    __cordl_parent: crate::System::Object,
    pub id: *mut crate::Org::BouncyCastle::Asn1::Ocsp::CertID,
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+CertificateID")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Ocsp::CertificateID =>
    "Org.BouncyCastle.Ocsp"."CertificateID"
);
#[cfg(feature = "Org+BouncyCastle+Ocsp+CertificateID")]
impl std::ops::Deref for crate::Org::BouncyCastle::Ocsp::CertificateID {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+CertificateID")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Ocsp::CertificateID {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+CertificateID")]
impl crate::Org::BouncyCastle::Ocsp::CertificateID {
    pub const HashSha1: &'static str = "1.3.14.3.2.26";
    pub fn Equals(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret)
    }
    pub fn ToAsn1Object(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Ocsp::CertID,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Ocsp::CertID = __cordl_object
            .invoke("ToAsn1Object", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_CertID0(
        &mut self,
        id: *mut crate::Org::BouncyCastle::Asn1::Ocsp::CertID,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (id))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String_X509Certificate_BigInteger1(
        &mut self,
        hashAlgorithm: *mut crate::System::String,
        issuerCert: *mut crate::Org::BouncyCastle::X509::X509Certificate,
        serialNumber: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (hashAlgorithm, issuerCert, serialNumber))?;
        Ok(__cordl_ret)
    }
    pub fn get_HashAlgOid(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_HashAlgOid", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetIssuerNameHash(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetIssuerNameHash", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetIssuerKeyHash(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetIssuerKeyHash", ())?;
        Ok(__cordl_ret)
    }
    pub fn MatchesIssuer(
        &mut self,
        issuerCert: *mut crate::Org::BouncyCastle::X509::X509Certificate,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MatchesIssuer", (issuerCert))?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SerialNumber(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("get_SerialNumber", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_CertID0(
        id: *mut crate::Org::BouncyCastle::Asn1::Ocsp::CertID,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (id))?;
        Ok(__cordl_object)
    }
    pub fn New_String_X509Certificate_BigInteger1(
        hashAlgorithm: *mut crate::System::String,
        issuerCert: *mut crate::Org::BouncyCastle::X509::X509Certificate,
        serialNumber: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (hashAlgorithm, issuerCert, serialNumber))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Ocsp+CertificateID")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Ocsp::CertificateID {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
