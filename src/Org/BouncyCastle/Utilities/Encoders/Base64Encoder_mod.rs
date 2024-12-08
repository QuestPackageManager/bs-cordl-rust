#[cfg(feature = "Org+BouncyCastle+Utilities+Encoders+Base64Encoder")]
#[repr(C)]
#[derive(Debug)]
pub struct Base64Encoder {
    __cordl_parent: crate::System::Object,
    pub encodingTable: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub padding: u8,
    pub decodingTable: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Encoders+Base64Encoder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Utilities::Encoders::Base64Encoder =>
    "Org.BouncyCastle.Utilities.Encoders"."Base64Encoder"
);
#[cfg(feature = "Org+BouncyCastle+Utilities+Encoders+Base64Encoder")]
impl std::ops::Deref for crate::Org::BouncyCastle::Utilities::Encoders::Base64Encoder {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Encoders+Base64Encoder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Utilities::Encoders::Base64Encoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Encoders+Base64Encoder")]
impl crate::Org::BouncyCastle::Utilities::Encoders::Base64Encoder {
    pub fn Decode(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        off: i32,
        length: i32,
        outStream: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Decode", (data, off, length, outStream))?;
        Ok(__cordl_ret)
    }
    pub fn DecodeString(
        &mut self,
        data: *mut crate::System::String,
        outStream: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("DecodeString", (data, outStream))?;
        Ok(__cordl_ret)
    }
    pub fn ignore(&mut self, c: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("ignore", (c))?;
        Ok(__cordl_ret)
    }
    pub fn nextI_Il2CppArray0(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        i: i32,
        finish: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("nextI", (data, i, finish))?;
        Ok(__cordl_ret)
    }
    pub fn nextI_String1(
        &mut self,
        data: *mut crate::System::String,
        i: i32,
        finish: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("nextI", (data, i, finish))?;
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
    pub fn InitialiseDecodingTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitialiseDecodingTable", ())?;
        Ok(__cordl_ret)
    }
    pub fn Encode_Il2CppArray_i32_0(
        &mut self,
        inBuf: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        inOff: i32,
        inLen: i32,
        outBuf: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        outOff: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Encode", (inBuf, inOff, inLen, outBuf, outOff))?;
        Ok(__cordl_ret)
    }
    pub fn Encode_Stream1(
        &mut self,
        buf: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        off: i32,
        len: i32,
        outStream: *mut crate::System::IO::Stream,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Encode", (buf, off, len, outStream))?;
        Ok(__cordl_ret)
    }
    pub fn decodeLastBlock(
        &mut self,
        outStream: *mut crate::System::IO::Stream,
        c1: char,
        c2: char,
        c3: char,
        c4: char,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("decodeLastBlock", (outStream, c1, c2, c3, c4))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Encoders+Base64Encoder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Utilities::Encoders::Base64Encoder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
