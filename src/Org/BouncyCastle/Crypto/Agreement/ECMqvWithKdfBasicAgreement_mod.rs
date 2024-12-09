#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+ECMqvWithKdfBasicAgreement")]
#[repr(C)]
#[derive(Debug)]
pub struct ECMqvWithKdfBasicAgreement {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Agreement::ECMqvBasicAgreement,
    pub algorithm: *mut quest_hook::libil2cpp::Il2CppString,
    pub kdf: *mut crate::Org::BouncyCastle::Crypto::IDerivationFunction,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+ECMqvWithKdfBasicAgreement")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Agreement::ECMqvWithKdfBasicAgreement =>
    "Org.BouncyCastle.Crypto.Agreement"."ECMqvWithKdfBasicAgreement"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+ECMqvWithKdfBasicAgreement")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Agreement::ECMqvWithKdfBasicAgreement {
    type Target = crate::Org::BouncyCastle::Crypto::Agreement::ECMqvBasicAgreement;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+ECMqvWithKdfBasicAgreement")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Agreement::ECMqvWithKdfBasicAgreement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+ECMqvWithKdfBasicAgreement")]
impl crate::Org::BouncyCastle::Crypto::Agreement::ECMqvWithKdfBasicAgreement {
    pub fn BigIntToBytes(
        &mut self,
        r: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("BigIntToBytes", (r))?;
        Ok(__cordl_ret)
    }
    pub fn CalculateAgreement(
        &mut self,
        pubKey: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("CalculateAgreement", (pubKey))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        algorithm: *mut quest_hook::libil2cpp::Il2CppString,
        kdf: *mut crate::Org::BouncyCastle::Crypto::IDerivationFunction,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algorithm, kdf))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        algorithm: *mut quest_hook::libil2cpp::Il2CppString,
        kdf: *mut crate::Org::BouncyCastle::Crypto::IDerivationFunction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (algorithm, kdf))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+ECMqvWithKdfBasicAgreement")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Agreement::ECMqvWithKdfBasicAgreement {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
