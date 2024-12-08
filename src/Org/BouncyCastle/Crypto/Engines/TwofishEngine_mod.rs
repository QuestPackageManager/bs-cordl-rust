#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+TwofishEngine")]
#[repr(C)]
#[derive(Debug)]
pub struct TwofishEngine {
    __cordl_parent: crate::System::Object,
    pub encrypting: bool,
    pub gMDS0: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub gMDS1: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub gMDS2: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub gMDS3: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub gSubKeys: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub gSBox: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub k64Cnt: i32,
    pub workingKey: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+TwofishEngine")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Engines::TwofishEngine =>
    "Org.BouncyCastle.Crypto.Engines"."TwofishEngine"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+TwofishEngine")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Engines::TwofishEngine {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+TwofishEngine")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Engines::TwofishEngine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+TwofishEngine")]
impl crate::Org::BouncyCastle::Crypto::Engines::TwofishEngine {
    pub const BLOCK_SIZE: i32 = 16i32;
    pub const GF256_FDBK: i32 = 361i32;
    pub const GF256_FDBK_2: i32 = 180i32;
    pub const GF256_FDBK_4: i32 = 90i32;
    pub const INPUT_WHITEN: i32 = 0i32;
    pub const MAX_KEY_BITS: i32 = 256i32;
    pub const MAX_ROUNDS: i32 = 16i32;
    pub const OUTPUT_WHITEN: i32 = 4i32;
    pub const P_00: i32 = 1i32;
    pub const P_01: i32 = 0i32;
    pub const P_02: i32 = 0i32;
    pub const P_03: i32 = 1i32;
    pub const P_04: i32 = 1i32;
    pub const P_10: i32 = 0i32;
    pub const P_11: i32 = 0i32;
    pub const P_12: i32 = 1i32;
    pub const P_13: i32 = 1i32;
    pub const P_14: i32 = 0i32;
    pub const P_20: i32 = 1i32;
    pub const P_21: i32 = 1i32;
    pub const P_22: i32 = 0i32;
    pub const P_23: i32 = 0i32;
    pub const P_24: i32 = 0i32;
    pub const P_30: i32 = 0i32;
    pub const P_31: i32 = 1i32;
    pub const P_32: i32 = 1i32;
    pub const P_33: i32 = 0i32;
    pub const P_34: i32 = 1i32;
    pub const ROUNDS: i32 = 16i32;
    pub const ROUND_SUBKEYS: i32 = 8i32;
    pub const RS_GF_FDBK: i32 = 333i32;
    pub const SK_BUMP: i32 = 16843009i32;
    pub const SK_ROTL: i32 = 9i32;
    pub const SK_STEP: i32 = 33686018i32;
    pub const TOTAL_SUBKEYS: i32 = 40i32;
    pub fn Bits32ToBytes(
        &mut self,
        inData: i32,
        b: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Bits32ToBytes", (inData, b, offset))?;
        Ok(__cordl_ret)
    }
    pub fn BytesTo32Bits(
        &mut self,
        b: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        p: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("BytesTo32Bits", (b, p))?;
        Ok(__cordl_ret)
    }
    pub fn DecryptBlock(
        &mut self,
        src: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        srcIndex: i32,
        dst: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        dstIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DecryptBlock", (src, srcIndex, dst, dstIndex))?;
        Ok(__cordl_ret)
    }
    pub fn EncryptBlock(
        &mut self,
        src: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        srcIndex: i32,
        dst: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        dstIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("EncryptBlock", (src, srcIndex, dst, dstIndex))?;
        Ok(__cordl_ret)
    }
    pub fn F32(
        &mut self,
        x: i32,
        k32: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("F32", (x, k32))?;
        Ok(__cordl_ret)
    }
    pub fn Fe32_0(&mut self, x: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Fe32_0", (x))?;
        Ok(__cordl_ret)
    }
    pub fn Fe32_3(&mut self, x: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Fe32_3", (x))?;
        Ok(__cordl_ret)
    }
    pub fn GetBlockSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetBlockSize", ())?;
        Ok(__cordl_ret)
    }
    pub fn Init(
        &mut self,
        forEncryption: bool,
        parameters: *mut crate::Org::BouncyCastle::Crypto::ICipherParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (forEncryption, parameters))?;
        Ok(__cordl_ret)
    }
    pub fn LFSR1(&mut self, x: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("LFSR1", (x))?;
        Ok(__cordl_ret)
    }
    pub fn LFSR2(&mut self, x: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("LFSR2", (x))?;
        Ok(__cordl_ret)
    }
    pub fn M_b0(&mut self, x: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("M_b0", (x))?;
        Ok(__cordl_ret)
    }
    pub fn M_b1(&mut self, x: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("M_b1", (x))?;
        Ok(__cordl_ret)
    }
    pub fn M_b2(&mut self, x: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("M_b2", (x))?;
        Ok(__cordl_ret)
    }
    pub fn M_b3(&mut self, x: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("M_b3", (x))?;
        Ok(__cordl_ret)
    }
    pub fn Mx_X(&mut self, x: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Mx_X", (x))?;
        Ok(__cordl_ret)
    }
    pub fn Mx_Y(&mut self, x: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Mx_Y", (x))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn ProcessBlock(
        &mut self,
        input: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        inOff: i32,
        output: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        outOff: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ProcessBlock", (input, inOff, output, outOff))?;
        Ok(__cordl_ret)
    }
    pub fn RS_MDS_Encode(
        &mut self,
        k0: i32,
        k1: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("RS_MDS_Encode", (k0, k1))?;
        Ok(__cordl_ret)
    }
    pub fn RS_rem(&mut self, x: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("RS_rem", (x))?;
        Ok(__cordl_ret)
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetKey(
        &mut self,
        key: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetKey", (key))?;
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
    pub fn get_AlgorithmName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_AlgorithmName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsPartialBlockOkay(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsPartialBlockOkay", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Engines+TwofishEngine")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Engines::TwofishEngine {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
