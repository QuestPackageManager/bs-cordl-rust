#[cfg(feature = "Org+BouncyCastle+Asn1+Ocsp+OcspResponse")]
#[repr(C)]
#[derive(Debug)]
pub struct OcspResponse {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub responseStatus: *mut crate::Org::BouncyCastle::Asn1::Ocsp::OcspResponseStatus,
    pub responseBytes: *mut crate::Org::BouncyCastle::Asn1::Ocsp::ResponseBytes,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Ocsp+OcspResponse")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Ocsp::OcspResponse =>
    "Org.BouncyCastle.Asn1.Ocsp"."OcspResponse"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Ocsp+OcspResponse")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Ocsp::OcspResponse {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Ocsp+OcspResponse")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Ocsp::OcspResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Ocsp+OcspResponse")]
impl crate::Org::BouncyCastle::Asn1::Ocsp::OcspResponse {
    pub fn New_Asn1Sequence1(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
    }
    pub fn New_OcspResponseStatus_ResponseBytes0(
        responseStatus: *mut crate::Org::BouncyCastle::Asn1::Ocsp::OcspResponseStatus,
        responseBytes: *mut crate::Org::BouncyCastle::Asn1::Ocsp::ResponseBytes,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (responseStatus, responseBytes))?;
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
    pub fn _ctor_OcspResponseStatus_ResponseBytes0(
        &mut self,
        responseStatus: *mut crate::Org::BouncyCastle::Asn1::Ocsp::OcspResponseStatus,
        responseBytes: *mut crate::Org::BouncyCastle::Asn1::Ocsp::ResponseBytes,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (responseStatus, responseBytes))?;
        Ok(__cordl_ret)
    }
    pub fn get_ResponseBytes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Ocsp::ResponseBytes,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Ocsp::ResponseBytes = __cordl_object
            .invoke("get_ResponseBytes", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ResponseStatus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Ocsp::OcspResponseStatus,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Ocsp::OcspResponseStatus = __cordl_object
            .invoke("get_ResponseStatus", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Ocsp+OcspResponse")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Ocsp::OcspResponse {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}