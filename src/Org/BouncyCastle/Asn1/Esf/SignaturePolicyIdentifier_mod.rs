#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+SignaturePolicyIdentifier")]
#[repr(C)]
#[derive(Debug)]
pub struct SignaturePolicyIdentifier {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub sigPolicy: *mut crate::Org::BouncyCastle::Asn1::Esf::SignaturePolicyId,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+SignaturePolicyIdentifier")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::Esf::SignaturePolicyIdentifier =>
    "Org.BouncyCastle.Asn1.Esf"."SignaturePolicyIdentifier"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+SignaturePolicyIdentifier")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Esf::SignaturePolicyIdentifier {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+SignaturePolicyIdentifier")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Esf::SignaturePolicyIdentifier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+SignaturePolicyIdentifier")]
impl crate::Org::BouncyCastle::Asn1::Esf::SignaturePolicyIdentifier {
    pub fn get_SignaturePolicyId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Esf::SignaturePolicyId,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Esf::SignaturePolicyId = __cordl_object
            .invoke("get_SignaturePolicyId", ())?;
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
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_SignaturePolicyId1(
        &mut self,
        signaturePolicyId: *mut crate::Org::BouncyCastle::Asn1::Esf::SignaturePolicyId,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (signaturePolicyId))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_SignaturePolicyId1(
        signaturePolicyId: *mut crate::Org::BouncyCastle::Asn1::Esf::SignaturePolicyId,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (signaturePolicyId))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+SignaturePolicyIdentifier")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Esf::SignaturePolicyIdentifier {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
