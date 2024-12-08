#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+RsaOaepWrapperProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct RsaOaepWrapperProvider {
    __cordl_parent: crate::System::Object,
    pub digestOid: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+RsaOaepWrapperProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Operators::RsaOaepWrapperProvider =>
    "Org.BouncyCastle.Crypto.Operators"."RsaOaepWrapperProvider"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+RsaOaepWrapperProvider")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Operators::RsaOaepWrapperProvider {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+RsaOaepWrapperProvider")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Operators::RsaOaepWrapperProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+RsaOaepWrapperProvider")]
impl crate::Org::BouncyCastle::Crypto::Operators::RsaOaepWrapperProvider {
    pub fn New(
        digestOid: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (digestOid))?;
        Ok(__cordl_object)
    }
    pub fn Org_BouncyCastle_Crypto_Operators_WrapperProvider_CreateWrapper(
        &mut self,
        forWrapping: bool,
        parameters: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke(
                "Org.BouncyCastle.Crypto.Operators.WrapperProvider.CreateWrapper",
                (forWrapping, parameters),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        digestOid: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (digestOid))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Operators+RsaOaepWrapperProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Operators::RsaOaepWrapperProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}