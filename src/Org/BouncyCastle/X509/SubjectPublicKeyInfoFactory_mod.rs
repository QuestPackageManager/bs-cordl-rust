#[cfg(feature = "Org+BouncyCastle+X509+SubjectPublicKeyInfoFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct SubjectPublicKeyInfoFactory {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+X509+SubjectPublicKeyInfoFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::X509::SubjectPublicKeyInfoFactory => "Org.BouncyCastle.X509"
    ."SubjectPublicKeyInfoFactory"
);
#[cfg(feature = "Org+BouncyCastle+X509+SubjectPublicKeyInfoFactory")]
impl std::ops::Deref for crate::Org::BouncyCastle::X509::SubjectPublicKeyInfoFactory {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+SubjectPublicKeyInfoFactory")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::X509::SubjectPublicKeyInfoFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+X509+SubjectPublicKeyInfoFactory")]
impl crate::Org::BouncyCastle::X509::SubjectPublicKeyInfoFactory {
    pub fn CreateSubjectPublicKeyInfo(
        publicKey: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::AsymmetricKeyParameter,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::SubjectPublicKeyInfo,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::SubjectPublicKeyInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateSubjectPublicKeyInfo", (publicKey))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtractBytes_BigInteger0(
        encKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        bI: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExtractBytes", (encKey, offset, bI))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExtractBytes_i32_BigInteger1(
        encKey: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        _cordl_size: i32,
        offSet: i32,
        bI: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Math::BigInteger>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExtractBytes", (encKey, _cordl_size, offSet, bI))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
#[cfg(feature = "Org+BouncyCastle+X509+SubjectPublicKeyInfoFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::X509::SubjectPublicKeyInfoFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
