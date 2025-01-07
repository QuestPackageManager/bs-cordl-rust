#[cfg(feature = "System+Xml+Schema+XsdDateTime")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct XsdDateTime {
    pub dt: crate::System::DateTime,
    pub extra: u32,
}
#[cfg(feature = "System+Xml+Schema+XsdDateTime")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Xml::Schema::XsdDateTime {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Xml.Schema";
    const CLASS_NAME: &'static str = "XsdDateTime";
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
unsafe impl quest_hook::libil2cpp::Argument for crate::System::Xml::Schema::XsdDateTime {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Xml::Schema::XsdDateTime {
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
unsafe impl quest_hook::libil2cpp::Returned for crate::System::Xml::Schema::XsdDateTime {
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
unsafe impl quest_hook::libil2cpp::Return for crate::System::Xml::Schema::XsdDateTime {
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
#[cfg(feature = "System+Xml+Schema+XsdDateTime")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Xml::Schema::XsdDateTime {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Xml+Schema+XsdDateTime")]
impl crate::System::Xml::Schema::XsdDateTime {
    #[cfg(feature = "System+Xml+Schema+XsdDateTime+DateTimeTypeCode")]
    pub type DateTimeTypeCode = crate::System::Xml::Schema::XsdDateTime_DateTimeTypeCode;
    #[cfg(feature = "System+Xml+Schema+XsdDateTime+Parser")]
    pub type Parser = crate::System::Xml::Schema::XsdDateTime_Parser;
    #[cfg(feature = "System+Xml+Schema+XsdDateTime+XsdDateTimeKind")]
    pub type XsdDateTimeKind = crate::System::Xml::Schema::XsdDateTime_XsdDateTimeKind;
    pub fn InitiateXsdDateTime(
        &mut self,
        parser: crate::System::Xml::Schema::XsdDateTime_Parser,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "InitiateXsdDateTime",
            (parser),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn IntToCharArray(
        &mut self,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        start: i32,
        value: i32,
        digits: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "IntToCharArray",
            (text, start, value, digits),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn PrintDate(
        &mut self,
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "PrintDate",
            (sb),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn PrintTime(
        &mut self,
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "PrintTime",
            (sb),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn PrintZone(
        &mut self,
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "PrintZone",
            (sb),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ShortToCharArray(
        &mut self,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<char>>,
        start: i32,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ShortToCharArray",
            (text, start, value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TryParse(
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        kinds: crate::System::Xml::Schema::XsdDateTimeFlags,
        result: quest_hook::libil2cpp::ByRefMut<crate::System::Xml::Schema::XsdDateTime>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryParse", (text, kinds, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_DateTimeOffset3(
        &mut self,
        dateTimeOffset: crate::System::DateTimeOffset,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (dateTimeOffset),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_DateTimeOffset_XsdDateTimeFlags4(
        &mut self,
        dateTimeOffset: crate::System::DateTimeOffset,
        kinds: crate::System::Xml::Schema::XsdDateTimeFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (dateTimeOffset, kinds),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_DateTime_XsdDateTimeFlags2(
        &mut self,
        dateTime: crate::System::DateTime,
        kinds: crate::System::Xml::Schema::XsdDateTimeFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (dateTime, kinds),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppString_XsdDateTimeFlags0(
        &mut self,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        kinds: crate::System::Xml::Schema::XsdDateTimeFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (text, kinds),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_XsdDateTime_Parser1(
        &mut self,
        parser: crate::System::Xml::Schema::XsdDateTime_Parser,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (parser),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Day(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Day",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Fraction(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Fraction",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Hour(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Hour",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InternalKind(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Xml::Schema::XsdDateTime_XsdDateTimeKind,
    > {
        let __cordl_ret: crate::System::Xml::Schema::XsdDateTime_XsdDateTimeKind = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_InternalKind",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InternalTypeCode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Xml::Schema::XsdDateTime_DateTimeTypeCode,
    > {
        let __cordl_ret: crate::System::Xml::Schema::XsdDateTime_DateTimeTypeCode = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_InternalTypeCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Minute(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Minute",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Month(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Month",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Second(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Second",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Year(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Year",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ZoneHour(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ZoneHour",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ZoneMinute(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_ZoneMinute",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_XsdDateTime0(
        xdt: crate::System::Xml::Schema::XsdDateTime,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (xdt))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_XsdDateTime1(
        xdt: crate::System::Xml::Schema::XsdDateTime,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTimeOffset> {
        let __cordl_ret: crate::System::DateTimeOffset = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (xdt))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+XsdDateTime+DateTimeTypeCode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum XsdDateTime_DateTimeTypeCode {
    #[default]
    Date = 2i32,
    DateTime = 0i32,
    GDay = 6i32,
    GMonth = 7i32,
    GMonthDay = 5i32,
    GYear = 4i32,
    GYearMonth = 3i32,
    Time = 1i32,
    XdrDateTime = 8i32,
}
#[cfg(feature = "System+Xml+Schema+XsdDateTime+DateTimeTypeCode")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Xml::Schema::XsdDateTime_DateTimeTypeCode {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Xml.Schema";
    const CLASS_NAME: &'static str = "DateTimeTypeCode";
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
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Xml::Schema::XsdDateTime_DateTimeTypeCode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Xml::Schema::XsdDateTime_DateTimeTypeCode {
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
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Xml::Schema::XsdDateTime_DateTimeTypeCode {
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
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Xml::Schema::XsdDateTime_DateTimeTypeCode {
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
#[cfg(feature = "System+Xml+Schema+XsdDateTime+Parser")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct XsdDateTime_Parser {
    pub typeCode: crate::System::Xml::Schema::XsdDateTime_DateTimeTypeCode,
    pub year: i32,
    pub month: i32,
    pub day: i32,
    pub hour: i32,
    pub minute: i32,
    pub second: i32,
    pub fraction: i32,
    pub kind: crate::System::Xml::Schema::XsdDateTime_XsdDateTimeKind,
    pub zoneHour: i32,
    pub zoneMinute: i32,
    pub text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub length: i32,
}
#[cfg(feature = "System+Xml+Schema+XsdDateTime+Parser")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Xml::Schema::XsdDateTime_Parser {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Xml.Schema";
    const CLASS_NAME: &'static str = "Parser";
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
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Xml::Schema::XsdDateTime_Parser {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Xml::Schema::XsdDateTime_Parser {
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
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Xml::Schema::XsdDateTime_Parser {
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
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Xml::Schema::XsdDateTime_Parser {
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
#[cfg(feature = "System+Xml+Schema+XsdDateTime+Parser")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::System::Xml::Schema::XsdDateTime_Parser {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Xml+Schema+XsdDateTime+Parser")]
impl crate::System::Xml::Schema::XsdDateTime_Parser {
    pub fn Parse(
        &mut self,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        kinds: crate::System::Xml::Schema::XsdDateTimeFlags,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Parse",
            (text, kinds),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Parse2Dig(
        &mut self,
        start: i32,
        num: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Parse2Dig",
            (start, num),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Parse4Dig(
        &mut self,
        start: i32,
        num: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Parse4Dig",
            (start, num),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseChar(
        &mut self,
        start: i32,
        ch: char,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ParseChar",
            (start, ch),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseDate(&mut self, start: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ParseDate",
            (start),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseTime(
        &mut self,
        start: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ParseTime",
            (start),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseTimeAndWhitespace(
        &mut self,
        start: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ParseTimeAndWhitespace",
            (start),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseTimeAndZoneAndWhitespace(
        &mut self,
        start: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ParseTimeAndZoneAndWhitespace",
            (start),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ParseZoneAndWhitespace(
        &mut self,
        start: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ParseZoneAndWhitespace",
            (start),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Test(
        left: crate::System::Xml::Schema::XsdDateTimeFlags,
        right: crate::System::Xml::Schema::XsdDateTimeFlags,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Test", (left, right))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Xml+Schema+XsdDateTime+XsdDateTimeKind")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum XsdDateTime_XsdDateTimeKind {
    #[default]
    LocalEastOfZulu = 3i32,
    LocalWestOfZulu = 2i32,
    Unspecified = 0i32,
    Zulu = 1i32,
}
#[cfg(feature = "System+Xml+Schema+XsdDateTime+XsdDateTimeKind")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Xml::Schema::XsdDateTime_XsdDateTimeKind {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Xml.Schema";
    const CLASS_NAME: &'static str = "XsdDateTimeKind";
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
unsafe impl quest_hook::libil2cpp::Argument
for crate::System::Xml::Schema::XsdDateTime_XsdDateTimeKind {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Xml::Schema::XsdDateTime_XsdDateTimeKind {
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
unsafe impl quest_hook::libil2cpp::Returned
for crate::System::Xml::Schema::XsdDateTime_XsdDateTimeKind {
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
unsafe impl quest_hook::libil2cpp::Return
for crate::System::Xml::Schema::XsdDateTime_XsdDateTimeKind {
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
