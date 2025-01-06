#[cfg(feature = "Newtonsoft+Json+Utilities+StringBuffer")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct StringBuffer {
    pub _buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    pub _position: i32,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+StringBuffer")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::StringBuffer =>
    "Newtonsoft.Json.Utilities"."StringBuffer"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+StringBuffer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Newtonsoft::Json::Utilities::StringBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+StringBuffer")]
impl crate::Newtonsoft::Json::Utilities::StringBuffer {
    pub fn Append_Il2CppArray_i32_i32_1(
        &mut self,
        bufferPool: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::IArrayPool_1<char>,
        >,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        startIndex: i32,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Append",
            (bufferPool, buffer, startIndex, count),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Append__cordl_char0(
        &mut self,
        bufferPool: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::IArrayPool_1<char>,
        >,
        value: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Append",
            (bufferPool, value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Clear(
        &mut self,
        bufferPool: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::IArrayPool_1<char>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Clear",
            (bufferPool),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureSize(
        &mut self,
        bufferPool: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::IArrayPool_1<char>,
        >,
        appendLength: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "EnsureSize",
            (bufferPool, appendLength),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_i32_i32_1(
        &mut self,
        start: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (start, length),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IArrayPool_1_i32_0(
        &mut self,
        bufferPool: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::IArrayPool_1<char>,
        >,
        initalSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (bufferPool, initalSize),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray1(
        &mut self,
        buffer: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (buffer),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InternalBuffer(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<char>,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_InternalBuffer", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsEmpty",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Position(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Position",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Position(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Position",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
