#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+ECDHWithKdfBasicAgreement")]
#[repr(C)]
#[derive(Debug)]
pub struct ECDHWithKdfBasicAgreement {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Agreement::ECDHBasicAgreement,
    pub algorithm: *mut crate::System::String,
    pub kdf: *mut crate::Org::BouncyCastle::Crypto::IDerivationFunction,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+ECDHWithKdfBasicAgreement")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Agreement::ECDHWithKdfBasicAgreement =>
    "Org.BouncyCastle.Crypto.Agreement"."ECDHWithKdfBasicAgreement"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+ECDHWithKdfBasicAgreement")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Agreement::ECDHWithKdfBasicAgreement {
    type Target = crate::Org::BouncyCastle::Crypto::Agreement::ECDHBasicAgreement;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+ECDHWithKdfBasicAgreement")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Agreement::ECDHWithKdfBasicAgreement {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+ECDHWithKdfBasicAgreement")]
impl crate::Org::BouncyCastle::Crypto::Agreement::ECDHWithKdfBasicAgreement {
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
    pub fn _ctor(
        &mut self,
        algorithm: *mut crate::System::String,
        kdf: *mut crate::Org::BouncyCastle::Crypto::IDerivationFunction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (algorithm, kdf))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        algorithm: *mut crate::System::String,
        kdf: *mut crate::Org::BouncyCastle::Crypto::IDerivationFunction,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algorithm, kdf))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+ECDHWithKdfBasicAgreement")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Agreement::ECDHWithKdfBasicAgreement {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
