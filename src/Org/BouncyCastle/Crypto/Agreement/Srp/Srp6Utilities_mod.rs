#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+Srp+Srp6Utilities")]
#[repr(C)]
#[derive(Debug)]
pub struct Srp6Utilities {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+Srp+Srp6Utilities")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crypto::Agreement::Srp::Srp6Utilities {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Agreement.Srp";
    const CLASS_NAME: &'static str = "Srp6Utilities";
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+Srp+Srp6Utilities")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Agreement::Srp::Srp6Utilities {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+Srp+Srp6Utilities")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Agreement::Srp::Srp6Utilities {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+Srp+Srp6Utilities")]
impl crate::Org::BouncyCastle::Crypto::Agreement::Srp::Srp6Utilities {
    pub fn CalculateK(
        digest: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
        N: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        g: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CalculateK", (digest, N, g))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateKey(
        digest: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
        N: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        S: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CalculateKey", (digest, N, S))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateM1(
        digest: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
        N: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        A: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        B: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        S: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CalculateM1", (digest, N, A, B, S))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateM2(
        digest: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
        N: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        A: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        M1: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        S: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CalculateM2", (digest, N, A, M1, S))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateU(
        digest: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
        N: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        A: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        B: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CalculateU", (digest, N, A, B))?;
        Ok(__cordl_ret.into())
    }
    pub fn CalculateX(
        digest: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
        N: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        salt: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        identity: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        password: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CalculateX", (digest, N, salt, identity, password))?;
        Ok(__cordl_ret.into())
    }
    pub fn GeneratePrivateValue(
        digest: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
        N: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        g: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GeneratePrivateValue", (digest, N, g, random))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPadded(
        n: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPadded", (n, length))?;
        Ok(__cordl_ret.into())
    }
    pub fn HashPaddedPair(
        digest: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
        N: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        n1: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        n2: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HashPaddedPair", (digest, N, n1, n2))?;
        Ok(__cordl_ret.into())
    }
    pub fn HashPaddedTriplet(
        digest: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
        N: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        n1: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        n2: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        n3: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HashPaddedTriplet", (digest, N, n1, n2, n3))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ValidatePublicValue(
        N: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        val: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ValidatePublicValue", (N, val))?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+Srp+Srp6Utilities")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Agreement::Srp::Srp6Utilities {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
