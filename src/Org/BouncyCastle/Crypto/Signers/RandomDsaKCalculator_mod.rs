#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+RandomDsaKCalculator")]
#[repr(C)]
#[derive(Debug)]
pub struct RandomDsaKCalculator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub q: *mut crate::Org::BouncyCastle::Math::BigInteger,
    pub random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+RandomDsaKCalculator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Signers::RandomDsaKCalculator =>
    "Org.BouncyCastle.Crypto.Signers"."RandomDsaKCalculator"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+RandomDsaKCalculator")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Signers::RandomDsaKCalculator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+RandomDsaKCalculator")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Signers::RandomDsaKCalculator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+RandomDsaKCalculator")]
impl crate::Org::BouncyCastle::Crypto::Signers::RandomDsaKCalculator {
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
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
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+RandomDsaKCalculator")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Signers::RandomDsaKCalculator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+RandomDsaKCalculator")]
impl AsRef<crate::Org::BouncyCastle::Crypto::Signers::IDsaKCalculator>
for crate::Org::BouncyCastle::Crypto::Signers::RandomDsaKCalculator {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::Signers::IDsaKCalculator {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+RandomDsaKCalculator")]
impl AsMut<crate::Org::BouncyCastle::Crypto::Signers::IDsaKCalculator>
for crate::Org::BouncyCastle::Crypto::Signers::RandomDsaKCalculator {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Crypto::Signers::IDsaKCalculator {
        unsafe { std::mem::transmute(self) }
    }
}
