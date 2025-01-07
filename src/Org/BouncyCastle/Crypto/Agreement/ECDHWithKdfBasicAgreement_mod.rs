#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+ECDHWithKdfBasicAgreement")]
#[repr(C)]
#[derive(Debug)]
pub struct ECDHWithKdfBasicAgreement {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Agreement::ECDHBasicAgreement,
    pub algorithm: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub kdf: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::IDerivationFunction,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+ECDHWithKdfBasicAgreement")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crypto::Agreement::ECDHWithKdfBasicAgreement {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Agreement";
    const CLASS_NAME: &'static str = "ECDHWithKdfBasicAgreement";
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
