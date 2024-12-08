#[cfg(feature = "Org+BouncyCastle+Asn1+Ocsp+Request")]
#[repr(C)]
#[derive(Debug)]
pub struct Request {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub reqCert: *mut crate::Org::BouncyCastle::Asn1::Ocsp::CertID,
    pub singleRequestExtensions: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Ocsp+Request")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Ocsp::Request =>
    "Org.BouncyCastle.Asn1.Ocsp"."Request"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Ocsp+Request")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Ocsp::Request {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Ocsp+Request")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Ocsp::Request {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Ocsp+Request")]
impl crate::Org::BouncyCastle::Asn1::Ocsp::Request {
    pub fn get_ReqCert(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Ocsp::CertID,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Ocsp::CertID = __cordl_object
            .invoke("get_ReqCert", ())?;
        Ok(__cordl_ret)
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
    pub fn _ctor_CertID_X509Extensions0(
        &mut self,
        reqCert: *mut crate::Org::BouncyCastle::Asn1::Ocsp::CertID,
        singleRequestExtensions: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (reqCert, singleRequestExtensions))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Asn1Sequence1(
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
    pub fn get_SingleRequestExtensions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions = __cordl_object
            .invoke("get_SingleRequestExtensions", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_CertID_X509Extensions0(
        reqCert: *mut crate::Org::BouncyCastle::Asn1::Ocsp::CertID,
        singleRequestExtensions: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (reqCert, singleRequestExtensions))?;
        Ok(__cordl_object)
    }
    pub fn New_Asn1Sequence1(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Ocsp+Request")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Ocsp::Request {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
