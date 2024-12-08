#[cfg(feature = "System+Xml+Xsl+Runtime+StringConcat")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct StringConcat {
    pub s1: *mut crate::System::String,
    pub s2: *mut crate::System::String,
    pub s3: *mut crate::System::String,
    pub s4: *mut crate::System::String,
    pub delimiter: *mut crate::System::String,
    pub strList: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::String,
    >,
    pub idxStr: i32,
}
#[cfg(feature = "System+Xml+Xsl+Runtime+StringConcat")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::Xsl::Runtime::StringConcat =>
    "System.Xml.Xsl.Runtime"."StringConcat"
);
#[cfg(feature = "System+Xml+Xsl+Runtime+StringConcat")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Xml::Xsl::Runtime::StringConcat {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Xml+Xsl+Runtime+StringConcat")]
impl crate::System::Xml::Xsl::Runtime::StringConcat {
    pub fn ConcatNoDelimiter(
        &mut self,
        s: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ConcatNoDelimiter",
            (s),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetResult",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Clear",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Count",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
