#[cfg(feature = "System+Xml+Ucs4Decoder")]
#[repr(C)]
#[derive(Debug)]
pub struct Ucs4Decoder {
    __cordl_parent: crate::System::Text::Decoder,
    pub lastBytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub lastBytesCount: i32,
}
#[cfg(feature = "System+Xml+Ucs4Decoder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Ucs4Decoder => "System.Xml"
    ."Ucs4Decoder"
);
#[cfg(feature = "System+Xml+Ucs4Decoder")]
impl std::ops::Deref for crate::System::Xml::Ucs4Decoder {
    type Target = crate::System::Text::Decoder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Ucs4Decoder")]
impl std::ops::DerefMut for crate::System::Xml::Ucs4Decoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+Ucs4Decoder")]
impl crate::System::Xml::Ucs4Decoder {
    pub fn Convert(
        &mut self,
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        byteIndex: i32,
        byteCount: i32,
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        charIndex: i32,
        charCount: i32,
        flush: bool,
        bytesUsed: quest_hook::libil2cpp::ByRefMut<i32>,
        charsUsed: quest_hook::libil2cpp::ByRefMut<i32>,
        completed: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "Convert",
                (
                    bytes,
                    byteIndex,
                    byteCount,
                    chars,
                    charIndex,
                    charCount,
                    flush,
                    bytesUsed,
                    charsUsed,
                    completed,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCharCount(
        &mut self,
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        index: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetCharCount", (bytes, index, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetChars(
        &mut self,
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        byteIndex: i32,
        byteCount: i32,
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        charIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetChars", (bytes, byteIndex, byteCount, chars, charIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFullChars(
        &mut self,
        bytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        byteIndex: i32,
        byteCount: i32,
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        charIndex: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetFullChars", (bytes, byteIndex, byteCount, chars, charIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Ucs4ToUTF16(
        &mut self,
        code: u32,
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        charIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Ucs4ToUTF16", (code, chars, charIndex))?;
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
}
#[cfg(feature = "System+Xml+Ucs4Decoder")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::Ucs4Decoder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
