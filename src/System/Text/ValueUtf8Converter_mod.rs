#[cfg(feature = "System+Text+ValueUtf8Converter")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ValueUtf8Converter {
    pub _arrayToReturnToPool: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    pub _bytes: crate::System::Span_1<u8>,
}
#[cfg(feature = "System+Text+ValueUtf8Converter")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Text::ValueUtf8Converter =>
    "System.Text"."ValueUtf8Converter"
);
#[cfg(feature = "System+Text+ValueUtf8Converter")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Text::ValueUtf8Converter {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Text+ValueUtf8Converter")]
impl crate::System::Text::ValueUtf8Converter {
    pub fn _ctor(
        &mut self,
        initialBuffer: crate::System::Span_1<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (initialBuffer),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ConvertAndTerminateString(
        &mut self,
        value: crate::System::ReadOnlySpan_1<char>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Span_1<u8>> {
        let __cordl_ret: crate::System::Span_1<u8> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ConvertAndTerminateString",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
