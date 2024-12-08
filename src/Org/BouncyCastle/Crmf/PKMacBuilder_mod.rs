#[cfg(feature = "Org+BouncyCastle+Crmf+PKMacBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct PKMacBuilder {
    __cordl_parent: crate::System::Object,
    pub owf: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    pub mac: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    pub provider: *mut crate::Org::BouncyCastle::Crmf::IPKMacPrimitivesProvider,
    pub random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    pub parameters: *mut crate::Org::BouncyCastle::Asn1::Cmp::PbmParameter,
    pub iterationCount: i32,
    pub saltLength: i32,
    pub maxIterations: i32,
}
#[cfg(feature = "Org+BouncyCastle+Crmf+PKMacBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crmf::PKMacBuilder =>
    "Org.BouncyCastle.Crmf"."PKMacBuilder"
);
#[cfg(feature = "Org+BouncyCastle+Crmf+PKMacBuilder")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crmf::PKMacBuilder {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+PKMacBuilder")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crmf::PKMacBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+PKMacBuilder")]
impl crate::Org::BouncyCastle::Crmf::PKMacBuilder {
    pub fn Build(
        &mut self,
        password: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::IMacFactory,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::IMacFactory = __cordl_object
            .invoke("Build", (password))?;
        Ok(__cordl_ret)
    }
    pub fn CheckIterationCountCeiling(
        &mut self,
        iterationCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckIterationCountCeiling", (iterationCount))?;
        Ok(__cordl_ret)
    }
    pub fn GenCalculator(
        &mut self,
        parameters: *mut crate::Org::BouncyCastle::Asn1::Cmp::PbmParameter,
        password: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crypto::IMacFactory,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crypto::IMacFactory = __cordl_object
            .invoke("GenCalculator", (parameters, password))?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_AlgorithmIdentifier_i32_AlgorithmIdentifier_IPKMacPrimitivesProvider4(
        digestAlgorithmIdentifier: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        iterationCount: i32,
        macAlgorithmIdentifier: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        provider: *mut crate::Org::BouncyCastle::Crmf::IPKMacPrimitivesProvider,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    digestAlgorithmIdentifier,
                    iterationCount,
                    macAlgorithmIdentifier,
                    provider,
                ),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_IPKMacPrimitivesProvider1(
        provider: *mut crate::Org::BouncyCastle::Crmf::IPKMacPrimitivesProvider,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (provider))?;
        Ok(__cordl_object)
    }
    pub fn New_IPKMacPrimitivesProvider_AlgorithmIdentifier_AlgorithmIdentifier2(
        provider: *mut crate::Org::BouncyCastle::Crmf::IPKMacPrimitivesProvider,
        digestAlgorithmIdentifier: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        macAlgorithmIdentifier: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (provider, digestAlgorithmIdentifier, macAlgorithmIdentifier),
            )?;
        Ok(__cordl_object)
    }
    pub fn New_IPKMacPrimitivesProvider_i32_3(
        provider: *mut crate::Org::BouncyCastle::Crmf::IPKMacPrimitivesProvider,
        maxIterations: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (provider, maxIterations))?;
        Ok(__cordl_object)
    }
    pub fn SetIterationCount(
        &mut self,
        iterationCount: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crmf::PKMacBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crmf::PKMacBuilder = __cordl_object
            .invoke("SetIterationCount", (iterationCount))?;
        Ok(__cordl_ret)
    }
    pub fn SetParameters(
        &mut self,
        parameters: *mut crate::Org::BouncyCastle::Asn1::Cmp::PbmParameter,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crmf::PKMacBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crmf::PKMacBuilder = __cordl_object
            .invoke("SetParameters", (parameters))?;
        Ok(__cordl_ret)
    }
    pub fn SetSaltLength(
        &mut self,
        saltLength: i32,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crmf::PKMacBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crmf::PKMacBuilder = __cordl_object
            .invoke("SetSaltLength", (saltLength))?;
        Ok(__cordl_ret)
    }
    pub fn SetSecureRandom(
        &mut self,
        random: *mut crate::Org::BouncyCastle::Security::SecureRandom,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Org::BouncyCastle::Crmf::PKMacBuilder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Org::BouncyCastle::Crmf::PKMacBuilder = __cordl_object
            .invoke("SetSecureRandom", (random))?;
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
    pub fn _ctor_AlgorithmIdentifier_i32_AlgorithmIdentifier_IPKMacPrimitivesProvider4(
        &mut self,
        digestAlgorithmIdentifier: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        iterationCount: i32,
        macAlgorithmIdentifier: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        provider: *mut crate::Org::BouncyCastle::Crmf::IPKMacPrimitivesProvider,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    digestAlgorithmIdentifier,
                    iterationCount,
                    macAlgorithmIdentifier,
                    provider,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IPKMacPrimitivesProvider1(
        &mut self,
        provider: *mut crate::Org::BouncyCastle::Crmf::IPKMacPrimitivesProvider,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (provider))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IPKMacPrimitivesProvider_AlgorithmIdentifier_AlgorithmIdentifier2(
        &mut self,
        provider: *mut crate::Org::BouncyCastle::Crmf::IPKMacPrimitivesProvider,
        digestAlgorithmIdentifier: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        macAlgorithmIdentifier: *mut crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (provider, digestAlgorithmIdentifier, macAlgorithmIdentifier),
            )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IPKMacPrimitivesProvider_i32_3(
        &mut self,
        provider: *mut crate::Org::BouncyCastle::Crmf::IPKMacPrimitivesProvider,
        maxIterations: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (provider, maxIterations))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+PKMacBuilder")]
impl quest_hook::libil2cpp::ObjectType for crate::Org::BouncyCastle::Crmf::PKMacBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}