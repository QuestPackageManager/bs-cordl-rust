#[cfg(feature = "System+DTSubString")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DTSubString {
    pub s: crate::System::ReadOnlySpan_1<char>,
    pub index: i32,
    pub length: i32,
    pub _cordl_type: crate::System::DTSubStringType,
    pub value: i32,
}
#[cfg(feature = "System+DTSubString")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::DTSubString => "System"."DTSubString"
);
#[cfg(feature = "System+DTSubString")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::DTSubString {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+DTSubString")]
impl crate::System::DTSubString {
    pub fn get_Item(
        &mut self,
        relativeIndex: i32,
    ) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Item",
            (relativeIndex),
        )?;
        Ok(__cordl_ret.into())
    }
}
