#[cfg(feature = "System+Xml+LineInfo")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct LineInfo {
    pub lineNo: i32,
    pub linePos: i32,
}
#[cfg(feature = "System+Xml+LineInfo")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::LineInfo => "System.Xml"."LineInfo"
);
#[cfg(feature = "System+Xml+LineInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::Xml::LineInfo {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Xml+LineInfo")]
impl crate::System::Xml::LineInfo {
    pub fn Set(
        &mut self,
        lineNo: i32,
        linePos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Set",
            (lineNo, linePos),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        lineNo: i32,
        linePos: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (lineNo, linePos),
        )?;
        Ok(__cordl_ret)
    }
}
