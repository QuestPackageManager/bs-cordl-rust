#[cfg(feature = "System+Xml+XmlCharType")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct XmlCharType {
    pub charProperties: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
}
#[cfg(feature = "System+Xml+XmlCharType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Xml::XmlCharType => "System.Xml"
    ."XmlCharType"
);
#[cfg(feature = "System+Xml+XmlCharType")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::Xml::XmlCharType {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Xml+XmlCharType")]
impl crate::System::Xml::XmlCharType {
    pub fn IsTextChar(&mut self, ch: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsTextChar",
            (ch),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IsOnlyWhitespaceWithPos(
        &mut self,
        str: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsOnlyWhitespaceWithPos",
            (str),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IsStartNCNameCharXml4e(
        &mut self,
        ch: char,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsStartNCNameCharXml4e",
            (ch),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IsNCNameSingleChar(
        &mut self,
        ch: char,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsNCNameSingleChar",
            (ch),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IsStartNCNameSingleChar(
        &mut self,
        ch: char,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsStartNCNameSingleChar",
            (ch),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IsNameSingleChar(&mut self, ch: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsNameSingleChar",
            (ch),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IsNCNameCharXml4e(
        &mut self,
        ch: char,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsNCNameCharXml4e",
            (ch),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IsWhiteSpace(&mut self, ch: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsWhiteSpace",
            (ch),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IsOnlyCharData(
        &mut self,
        str: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsOnlyCharData",
            (str),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IsPublicId(
        &mut self,
        str: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsPublicId",
            (str),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IsPubidChar(&mut self, ch: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsPubidChar",
            (ch),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IsNameCharXml4e(&mut self, ch: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsNameCharXml4e",
            (ch),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IsOnlyWhitespace(
        &mut self,
        str: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsOnlyWhitespace",
            (str),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IsCharData(&mut self, ch: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsCharData",
            (ch),
        )?;
        Ok(__cordl_ret)
    }
    pub fn IsLetter(&mut self, ch: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsLetter",
            (ch),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        charProperties: *mut quest_hook::libil2cpp::Il2CppArray<u8>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (charProperties),
        )?;
        Ok(__cordl_ret)
    }
}
