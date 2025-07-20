#[cfg(feature = "Org+BouncyCastle+Bcpg+SecretSubkeyPacket")]
#[repr(C)]
#[derive(Debug)]
pub struct SecretSubkeyPacket {
    __cordl_parent: crate::Org::BouncyCastle::Bcpg::SecretKeyPacket,
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+SecretSubkeyPacket")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Bcpg::SecretSubkeyPacket {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Bcpg";
    const CLASS_NAME: &'static str = "SecretSubkeyPacket";
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
#[cfg(feature = "Org+BouncyCastle+Bcpg+SecretSubkeyPacket")]
impl std::ops::Deref for crate::Org::BouncyCastle::Bcpg::SecretSubkeyPacket {
    type Target = crate::Org::BouncyCastle::Bcpg::SecretKeyPacket;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+SecretSubkeyPacket")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Bcpg::SecretSubkeyPacket {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+SecretSubkeyPacket")]
impl crate::Org::BouncyCastle::Bcpg::SecretSubkeyPacket {
    pub fn Encode(
        &mut self,
        bcpgOut: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::BcpgOutputStream,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Bcpg::BcpgOutputStream,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("Encode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), "Encode", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (bcpgOut))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New_BcpgInputStream0(
        bcpgIn: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::BcpgInputStream,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bcpgIn))?;
        Ok(__cordl_object.into())
    }
    pub fn New_PublicKeyPacket_SymmetricKeyAlgorithmTag_S2k_Il2CppArray_Il2CppArray1(
        pubKeyPacket: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::PublicKeyPacket,
        >,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        s2k: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::S2k>,
        iv: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        secKeyData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (pubKeyPacket, encAlgorithm, s2k, iv, secKeyData))?;
        Ok(__cordl_object.into())
    }
    pub fn New_PublicKeyPacket_SymmetricKeyAlgorithmTag_i32_S2k_Il2CppArray_Il2CppArray2(
        pubKeyPacket: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::PublicKeyPacket,
        >,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        s2kUsage: i32,
        s2k: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::S2k>,
        iv: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        secKeyData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (pubKeyPacket, encAlgorithm, s2kUsage, s2k, iv, secKeyData),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_BcpgInputStream0(
        &mut self,
        bcpgIn: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::BcpgInputStream,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::Org::BouncyCastle::Bcpg::BcpgInputStream,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (bcpgIn))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_PublicKeyPacket_SymmetricKeyAlgorithmTag_S2k_Il2CppArray_Il2CppArray1(
        &mut self,
        pubKeyPacket: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::PublicKeyPacket,
        >,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        s2k: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::S2k>,
        iv: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        secKeyData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::PublicKeyPacket,
                            >,
                            crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::S2k,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        5usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 5usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (pubKeyPacket, encAlgorithm, s2k, iv, secKeyData),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_PublicKeyPacket_SymmetricKeyAlgorithmTag_i32_S2k_Il2CppArray_Il2CppArray2(
        &mut self,
        pubKeyPacket: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Bcpg::PublicKeyPacket,
        >,
        encAlgorithm: crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
        s2kUsage: i32,
        s2k: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Bcpg::S2k>,
        iv: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        secKeyData: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let method: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                Self::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::PublicKeyPacket,
                            >,
                            crate::Org::BouncyCastle::Bcpg::SymmetricKeyAlgorithmTag,
                            i32,
                            quest_hook::libil2cpp::Gc<
                                crate::Org::BouncyCastle::Bcpg::S2k,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                quest_hook::libil2cpp::Il2CppArray<u8>,
                            >,
                        ),
                        quest_hook::libil2cpp::Void,
                        6usize,
                    >(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            Self::class(), ".ctor", 6usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (pubKeyPacket, encAlgorithm, s2kUsage, s2k, iv, secKeyData),
                )?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Bcpg+SecretSubkeyPacket")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Bcpg::SecretSubkeyPacket {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
