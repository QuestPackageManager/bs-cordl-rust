#[cfg(feature = "System+Xml+BinHexDecoder")]
#[repr(C)]
#[derive(Debug)]
pub struct BinHexDecoder {
    __cordl_parent: crate::System::Xml::IncrementalReadDecoder,
    pub buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    pub curIndex: i32,
    pub endIndex: i32,
    pub hasHalfByteCached: bool,
    pub cachedHalfByte: u8,
}
#[cfg(feature = "System+Xml+BinHexDecoder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::BinHexDecoder => "System.Xml"
    ."BinHexDecoder"
);
#[cfg(feature = "System+Xml+BinHexDecoder")]
impl std::ops::Deref for crate::System::Xml::BinHexDecoder {
    type Target = crate::System::Xml::IncrementalReadDecoder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+BinHexDecoder")]
impl std::ops::DerefMut for crate::System::Xml::BinHexDecoder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Xml+BinHexDecoder")]
impl crate::System::Xml::BinHexDecoder {
    pub fn Decode_Il2CppArray__cordl_bool1(
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        allowOddChars: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Decode", (chars, allowOddChars))?;
        Ok(__cordl_ret.into())
    }
    pub fn Decode_Il2CppArray_i32_i32_0(
        &mut self,
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        startPos: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Decode", (chars, startPos, len))?;
        Ok(__cordl_ret.into())
    }
    pub fn Decode_Il2CppObject_Il2CppObject_Il2CppObject_Il2CppObject_ByRefMut_ByRefMut_ByRefMut_ByRefMut2(
        pChars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        pCharsEndPos: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        pBytes: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        pBytesEndPos: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        hasHalfByteCached: quest_hook::libil2cpp::ByRefMut<bool>,
        cachedHalfByte: quest_hook::libil2cpp::ByRefMut<u8>,
        charsDecoded: quest_hook::libil2cpp::ByRefMut<i32>,
        bytesDecoded: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "Decode",
                (
                    pChars,
                    pCharsEndPos,
                    pBytes,
                    pBytesEndPos,
                    hasHalfByteCached,
                    cachedHalfByte,
                    charsDecoded,
                    bytesDecoded,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsFull(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsFull", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+BinHexDecoder")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Xml::BinHexDecoder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
