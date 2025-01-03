#[cfg(feature = "Org+BouncyCastle+Utilities+Encoders+BufferedEncoder")]
#[repr(C)]
#[derive(Debug)]
pub struct BufferedEncoder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub bufOff: i32,
    pub translator: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Utilities::Encoders::ITranslator,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Encoders+BufferedEncoder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Org::BouncyCastle::Utilities::Encoders::BufferedEncoder =>
    "Org.BouncyCastle.Utilities.Encoders"."BufferedEncoder"
);
#[cfg(feature = "Org+BouncyCastle+Utilities+Encoders+BufferedEncoder")]
impl std::ops::Deref for crate::Org::BouncyCastle::Utilities::Encoders::BufferedEncoder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Encoders+BufferedEncoder")]
impl std::ops::DerefMut
for crate::Org::BouncyCastle::Utilities::Encoders::BufferedEncoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Encoders+BufferedEncoder")]
impl crate::Org::BouncyCastle::Utilities::Encoders::BufferedEncoder {
    pub fn New(
        translator: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Encoders::ITranslator,
        >,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (translator, bufferSize))?;
        Ok(__cordl_object.into())
    }
    pub fn ProcessByte(
        &mut self,
        input: u8,
        outBytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        outOff: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ProcessByte", (input, outBytes, outOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProcessBytes(
        &mut self,
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        inOff: i32,
        len: i32,
        outBytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        outOff: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ProcessBytes", (input, inOff, len, outBytes, outOff))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        translator: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Utilities::Encoders::ITranslator,
        >,
        bufferSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (translator, bufferSize))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Utilities+Encoders+BufferedEncoder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Utilities::Encoders::BufferedEncoder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
