#[cfg(feature = "System+Xml+Xsl+Runtime+StringConcat")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct StringConcat {
    pub s1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub s2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub s3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub s4: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub delimiter: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub strList: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
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
    pub fn Clear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Clear",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ConcatNoDelimiter(
        &mut self,
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ConcatNoDelimiter",
            (s),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetResult(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "GetResult", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Count",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
