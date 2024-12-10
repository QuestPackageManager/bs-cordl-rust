#[cfg(feature = "Org+BouncyCastle+Crypto+Digests+SkeinEngine")]
#[repr(C)]
#[derive(Debug)]
pub struct SkeinEngine {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub threefish: *mut crate::Org::BouncyCastle::Crypto::Engines::ThreefishEngine,
    pub outputSizeBytes: i32,
    pub chain: *mut quest_hook::libil2cpp::Il2CppArray<u64>,
    pub initialState: *mut quest_hook::libil2cpp::Il2CppArray<u64>,
    pub key: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub preMessageParameters: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::Org::BouncyCastle::Crypto::Digests::SkeinEngine_Parameter,
    >,
    pub postMessageParameters: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::Org::BouncyCastle::Crypto::Digests::SkeinEngine_Parameter,
    >,
    pub ubi: *mut crate::Org::BouncyCastle::Crypto::Digests::SkeinEngine_UBI,
    pub singleByte: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Digests+SkeinEngine")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Crypto::Digests::SkeinEngine
    => "Org.BouncyCastle.Crypto.Digests"."SkeinEngine"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Digests+SkeinEngine")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Digests::SkeinEngine {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Digests+SkeinEngine")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Digests::SkeinEngine {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Digests+SkeinEngine")]
impl crate::Org::BouncyCastle::Crypto::Digests::SkeinEngine {
    pub const PARAM_TYPE_CONFIG: i32 = 4i32;
    pub const PARAM_TYPE_KEY: i32 = 0i32;
    pub const PARAM_TYPE_MESSAGE: i32 = 48i32;
    pub const PARAM_TYPE_OUTPUT: i32 = 63i32;
    pub const SKEIN_1024: i32 = 1024i32;
    pub const SKEIN_256: i32 = 256i32;
    pub const SKEIN_512: i32 = 512i32;
    #[cfg(feature = "Org+BouncyCastle+Crypto+Digests+SkeinEngine+Configuration")]
    pub type Configuration = crate::Org::BouncyCastle::Crypto::Digests::SkeinEngine_Configuration;
    #[cfg(feature = "Org+BouncyCastle+Crypto+Digests+SkeinEngine+Parameter")]
    pub type Parameter = crate::Org::BouncyCastle::Crypto::Digests::SkeinEngine_Parameter;
    #[cfg(feature = "Org+BouncyCastle+Crypto+Digests+SkeinEngine+UBI")]
    pub type UBI = crate::Org::BouncyCastle::Crypto::Digests::SkeinEngine_UBI;
    #[cfg(feature = "Org+BouncyCastle+Crypto+Digests+SkeinEngine+UbiTweak")]
    pub type UbiTweak = crate::Org::BouncyCastle::Crypto::Digests::SkeinEngine_UbiTweak;
    pub fn CheckInitialised(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CheckInitialised", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Copy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Utilities::IMemoable>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::IMemoable,
        > = __cordl_object.invoke("Copy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyIn(
        &mut self,
        engine: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Digests::SkeinEngine,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyIn", (engine))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateInitialState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateInitialState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn DoFinal(
        &mut self,
        outBytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        outOff: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("DoFinal", (outBytes, outOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        parameters: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Parameters::SkeinParameters,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Init", (parameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitParams(
        &mut self,
        parameters: quest_hook::libil2cpp::Gc<crate::System::Collections::IDictionary>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitParams", (parameters))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_SkeinEngine1(
        engine: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Digests::SkeinEngine,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (engine))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_i32_0(
        blockSizeBits: i32,
        outputSizeBits: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (blockSizeBits, outputSizeBits))?;
        Ok(__cordl_object.into())
    }
    pub fn Output(
        &mut self,
        outputSequence: u64,
        outBytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        outOff: i32,
        outputBytes: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Output", (outputSequence, outBytes, outOff, outputBytes))?;
        Ok(__cordl_ret.into())
    }
    pub fn Reset_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Reset_IMemoable0(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::Org::BouncyCastle::Utilities::IMemoable>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn UbiComplete(
        &mut self,
        _cordl_type: i32,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UbiComplete", (_cordl_type, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn UbiFinal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UbiFinal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UbiInit(
        &mut self,
        _cordl_type: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UbiInit", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn Update_Il2CppArray_i32_i32_1(
        &mut self,
        inBytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        inOff: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (inBytes, inOff, len))?;
        Ok(__cordl_ret.into())
    }
    pub fn Update_u8_0(
        &mut self,
        inByte: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (inByte))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SkeinEngine1(
        &mut self,
        engine: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Digests::SkeinEngine,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (engine))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_i32_0(
        &mut self,
        blockSizeBits: i32,
        outputSizeBits: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (blockSizeBits, outputSizeBits))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_BlockSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_BlockSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_OutputSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_OutputSize", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Digests+SkeinEngine")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Digests::SkeinEngine {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Digests+SkeinEngine")]
impl AsRef<crate::Org::BouncyCastle::Utilities::IMemoable>
for crate::Org::BouncyCastle::Crypto::Digests::SkeinEngine {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Utilities::IMemoable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Digests+SkeinEngine")]
impl AsMut<crate::Org::BouncyCastle::Utilities::IMemoable>
for crate::Org::BouncyCastle::Crypto::Digests::SkeinEngine {
    fn as_mut(&mut self) -> &mut crate::Org::BouncyCastle::Utilities::IMemoable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Digests+SkeinEngine+Configuration")]
#[repr(C)]
#[derive(Debug)]
pub struct SkeinEngine_Configuration {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub bytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Digests+SkeinEngine+Configuration")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Digests::SkeinEngine_Configuration =>
    "Org.BouncyCastle.Crypto.Digests"."SkeinEngine/Configuration"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Digests+SkeinEngine+Configuration")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Digests::SkeinEngine_Configuration {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Digests+SkeinEngine+Configuration")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Digests::SkeinEngine_Configuration {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Digests+SkeinEngine+Configuration")]
impl crate::Org::BouncyCastle::Crypto::Digests::SkeinEngine_Configuration {
    pub fn New(
        outputSizeBits: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (outputSizeBits))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        outputSizeBits: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (outputSizeBits))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Bytes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("get_Bytes", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Digests+SkeinEngine+Configuration")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Digests::SkeinEngine_Configuration {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Digests+SkeinEngine+Parameter")]
#[repr(C)]
#[derive(Debug)]
pub struct SkeinEngine_Parameter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _cordl_type: i32,
    pub value: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Digests+SkeinEngine+Parameter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Digests::SkeinEngine_Parameter =>
    "Org.BouncyCastle.Crypto.Digests"."SkeinEngine/Parameter"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Digests+SkeinEngine+Parameter")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Digests::SkeinEngine_Parameter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Digests+SkeinEngine+Parameter")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Digests::SkeinEngine_Parameter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Digests+SkeinEngine+Parameter")]
impl crate::Org::BouncyCastle::Crypto::Digests::SkeinEngine_Parameter {
    pub fn New(
        _cordl_type: i32,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, value))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_type: i32,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Type(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Type", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("get_Value", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Digests+SkeinEngine+Parameter")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Digests::SkeinEngine_Parameter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Digests+SkeinEngine+UBI")]
#[repr(C)]
#[derive(Debug)]
pub struct SkeinEngine_UBI {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub tweak: *mut crate::Org::BouncyCastle::Crypto::Digests::SkeinEngine_UbiTweak,
    pub engine: *mut crate::Org::BouncyCastle::Crypto::Digests::SkeinEngine,
    pub currentBlock: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub currentOffset: i32,
    pub message: *mut quest_hook::libil2cpp::Il2CppArray<u64>,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Digests+SkeinEngine+UBI")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Digests::SkeinEngine_UBI =>
    "Org.BouncyCastle.Crypto.Digests"."SkeinEngine/UBI"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Digests+SkeinEngine+UBI")]
impl std::ops::Deref for crate::Org::BouncyCastle::Crypto::Digests::SkeinEngine_UBI {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Digests+SkeinEngine+UBI")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Crypto::Digests::SkeinEngine_UBI {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Digests+SkeinEngine+UBI")]
impl crate::Org::BouncyCastle::Crypto::Digests::SkeinEngine_UBI {
    pub fn DoFinal(
        &mut self,
        output: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DoFinal", (output))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        engine: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Digests::SkeinEngine,
        >,
        blockSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (engine, blockSize))?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessBlock(
        &mut self,
        output: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ProcessBlock", (output))?;
        Ok(__cordl_ret.into())
    }
    pub fn Reset_SkeinEngine_UBI0(
        &mut self,
        ubi: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Digests::SkeinEngine_UBI,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", (ubi))?;
        Ok(__cordl_ret.into())
    }
    pub fn Reset_i32_1(
        &mut self,
        _cordl_type: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        offset: i32,
        len: i32,
        output: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", (value, offset, len, output))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        engine: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Digests::SkeinEngine,
        >,
        blockSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (engine, blockSize))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Digests+SkeinEngine+UBI")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Digests::SkeinEngine_UBI {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Digests+SkeinEngine+UbiTweak")]
#[repr(C)]
#[derive(Debug)]
pub struct SkeinEngine_UbiTweak {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub tweak: *mut quest_hook::libil2cpp::Il2CppArray<u64>,
    pub extendedPosition: bool,
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Digests+SkeinEngine+UbiTweak")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Crypto::Digests::SkeinEngine_UbiTweak =>
    "Org.BouncyCastle.Crypto.Digests"."SkeinEngine/UbiTweak"
);
#[cfg(feature = "Org+BouncyCastle+Crypto+Digests+SkeinEngine+UbiTweak")]
impl std::ops::Deref
for crate::Org::BouncyCastle::Crypto::Digests::SkeinEngine_UbiTweak {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Digests+SkeinEngine+UbiTweak")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Crypto::Digests::SkeinEngine_UbiTweak {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Digests+SkeinEngine+UbiTweak")]
impl crate::Org::BouncyCastle::Crypto::Digests::SkeinEngine_UbiTweak {
    pub const LOW_RANGE: u64 = 18446744069414584320u64;
    pub const T1_FINAL: u64 = 9223372036854775808u64;
    pub const T1_FIRST: u64 = 4611686018427387904u64;
    pub fn AdvancePosition(
        &mut self,
        advance: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AdvancePosition", (advance))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetWords(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u64>,
        > = __cordl_object.invoke("GetWords", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Reset_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Reset_SkeinEngine_UbiTweak0(
        &mut self,
        tweak: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Crypto::Digests::SkeinEngine_UbiTweak,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", (tweak))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
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
    pub fn get_Final(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Final", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_First(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_First", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Type(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("get_Type", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Final(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Final", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_First(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_First", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Type(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Type", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Crypto+Digests+SkeinEngine+UbiTweak")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Crypto::Digests::SkeinEngine_UbiTweak {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
