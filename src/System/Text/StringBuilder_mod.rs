#[cfg(feature = "System+Text+StringBuilder")]
#[repr(C)]
#[derive(Debug)]
pub struct StringBuilder {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_ChunkChars: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    pub m_ChunkPrevious: *mut crate::System::Text::StringBuilder,
    pub m_ChunkLength: i32,
    pub m_ChunkOffset: i32,
    pub m_MaxCapacity: i32,
}
#[cfg(feature = "System+Text+StringBuilder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Text::StringBuilder => "System.Text"
    ."StringBuilder"
);
#[cfg(feature = "System+Text+StringBuilder")]
impl std::ops::Deref for crate::System::Text::StringBuilder {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+StringBuilder")]
impl std::ops::DerefMut for crate::System::Text::StringBuilder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+StringBuilder")]
impl crate::System::Text::StringBuilder {
    pub const CapacityField: &'static str = "Capacity";
    pub const DefaultCapacity: i32 = 16i32;
    pub const IndexLimit: i32 = 1000000i32;
    pub const MaxCapacityField: &'static str = "m_MaxCapacity";
    pub const MaxChunkSize: i32 = 8000i32;
    pub const StringValueField: &'static str = "m_StringValue";
    pub const ThreadIDField: &'static str = "m_currentThread";
    pub const WidthLimit: i32 = 1000000i32;
    pub fn AppendCore(
        &mut self,
        value: *mut crate::System::Text::StringBuilder,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::StringBuilder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::StringBuilder = __cordl_object
            .invoke("AppendCore", (value, startIndex, count))?;
        Ok(__cordl_ret)
    }
    pub fn AppendFormatHelper(
        &mut self,
        provider: *mut crate::System::IFormatProvider,
        format: *mut quest_hook::libil2cpp::Il2CppString,
        args: crate::System::ParamsArray,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::StringBuilder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::StringBuilder = __cordl_object
            .invoke("AppendFormatHelper", (provider, format, args))?;
        Ok(__cordl_ret)
    }
    pub fn AppendFormat_IFormatProvider_Il2CppString_Il2CppObject4(
        &mut self,
        provider: *mut crate::System::IFormatProvider,
        format: *mut quest_hook::libil2cpp::Il2CppString,
        arg0: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::StringBuilder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::StringBuilder = __cordl_object
            .invoke("AppendFormat", (provider, format, arg0))?;
        Ok(__cordl_ret)
    }
    pub fn AppendFormat_IFormatProvider_Il2CppString_Il2CppObject_Il2CppObject_Il2CppObject5(
        &mut self,
        provider: *mut crate::System::IFormatProvider,
        format: *mut quest_hook::libil2cpp::Il2CppString,
        arg0: *mut quest_hook::libil2cpp::Il2CppObject,
        arg1: *mut quest_hook::libil2cpp::Il2CppObject,
        arg2: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::StringBuilder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::StringBuilder = __cordl_object
            .invoke("AppendFormat", (provider, format, arg0, arg1, arg2))?;
        Ok(__cordl_ret)
    }
    pub fn AppendFormat_Il2CppString_Il2CppArray3(
        &mut self,
        format: *mut quest_hook::libil2cpp::Il2CppString,
        args: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::StringBuilder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::StringBuilder = __cordl_object
            .invoke("AppendFormat", (format, args))?;
        Ok(__cordl_ret)
    }
    pub fn AppendFormat_Il2CppString_Il2CppObject0(
        &mut self,
        format: *mut quest_hook::libil2cpp::Il2CppString,
        arg0: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::StringBuilder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::StringBuilder = __cordl_object
            .invoke("AppendFormat", (format, arg0))?;
        Ok(__cordl_ret)
    }
    pub fn AppendFormat_Il2CppString_Il2CppObject_Il2CppObject1(
        &mut self,
        format: *mut quest_hook::libil2cpp::Il2CppString,
        arg0: *mut quest_hook::libil2cpp::Il2CppObject,
        arg1: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::StringBuilder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::StringBuilder = __cordl_object
            .invoke("AppendFormat", (format, arg0, arg1))?;
        Ok(__cordl_ret)
    }
    pub fn AppendFormat_Il2CppString_Il2CppObject_Il2CppObject_Il2CppObject2(
        &mut self,
        format: *mut quest_hook::libil2cpp::Il2CppString,
        arg0: *mut quest_hook::libil2cpp::Il2CppObject,
        arg1: *mut quest_hook::libil2cpp::Il2CppObject,
        arg2: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::StringBuilder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::StringBuilder = __cordl_object
            .invoke("AppendFormat", (format, arg0, arg1, arg2))?;
        Ok(__cordl_ret)
    }
    pub fn AppendHelper(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AppendHelper", (value))?;
        Ok(__cordl_ret)
    }
    pub fn AppendLine_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::StringBuilder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::StringBuilder = __cordl_object
            .invoke("AppendLine", ())?;
        Ok(__cordl_ret)
    }
    pub fn AppendLine_Il2CppString1(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::StringBuilder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::StringBuilder = __cordl_object
            .invoke("AppendLine", (value))?;
        Ok(__cordl_ret)
    }
    pub fn AppendSpanFormattable<T>(
        &mut self,
        value: T,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::StringBuilder>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::StringBuilder = __cordl_object
            .invoke("AppendSpanFormattable", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Append_Il2CppArray13(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::StringBuilder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::StringBuilder = __cordl_object
            .invoke("Append", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Append_Il2CppArray_i32_i32_1(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        startIndex: i32,
        charCount: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::StringBuilder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::StringBuilder = __cordl_object
            .invoke("Append", (value, startIndex, charCount))?;
        Ok(__cordl_ret)
    }
    pub fn Append_Il2CppObject12(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::StringBuilder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::StringBuilder = __cordl_object
            .invoke("Append", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Append_Il2CppObject_i32_15(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppObject,
        valueCount: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::StringBuilder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::StringBuilder = __cordl_object
            .invoke("Append", (value, valueCount))?;
        Ok(__cordl_ret)
    }
    pub fn Append_Il2CppString2(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::StringBuilder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::StringBuilder = __cordl_object
            .invoke("Append", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Append_Il2CppString_i32_i32_3(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppString,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::StringBuilder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::StringBuilder = __cordl_object
            .invoke("Append", (value, startIndex, count))?;
        Ok(__cordl_ret)
    }
    pub fn Append_ReadOnlySpan_1_14(
        &mut self,
        value: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::StringBuilder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::StringBuilder = __cordl_object
            .invoke("Append", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Append_StringBuilder4(
        &mut self,
        value: *mut crate::System::Text::StringBuilder,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::StringBuilder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::StringBuilder = __cordl_object
            .invoke("Append", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Append__cordl_bool5(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::StringBuilder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::StringBuilder = __cordl_object
            .invoke("Append", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Append__cordl_char6(
        &mut self,
        value: char,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::StringBuilder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::StringBuilder = __cordl_object
            .invoke("Append", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Append__cordl_char_i32_0(
        &mut self,
        value: char,
        repeatCount: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::StringBuilder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::StringBuilder = __cordl_object
            .invoke("Append", (value, repeatCount))?;
        Ok(__cordl_ret)
    }
    pub fn Append_f64_10(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::StringBuilder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::StringBuilder = __cordl_object
            .invoke("Append", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Append_i32_8(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::StringBuilder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::StringBuilder = __cordl_object
            .invoke("Append", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Append_i64_9(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::StringBuilder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::StringBuilder = __cordl_object
            .invoke("Append", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Append_u32_11(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::StringBuilder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::StringBuilder = __cordl_object
            .invoke("Append", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Append_u8_7(
        &mut self,
        value: u8,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::StringBuilder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::StringBuilder = __cordl_object
            .invoke("Append", (value))?;
        Ok(__cordl_ret)
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::StringBuilder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::StringBuilder = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret)
    }
    pub fn CopyTo(
        &mut self,
        sourceIndex: i32,
        destination: crate::System::Span_1<char>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyTo", (sourceIndex, destination, count))?;
        Ok(__cordl_ret)
    }
    pub fn EnsureCapacity(
        &mut self,
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("EnsureCapacity", (capacity))?;
        Ok(__cordl_ret)
    }
    pub fn ExpandByABlock(
        &mut self,
        minBlockCharCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExpandByABlock", (minBlockCharCount))?;
        Ok(__cordl_ret)
    }
    pub fn FindChunkForIndex(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::StringBuilder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::StringBuilder = __cordl_object
            .invoke("FindChunkForIndex", (index))?;
        Ok(__cordl_ret)
    }
    pub fn Insert_Il2CppObject_i32_2(
        &mut self,
        index: i32,
        value: *mut quest_hook::libil2cpp::Il2CppObject,
        valueCount: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Insert", (index, value, valueCount))?;
        Ok(__cordl_ret)
    }
    pub fn Insert_Il2CppString0(
        &mut self,
        index: i32,
        value: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::StringBuilder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::StringBuilder = __cordl_object
            .invoke("Insert", (index, value))?;
        Ok(__cordl_ret)
    }
    pub fn Insert__cordl_char1(
        &mut self,
        index: i32,
        value: char,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::StringBuilder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::StringBuilder = __cordl_object
            .invoke("Insert", (index, value))?;
        Ok(__cordl_ret)
    }
    pub fn MakeRoom(
        &mut self,
        index: i32,
        count: i32,
        chunk: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Text::StringBuilder>,
        indexInChunk: quest_hook::libil2cpp::ByRefMut<i32>,
        doNotMoveFollowingChars: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "MakeRoom",
                (index, count, chunk, indexInChunk, doNotMoveFollowingChars),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppString2(
        value: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (value))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppString_i32_3(
        value: *mut quest_hook::libil2cpp::Il2CppString,
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (value, capacity))?;
        Ok(__cordl_object)
    }
    pub fn New_Il2CppString_i32_i32_i32_4(
        value: *mut quest_hook::libil2cpp::Il2CppString,
        startIndex: i32,
        length: i32,
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (value, startIndex, length, capacity))?;
        Ok(__cordl_object)
    }
    pub fn New_SerializationInfo_StreamingContext6(
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info, context))?;
        Ok(__cordl_object)
    }
    pub fn New_StringBuilder7(
        from: *mut crate::System::Text::StringBuilder,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (from))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_1(capacity: i32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (capacity))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_i32_5(
        capacity: i32,
        maxCapacity: i32,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (capacity, maxCapacity))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_i32_StringBuilder8(
        _cordl_size: i32,
        maxCapacity: i32,
        previousBlock: *mut crate::System::Text::StringBuilder,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_size, maxCapacity, previousBlock))?;
        Ok(__cordl_object)
    }
    pub fn Next(
        &mut self,
        chunk: *mut crate::System::Text::StringBuilder,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::StringBuilder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::StringBuilder = __cordl_object
            .invoke("Next", (chunk))?;
        Ok(__cordl_ret)
    }
    pub fn Remove_ByRefMut_ByRefMut1(
        &mut self,
        startIndex: i32,
        count: i32,
        chunk: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Text::StringBuilder>,
        indexInChunk: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Remove", (startIndex, count, chunk, indexInChunk))?;
        Ok(__cordl_ret)
    }
    pub fn Remove_i32_i32_0(
        &mut self,
        startIndex: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::StringBuilder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::StringBuilder = __cordl_object
            .invoke("Remove", (startIndex, length))?;
        Ok(__cordl_ret)
    }
    pub fn ReplaceAllInChunk(
        &mut self,
        replacements: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
        replacementsCount: i32,
        sourceChunk: *mut crate::System::Text::StringBuilder,
        removeCount: i32,
        value: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "ReplaceAllInChunk",
                (replacements, replacementsCount, sourceChunk, removeCount, value),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ReplaceInPlaceAtChunk(
        &mut self,
        chunk: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Text::StringBuilder>,
        indexInChunk: quest_hook::libil2cpp::ByRefMut<i32>,
        value: *mut quest_hook::libil2cpp::Il2CppObject,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReplaceInPlaceAtChunk", (chunk, indexInChunk, value, count))?;
        Ok(__cordl_ret)
    }
    pub fn Replace_Il2CppString_Il2CppString0(
        &mut self,
        oldValue: *mut quest_hook::libil2cpp::Il2CppString,
        newValue: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::StringBuilder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::StringBuilder = __cordl_object
            .invoke("Replace", (oldValue, newValue))?;
        Ok(__cordl_ret)
    }
    pub fn Replace_i32_i32_1(
        &mut self,
        oldValue: *mut quest_hook::libil2cpp::Il2CppString,
        newValue: *mut quest_hook::libil2cpp::Il2CppString,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Text::StringBuilder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::StringBuilder = __cordl_object
            .invoke("Replace", (oldValue, newValue, startIndex, count))?;
        Ok(__cordl_ret)
    }
    pub fn StartsWith(
        &mut self,
        chunk: *mut crate::System::Text::StringBuilder,
        indexInChunk: i32,
        count: i32,
        value: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("StartsWith", (chunk, indexInChunk, count, value))?;
        Ok(__cordl_ret)
    }
    pub fn System_Runtime_Serialization_ISerializable_GetObjectData(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Runtime.Serialization.ISerializable.GetObjectData",
                (info, context),
            )?;
        Ok(__cordl_ret)
    }
    pub fn ToString_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn ToString_i32_i32_1(
        &mut self,
        startIndex: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("ToString", (startIndex, length))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppString2(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppString,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppString_i32_3(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppString,
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (value, capacity))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Il2CppString_i32_i32_i32_4(
        &mut self,
        value: *mut quest_hook::libil2cpp::Il2CppString,
        startIndex: i32,
        length: i32,
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (value, startIndex, length, capacity))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_SerializationInfo_StreamingContext6(
        &mut self,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (info, context))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_StringBuilder7(
        &mut self,
        from: *mut crate::System::Text::StringBuilder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (from))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_1(
        &mut self,
        capacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (capacity))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_i32_5(
        &mut self,
        capacity: i32,
        maxCapacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (capacity, maxCapacity))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_i32_StringBuilder8(
        &mut self,
        _cordl_size: i32,
        maxCapacity: i32,
        previousBlock: *mut crate::System::Text::StringBuilder,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_size, maxCapacity, previousBlock))?;
        Ok(__cordl_ret)
    }
    pub fn get_Capacity(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Capacity", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Chars(&mut self, index: i32) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: char = __cordl_object.invoke("get_Chars", (index))?;
        Ok(__cordl_ret)
    }
    pub fn get_Length(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Length", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_MaxCapacity(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_MaxCapacity", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_RemainingCurrentChunk(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Span_1<char>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Span_1<char> = __cordl_object
            .invoke("get_RemainingCurrentChunk", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Capacity(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Capacity", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Chars(
        &mut self,
        index: i32,
        value: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Chars", (index, value))?;
        Ok(__cordl_ret)
    }
    pub fn set_Length(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Length", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Text+StringBuilder")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Text::StringBuilder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
