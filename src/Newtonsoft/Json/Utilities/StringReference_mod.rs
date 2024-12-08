#[cfg(feature = "Newtonsoft+Json+Utilities+StringReference")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct StringReference {
    pub _chars: *mut quest_hook::libil2cpp::Il2CppArray<char>,
    pub _startIndex: i32,
    pub _length: i32,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+StringReference")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::StringReference =>
    "Newtonsoft.Json.Utilities"."StringReference"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+StringReference")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Newtonsoft::Json::Utilities::StringReference {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+StringReference")]
impl crate::Newtonsoft::Json::Utilities::StringReference {
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        chars: *mut quest_hook::libil2cpp::Il2CppArray<char>,
        startIndex: i32,
        length: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (chars, startIndex, length),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Chars(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppArray<char>> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<char> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Chars",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Item(&mut self, i: i32) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Item",
            (i),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Length(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Length",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_StartIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_StartIndex",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
