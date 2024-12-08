#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+SM2KeyExchange")]
#[repr(C)]
#[derive(Debug)]
pub struct SM2KeyExchange {
    __cordl_parent: crate::System::Object,
    pub mDigest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
    pub mUserID: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub mStaticKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
    pub mStaticPubPoint: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
    pub mEphemeralPubPoint: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
    pub mECParams: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECDomainParameters,
    pub mW: i32,
    pub mEphemeralKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECPrivateKeyParameters,
    pub mInitiator: bool,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+SM2KeyExchange")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Agreement::SM2KeyExchange =>
    "Org.BouncyCastle.Crypto.Agreement"."SM2KeyExchange"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+SM2KeyExchange")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Agreement::SM2KeyExchange {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+SM2KeyExchange")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Agreement::SM2KeyExchange {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+SM2KeyExchange")]
impl crate::Org::BouncyCastle::Crypto::Agreement::SM2KeyExchange {
    pub fn AddFieldElement(
        &mut self,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        v: *mut crate::Org::BouncyCastle::Math::EC::ECFieldElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddFieldElement", (digest, v))?;
        Ok(__cordl_ret)
    }
    pub fn AddUserID(
        &mut self,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        userID: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddUserID", (digest, userID))?;
        Ok(__cordl_ret)
    }
    pub fn CalculateInnerHash(
        &mut self,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        u: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
        za: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        zb: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        p1: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
        p2: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("CalculateInnerHash", (digest, u, za, zb, p1, p2))?;
        Ok(__cordl_ret)
    }
    pub fn CalculateKey(
        &mut self,
        kLen: i32,
        pubParam: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("CalculateKey", (kLen, pubParam))?;
        Ok(__cordl_ret)
    }
    pub fn CalculateKeyWithConfirmation(
        &mut self,
        kLen: i32,
        confirmationTag: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        pubParam: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object
            .invoke("CalculateKeyWithConfirmation", (kLen, confirmationTag, pubParam))?;
        Ok(__cordl_ret)
    }
    pub fn CalculateU(
        &mut self,
        otherPub: *mut crate::Org::BouncyCastle::Crypto::Parameters::SM2KeyExchangePublicParameters,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::ECPoint = __cordl_object
            .invoke("CalculateU", (otherPub))?;
        Ok(__cordl_ret)
    }
    pub fn GetZ(
        &mut self,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        userID: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        pubPoint: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("GetZ", (digest, userID, pubPoint))?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        privParam: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (privParam))?;
        Ok(__cordl_ret)
    }
    pub fn Kdf(
        &mut self,
        u: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
        za: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        zb: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        klen: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("Kdf", (u, za, zb, klen))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_IDigest1(
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (digest))?;
        Ok(__cordl_object)
    }
    pub fn Reduce(
        &mut self,
        x: *mut crate::Org::BouncyCastle::Math::BigInteger,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("Reduce", (x))?;
        Ok(__cordl_ret)
    }
    pub fn S1(
        &mut self,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        u: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
        inner: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("S1", (digest, u, inner))?;
        Ok(__cordl_ret)
    }
    pub fn S2(
        &mut self,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        u: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
        inner: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("S2", (digest, u, inner))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IDigest1(
        &mut self,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (digest))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Agreement+SM2KeyExchange")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Agreement::SM2KeyExchange {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
