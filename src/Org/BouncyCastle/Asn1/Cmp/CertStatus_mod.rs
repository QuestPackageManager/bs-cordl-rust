#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CertStatus")]
#[repr(C)]
#[derive(Debug)]
pub struct CertStatus {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub certHash: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    pub certReqId: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
    pub statusInfo: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiStatusInfo,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CertStatus")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Cmp::CertStatus =>
    "Org.BouncyCastle.Asn1.Cmp"."CertStatus"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CertStatus")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Cmp::CertStatus {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CertStatus")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Cmp::CertStatus {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CertStatus")]
impl crate::Org::BouncyCastle::Asn1::Cmp::CertStatus {
    pub fn New_Asn1Sequence0(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray_BigInteger1(
        certHash: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        certReqId: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (certHash, certReqId))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppArray_BigInteger_PkiStatusInfo2(
        certHash: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        certReqId: *mut crate::Org::BouncyCastle::Math::BigInteger,
        statusInfo: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiStatusInfo,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (certHash, certReqId, statusInfo))?;
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
    pub fn _ctor_Asn1Sequence0(
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
    pub fn _ctor_Il2CppArray_BigInteger1(
        &mut self,
        certHash: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        certReqId: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (certHash, certReqId))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray_BigInteger_PkiStatusInfo2(
        &mut self,
        certHash: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        certReqId: *mut crate::Org::BouncyCastle::Math::BigInteger,
        statusInfo: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiStatusInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (certHash, certReqId, statusInfo))?;
        Ok(__cordl_ret)
    }
    pub fn get_CertHash(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString = __cordl_object
            .invoke("get_CertHash", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_CertReqID(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::DerInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerInteger = __cordl_object
            .invoke("get_CertReqID", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_StatusInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiStatusInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cmp::PkiStatusInfo = __cordl_object
            .invoke("get_StatusInfo", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cmp+CertStatus")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Cmp::CertStatus {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
