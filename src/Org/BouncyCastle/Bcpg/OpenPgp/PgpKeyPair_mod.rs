#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpKeyPair")]
#[repr(C)]
#[derive(Debug)]
pub struct PgpKeyPair {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _cordl_pub: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
    pub _cordl_priv: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpKeyPair")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair =>
    "Org.BouncyCastle.Bcpg.OpenPgp"."PgpKeyPair"
);
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpKeyPair")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpKeyPair")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpKeyPair")]
impl crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair {
    pub fn New_PgpPublicKey_PgpPrivateKey2(
        _cordl_pub: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        _cordl_priv: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_pub, _cordl_priv))?;
        Ok(__cordl_object)
    }
    pub fn New_PublicKeyAlgorithmTag_AsymmetricCipherKeyPair_DateTime0(
        algorithm: crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
        keyPair: *mut crate::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair,
        _cordl_time: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algorithm, keyPair, _cordl_time))?;
        Ok(__cordl_object)
    }
    pub fn New_PublicKeyAlgorithmTag_AsymmetricKeyParameter_AsymmetricKeyParameter_DateTime1(
        algorithm: crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
        pubKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        privKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        _cordl_time: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (algorithm, pubKey, privKey, _cordl_time))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_PgpPublicKey_PgpPrivateKey2(
        &mut self,
        _cordl_pub: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
        _cordl_priv: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_pub, _cordl_priv))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_PublicKeyAlgorithmTag_AsymmetricCipherKeyPair_DateTime0(
        &mut self,
        algorithm: crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
        keyPair: *mut crate::Org::BouncyCastle::Crypto::AsymmetricCipherKeyPair,
        _cordl_time: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (algorithm, keyPair, _cordl_time))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_PublicKeyAlgorithmTag_AsymmetricKeyParameter_AsymmetricKeyParameter_DateTime1(
        &mut self,
        algorithm: crate::Org::BouncyCastle::Bcpg::PublicKeyAlgorithmTag,
        pubKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        privKey: *mut crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        _cordl_time: crate::System::DateTime,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (algorithm, pubKey, privKey, _cordl_time))?;
        Ok(__cordl_ret)
    }
    pub fn get_KeyId(&mut self) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object.invoke("get_KeyId", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PrivateKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPrivateKey = __cordl_object
            .invoke("get_PrivateKey", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PublicKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpPublicKey = __cordl_object
            .invoke("get_PublicKey", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+OpenPgp+PgpKeyPair")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::OpenPgp::PgpKeyPair {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
