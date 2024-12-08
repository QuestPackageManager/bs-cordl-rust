#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ECGost3410Parameters")]
#[repr(C)]
#[derive(Debug)]
pub struct ECGost3410Parameters {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Parameters::ECNamedDomainParameters,
    pub _publicKeyParamSet: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    pub _digestParamSet: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    pub _encryptionParamSet: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ECGost3410Parameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Parameters::ECGost3410Parameters =>
    "Org.BouncyCastle.Crypto.Parameters"."ECGost3410Parameters"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ECGost3410Parameters")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Parameters::ECGost3410Parameters {
    type Target = crate::Org::BouncyCastle::Crypto::Parameters::ECNamedDomainParameters;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ECGost3410Parameters")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Parameters::ECGost3410Parameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ECGost3410Parameters")]
impl crate::Org::BouncyCastle::Crypto::Parameters::ECGost3410Parameters {
    pub fn New_ECDomainParameters1(
        dp: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECDomainParameters,
        publicKeyParamSet: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        digestParamSet: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        encryptionParamSet: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (dp, publicKeyParamSet, digestParamSet, encryptionParamSet),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_ECNamedDomainParameters0(
        dp: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECNamedDomainParameters,
        publicKeyParamSet: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        digestParamSet: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        encryptionParamSet: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (dp, publicKeyParamSet, digestParamSet, encryptionParamSet),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor_ECDomainParameters1(
        &mut self,
        dp: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECDomainParameters,
        publicKeyParamSet: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        digestParamSet: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        encryptionParamSet: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (dp, publicKeyParamSet, digestParamSet, encryptionParamSet),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_ECNamedDomainParameters0(
        &mut self,
        dp: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECNamedDomainParameters,
        publicKeyParamSet: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        digestParamSet: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        encryptionParamSet: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (dp, publicKeyParamSet, digestParamSet, encryptionParamSet),
            )?;
        Ok(__cordl_ret)
    }
    pub fn get_DigestParamSet(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier = __cordl_object
            .invoke("get_DigestParamSet", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_EncryptionParamSet(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier = __cordl_object
            .invoke("get_EncryptionParamSet", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PublicKeyParamSet(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier = __cordl_object
            .invoke("get_PublicKeyParamSet", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Parameters+ECGost3410Parameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Parameters::ECGost3410Parameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}