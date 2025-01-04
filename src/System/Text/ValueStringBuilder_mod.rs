#[cfg(feature = "System+Text+ValueStringBuilder")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct ValueStringBuilder {
    pub _arrayToReturnToPool: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<char>,
    >,
    pub _chars: crate::System::Span_1<char>,
    pub _pos: i32,
}
#[cfg(feature = "System+Text+ValueStringBuilder")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Text::ValueStringBuilder =>
    "System.Text"."ValueStringBuilder"
);
#[cfg(feature = "System+Text+ValueStringBuilder")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Text::ValueStringBuilder {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Text+ValueStringBuilder")]
impl crate::System::Text::ValueStringBuilder {
    pub fn AppendSlow(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AppendSlow",
            (s),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn AppendSpan(
        &mut self,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Span_1<char>> {
        let __cordl_ret: crate::System::Span_1<char> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "AppendSpan",
            (length),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Append_Il2CppObject_i32_3(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Append",
            (value, length),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Append_Il2CppString1(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Append",
            (s),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Append__cordl_char0(
        &mut self,
        c: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Append",
            (c),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Append__cordl_char_i32_2(
        &mut self,
        c: char,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Append",
            (c, count),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Grow(
        &mut self,
        requiredAdditionalCapacity: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Grow",
            (requiredAdditionalCapacity),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GrowAndAppend(
        &mut self,
        c: char,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GrowAndAppend",
            (c),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TryCopyTo(
        &mut self,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryCopyTo",
            (destination, charsWritten),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        initialBuffer: crate::System::Span_1<char>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (initialBuffer),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::ByRefMut<char>> {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<char> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Item",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Length(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Length",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
