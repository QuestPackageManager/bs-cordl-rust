#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Gost3410Signer")]
#[repr(C)]
#[derive(Debug)]
pub struct Gost3410Signer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub key: *mut crate::Org::BouncyCastle::Crypto::Parameters::Gost3410KeyParameters,
    pub random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Gost3410Signer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Signers::Gost3410Signer =>
    "Org.BouncyCastle.Crypto.Signers"."Gost3410Signer"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Gost3410Signer")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Signers::Gost3410Signer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Gost3410Signer")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Signers::Gost3410Signer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Gost3410Signer")]
impl crate::Org::BouncyCastle::Crypto::Signers::Gost3410Signer {
    pub fn GenerateSignature(
        &mut self,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Math::BigInteger,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::Org::BouncyCastle::Math::BigInteger,
            >,
        > = __cordl_object.invoke("GenerateSignature", (message))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        forSigning: bool,
        parameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (forSigning, parameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn VerifySignature(
        &mut self,
        message: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        r: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        s: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("VerifySignature", (message, r, s))?;
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
    pub fn get_AlgorithmName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_AlgorithmName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Order(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("get_Order", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Gost3410Signer")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Signers::Gost3410Signer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Gost3410Signer")]
impl AsRef<crate::Org::BouncyCastle::Crypto::IDsa>
for crate::Org::BouncyCastle::Crypto::Signers::Gost3410Signer {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::IDsa {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Gost3410Signer")]
impl AsMut<crate::Org::BouncyCastle::Crypto::IDsa>
for crate::Org::BouncyCastle::Crypto::Signers::Gost3410Signer {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::IDsa {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Gost3410Signer")]
impl AsRef<crate::Org::BouncyCastle::Crypto::IDsaExt>
for crate::Org::BouncyCastle::Crypto::Signers::Gost3410Signer {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::IDsaExt {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Gost3410Signer")]
impl AsMut<crate::Org::BouncyCastle::Crypto::IDsaExt>
for crate::Org::BouncyCastle::Crypto::Signers::Gost3410Signer {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::IDsaExt {
        unsafe { std::mem::transmute(self) }
    }
}
