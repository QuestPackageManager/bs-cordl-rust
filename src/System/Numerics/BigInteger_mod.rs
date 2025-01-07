#[cfg(feature = "System+Numerics+BigInteger")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BigInteger {
    pub _sign: i32,
    pub _bits: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
}
#[cfg(feature = "System+Numerics+BigInteger")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Numerics::BigInteger {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Numerics";
    const CLASS_NAME: &'static str = "BigInteger";
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
unsafe impl quest_hook::libil2cpp::Argument for crate::System::Numerics::BigInteger {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter for crate::System::Numerics::BigInteger {
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
unsafe impl quest_hook::libil2cpp::Returned for crate::System::Numerics::BigInteger {
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
unsafe impl quest_hook::libil2cpp::Return for crate::System::Numerics::BigInteger {
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
#[cfg(feature = "System+Numerics+BigInteger")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::System::Numerics::BigInteger {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Numerics+BigInteger")]
impl crate::System::Numerics::BigInteger {
    #[cfg(feature = "System+Numerics+BigInteger+GetBytesMode")]
    pub type GetBytesMode = crate::System::Numerics::BigInteger_GetBytesMode;
    pub fn Add(
        leftBits: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        leftSign: i32,
        rightBits: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        rightSign: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Numerics::BigInteger> {
        let __cordl_ret: crate::System::Numerics::BigInteger = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Add", (leftBits, leftSign, rightBits, rightSign))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo_BigInteger1(
        &mut self,
        other: crate::System::Numerics::BigInteger,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompareTo",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo_Il2CppObject2(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompareTo",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn CompareTo_i64_0(&mut self, other: i64) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "CompareTo",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_BigInteger2(
        &mut self,
        other: crate::System::Numerics::BigInteger,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject0(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_i64_1(&mut self, other: i64) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDiffLength(
        rgu1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        rgu2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        cu: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDiffLength", (rgu1, rgu2, cu))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPartsForBitManipulation(
        x: quest_hook::libil2cpp::ByRefMut<crate::System::Numerics::BigInteger>,
        xd: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        >,
        xl: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPartsForBitManipulation", (x, xd, xl))?;
        Ok(__cordl_ret.into())
    }
    pub fn Parse_IFormatProvider0(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Numerics::BigInteger> {
        let __cordl_ret: crate::System::Numerics::BigInteger = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Parse", (value, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn Parse_NumberStyles_IFormatProvider1(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        style: crate::System::Globalization::NumberStyles,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<crate::System::Numerics::BigInteger> {
        let __cordl_ret: crate::System::Numerics::BigInteger = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Parse", (value, style, provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn Subtract(
        leftBits: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        leftSign: i32,
        rightBits: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        rightSign: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Numerics::BigInteger> {
        let __cordl_ret: crate::System::Numerics::BigInteger = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Subtract", (leftBits, leftSign, rightBits, rightSign))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToByteArray_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToByteArray", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToByteArray__cordl_bool__cordl_bool1(
        &mut self,
        isUnsigned: bool,
        isBigEndian: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToByteArray",
            (isUnsigned, isBigEndian),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_IFormatProvider1(
        &mut self,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", (provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Il2CppString_IFormatProvider2(
        &mut self,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        provider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (format, provider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetBytes(
        &mut self,
        mode: crate::System::Numerics::BigInteger_GetBytesMode,
        destination: crate::System::Span_1<u8>,
        isUnsigned: bool,
        isBigEndian: bool,
        bytesWritten: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<u8>,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryGetBytes",
            (mode, destination, isUnsigned, isBigEndian, bytesWritten),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryWriteBytes(
        &mut self,
        destination: crate::System::Span_1<u8>,
        bytesWritten: quest_hook::libil2cpp::ByRefMut<i32>,
        isUnsigned: bool,
        isBigEndian: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryWriteBytes",
            (destination, bytesWritten, isUnsigned, isBigEndian),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryWriteOrCountBytes(
        &mut self,
        destination: crate::System::Span_1<u8>,
        bytesWritten: quest_hook::libil2cpp::ByRefMut<i32>,
        isUnsigned: bool,
        isBigEndian: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "TryWriteOrCountBytes",
            (destination, bytesWritten, isUnsigned, isBigEndian),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Decimal6(
        &mut self,
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray7(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Il2CppArray__cordl_bool10(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
        negative: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value, negative),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_ReadOnlySpan_1__cordl_bool__cordl_bool8(
        &mut self,
        value: crate::System::ReadOnlySpan_1<u8>,
        isUnsigned: bool,
        isBigEndian: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value, isUnsigned, isBigEndian),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f32_4(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f64_5(
        &mut self,
        value: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_0(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_Il2CppArray9(
        &mut self,
        n: i32,
        rgu: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u32>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (n, rgu),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i64_2(
        &mut self,
        value: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u32_1(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u64_3(
        &mut self,
        value: u64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsZero(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_IsZero",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MinusOne() -> quest_hook::libil2cpp::Result<
        crate::System::Numerics::BigInteger,
    > {
        let __cordl_ret: crate::System::Numerics::BigInteger = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_MinusOne", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Zero() -> quest_hook::libil2cpp::Result<
        crate::System::Numerics::BigInteger,
    > {
        let __cordl_ret: crate::System::Numerics::BigInteger = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_Zero", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition(
        left: crate::System::Numerics::BigInteger,
        right: crate::System::Numerics::BigInteger,
    ) -> quest_hook::libil2cpp::Result<crate::System::Numerics::BigInteger> {
        let __cordl_ret: crate::System::Numerics::BigInteger = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Division(
        dividend: crate::System::Numerics::BigInteger,
        divisor: crate::System::Numerics::BigInteger,
    ) -> quest_hook::libil2cpp::Result<crate::System::Numerics::BigInteger> {
        let __cordl_ret: crate::System::Numerics::BigInteger = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Division", (dividend, divisor))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        left: crate::System::Numerics::BigInteger,
        right: i64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_BigInteger0(
        value: crate::System::Numerics::BigInteger,
    ) -> quest_hook::libil2cpp::Result<u8> {
        let __cordl_ret: u8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_BigInteger1(
        value: crate::System::Numerics::BigInteger,
    ) -> quest_hook::libil2cpp::Result<i8> {
        let __cordl_ret: i8 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_BigInteger10(
        value: crate::System::Numerics::BigInteger,
    ) -> quest_hook::libil2cpp::Result<crate::System::Decimal> {
        let __cordl_ret: crate::System::Decimal = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_BigInteger2(
        value: crate::System::Numerics::BigInteger,
    ) -> quest_hook::libil2cpp::Result<i16> {
        let __cordl_ret: i16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_BigInteger3(
        value: crate::System::Numerics::BigInteger,
    ) -> quest_hook::libil2cpp::Result<u16> {
        let __cordl_ret: u16 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_BigInteger4(
        value: crate::System::Numerics::BigInteger,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_BigInteger5(
        value: crate::System::Numerics::BigInteger,
    ) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_ret: u32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_BigInteger6(
        value: crate::System::Numerics::BigInteger,
    ) -> quest_hook::libil2cpp::Result<i64> {
        let __cordl_ret: i64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_BigInteger7(
        value: crate::System::Numerics::BigInteger,
    ) -> quest_hook::libil2cpp::Result<u64> {
        let __cordl_ret: u64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_BigInteger8(
        value: crate::System::Numerics::BigInteger,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_BigInteger9(
        value: crate::System::Numerics::BigInteger,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_i16_2(
        value: i16,
    ) -> quest_hook::libil2cpp::Result<crate::System::Numerics::BigInteger> {
        let __cordl_ret: crate::System::Numerics::BigInteger = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_i32_4(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Numerics::BigInteger> {
        let __cordl_ret: crate::System::Numerics::BigInteger = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_i64_6(
        value: i64,
    ) -> quest_hook::libil2cpp::Result<crate::System::Numerics::BigInteger> {
        let __cordl_ret: crate::System::Numerics::BigInteger = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_i8_1(
        value: i8,
    ) -> quest_hook::libil2cpp::Result<crate::System::Numerics::BigInteger> {
        let __cordl_ret: crate::System::Numerics::BigInteger = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_u16_3(
        value: u16,
    ) -> quest_hook::libil2cpp::Result<crate::System::Numerics::BigInteger> {
        let __cordl_ret: crate::System::Numerics::BigInteger = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_u32_5(
        value: u32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Numerics::BigInteger> {
        let __cordl_ret: crate::System::Numerics::BigInteger = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_u64_7(
        value: u64,
    ) -> quest_hook::libil2cpp::Result<crate::System::Numerics::BigInteger> {
        let __cordl_ret: crate::System::Numerics::BigInteger = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_u8_0(
        value: u8,
    ) -> quest_hook::libil2cpp::Result<crate::System::Numerics::BigInteger> {
        let __cordl_ret: crate::System::Numerics::BigInteger = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_BigInteger0(
        left: crate::System::Numerics::BigInteger,
        right: crate::System::Numerics::BigInteger,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_i64_1(
        left: crate::System::Numerics::BigInteger,
        right: i64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LeftShift(
        value: crate::System::Numerics::BigInteger,
        shift: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Numerics::BigInteger> {
        let __cordl_ret: crate::System::Numerics::BigInteger = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LeftShift", (value, shift))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_BigInteger_BigInteger0(
        left: crate::System::Numerics::BigInteger,
        right: crate::System::Numerics::BigInteger,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThanOrEqual", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_BigInteger_i64_1(
        left: crate::System::Numerics::BigInteger,
        right: i64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThanOrEqual", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_i64_BigInteger2(
        left: i64,
        right: crate::System::Numerics::BigInteger,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThanOrEqual", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_BigInteger_i64_0(
        left: crate::System::Numerics::BigInteger,
        right: i64,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThan", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_i64_BigInteger1(
        left: i64,
        right: crate::System::Numerics::BigInteger,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThan", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus(
        dividend: crate::System::Numerics::BigInteger,
        divisor: crate::System::Numerics::BigInteger,
    ) -> quest_hook::libil2cpp::Result<crate::System::Numerics::BigInteger> {
        let __cordl_ret: crate::System::Numerics::BigInteger = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Modulus", (dividend, divisor))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply(
        left: crate::System::Numerics::BigInteger,
        right: crate::System::Numerics::BigInteger,
    ) -> quest_hook::libil2cpp::Result<crate::System::Numerics::BigInteger> {
        let __cordl_ret: crate::System::Numerics::BigInteger = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_RightShift(
        value: crate::System::Numerics::BigInteger,
        shift: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Numerics::BigInteger> {
        let __cordl_ret: crate::System::Numerics::BigInteger = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_RightShift", (value, shift))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction(
        left: crate::System::Numerics::BigInteger,
        right: crate::System::Numerics::BigInteger,
    ) -> quest_hook::libil2cpp::Result<crate::System::Numerics::BigInteger> {
        let __cordl_ret: crate::System::Numerics::BigInteger = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (left, right))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_UnaryNegation(
        value: crate::System::Numerics::BigInteger,
    ) -> quest_hook::libil2cpp::Result<crate::System::Numerics::BigInteger> {
        let __cordl_ret: crate::System::Numerics::BigInteger = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_UnaryNegation", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Numerics+BigInteger")]
impl AsRef<crate::System::IComparable> for crate::System::Numerics::BigInteger {
    fn as_ref(&self) -> &crate::System::IComparable {
        todo!()
    }
}
#[cfg(feature = "System+Numerics+BigInteger")]
impl AsMut<crate::System::IComparable> for crate::System::Numerics::BigInteger {
    fn as_mut(&mut self) -> &mut crate::System::IComparable {
        todo!()
    }
}
#[cfg(feature = "System+Numerics+BigInteger")]
impl AsRef<crate::System::IComparable_1<crate::System::Numerics::BigInteger>>
for crate::System::Numerics::BigInteger {
    fn as_ref(
        &self,
    ) -> &crate::System::IComparable_1<crate::System::Numerics::BigInteger> {
        todo!()
    }
}
#[cfg(feature = "System+Numerics+BigInteger")]
impl AsMut<crate::System::IComparable_1<crate::System::Numerics::BigInteger>>
for crate::System::Numerics::BigInteger {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IComparable_1<crate::System::Numerics::BigInteger> {
        todo!()
    }
}
#[cfg(feature = "System+Numerics+BigInteger")]
impl AsRef<crate::System::IEquatable_1<crate::System::Numerics::BigInteger>>
for crate::System::Numerics::BigInteger {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::System::Numerics::BigInteger> {
        todo!()
    }
}
#[cfg(feature = "System+Numerics+BigInteger")]
impl AsMut<crate::System::IEquatable_1<crate::System::Numerics::BigInteger>>
for crate::System::Numerics::BigInteger {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::System::Numerics::BigInteger> {
        todo!()
    }
}
#[cfg(feature = "System+Numerics+BigInteger")]
impl AsRef<crate::System::IFormattable> for crate::System::Numerics::BigInteger {
    fn as_ref(&self) -> &crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "System+Numerics+BigInteger")]
impl AsMut<crate::System::IFormattable> for crate::System::Numerics::BigInteger {
    fn as_mut(&mut self) -> &mut crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "System+Numerics+BigInteger+GetBytesMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BigInteger_GetBytesMode {
    #[default]
    AllocateArray = 0i32,
    Count = 1i32,
    Span = 2i32,
}
#[cfg(feature = "System+Numerics+BigInteger+GetBytesMode")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Numerics::BigInteger_GetBytesMode {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Numerics";
    const CLASS_NAME: &'static str = "GetBytesMode";
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
for crate::System::Numerics::BigInteger_GetBytesMode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Numerics::BigInteger_GetBytesMode {
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
for crate::System::Numerics::BigInteger_GetBytesMode {
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
for crate::System::Numerics::BigInteger_GetBytesMode {
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
