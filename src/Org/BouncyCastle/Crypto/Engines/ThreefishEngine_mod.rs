#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+ThreefishEngine")]
#[repr(C)]
#[derive(Debug)]
pub struct ThreefishEngine {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub blocksizeBytes: i32,
    pub blocksizeWords: i32,
    pub currentBlock: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    pub t: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    pub kw: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    pub cipher: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Crypto::Engines::ThreefishEngine_ThreefishCipher,
    >,
    pub forEncryption: bool,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+ThreefishEngine")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crypto::Engines::ThreefishEngine {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Engines";
    const CLASS_NAME: &'static str = "ThreefishEngine";
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+ThreefishEngine")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Engines::ThreefishEngine {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+ThreefishEngine")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Engines::ThreefishEngine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+ThreefishEngine")]
impl crate::Org::BouncyCastle::Crypto::Engines::ThreefishEngine {
    pub const BLOCKSIZE_1024: i32 = 1024i32;
    pub const BLOCKSIZE_256: i32 = 256i32;
    pub const BLOCKSIZE_512: i32 = 512i32;
    pub const C_240: u64 = 2004413935125273122u64;
    pub const MAX_ROUNDS: i32 = 80i32;
    pub const ROUNDS_1024: i32 = 80i32;
    pub const ROUNDS_256: i32 = 72i32;
    pub const ROUNDS_512: i32 = 72i32;
    pub const TWEAK_SIZE_BYTES: i32 = 16i32;
    pub const TWEAK_SIZE_WORDS: i32 = 2i32;
    #[cfg(
        feature = "Org+BouncyCastle+Crypto+Engines+ThreefishEngine+Threefish1024Cipher"
    )]
    pub type Threefish1024Cipher = crate::Org::BouncyCastle::Crypto::Engines::ThreefishEngine_Threefish1024Cipher;
    #[cfg(
        feature = "Org+BouncyCastle+Crypto+Engines+ThreefishEngine+Threefish256Cipher"
    )]
    pub type Threefish256Cipher = crate::Org::BouncyCastle::Crypto::Engines::ThreefishEngine_Threefish256Cipher;
    #[cfg(
        feature = "Org+BouncyCastle+Crypto+Engines+ThreefishEngine+Threefish512Cipher"
    )]
    pub type Threefish512Cipher = crate::Org::BouncyCastle::Crypto::Engines::ThreefishEngine_Threefish512Cipher;
    #[cfg(feature = "Org+BouncyCastle+Crypto+Engines+ThreefishEngine+ThreefishCipher")]
    pub type ThreefishCipher = crate::Org::BouncyCastle::Crypto::Engines::ThreefishEngine_ThreefishCipher;
    pub fn BytesToWord(
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BytesToWord", (bytes, off))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBlockSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetBlockSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Init_ICipherParameters0(
        &mut self,
        forEncryption: bool,
        parameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::ICipherParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (forEncryption, parameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init_Il2CppArray_Il2CppArray1(
        &mut self,
        forEncryption: bool,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        tweak: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (forEncryption, key, tweak))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        blocksizeBits: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (blocksizeBits))?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessBlock_Il2CppArray1(
        &mut self,
        inWords: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        outWords: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ProcessBlock", (inWords, outWords))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessBlock_i32_Il2CppArray_i32_0(
        &mut self,
        inBytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        inOff: i32,
        outBytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        outOff: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ProcessBlock", (inBytes, inOff, outBytes, outOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RotlXor(
        x: u64,
        n: i32,
        _cordl_xor: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RotlXor", (x, n, _cordl_xor))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetKey(
        &mut self,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetKey", (key))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTweak(
        &mut self,
        tweak: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetTweak", (tweak))?;
        Ok(__cordl_ret.into())
    }
    pub fn WordToBytes(
        word: u64,
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("WordToBytes", (word, bytes, off))?;
        Ok(__cordl_ret.into())
    }
    pub fn XorRotr(
        x: u64,
        n: i32,
        _cordl_xor: u64,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("XorRotr", (x, n, _cordl_xor))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        blocksizeBits: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (blocksizeBits))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AlgorithmName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_AlgorithmName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsPartialBlockOkay(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsPartialBlockOkay", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+ThreefishEngine")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Engines::ThreefishEngine {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+ThreefishEngine")]
impl AsRef<crate::Org::BouncyCastle::Crypto::IBlockCipher>
for crate::Org::BouncyCastle::Crypto::Engines::ThreefishEngine {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Crypto::IBlockCipher {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+ThreefishEngine")]
impl AsMut<crate::Org::BouncyCastle::Crypto::IBlockCipher>
for crate::Org::BouncyCastle::Crypto::Engines::ThreefishEngine {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Crypto::IBlockCipher {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+ThreefishEngine+Threefish1024Cipher")]
#[repr(C)]
#[derive(Debug)]
pub struct ThreefishEngine_Threefish1024Cipher {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Engines::ThreefishEngine_ThreefishCipher,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+ThreefishEngine+Threefish1024Cipher")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crypto::Engines::ThreefishEngine_Threefish1024Cipher {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Engines";
    const CLASS_NAME: &'static str = "Threefish1024Cipher";
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+ThreefishEngine+Threefish1024Cipher")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Engines::ThreefishEngine_Threefish1024Cipher {
    type Target = crate::Org::BouncyCastle::Crypto::Engines::ThreefishEngine_ThreefishCipher;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+ThreefishEngine+Threefish1024Cipher")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Engines::ThreefishEngine_Threefish1024Cipher {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+ThreefishEngine+Threefish1024Cipher")]
impl crate::Org::BouncyCastle::Crypto::Engines::ThreefishEngine_Threefish1024Cipher {
    pub const ROTATION_0_0: i32 = 24i32;
    pub const ROTATION_0_1: i32 = 13i32;
    pub const ROTATION_0_2: i32 = 8i32;
    pub const ROTATION_0_3: i32 = 47i32;
    pub const ROTATION_0_4: i32 = 8i32;
    pub const ROTATION_0_5: i32 = 17i32;
    pub const ROTATION_0_6: i32 = 22i32;
    pub const ROTATION_0_7: i32 = 37i32;
    pub const ROTATION_1_0: i32 = 38i32;
    pub const ROTATION_1_1: i32 = 19i32;
    pub const ROTATION_1_2: i32 = 10i32;
    pub const ROTATION_1_3: i32 = 55i32;
    pub const ROTATION_1_4: i32 = 49i32;
    pub const ROTATION_1_5: i32 = 18i32;
    pub const ROTATION_1_6: i32 = 23i32;
    pub const ROTATION_1_7: i32 = 52i32;
    pub const ROTATION_2_0: i32 = 33i32;
    pub const ROTATION_2_1: i32 = 4i32;
    pub const ROTATION_2_2: i32 = 51i32;
    pub const ROTATION_2_3: i32 = 13i32;
    pub const ROTATION_2_4: i32 = 34i32;
    pub const ROTATION_2_5: i32 = 41i32;
    pub const ROTATION_2_6: i32 = 59i32;
    pub const ROTATION_2_7: i32 = 17i32;
    pub const ROTATION_3_0: i32 = 5i32;
    pub const ROTATION_3_1: i32 = 20i32;
    pub const ROTATION_3_2: i32 = 48i32;
    pub const ROTATION_3_3: i32 = 41i32;
    pub const ROTATION_3_4: i32 = 47i32;
    pub const ROTATION_3_5: i32 = 28i32;
    pub const ROTATION_3_6: i32 = 16i32;
    pub const ROTATION_3_7: i32 = 25i32;
    pub const ROTATION_4_0: i32 = 41i32;
    pub const ROTATION_4_1: i32 = 9i32;
    pub const ROTATION_4_2: i32 = 37i32;
    pub const ROTATION_4_3: i32 = 31i32;
    pub const ROTATION_4_4: i32 = 12i32;
    pub const ROTATION_4_5: i32 = 47i32;
    pub const ROTATION_4_6: i32 = 44i32;
    pub const ROTATION_4_7: i32 = 30i32;
    pub const ROTATION_5_0: i32 = 16i32;
    pub const ROTATION_5_1: i32 = 34i32;
    pub const ROTATION_5_2: i32 = 56i32;
    pub const ROTATION_5_3: i32 = 51i32;
    pub const ROTATION_5_4: i32 = 4i32;
    pub const ROTATION_5_5: i32 = 53i32;
    pub const ROTATION_5_6: i32 = 42i32;
    pub const ROTATION_5_7: i32 = 41i32;
    pub const ROTATION_6_0: i32 = 31i32;
    pub const ROTATION_6_1: i32 = 44i32;
    pub const ROTATION_6_2: i32 = 47i32;
    pub const ROTATION_6_3: i32 = 46i32;
    pub const ROTATION_6_4: i32 = 19i32;
    pub const ROTATION_6_5: i32 = 42i32;
    pub const ROTATION_6_6: i32 = 44i32;
    pub const ROTATION_6_7: i32 = 25i32;
    pub const ROTATION_7_0: i32 = 9i32;
    pub const ROTATION_7_1: i32 = 48i32;
    pub const ROTATION_7_2: i32 = 35i32;
    pub const ROTATION_7_3: i32 = 52i32;
    pub const ROTATION_7_4: i32 = 23i32;
    pub const ROTATION_7_5: i32 = 31i32;
    pub const ROTATION_7_6: i32 = 37i32;
    pub const ROTATION_7_7: i32 = 20i32;
    pub fn DecryptBlock(
        &mut self,
        block: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DecryptBlock", (block, state))?;
        Ok(__cordl_ret.into())
    }
    pub fn EncryptBlock(
        &mut self,
        block: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        outWords: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EncryptBlock", (block, outWords))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        kw: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        t: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (kw, t))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        kw: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        t: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (kw, t))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+ThreefishEngine+Threefish1024Cipher")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Engines::ThreefishEngine_Threefish1024Cipher {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+ThreefishEngine+Threefish256Cipher")]
#[repr(C)]
#[derive(Debug)]
pub struct ThreefishEngine_Threefish256Cipher {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Engines::ThreefishEngine_ThreefishCipher,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+ThreefishEngine+Threefish256Cipher")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crypto::Engines::ThreefishEngine_Threefish256Cipher {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Engines";
    const CLASS_NAME: &'static str = "Threefish256Cipher";
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+ThreefishEngine+Threefish256Cipher")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Engines::ThreefishEngine_Threefish256Cipher {
    type Target = crate::Org::BouncyCastle::Crypto::Engines::ThreefishEngine_ThreefishCipher;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+ThreefishEngine+Threefish256Cipher")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Engines::ThreefishEngine_Threefish256Cipher {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+ThreefishEngine+Threefish256Cipher")]
impl crate::Org::BouncyCastle::Crypto::Engines::ThreefishEngine_Threefish256Cipher {
    pub const ROTATION_0_0: i32 = 14i32;
    pub const ROTATION_0_1: i32 = 16i32;
    pub const ROTATION_1_0: i32 = 52i32;
    pub const ROTATION_1_1: i32 = 57i32;
    pub const ROTATION_2_0: i32 = 23i32;
    pub const ROTATION_2_1: i32 = 40i32;
    pub const ROTATION_3_0: i32 = 5i32;
    pub const ROTATION_3_1: i32 = 37i32;
    pub const ROTATION_4_0: i32 = 25i32;
    pub const ROTATION_4_1: i32 = 33i32;
    pub const ROTATION_5_0: i32 = 46i32;
    pub const ROTATION_5_1: i32 = 12i32;
    pub const ROTATION_6_0: i32 = 58i32;
    pub const ROTATION_6_1: i32 = 22i32;
    pub const ROTATION_7_0: i32 = 32i32;
    pub const ROTATION_7_1: i32 = 32i32;
    pub fn DecryptBlock(
        &mut self,
        block: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DecryptBlock", (block, state))?;
        Ok(__cordl_ret.into())
    }
    pub fn EncryptBlock(
        &mut self,
        block: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        outWords: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EncryptBlock", (block, outWords))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        kw: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        t: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (kw, t))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        kw: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        t: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (kw, t))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+ThreefishEngine+Threefish256Cipher")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Engines::ThreefishEngine_Threefish256Cipher {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+ThreefishEngine+Threefish512Cipher")]
#[repr(C)]
#[derive(Debug)]
pub struct ThreefishEngine_Threefish512Cipher {
    __cordl_parent: crate::Org::BouncyCastle::Crypto::Engines::ThreefishEngine_ThreefishCipher,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+ThreefishEngine+Threefish512Cipher")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crypto::Engines::ThreefishEngine_Threefish512Cipher {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Engines";
    const CLASS_NAME: &'static str = "Threefish512Cipher";
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+ThreefishEngine+Threefish512Cipher")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Engines::ThreefishEngine_Threefish512Cipher {
    type Target = crate::Org::BouncyCastle::Crypto::Engines::ThreefishEngine_ThreefishCipher;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+ThreefishEngine+Threefish512Cipher")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Engines::ThreefishEngine_Threefish512Cipher {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+ThreefishEngine+Threefish512Cipher")]
impl crate::Org::BouncyCastle::Crypto::Engines::ThreefishEngine_Threefish512Cipher {
    pub const ROTATION_0_0: i32 = 46i32;
    pub const ROTATION_0_1: i32 = 36i32;
    pub const ROTATION_0_2: i32 = 19i32;
    pub const ROTATION_0_3: i32 = 37i32;
    pub const ROTATION_1_0: i32 = 33i32;
    pub const ROTATION_1_1: i32 = 27i32;
    pub const ROTATION_1_2: i32 = 14i32;
    pub const ROTATION_1_3: i32 = 42i32;
    pub const ROTATION_2_0: i32 = 17i32;
    pub const ROTATION_2_1: i32 = 49i32;
    pub const ROTATION_2_2: i32 = 36i32;
    pub const ROTATION_2_3: i32 = 39i32;
    pub const ROTATION_3_0: i32 = 44i32;
    pub const ROTATION_3_1: i32 = 9i32;
    pub const ROTATION_3_2: i32 = 54i32;
    pub const ROTATION_3_3: i32 = 56i32;
    pub const ROTATION_4_0: i32 = 39i32;
    pub const ROTATION_4_1: i32 = 30i32;
    pub const ROTATION_4_2: i32 = 34i32;
    pub const ROTATION_4_3: i32 = 24i32;
    pub const ROTATION_5_0: i32 = 13i32;
    pub const ROTATION_5_1: i32 = 50i32;
    pub const ROTATION_5_2: i32 = 10i32;
    pub const ROTATION_5_3: i32 = 17i32;
    pub const ROTATION_6_0: i32 = 25i32;
    pub const ROTATION_6_1: i32 = 29i32;
    pub const ROTATION_6_2: i32 = 39i32;
    pub const ROTATION_6_3: i32 = 43i32;
    pub const ROTATION_7_0: i32 = 8i32;
    pub const ROTATION_7_1: i32 = 35i32;
    pub const ROTATION_7_2: i32 = 56i32;
    pub const ROTATION_7_3: i32 = 22i32;
    pub fn DecryptBlock(
        &mut self,
        block: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        state: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DecryptBlock", (block, state))?;
        Ok(__cordl_ret.into())
    }
    pub fn EncryptBlock(
        &mut self,
        block: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        outWords: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EncryptBlock", (block, outWords))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        kw: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        t: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (kw, t))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        kw: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        t: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (kw, t))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+ThreefishEngine+Threefish512Cipher")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Engines::ThreefishEngine_Threefish512Cipher {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+ThreefishEngine+ThreefishCipher")]
#[repr(C)]
#[derive(Debug)]
pub struct ThreefishEngine_ThreefishCipher {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub t: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    pub kw: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+ThreefishEngine+ThreefishCipher")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Crypto::Engines::ThreefishEngine_ThreefishCipher {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Crypto.Engines";
    const CLASS_NAME: &'static str = "ThreefishCipher";
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
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+ThreefishEngine+ThreefishCipher")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Engines::ThreefishEngine_ThreefishCipher {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+ThreefishEngine+ThreefishCipher")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Engines::ThreefishEngine_ThreefishCipher {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+ThreefishEngine+ThreefishCipher")]
impl crate::Org::BouncyCastle::Crypto::Engines::ThreefishEngine_ThreefishCipher {
    pub fn DecryptBlock(
        &mut self,
        block: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        outWords: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DecryptBlock", (block, outWords))?;
        Ok(__cordl_ret.into())
    }
    pub fn EncryptBlock(
        &mut self,
        block: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        outWords: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EncryptBlock", (block, outWords))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        kw: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        t: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (kw, t))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        kw: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        t: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (kw, t))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+ThreefishEngine+ThreefishCipher")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Engines::ThreefishEngine_ThreefishCipher {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
