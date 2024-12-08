#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+SM2Engine")]
#[repr(C)]
#[derive(Debug)]
pub struct SM2Engine {
    __cordl_parent: crate::System::Object,
    pub mDigest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
    pub mForEncryption: bool,
    pub mECKey: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECKeyParameters,
    pub mECParams: *mut crate::Org::BouncyCastle::Crypto::Parameters::ECDomainParameters,
    pub mCurveLength: i32,
    pub mRandom: *mut crate::Org::BouncyCastle::Security::SecureRandom,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+SM2Engine")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Engines::SM2Engine =>
    "Org.BouncyCastle.Crypto.Engines"."SM2Engine"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+SM2Engine")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Engines::SM2Engine {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+SM2Engine")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Engines::SM2Engine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+SM2Engine")]
impl crate::Org::BouncyCastle::Crypto::Engines::SM2Engine {
    pub fn Encrypt(
        &mut self,
        input: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        inOff: i32,
        inLen: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("Encrypt", (input, inOff, inLen))?;
        Ok(__cordl_ret)
    }
    pub fn NotEncrypted(
        &mut self,
        encData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        input: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        inOff: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("NotEncrypted", (encData, input, inOff))?;
        Ok(__cordl_ret)
    }
    pub fn Xor(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        kdfOut: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        dOff: i32,
        dRemaining: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Xor", (data, kdfOut, dOff, dRemaining))?;
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
    pub fn Init(
        &mut self,
        forEncryption: bool,
        param: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (forEncryption, param))?;
        Ok(__cordl_ret)
    }
    pub fn Decrypt(
        &mut self,
        input: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        inOff: i32,
        inLen: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("Decrypt", (input, inOff, inLen))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessBlock(
        &mut self,
        input: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        inOff: i32,
        inLen: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<u8>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<u8> = __cordl_object
            .invoke("ProcessBlock", (input, inOff, inLen))?;
        Ok(__cordl_ret)
    }
    pub fn CreateBasePointMultiplier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Math::EC::Multiplier::ECMultiplier,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::EC::Multiplier::ECMultiplier = __cordl_object
            .invoke("CreateBasePointMultiplier", ())?;
        Ok(__cordl_ret)
    }
    pub fn Kdf(
        &mut self,
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
        c1: *mut crate::Org::BouncyCastle::Math::EC::ECPoint,
        encData: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Kdf", (digest, c1, encData))?;
        Ok(__cordl_ret)
    }
    pub fn NextK(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Math::BigInteger> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Math::BigInteger = __cordl_object
            .invoke("NextK", ())?;
        Ok(__cordl_ret)
    }
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
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_IDigest1(
        digest: *mut crate::Org::BouncyCastle::Crypto::IDigest,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (digest))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+SM2Engine")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Engines::SM2Engine {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
