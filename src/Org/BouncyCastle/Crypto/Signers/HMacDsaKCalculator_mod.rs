#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+HMacDsaKCalculator")]
#[repr(C)]
#[derive(Debug)]
pub struct HMacDsaKCalculator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub hMac: *mut crate::Org::BouncyCastle::Crypto::Macs::HMac,
    pub K: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub V: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub n: *mut crate::Org::BouncyCastle::Math::BigInteger,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+HMacDsaKCalculator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Signers::HMacDsaKCalculator =>
    "Org.BouncyCastle.Crypto.Signers"."HMacDsaKCalculator"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+HMacDsaKCalculator")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Signers::HMacDsaKCalculator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+HMacDsaKCalculator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Signers::HMacDsaKCalculator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+HMacDsaKCalculator")]
impl crate::Org::BouncyCastle::Crypto::Signers::HMacDsaKCalculator {
    pub fn BitsToInt(
        &mut self,
        t: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("BitsToInt", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init_BigInteger_Il2CppArray1(
        &mut self,
        n: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        d: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (n, d, message))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init_SecureRandom0(
        &mut self,
        n: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        random: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Security::SecureRandom,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (n, random))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        digest: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (digest))?;
        Ok(__cordl_object.into())
    }
    pub fn NextK(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("NextK", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        digest: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (digest))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsDeterministic(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsDeterministic", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+HMacDsaKCalculator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Signers::HMacDsaKCalculator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+HMacDsaKCalculator")]
impl AsRef<crate::Org::BouncyCastle::Crypto::Signers::IDsaKCalculator>
for crate::Org::BouncyCastle::Crypto::Signers::HMacDsaKCalculator {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::Signers::IDsaKCalculator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+HMacDsaKCalculator")]
impl AsMut<crate::Org::BouncyCastle::Crypto::Signers::IDsaKCalculator>
for crate::Org::BouncyCastle::Crypto::Signers::HMacDsaKCalculator {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Crypto::Signers::IDsaKCalculator {
        unsafe { std::mem::transmute(self) }
    }
}
