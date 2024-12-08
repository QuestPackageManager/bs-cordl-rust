#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+KeyDerivationFunc")]
#[repr(C)]
#[derive(Debug)]
pub struct KeyDerivationFunc {
    __cordl_parent: crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+KeyDerivationFunc")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Asn1::Pkcs::KeyDerivationFunc
    => "Org.BouncyCastle.Asn1.Pkcs"."KeyDerivationFunc"
);
#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+KeyDerivationFunc")]
impl std::ops::Deref for crate::Org::BouncyCastle::Asn1::Pkcs::KeyDerivationFunc {
    type Target = crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+KeyDerivationFunc")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Asn1::Pkcs::KeyDerivationFunc {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+KeyDerivationFunc")]
impl crate::Org::BouncyCastle::Asn1::Pkcs::KeyDerivationFunc {
    pub fn New_Asn1Sequence0(
        seq: *mut crate::Org::BouncyCastle::Asn1::Asn1Sequence,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (seq))?;
        Ok(__cordl_object)
    }
    pub fn New_DerObjectIdentifier_Asn1Encodable1(
        id: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        parameters: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (id, parameters))?;
        Ok(__cordl_object)
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
    pub fn _ctor_DerObjectIdentifier_Asn1Encodable1(
        &mut self,
        id: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        parameters: *mut crate::Org::BouncyCastle::Asn1::Asn1Encodable,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (id, parameters))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Asn1+Pkcs+KeyDerivationFunc")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Asn1::Pkcs::KeyDerivationFunc {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
