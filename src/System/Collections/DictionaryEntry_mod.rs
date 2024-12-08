#[cfg(feature = "System+Collections+DictionaryEntry")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct DictionaryEntry {
    pub _key: *mut crate::System::Object,
    pub _value: *mut crate::System::Object,
}
#[cfg(feature = "System+Collections+DictionaryEntry")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Collections::DictionaryEntry =>
    "System.Collections"."DictionaryEntry"
);
#[cfg(feature = "System+Collections+DictionaryEntry")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Collections::DictionaryEntry {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Collections+DictionaryEntry")]
impl crate::System::Collections::DictionaryEntry {
    pub fn _ctor(
        &mut self,
        key: *mut crate::System::Object,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (key, value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Key(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_ret: *mut crate::System::Object = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Key",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_ret: *mut crate::System::Object = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Value",
            (),
        )?;
        Ok(__cordl_ret)
    }
}