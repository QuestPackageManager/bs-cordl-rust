#[cfg(feature = "System+Xml+XmlCharType")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct XmlCharType {
    pub charProperties: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<u8>,
    >,
}
#[cfg(feature = "System+Xml+XmlCharType")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Xml::XmlCharType {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Xml";
    const CLASS_NAME: &'static str = "XmlCharType";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
unsafe impl quest_hook::libil2cpp::Argument for crate::System::Xml::XmlCharType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter for crate::System::Xml::XmlCharType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
unsafe impl quest_hook::libil2cpp::Returned for crate::System::Xml::XmlCharType {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
unsafe impl quest_hook::libil2cpp::Return for crate::System::Xml::XmlCharType {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
    pub fn CombineSurrogateChar(
        lowChar: i32,
        highChar: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CombineSurrogateChar", (lowChar, highChar))?;
        Ok(__cordl_ret.into())
    }
    pub fn InRange(
        value: i32,
        start: i32,
        end: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InRange", (value, start, end))?;
        Ok(__cordl_ret.into())
    }
    pub fn InitInstance() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InitInstance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsCharData(&mut self, ch: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsCharData",
            (ch),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsDigit(ch: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsDigit", (ch))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsHighSurrogate(ch: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsHighSurrogate", (ch))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsLetter(&mut self, ch: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsLetter",
            (ch),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsLowSurrogate(ch: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsLowSurrogate", (ch))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn IsNameCharXml4e(&mut self, ch: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsNameCharXml4e",
            (ch),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNameSingleChar(&mut self, ch: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsNameSingleChar",
            (ch),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsOnlyCharData(
        &mut self,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsOnlyCharData",
            (str),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsOnlyDigits(
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        startPos: i32,
        len: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsOnlyDigits", (str, startPos, len))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsOnlyWhitespace(
        &mut self,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsOnlyWhitespace",
            (str),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsOnlyWhitespaceWithPos(
        &mut self,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsOnlyWhitespaceWithPos",
            (str),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPubidChar(&mut self, ch: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsPubidChar",
            (ch),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPublicId(
        &mut self,
        str: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsPublicId",
            (str),
        )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn IsSurrogate(ch: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSurrogate", (ch))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsTextChar(&mut self, ch: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsTextChar",
            (ch),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IsWhiteSpace(&mut self, ch: char) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IsWhiteSpace",
            (ch),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetProperties(
        chProps: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        ranges: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        value: u8,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetProperties", (chProps, ranges, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SplitSurrogateChar(
        combinedChar: i32,
        lowChar: quest_hook::libil2cpp::ByRefMut<char>,
        highChar: quest_hook::libil2cpp::ByRefMut<char>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SplitSurrogateChar", (combinedChar, lowChar, highChar))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        charProperties: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (charProperties),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Instance() -> quest_hook::libil2cpp::Result<
        crate::System::Xml::XmlCharType,
    > {
        let __cordl_ret: crate::System::Xml::XmlCharType = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Instance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_StaticLock() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_StaticLock", ())?;
        Ok(__cordl_ret.into())
    }
}
