#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs12StoreBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct Pkcs12StoreBuilder {
    __cordl_parent: crate::System::Object,
    pub keyAlgorithm: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    pub certAlgorithm: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    pub keyPrfAlgorithm: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    pub certPrfAlgorithm: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    pub useDerEncoding: bool,
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs12StoreBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Pkcs::Pkcs12StoreBuilder =>
    "Org.BouncyCastle.Pkcs"."Pkcs12StoreBuilder"
);
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs12StoreBuilder")]
impl std::ops::Deref for crate::Org::BouncyCastle::Pkcs::Pkcs12StoreBuilder {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs12StoreBuilder")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Pkcs::Pkcs12StoreBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs12StoreBuilder")]
impl crate::Org::BouncyCastle::Pkcs::Pkcs12StoreBuilder {
    pub fn Build(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Pkcs::Pkcs12Store,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Pkcs::Pkcs12Store = __cordl_object
            .invoke("Build", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn SetCertAlgorithm(
        &mut self,
        certAlgorithm: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Pkcs::Pkcs12StoreBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Pkcs::Pkcs12StoreBuilder = __cordl_object
            .invoke("SetCertAlgorithm", (certAlgorithm))?;
        Ok(__cordl_ret)
    }
    pub fn SetKeyAlgorithm_DerObjectIdentifier0(
        &mut self,
        keyAlgorithm: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Pkcs::Pkcs12StoreBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Pkcs::Pkcs12StoreBuilder = __cordl_object
            .invoke("SetKeyAlgorithm", (keyAlgorithm))?;
        Ok(__cordl_ret)
    }
    pub fn SetKeyAlgorithm_DerObjectIdentifier1(
        &mut self,
        keyAlgorithm: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
        keyPrfAlgorithm: *mut crate::Org::BouncyCastle::Asn1::DerObjectIdentifier,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Pkcs::Pkcs12StoreBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Pkcs::Pkcs12StoreBuilder = __cordl_object
            .invoke("SetKeyAlgorithm", (keyAlgorithm, keyPrfAlgorithm))?;
        Ok(__cordl_ret)
    }
    pub fn SetUseDerEncoding(
        &mut self,
        useDerEncoding: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Pkcs::Pkcs12StoreBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Pkcs::Pkcs12StoreBuilder = __cordl_object
            .invoke("SetUseDerEncoding", (useDerEncoding))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Pkcs+Pkcs12StoreBuilder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Pkcs::Pkcs12StoreBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}