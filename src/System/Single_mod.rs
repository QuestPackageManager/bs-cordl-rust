#[cfg(feature = "System+Single")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Single {
    pub m_value: f32,
}
#[cfg(feature = "System+Single")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Single {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System";
    const CLASS_NAME: &'static str = "Single";
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
#[cfg(feature = "System+Single")]
unsafe impl quest_hook::libil2cpp::Argument for crate::System::Single {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "System+Single")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::System::Single {
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
#[cfg(feature = "System+Single")]
unsafe impl quest_hook::libil2cpp::Returned for crate::System::Single {
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
#[cfg(feature = "System+Single")]
unsafe impl quest_hook::libil2cpp::Return for crate::System::Single {
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
    pub fn CompareTo_Il2CppObject0(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                i32,
                1usize,
            >("CompareTo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompareTo", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo_f32_1(&mut self, value: f32) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f32), i32, 1usize>("CompareTo")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CompareTo", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (value)) };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject0(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                bool,
                1usize,
            >("Equals")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Equals", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn Equals_f32_1(&mut self, obj: f32) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f32), bool, 1usize>("Equals")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Equals", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (obj)) };
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("GetHashCode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetHashCode", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn GetTypeCode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::System::TypeCode> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::System::TypeCode, 0usize>("GetTypeCode")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetTypeCode", 0usize
                )
            });
        let __cordl_ret: crate::System::TypeCode = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsFinite(f: f32) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), bool, 1usize>("IsFinite")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsFinite", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (f)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsInfinity(f: f32) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), bool, 1usize>("IsInfinity")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsInfinity", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (f)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsNaN(f: f32) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), bool, 1usize>("IsNaN")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsNaN", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (f)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsNegativeInfinity(f: f32) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), bool, 1usize>("IsNegativeInfinity")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsNegativeInfinity", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (f)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsPositiveInfinity(f: f32) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32), bool, 1usize>("IsPositiveInfinity")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsPositiveInfinity", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (f)) };
        Ok(__cordl_ret.into())
    }
    pub fn Parse_IFormatProvider1(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                f32,
                2usize,
            >("Parse")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Parse", 2usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (s, provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn Parse_Il2CppString0(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                f32,
                1usize,
            >("Parse")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Parse", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (s)) };
        Ok(__cordl_ret.into())
    }
    pub fn Parse_NumberStyles_IFormatProvider2(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        style: crate::System::Globalization::NumberStyles,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::System::Globalization::NumberStyles,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                f32,
                3usize,
            >("Parse")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Parse", 3usize
                )
            });
        let __cordl_ret: f32 = unsafe {
            method.invoke_unchecked((), (s, style, provider))
        };
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToBoolean(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>),
                bool,
                1usize,
            >("System.IConvertible.ToBoolean")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.IConvertible.ToBoolean", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, (provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToByte(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<u8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>),
                u8,
                1usize,
            >("System.IConvertible.ToByte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.IConvertible.ToByte", 1usize
                )
            });
        let __cordl_ret: u8 = unsafe { method.invoke_unchecked(self, (provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToChar(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<char> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>),
                char,
                1usize,
            >("System.IConvertible.ToChar")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.IConvertible.ToChar", 1usize
                )
            });
        let __cordl_ret: char = unsafe { method.invoke_unchecked(self, (provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToDateTime(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<crate::System::DateTime> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>),
                crate::System::DateTime,
                1usize,
            >("System.IConvertible.ToDateTime")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.IConvertible.ToDateTime", 1usize
                )
            });
        let __cordl_ret: crate::System::DateTime = unsafe {
            method.invoke_unchecked(self, (provider))
        };
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToDecimal(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>),
                crate::System::Decimal,
                1usize,
            >("System.IConvertible.ToDecimal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.IConvertible.ToDecimal", 1usize
                )
            });
        let __cordl_ret: crate::System::Decimal = unsafe {
            method.invoke_unchecked(self, (provider))
        };
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToDouble(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>),
                f64,
                1usize,
            >("System.IConvertible.ToDouble")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.IConvertible.ToDouble", 1usize
                )
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked(self, (provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToInt16(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<i16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>),
                i16,
                1usize,
            >("System.IConvertible.ToInt16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.IConvertible.ToInt16", 1usize
                )
            });
        let __cordl_ret: i16 = unsafe { method.invoke_unchecked(self, (provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToInt32(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>),
                i32,
                1usize,
            >("System.IConvertible.ToInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.IConvertible.ToInt32", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, (provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToInt64(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>),
                i64,
                1usize,
            >("System.IConvertible.ToInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.IConvertible.ToInt64", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked(self, (provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToSByte(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<i8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>),
                i8,
                1usize,
            >("System.IConvertible.ToSByte")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.IConvertible.ToSByte", 1usize
                )
            });
        let __cordl_ret: i8 = unsafe { method.invoke_unchecked(self, (provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToSingle(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>),
                f32,
                1usize,
            >("System.IConvertible.ToSingle")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.IConvertible.ToSingle", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, (provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToType(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Type>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                2usize,
            >("System.IConvertible.ToType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.IConvertible.ToType", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = unsafe { method.invoke_unchecked(self, (_cordl_type, provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToUInt16(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<u16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>),
                u16,
                1usize,
            >("System.IConvertible.ToUInt16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.IConvertible.ToUInt16", 1usize
                )
            });
        let __cordl_ret: u16 = unsafe { method.invoke_unchecked(self, (provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToUInt32(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>),
                u32,
                1usize,
            >("System.IConvertible.ToUInt32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.IConvertible.ToUInt32", 1usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked(self, (provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn System_IConvertible_ToUInt64(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>),
                u64,
                1usize,
            >("System.IConvertible.ToUInt64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "System.IConvertible.ToUInt64", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked(self, (provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                0usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_IFormatProvider1(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Il2CppString2(
        &mut self,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                1usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (format)) };
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Il2CppString_IFormatProvider3(
        &mut self,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                2usize,
            >("ToString")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToString", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = unsafe { method.invoke_unchecked(self, (format, provider)) };
        Ok(__cordl_ret.into())
    }
    pub fn TryFormat(
        &mut self,
        destination: crate::System::Span_1<char>,
        charsWritten: quest_hook::libil2cpp::ByRefMut<i32>,
        format: crate::System::ReadOnlySpan_1<char>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::System::Span_1<char>,
                    quest_hook::libil2cpp::ByRefMut<i32>,
                    crate::System::ReadOnlySpan_1<char>,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                ),
                bool,
                4usize,
            >("TryFormat")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryFormat", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (destination, charsWritten, format, provider))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryParse_Il2CppString_ByRefMut0(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        result: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::ByRefMut<f32>,
                ),
                bool,
                2usize,
            >("TryParse")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryParse", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (s, result)) };
        Ok(__cordl_ret.into())
    }
    pub fn TryParse_Il2CppString_NumberStyles_IFormatProvider_ByRefMut2(
        s: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        style: crate::System::Globalization::NumberStyles,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
        result: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    crate::System::Globalization::NumberStyles,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                    quest_hook::libil2cpp::ByRefMut<f32>,
                ),
                bool,
                4usize,
            >("TryParse")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryParse", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (s, style, provider, result))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryParse_ReadOnlySpan_1_ByRefMut1(
        s: crate::System::ReadOnlySpan_1<char>,
        result: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::ReadOnlySpan_1<char>,
                    quest_hook::libil2cpp::ByRefMut<f32>,
                ),
                bool,
                2usize,
            >("TryParse")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryParse", 2usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (s, result)) };
        Ok(__cordl_ret.into())
    }
    pub fn TryParse_ReadOnlySpan_1_NumberStyles_IFormatProvider_ByRefMut3(
        s: crate::System::ReadOnlySpan_1<char>,
        style: crate::System::Globalization::NumberStyles,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
        result: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::ReadOnlySpan_1<char>,
                    crate::System::Globalization::NumberStyles,
                    quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
                    quest_hook::libil2cpp::ByRefMut<f32>,
                ),
                bool,
                4usize,
            >("TryParse")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryParse", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (s, style, provider, result))
        };
        Ok(__cordl_ret.into())
    }
    pub fn TryParse_ReadOnlySpan_1_NumberStyles_NumberFormatInfo_ByRefMut4(
        s: crate::System::ReadOnlySpan_1<char>,
        style: crate::System::Globalization::NumberStyles,
        info: quest_hook::libil2cpp::Gc<crate::System::Globalization::NumberFormatInfo>,
        result: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::ReadOnlySpan_1<char>,
                    crate::System::Globalization::NumberStyles,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Globalization::NumberFormatInfo,
                    >,
                    quest_hook::libil2cpp::ByRefMut<f32>,
                ),
                bool,
                4usize,
            >("TryParse")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TryParse", 4usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (s, style, info, result))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Single")]
impl AsRef<crate::System::IComparable> for crate::System::Single {
    fn as_ref(&self) -> &crate::System::IComparable {
        todo!()
    }
}
#[cfg(feature = "System+Single")]
impl AsMut<crate::System::IComparable> for crate::System::Single {
    fn as_mut(&mut self) -> &mut crate::System::IComparable {
        todo!()
    }
}
#[cfg(feature = "System+Single")]
impl AsRef<crate::System::IComparable_1<f32>> for crate::System::Single {
    fn as_ref(&self) -> &crate::System::IComparable_1<f32> {
        todo!()
    }
}
#[cfg(feature = "System+Single")]
impl AsMut<crate::System::IComparable_1<f32>> for crate::System::Single {
    fn as_mut(&mut self) -> &mut crate::System::IComparable_1<f32> {
        todo!()
    }
}
#[cfg(feature = "System+Single")]
impl AsRef<crate::System::IConvertible> for crate::System::Single {
    fn as_ref(&self) -> &crate::System::IConvertible {
        todo!()
    }
}
#[cfg(feature = "System+Single")]
impl AsMut<crate::System::IConvertible> for crate::System::Single {
    fn as_mut(&mut self) -> &mut crate::System::IConvertible {
        todo!()
    }
}
#[cfg(feature = "System+Single")]
impl AsRef<crate::System::IEquatable_1<f32>> for crate::System::Single {
    fn as_ref(&self) -> &crate::System::IEquatable_1<f32> {
        todo!()
    }
}
#[cfg(feature = "System+Single")]
impl AsMut<crate::System::IEquatable_1<f32>> for crate::System::Single {
    fn as_mut(&mut self) -> &mut crate::System::IEquatable_1<f32> {
        todo!()
    }
}
#[cfg(feature = "System+Single")]
impl AsRef<crate::System::IFormattable> for crate::System::Single {
    fn as_ref(&self) -> &crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "System+Single")]
impl AsMut<crate::System::IFormattable> for crate::System::Single {
    fn as_mut(&mut self) -> &mut crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "System+Single")]
impl AsRef<crate::System::ISpanFormattable> for crate::System::Single {
    fn as_ref(&self) -> &crate::System::ISpanFormattable {
        todo!()
    }
}
#[cfg(feature = "System+Single")]
impl AsMut<crate::System::ISpanFormattable> for crate::System::Single {
    fn as_mut(&mut self) -> &mut crate::System::ISpanFormattable {
        todo!()
    }
}
