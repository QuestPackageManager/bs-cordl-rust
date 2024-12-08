#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+SignaturePolicyId")]
#[repr(C)]
#[derive(Debug)]
pub struct SignaturePolicyId {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub sigPolicyIdentifier: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    pub sigPolicyHash: *mut crate::Org::BouncyCastle::Asn1::Esf::OtherHashAlgAndValue,
    pub sigPolicyQualifiers: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+SignaturePolicyId")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Esf::SignaturePolicyId
    => "Org.BouncyCastle.Asn1.Esf"."SignaturePolicyId"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+SignaturePolicyId")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Esf::SignaturePolicyId {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+SignaturePolicyId")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Esf::SignaturePolicyId {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+SignaturePolicyId")]
impl crate::Org::BouncyCastle::Asn1::Esf::SignaturePolicyId {
    pub fn GetSigPolicyQualifiers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Asn1::Esf::SigPolicyQualifierInfo,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Asn1::Esf::SigPolicyQualifierInfo,
        > = __cordl_object.invoke("GetSigPolicyQualifiers", ())?;
        Ok(__cordl_ret)
    }
    pub fn New_Asn1Sequence0(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
    }
    pub fn New_DerObjectIdentifier_OtherHashAlgAndValue1(
        sigPolicyIdentifier: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        sigPolicyHash: *mut crate::Org::BouncyCastle::Asn1::Esf::OtherHashAlgAndValue,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (sigPolicyIdentifier, sigPolicyHash))?;
        Ok(__cordl_object)
    }
    pub fn New_DerObjectIdentifier_OtherHashAlgAndValue_IEnumerable3(
        sigPolicyIdentifier: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        sigPolicyHash: *mut crate::Org::BouncyCastle::Asn1::Esf::OtherHashAlgAndValue,
        sigPolicyQualifiers: *mut crate::System::Collections::IEnumerable,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (sigPolicyIdentifier, sigPolicyHash, sigPolicyQualifiers),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_DerObjectIdentifier_OtherHashAlgAndValue_Il2CppArray2(
        sigPolicyIdentifier: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        sigPolicyHash: *mut crate::Org::BouncyCastle::Asn1::Esf::OtherHashAlgAndValue,
        sigPolicyQualifiers: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Asn1::Esf::SigPolicyQualifierInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (sigPolicyIdentifier, sigPolicyHash, sigPolicyQualifiers),
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
    pub fn _ctor_DerObjectIdentifier_OtherHashAlgAndValue1(
        &mut self,
        sigPolicyIdentifier: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        sigPolicyHash: *mut crate::Org::BouncyCastle::Asn1::Esf::OtherHashAlgAndValue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sigPolicyIdentifier, sigPolicyHash))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DerObjectIdentifier_OtherHashAlgAndValue_IEnumerable3(
        &mut self,
        sigPolicyIdentifier: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        sigPolicyHash: *mut crate::Org::BouncyCastle::Asn1::Esf::OtherHashAlgAndValue,
        sigPolicyQualifiers: *mut crate::System::Collections::IEnumerable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sigPolicyIdentifier, sigPolicyHash, sigPolicyQualifiers))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DerObjectIdentifier_OtherHashAlgAndValue_Il2CppArray2(
        &mut self,
        sigPolicyIdentifier: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        sigPolicyHash: *mut crate::Org::BouncyCastle::Asn1::Esf::OtherHashAlgAndValue,
        sigPolicyQualifiers: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::Org::BouncyCastle::Asn1::Esf::SigPolicyQualifierInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (sigPolicyIdentifier, sigPolicyHash, sigPolicyQualifiers))?;
        Ok(__cordl_ret)
    }
    pub fn get_SigPolicyHash(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::Esf::OtherHashAlgAndValue,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Esf::OtherHashAlgAndValue = __cordl_object
            .invoke("get_SigPolicyHash", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_SigPolicyIdentifier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier = __cordl_object
            .invoke("get_SigPolicyIdentifier", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+SignaturePolicyId")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Esf::SignaturePolicyId {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
