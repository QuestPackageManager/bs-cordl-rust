#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Iso9796d2PssSigner")]
#[repr(C)]
#[derive(Debug)]
pub struct Iso9796d2PssSigner {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub digest: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
    pub cipher: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
    >,
    pub random: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Security::SecureRandom,
    >,
    pub standardSalt: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub hLen: i32,
    pub trailer: i32,
    pub keyBits: i32,
    pub block: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub mBuf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub messageLength: i32,
    pub saltLength: i32,
    pub fullMessage: bool,
    pub recoveredMessage: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<u8>,
    >,
    pub preSig: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub preBlock: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub preMStart: i32,
    pub preTLength: i32,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Iso9796d2PssSigner")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Signers";
    const CLASS_NAME: &'static str = "Iso9796d2PssSigner";
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Iso9796d2PssSigner")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Iso9796d2PssSigner")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Iso9796d2PssSigner")]
impl crate::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner {
    pub const TrailerImplicit: i32 = 188i32;
    pub const TrailerRipeMD128: i32 = 13004i32;
    pub const TrailerRipeMD160: i32 = 12748i32;
    pub const TrailerSha1: i32 = 13260i32;
    pub const TrailerSha256: i32 = 13516i32;
    pub const TrailerSha384: i32 = 14028i32;
    pub const TrailerSha512: i32 = 13772i32;
    pub const TrailerWhirlpool: i32 = 14284i32;
    pub fn BlockUpdate(
        &mut self,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        inOff: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    i32,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("BlockUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner as
                    quest_hook::libil2cpp::Type > ::class(), "BlockUpdate", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (input, inOff, length))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ClearBlock(
        &mut self,
        block: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ClearBlock")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner as
                    quest_hook::libil2cpp::Type > ::class(), "ClearBlock", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (block))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn GenerateSignature(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                0usize,
            >("GenerateSignature")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner as
                    quest_hook::libil2cpp::Type > ::class(), "GenerateSignature", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetRecoveredMessage(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                0usize,
            >("GetRecoveredMessage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner as
                    quest_hook::libil2cpp::Type > ::class(), "GetRecoveredMessage",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn HasFullMessage(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("HasFullMessage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner as
                    quest_hook::libil2cpp::Type > ::class(), "HasFullMessage", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        forSigning: bool,
        parameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    bool,
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::ICipherParameters,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Init")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner as
                    quest_hook::libil2cpp::Type > ::class(), "Init", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (forSigning, parameters))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsSameAs(
        &mut self,
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                ),
                bool,
                2usize,
            >("IsSameAs")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner as
                    quest_hook::libil2cpp::Type > ::class(), "IsSameAs", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (a, b))? };
        Ok(__cordl_ret.into())
    }
    pub fn ItoOSP(
        &mut self,
        i: i32,
        sp: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("ItoOSP")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner as
                    quest_hook::libil2cpp::Type > ::class(), "ItoOSP", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (i, sp))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn LtoOSP(
        &mut self,
        l: i64,
        sp: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i64, quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>),
                quest_hook::libil2cpp::Void,
                2usize,
            >("LtoOSP")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner as
                    quest_hook::libil2cpp::Type > ::class(), "LtoOSP", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (l, sp))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn MaskGeneratorFunction1(
        &mut self,
        Z: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        zOff: i32,
        zLen: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    i32,
                    i32,
                    i32,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                4usize,
            >("MaskGeneratorFunction1")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner as
                    quest_hook::libil2cpp::Type > ::class(), "MaskGeneratorFunction1",
                    4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked(self, (Z, zOff, zLen, length))? };
        Ok(__cordl_ret.into())
    }
    pub fn New_IAsymmetricBlockCipher_IDigest_i32_1(
        cipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
        >,
        digest: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
        saltLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cipher, digest, saltLength))?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool0(
        cipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
        >,
        digest: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
        saltLength: i32,
        isImplicit: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (cipher, digest, saltLength, isImplicit))?;
        Ok(__cordl_object.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Reset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner as
                    quest_hook::libil2cpp::Type > ::class(), "Reset", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
        input: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner as quest_hook::libil2cpp::Type>::class()
            .find_method::<(u8), quest_hook::libil2cpp::Void, 1usize>("Update")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner as
                    quest_hook::libil2cpp::Type > ::class(), "Update", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (input))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateWithRecoveredMessage(
        &mut self,
        signature: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("UpdateWithRecoveredMessage")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner as
                    quest_hook::libil2cpp::Type > ::class(),
                    "UpdateWithRecoveredMessage", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (signature))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn VerifySignature(
        &mut self,
        signature: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>),
                bool,
                1usize,
            >("VerifySignature")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner as
                    quest_hook::libil2cpp::Type > ::class(), "VerifySignature", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (signature))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IAsymmetricBlockCipher_IDigest_i32_1(
        &mut self,
        cipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
        >,
        digest: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
        saltLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
                    >,
                    quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (cipher, digest, saltLength))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool0(
        &mut self,
        cipher: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
        >,
        digest: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
        saltLength: i32,
        isImplicit: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::Org::BouncyCastle::Crypto::IAsymmetricBlockCipher,
                    >,
                    quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Crypto::IDigest>,
                    i32,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (cipher, digest, saltLength, isImplicit))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_AlgorithmName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("get_AlgorithmName")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner as
                    quest_hook::libil2cpp::Type > ::class(), "get_AlgorithmName", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Iso9796d2PssSigner")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Iso9796d2PssSigner")]
impl AsRef<crate::Org::BouncyCastle::Crypto::ISigner>
for crate::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::ISigner {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Iso9796d2PssSigner")]
impl AsMut<crate::Org::BouncyCastle::Crypto::ISigner>
for crate::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::ISigner {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Iso9796d2PssSigner")]
impl AsRef<crate::Org::BouncyCastle::Crypto::ISignerWithRecovery>
for crate::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::ISignerWithRecovery {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Signers+Iso9796d2PssSigner")]
impl AsMut<crate::Org::BouncyCastle::Crypto::ISignerWithRecovery>
for crate::Org::BouncyCastle::Crypto::Signers::Iso9796d2PssSigner {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::ISignerWithRecovery {
        unsafe { std::mem::transmute(self) }
    }
}
