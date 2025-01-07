#[cfg(feature = "Org+BouncyCastle+Security+AgreementUtilities")]
#[repr(C)]
#[derive(Debug)]
pub struct AgreementUtilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Security+AgreementUtilities")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Security::AgreementUtilities {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Security";
    const CLASS_NAME: &'static str = "AgreementUtilities";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+AgreementUtilities")]
impl std::ops::Deref for crate::Org::BouncyCastle::Security::AgreementUtilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+AgreementUtilities")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Security::AgreementUtilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+AgreementUtilities")]
impl crate::Org::BouncyCastle::Security::AgreementUtilities {
    pub fn GetAlgorithmName(
        oid: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAlgorithmName", (oid))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBasicAgreementWithKdf_DerObjectIdentifier0(
        oid: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
        wrapAlgorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IBasicAgreement>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IBasicAgreement,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBasicAgreementWithKdf", (oid, wrapAlgorithm))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBasicAgreementWithKdf_Il2CppString1(
        agreeAlgorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        wrapAlgorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IBasicAgreement>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IBasicAgreement,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBasicAgreementWithKdf", (agreeAlgorithm, wrapAlgorithm))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBasicAgreement_DerObjectIdentifier0(
        oid: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IBasicAgreement>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IBasicAgreement,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBasicAgreement", (oid))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBasicAgreement_Il2CppString1(
        algorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IBasicAgreement>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IBasicAgreement,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBasicAgreement", (algorithm))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMechanism(
        algorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMechanism", (algorithm))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRawAgreement_DerObjectIdentifier0(
        oid: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IRawAgreement>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IRawAgreement,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRawAgreement", (oid))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRawAgreement_Il2CppString1(
        algorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IRawAgreement>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IRawAgreement,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRawAgreement", (algorithm))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Security+AgreementUtilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Security::AgreementUtilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
