#[cfg(feature = "System+Xml+BinXmlSqlDecimal")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct BinXmlSqlDecimal {
    pub m_bLen: u8,
    pub m_bPrec: u8,
    pub m_bScale: u8,
    pub m_bSign: u8,
    pub m_data1: u32,
    pub m_data2: u32,
    pub m_data3: u32,
    pub m_data4: u32,
}
#[cfg(feature = "System+Xml+BinXmlSqlDecimal")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::BinXmlSqlDecimal => "System.Xml"
    ."BinXmlSqlDecimal"
);
#[cfg(feature = "System+Xml+BinXmlSqlDecimal")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Xml::BinXmlSqlDecimal {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Xml+BinXmlSqlDecimal")]
impl crate::System::Xml::BinXmlSqlDecimal {
    pub fn ToDecimal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToDecimal",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn TrimTrailingZeros(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TrimTrailingZeros",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        data: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
        offset: i32,
        trim: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (data, offset, trim),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_IsPositive(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsPositive",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
