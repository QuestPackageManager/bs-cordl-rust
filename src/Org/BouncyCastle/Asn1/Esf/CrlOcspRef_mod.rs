#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+CrlOcspRef")]
#[repr(C)]
#[derive(Debug)]
pub struct CrlOcspRef {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub crlids: *mut crate::Org::BouncyCastle::Asn1::Esf::CrlListID,
    pub ocspids: *mut crate::Org::BouncyCastle::Asn1::Esf::OcspListID,
    pub otherRev: *mut crate::Org::BouncyCastle::Asn1::Esf::OtherRevRefs,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+CrlOcspRef")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Esf::CrlOcspRef =>
    "Org.BouncyCastle.Asn1.Esf"."CrlOcspRef"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+CrlOcspRef")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Esf::CrlOcspRef {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+CrlOcspRef")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Esf::CrlOcspRef {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+CrlOcspRef")]
impl crate::Org::BouncyCastle::Asn1::Esf::CrlOcspRef {
    pub fn get_CrlIDs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Esf::CrlListID,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Esf::CrlListID = __cordl_object
            .invoke("get_CrlIDs", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_OtherRev(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Esf::OtherRevRefs,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Esf::OtherRevRefs = __cordl_object
            .invoke("get_OtherRev", ())?;
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
    pub fn _ctor_CrlListID_OcspListID_OtherRevRefs1(
        &mut self,
        crlids: *mut crate::Org::BouncyCastle::Asn1::Esf::CrlListID,
        ocspids: *mut crate::Org::BouncyCastle::Asn1::Esf::OcspListID,
        otherRev: *mut crate::Org::BouncyCastle::Asn1::Esf::OtherRevRefs,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (crlids, ocspids, otherRev))?;
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
    pub fn get_OcspIDs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Esf::OcspListID,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Esf::OcspListID = __cordl_object
            .invoke("get_OcspIDs", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_Asn1Sequence0(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
    }
    pub fn New_CrlListID_OcspListID_OtherRevRefs1(
        crlids: *mut crate::Org::BouncyCastle::Asn1::Esf::CrlListID,
        ocspids: *mut crate::Org::BouncyCastle::Asn1::Esf::OcspListID,
        otherRev: *mut crate::Org::BouncyCastle::Asn1::Esf::OtherRevRefs,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (crlids, ocspids, otherRev))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+CrlOcspRef")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Esf::CrlOcspRef {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
