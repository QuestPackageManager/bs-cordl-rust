#[cfg(feature = "Org+BouncyCastle+Utilities+Encoders+BufferedDecoder")]
#[repr(C)]
#[derive(Debug)]
pub struct BufferedDecoder {
    __cordl_parent: crate::System::Object,
    pub buffer: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub bufOff: i32,
    pub translator: *mut crate::Org::BouncyCastle::Utilities::Encoders::ITranslator,
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Encoders+BufferedDecoder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Utilities::Encoders::BufferedDecoder =>
    "Org.BouncyCastle.Utilities.Encoders"."BufferedDecoder"
);
#[cfg(feature = "Org+BouncyCastle+Utilities+Encoders+BufferedDecoder")]
impl std::ops::Deref for crate::Org::BouncyCastle::Utilities::Encoders::BufferedDecoder {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Encoders+BufferedDecoder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Utilities::Encoders::BufferedDecoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Encoders+BufferedDecoder")]
impl crate::Org::BouncyCastle::Utilities::Encoders::BufferedDecoder {
    pub fn ProcessByte(
        &mut self,
        input: u8,
        output: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        outOff: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ProcessByte", (input, output, outOff))?;
        Ok(__cordl_ret)
    }
    pub fn ProcessBytes(
        &mut self,
        input: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        inOff: i32,
        len: i32,
        outBytes: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        outOff: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ProcessBytes", (input, inOff, len, outBytes, outOff))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        translator: *mut crate::Org::BouncyCastle::Utilities::Encoders::ITranslator,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (translator, bufferSize))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        translator: *mut crate::Org::BouncyCastle::Utilities::Encoders::ITranslator,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (translator, bufferSize))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Encoders+BufferedDecoder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Utilities::Encoders::BufferedDecoder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
