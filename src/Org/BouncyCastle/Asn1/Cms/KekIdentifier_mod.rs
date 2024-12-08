#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+KekIdentifier")]
#[repr(C)]
#[derive(Debug)]
pub struct KekIdentifier {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub keyIdentifier: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    pub date: *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
    pub other: *mut crate::Org::BouncyCastle::Asn1::Cms::OtherKeyAttribute,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+KekIdentifier")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Cms::KekIdentifier =>
    "Org.BouncyCastle.Asn1.Cms"."KekIdentifier"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+KekIdentifier")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Cms::KekIdentifier {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+KekIdentifier")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Cms::KekIdentifier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+KekIdentifier")]
impl crate::Org::BouncyCastle::Asn1::Cms::KekIdentifier {
    pub fn get_Other(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Cms::OtherKeyAttribute,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Cms::OtherKeyAttribute = __cordl_object
            .invoke("get_Other", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppArray_DerGeneralizedTime_OtherKeyAttribute0(
        &mut self,
        keyIdentifier: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        date: *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
        other: *mut crate::Org::BouncyCastle::Asn1::Cms::OtherKeyAttribute,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (keyIdentifier, date, other))?;
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
    pub fn get_KeyIdentifier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1OctetString = __cordl_object
            .invoke("get_KeyIdentifier", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Date(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime = __cordl_object
            .invoke("get_Date", ())?;
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
    pub fn New_Il2CppArray_DerGeneralizedTime_OtherKeyAttribute0(
        keyIdentifier: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        date: *mut crate::Org::BouncyCastle::Asn1::DerGeneralizedTime,
        other: *mut crate::Org::BouncyCastle::Asn1::Cms::OtherKeyAttribute,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (keyIdentifier, date, other))?;
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
#[cfg(feature = "Org+BouncyCastle+Asn1+Cms+KekIdentifier")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Cms::KekIdentifier {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
