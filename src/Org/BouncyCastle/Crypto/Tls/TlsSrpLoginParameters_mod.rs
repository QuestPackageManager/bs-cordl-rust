#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSrpLoginParameters")]
#[repr(C)]
#[derive(Debug)]
pub struct TlsSrpLoginParameters {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub mGroup: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Parameters::Srp6GroupParameters,
    >,
    pub mVerifier: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    pub mSalt: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSrpLoginParameters")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Tls::TlsSrpLoginParameters =>
    "Org.BouncyCastle.Crypto.Tls"."TlsSrpLoginParameters"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSrpLoginParameters")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Tls::TlsSrpLoginParameters {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSrpLoginParameters")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Tls::TlsSrpLoginParameters {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSrpLoginParameters")]
impl crate::Org::BouncyCastle::Crypto::Tls::TlsSrpLoginParameters {
    pub fn New(
        group: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::Srp6GroupParameters,
        >,
        verifier: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        salt: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (group, verifier, salt))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        group: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::Srp6GroupParameters,
        >,
        verifier: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
        salt: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (group, verifier, salt))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Group(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::Srp6GroupParameters,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::Srp6GroupParameters,
        > = __cordl_object.invoke("get_Group", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Salt(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("get_Salt", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Verifier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Math::BigInteger,
        > = __cordl_object.invoke("get_Verifier", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Tls+TlsSrpLoginParameters")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Tls::TlsSrpLoginParameters {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
