#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+CommitmentTypeQualifier")]
#[repr(C)]
#[derive(Debug)]
pub struct CommitmentTypeQualifier {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    pub commitmentTypeIdentifier: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    pub qualifier: *mut crate::Org::BouncyCastle::Asn1::Asn1Object,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+CommitmentTypeQualifier")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Asn1::Esf::CommitmentTypeQualifier =>
    "Org.BouncyCastle.Asn1.Esf"."CommitmentTypeQualifier"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+CommitmentTypeQualifier")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Esf::CommitmentTypeQualifier {
    type Target = crate::Org::BouncyCastle::Asn1::Asn1Encodable;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+CommitmentTypeQualifier")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Asn1::Esf::CommitmentTypeQualifier {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+CommitmentTypeQualifier")]
impl crate::Org::BouncyCastle::Asn1::Esf::CommitmentTypeQualifier {
    pub fn New_Asn1Sequence2(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
    }
    pub fn New_DerObjectIdentifier0(
        commitmentTypeIdentifier: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (commitmentTypeIdentifier))?;
        Ok(__cordl_object)
    }
    pub fn New_DerObjectIdentifier_Asn1Encodable1(
        commitmentTypeIdentifier: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        qualifier: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (commitmentTypeIdentifier, qualifier))?;
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
    pub fn _ctor_Asn1Sequence2(
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
    pub fn _ctor_DerObjectIdentifier0(
        &mut self,
        commitmentTypeIdentifier: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (commitmentTypeIdentifier))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_DerObjectIdentifier_Asn1Encodable1(
        &mut self,
        commitmentTypeIdentifier: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        qualifier: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (commitmentTypeIdentifier, qualifier))?;
        Ok(__cordl_ret)
    }
    pub fn get_CommitmentTypeIdentifier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier = __cordl_object
            .invoke("get_CommitmentTypeIdentifier", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Qualifier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Asn1::Asn1Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::Asn1Object = __cordl_object
            .invoke("get_Qualifier", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Esf+CommitmentTypeQualifier")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Esf::CommitmentTypeQualifier {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}