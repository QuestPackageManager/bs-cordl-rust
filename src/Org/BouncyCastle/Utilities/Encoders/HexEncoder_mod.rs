#[cfg(feature = "Org+BouncyCastle+Utilities+Encoders+HexEncoder")]
#[repr(C)]
#[derive(Debug)]
pub struct HexEncoder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub encodingTable: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub decodingTable: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Encoders+HexEncoder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Utilities::Encoders::HexEncoder =>
    "Org.BouncyCastle.Utilities.Encoders"."HexEncoder"
);
#[cfg(feature = "Org+BouncyCastle+Utilities+Encoders+HexEncoder")]
impl std::ops::Deref for crate::Org::BouncyCastle::Utilities::Encoders::HexEncoder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Encoders+HexEncoder")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Utilities::Encoders::HexEncoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Encoders+HexEncoder")]
impl crate::Org::BouncyCastle::Utilities::Encoders::HexEncoder {
    pub fn Decode(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
        length: i32,
        outStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Decode", (data, off, length, outStream))?;
        Ok(__cordl_ret.into())
    }
    pub fn DecodeStrict(
        &mut self,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        off: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = __cordl_object.invoke("DecodeStrict", (str, off, len))?;
        Ok(__cordl_ret.into())
    }
    pub fn DecodeString(
        &mut self,
        data: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        outStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("DecodeString", (data, outStream))?;
        Ok(__cordl_ret.into())
    }
    pub fn Encode_Il2CppArray_i32_0(
        &mut self,
        inBuf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        inOff: i32,
        inLen: i32,
        outBuf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        outOff: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Encode", (inBuf, inOff, inLen, outBuf, outOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn Encode_Stream1(
        &mut self,
        buf: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        off: i32,
        len: i32,
        outStream: quest_hook::libil2cpp::Gc<crate::System::IO::Stream>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("Encode", (buf, off, len, outStream))?;
        Ok(__cordl_ret.into())
    }
    pub fn Ignore(c: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Ignore", (c))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitialiseDecodingTable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitialiseDecodingTable", ())?;
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Encoders+HexEncoder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Utilities::Encoders::HexEncoder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Encoders+HexEncoder")]
impl AsRef<crate::Org::BouncyCastle::Utilities::Encoders::IEncoder>
for crate::Org::BouncyCastle::Utilities::Encoders::HexEncoder {
    fn as_ref(&self) -> &crate::Org::BouncyCastle::Utilities::Encoders::IEncoder {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Encoders+HexEncoder")]
impl AsMut<crate::Org::BouncyCastle::Utilities::Encoders::IEncoder>
for crate::Org::BouncyCastle::Utilities::Encoders::HexEncoder {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Org::BouncyCastle::Utilities::Encoders::IEncoder {
        unsafe { std::mem::transmute(self) }
    }
}
