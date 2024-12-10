#[cfg(feature = "Newtonsoft+Json+JsonPosition")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct JsonPosition {
    pub Type: crate::Newtonsoft::Json::JsonContainerType,
    pub Position: i32,
    pub PropertyName: *mut quest_hook::libil2cpp::Il2CppString,
    pub HasIndex: bool,
}
#[cfg(feature = "Newtonsoft+Json+JsonPosition")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::JsonPosition =>
    "Newtonsoft.Json"."JsonPosition"
);
#[cfg(feature = "Newtonsoft+Json+JsonPosition")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Newtonsoft::Json::JsonPosition {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Newtonsoft+Json+JsonPosition")]
impl crate::Newtonsoft::Json::JsonPosition {
    pub fn CalculateLength(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CalculateLength",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteTo(
        &mut self,
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        writer: quest_hook::libil2cpp::ByRefMut<*mut crate::System::IO::StringWriter>,
        buffer: quest_hook::libil2cpp::ByRefMut<
            *mut quest_hook::libil2cpp::Il2CppArray<char>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "WriteTo",
            (sb, writer, buffer),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_type: crate::Newtonsoft::Json::JsonContainerType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (_cordl_type),
        )?;
        Ok(__cordl_ret.into())
    }
}
