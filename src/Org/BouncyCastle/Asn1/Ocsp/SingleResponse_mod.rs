#[cfg(feature = "Org+BouncyCastle+Asn1+Ocsp+SingleResponse")]
#[repr(C)]
#[derive(Debug)]
pub struct SingleResponse {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub certID: *mut crate::Org::BouncyCastle::Asn1::Ocsp::CertID,
    pub certStatus: *mut crate::Org::BouncyCastle::Asn1::Ocsp::CertStatus,
    pub thisUpdate: *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
    pub nextUpdate: *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
    pub singleExtensions: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Ocsp+SingleResponse")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Ocsp::SingleResponse =>
    "Org.BouncyCastle.Asn1.Ocsp"."SingleResponse"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Ocsp+SingleResponse")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Ocsp::SingleResponse {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Ocsp+SingleResponse")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Ocsp::SingleResponse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Ocsp+SingleResponse")]
impl crate::Org::BouncyCastle::Asn1::Ocsp::SingleResponse {
    pub fn New_Asn1Sequence1(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
    }
    pub fn New_CertID_CertStatus_DerGeneralizedTime_DerGeneralizedTime_X509Extensions0(
        certID: *mut crate::Org::BouncyCastle::Asn1::Ocsp::CertID,
        certStatus: *mut crate::Org::BouncyCastle::Asn1::Ocsp::CertStatus,
        thisUpdate: *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
        nextUpdate: *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
        singleExtensions: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (certID, certStatus, thisUpdate, nextUpdate, singleExtensions),
            )?;
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
    pub fn _ctor_CertID_CertStatus_DerGeneralizedTime_DerGeneralizedTime_X509Extensions0(
        &mut self,
        certID: *mut crate::Org::BouncyCastle::Asn1::Ocsp::CertID,
        certStatus: *mut crate::Org::BouncyCastle::Asn1::Ocsp::CertStatus,
        thisUpdate: *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
        nextUpdate: *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
        singleExtensions: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (certID, certStatus, thisUpdate, nextUpdate, singleExtensions),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_CertId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Ocsp::CertID,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Ocsp::CertID = __cordl_object
            .invoke("get_CertId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CertStatus(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Ocsp::CertStatus,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Ocsp::CertStatus = __cordl_object
            .invoke("get_CertStatus", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_NextUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime = __cordl_object
            .invoke("get_NextUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SingleExtensions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::X509::X509Extensions = __cordl_object
            .invoke("get_SingleExtensions", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_ThisUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime = __cordl_object
            .invoke("get_ThisUpdate", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Ocsp+SingleResponse")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Ocsp::SingleResponse {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}