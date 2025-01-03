#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+ECMqvWithKdfBasicAgreement")]
#[repr(C)]
#[derive(Debug)]
pub struct ECMqvWithKdfBasicAgreement {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Agreement::ECMqvBasicAgreement,
    pub algorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub kdf: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::IDerivationFunction,
    >,
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
        r: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("BigIntToBytes", (r))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateAgreement(
        &mut self,
        pubKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("CalculateAgreement", (pubKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        algorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        kdf: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IDerivationFunction,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algorithm, kdf))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        algorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        kdf: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IDerivationFunction,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (algorithm, kdf))?;
        Ok(__cordl_ret.into())
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
