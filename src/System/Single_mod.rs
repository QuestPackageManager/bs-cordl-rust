#[cfg(feature = "System+Single")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Single {
    pub m_value: f32,
}
#[cfg(feature = "System+Single")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Single => "System"."Single"
);
#[cfg(feature = "System+Single")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::Single {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Single")]
impl crate::System::Single {
    pub const Epsilon: f32 = 0.000000000000000000000000000000000000000000001f32;
    pub const MaxValue: f32 = 340282350000000000000000000000000000000f32;
    pub const MinValue: f32 = -340282350000000000000000000000000000000f32;
    pub const NaN: f32 = std::f64::NAN;
    pub const NegativeInfinity: f32 = std::f32::NEG_INFINITY;
    pub const NegativeZero: f32 = -0f32;
    pub const PositiveInfinity: f32 = std::f32::INFINITY;
    pub fn CompareTo_Object0(
        &mut self,
        value: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompareTo",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn CompareTo_f32_1(&mut self, value: f32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompareTo",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Object0(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_f32_1(&mut self, obj: f32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetTypeCode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::TypeCode> {
        let __cordl_ret: crate::System::TypeCode = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetTypeCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_IConvertible_ToBoolean(
        &mut self,
        provider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToBoolean",
            (provider),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_IConvertible_ToByte(
        &mut self,
        provider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToByte",
            (provider),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_IConvertible_ToChar(
        &mut self,
        provider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<char> {
        let __cordl_ret: char = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToChar",
            (provider),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_IConvertible_ToDateTime(
        &mut self,
        provider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        let __cordl_ret: crate::System::DateTime = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToDateTime",
            (provider),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_IConvertible_ToDecimal(
        &mut self,
        provider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToDecimal",
            (provider),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_IConvertible_ToDouble(
        &mut self,
        provider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToDouble",
            (provider),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_IConvertible_ToInt16(
        &mut self,
        provider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToInt16",
            (provider),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_IConvertible_ToInt32(
        &mut self,
        provider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToInt32",
            (provider),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_IConvertible_ToInt64(
        &mut self,
        provider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToInt64",
            (provider),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_IConvertible_ToSByte(
        &mut self,
        provider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToSByte",
            (provider),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_IConvertible_ToSingle(
        &mut self,
        provider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToSingle",
            (provider),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_IConvertible_ToType(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        provider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_ret: *mut crate::System::Object = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToType",
            (_cordl_type, provider),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_IConvertible_ToUInt16(
        &mut self,
        provider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToUInt16",
            (provider),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_IConvertible_ToUInt32(
        &mut self,
        provider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToUInt32",
            (provider),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_IConvertible_ToUInt64(
        &mut self,
        provider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.IConvertible.ToUInt64",
            (provider),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString_IFormatProvider1(
        &mut self,
        provider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (provider),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString_String2(
        &mut self,
        format: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (format),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString_String_IFormatProvider3(
        &mut self,
        format: *mut crate::System::String,
        provider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (format, provider),
        )?;
        Ok(__cordl_ret)
    }
    pub fn TryFormat(
        &mut self,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
        format: crate::System::ReadOnlySpan_1<char>,
        provider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryFormat",
            (destination, charsWritten, format, provider),
        )?;
        Ok(__cordl_ret)
    }
}
