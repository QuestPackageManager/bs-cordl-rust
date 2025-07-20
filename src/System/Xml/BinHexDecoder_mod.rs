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
unsafe impl quest_hook::libil2cpp::Type for crate::System::Xml::BinHexDecoder {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Xml";
    const CLASS_NAME: &'static str = "BinHexDecoder";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Xml::BinHexDecoder as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                2usize,
            >("Decode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Xml::BinHexDecoder as quest_hook::libil2cpp::Type >
                    ::class(), "Decode", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = unsafe { method.invoke_unchecked((), (chars, allowOddChars))? };
        Ok(__cordl_ret.into())
    }
    pub fn Decode_Il2CppArray_i32_i32_0(
        &mut self,
        chars: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        startPos: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Xml::BinHexDecoder as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
                    i32,
                    i32,
                ),
                i32,
                3usize,
            >("Decode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Xml::BinHexDecoder as quest_hook::libil2cpp::Type >
                    ::class(), "Decode", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked(self, (chars, startPos, len))?
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Xml::BinHexDecoder as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::ByRefMut<bool>,
                    quest_hook::libil2cpp::ByRefMut<u8>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                ),
                quest_hook::libil2cpp::Void,
                8usize,
            >("Decode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Xml::BinHexDecoder as quest_hook::libil2cpp::Type >
                    ::class(), "Decode", 8usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
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
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsFull(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Xml::BinHexDecoder as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_IsFull")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Xml::BinHexDecoder as quest_hook::libil2cpp::Type >
                    ::class(), "get_IsFull", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
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
