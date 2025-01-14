#[cfg(feature = "Org+BouncyCastle+Crmf+DefaultPKMacPrimitivesProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultPKMacPrimitivesProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Org+BouncyCastle+Crmf+DefaultPKMacPrimitivesProvider")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crmf::DefaultPKMacPrimitivesProvider {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crmf";
    const CLASS_NAME: &'static str = "DefaultPKMacPrimitivesProvider";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+DefaultPKMacPrimitivesProvider")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crmf::DefaultPKMacPrimitivesProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        digestAlg: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
                >),
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
                1usize,
            >("CreateDigest")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CreateDigest", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IDigest,
        > = unsafe { method.invoke_unchecked(self, (digestAlg)) };
        Ok(__cordl_ret.into())
    }
    pub fn CreateMac(
        &mut self,
        macAlg: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IMac>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Org::BouncyCastle::Asn1::X509::AlgorithmIdentifier,
                >),
                quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IMac>,
                1usize,
            >("CreateMac")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CreateMac", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IMac,
        > = unsafe { method.invoke_unchecked(self, (macAlg)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
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
#[cfg(feature = "Org+BouncyCastle+Crmf+DefaultPKMacPrimitivesProvider")]
impl AsRef<crate::Org::BouncyCastle::Crmf::IPKMacPrimitivesProvider>
for crate::Org::BouncyCastle::Crmf::DefaultPKMacPrimitivesProvider {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crmf::IPKMacPrimitivesProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crmf+DefaultPKMacPrimitivesProvider")]
impl AsMut<crate::Org::BouncyCastle::Crmf::IPKMacPrimitivesProvider>
for crate::Org::BouncyCastle::Crmf::DefaultPKMacPrimitivesProvider {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Crmf::IPKMacPrimitivesProvider {
        unsafe { std::mem::transmute(self) }
    }
}
