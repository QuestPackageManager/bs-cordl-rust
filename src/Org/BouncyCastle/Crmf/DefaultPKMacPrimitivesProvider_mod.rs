#[cfg(feature = "Org+BouncyCastle+Crmf+DefaultPKMacPrimitivesProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultPKMacPrimitivesProvider {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Org+BouncyCastle+Crmf+DefaultPKMacPrimitivesProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crmf::DefaultPKMacPrimitivesProvider => "Org.BouncyCastle.Crmf"
    ."DefaultPKMacPrimitivesProvider"
);
#[cfg(feature = "Org+BouncyCastle+Crmf+DefaultPKMacPrimitivesProvider")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crmf::DefaultPKMacPrimitivesProvider {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+DefaultPKMacPrimitivesProvider")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crmf::DefaultPKMacPrimitivesProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+DefaultPKMacPrimitivesProvider")]
impl crate::Org::BouncyCastle::Crmf::DefaultPKMacPrimitivesProvider {
    pub fn CreateDigest(
        &mut self,
        digestAlg: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Crypto::IDigest> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::IDigest = __cordl_object
            .invoke("CreateDigest", (digestAlg))?;
        Ok(__cordl_ret)
    }
    pub fn CreateMac(
        &mut self,
        macAlg: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Org::BouncyCastle::Crypto::IMac> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::IMac = __cordl_object
            .invoke("CreateMac", (macAlg))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
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
#[cfg(feature = "Org+BouncyCastle+Crmf+DefaultPKMacPrimitivesProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crmf::DefaultPKMacPrimitivesProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
