#[cfg(feature = "System+Enum")]
#[repr(C)]
#[derive(Debug)]
pub struct Enum {
    __cordl_parent: crate::System::ValueType,
}
#[cfg(feature = "System+Enum")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Enum => "System"."Enum"
);
#[cfg(feature = "System+Enum")]
impl std::ops::Deref for crate::System::Enum {
    type Target = crate::System::ValueType;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Enum")]
impl std::ops::DerefMut for crate::System::Enum {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Enum")]
impl crate::System::Enum {
    pub const enumSeperator: &'static str = ", ";
    #[cfg(feature = "System+Enum+EnumResult")]
    pub type EnumResult = crate::System::Enum_EnumResult;
    #[cfg(feature = "System+Enum+ParseFailureKind")]
    pub type ParseFailureKind = crate::GlobalNamespace::Enum_ParseFailureKind;
    #[cfg(feature = "System+Enum+ValuesAndNames")]
    pub type ValuesAndNames = crate::System::Enum_ValuesAndNames;
    pub fn CompareTo(
        &mut self,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CompareTo", (target))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTypeCode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::TypeCode> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::TypeCode = __cordl_object
            .invoke("GetTypeCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetValue(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("GetValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HasFlag(
        &mut self,
        flag: quest_hook::libil2cpp::Gc<crate::System::Enum>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasFlag", (flag))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalHasFlag(
        &mut self,
        flags: quest_hook::libil2cpp::Gc<crate::System::Enum>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("InternalHasFlag", (flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn System_IConvertible_ToBoolean(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.IConvertible.ToBoolean", (provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToByte(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u8 = __cordl_object
            .invoke("System.IConvertible.ToByte", (provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToChar(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: char = __cordl_object
            .invoke("System.IConvertible.ToChar", (provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToDateTime(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::DateTime = __cordl_object
            .invoke("System.IConvertible.ToDateTime", (provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToDecimal(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Decimal = __cordl_object
            .invoke("System.IConvertible.ToDecimal", (provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToDouble(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object
            .invoke("System.IConvertible.ToDouble", (provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToInt16(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i16 = __cordl_object
            .invoke("System.IConvertible.ToInt16", (provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToInt32(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("System.IConvertible.ToInt32", (provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToInt64(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i64 = __cordl_object
            .invoke("System.IConvertible.ToInt64", (provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToSByte(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i8 = __cordl_object
            .invoke("System.IConvertible.ToSByte", (provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToSingle(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("System.IConvertible.ToSingle", (provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToType(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object
            .invoke("System.IConvertible.ToType", (_cordl_type, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToUInt16(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u16 = __cordl_object
            .invoke("System.IConvertible.ToUInt16", (provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToUInt32(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object
            .invoke("System.IConvertible.ToUInt32", (provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToUInt64(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u64 = __cordl_object
            .invoke("System.IConvertible.ToUInt64", (provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_IFormatProvider3(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", (provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Il2CppString2(
        &mut self,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", (format))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Il2CppString_IFormatProvider1(
        &mut self,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", (format, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hashcode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_hashcode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_value(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_value", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Enum")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Enum {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Enum")]
impl AsRef<crate::System::IComparable> for crate::System::Enum {
    fn as_ref(&self) -> &crate::System::IComparable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Enum")]
impl AsMut<crate::System::IComparable> for crate::System::Enum {
    fn as_mut(&mut self) -> &mut crate::System::IComparable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Enum")]
impl AsRef<crate::System::IConvertible> for crate::System::Enum {
    fn as_ref(&self) -> &crate::System::IConvertible {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Enum")]
impl AsMut<crate::System::IConvertible> for crate::System::Enum {
    fn as_mut(&mut self) -> &mut crate::System::IConvertible {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Enum")]
impl AsRef<crate::System::IFormattable> for crate::System::Enum {
    fn as_ref(&self) -> &crate::System::IFormattable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Enum")]
impl AsMut<crate::System::IFormattable> for crate::System::Enum {
    fn as_mut(&mut self) -> &mut crate::System::IFormattable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Enum+EnumResult")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Enum_EnumResult {
    pub parsedEnum: *mut quest_hook::libil2cpp::Il2CppObject,
    pub canThrow: bool,
    pub m_failure: crate::GlobalNamespace::Enum_ParseFailureKind,
    pub m_failureMessageID: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_failureParameter: *mut quest_hook::libil2cpp::Il2CppString,
    pub m_failureMessageFormatArgument: *mut quest_hook::libil2cpp::Il2CppObject,
    pub m_innerException: *mut crate::System::Exception,
}
#[cfg(feature = "System+Enum+EnumResult")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Enum_EnumResult => "System"
    ."Enum/EnumResult"
);
#[cfg(feature = "System+Enum+EnumResult")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::Enum_EnumResult {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Enum+EnumResult")]
impl crate::System::Enum_EnumResult {
    pub fn GetEnumParseException(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Exception>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Exception> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetEnumParseException",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Init(
        &mut self,
        canMethodThrow: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Init",
            (canMethodThrow),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetFailure_Enum_ParseFailureKind_Il2CppString1(
        &mut self,
        failure: crate::GlobalNamespace::Enum_ParseFailureKind,
        failureParameter: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetFailure",
            (failure, failureParameter),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetFailure_Enum_ParseFailureKind_Il2CppString_Il2CppObject2(
        &mut self,
        failure: crate::GlobalNamespace::Enum_ParseFailureKind,
        failureMessageID: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        failureMessageFormatArgument: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetFailure",
            (failure, failureMessageID, failureMessageFormatArgument),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetFailure_Exception0(
        &mut self,
        unhandledException: quest_hook::libil2cpp::Gc<crate::System::Exception>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetFailure",
            (unhandledException),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Enum+ValuesAndNames")]
#[repr(C)]
#[derive(Debug)]
pub struct Enum_ValuesAndNames {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Values: *mut quest_hook::libil2cpp::Il2CppArray<u64>,
    pub Names: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut quest_hook::libil2cpp::Il2CppString,
    >,
}
#[cfg(feature = "System+Enum+ValuesAndNames")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Enum_ValuesAndNames => "System"
    ."Enum/ValuesAndNames"
);
#[cfg(feature = "System+Enum+ValuesAndNames")]
impl std::ops::Deref for crate::System::Enum_ValuesAndNames {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Enum+ValuesAndNames")]
impl std::ops::DerefMut for crate::System::Enum_ValuesAndNames {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Enum+ValuesAndNames")]
impl crate::System::Enum_ValuesAndNames {
    pub fn New(
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        names: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (values, names))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u64>>,
        names: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut quest_hook::libil2cpp::Il2CppString>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (values, names))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Enum+ValuesAndNames")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Enum_ValuesAndNames {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
