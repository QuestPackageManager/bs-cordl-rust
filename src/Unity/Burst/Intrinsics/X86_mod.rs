#[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx+CMP")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Avx_X86_CMP {
    #[default]
    EQ_OQ = 0i32,
    EQ_OS = 16i32,
    EQ_UQ = 8i32,
    EQ_US = 24i32,
    FALSE_OQ = 11i32,
    FALSE_OS = 27i32,
    GE_OQ = 29i32,
    GE_OS = 13i32,
    GT_OQ = 30i32,
    GT_OS = 14i32,
    LE_OQ = 18i32,
    LE_OS = 2i32,
    LT_OQ = 17i32,
    LT_OS = 1i32,
    NEQ_OQ = 12i32,
    NEQ_OS = 28i32,
    NEQ_UQ = 4i32,
    NEQ_US = 20i32,
    NGE_UQ = 25i32,
    NGE_US = 9i32,
    NGT_UQ = 26i32,
    NGT_US = 10i32,
    NLE_UQ = 22i32,
    NLE_US = 6i32,
    NLT_UQ = 21i32,
    NLT_US = 5i32,
    ORD_Q = 7i32,
    ORD_S = 23i32,
    TRUE_UQ = 15i32,
    TRUE_US = 31i32,
    UNORD_Q = 3i32,
    UNORD_S = 19i32,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx+CMP")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Burst::Intrinsics::Avx_X86_CMP {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Burst.Intrinsics";
    const CLASS_NAME: &'static str = "X86/Avx/CMP";
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
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx+CMP")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Unity::Burst::Intrinsics::Avx_X86_CMP {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx+CMP")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Unity::Burst::Intrinsics::Avx_X86_CMP {
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
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx+CMP")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Unity::Burst::Intrinsics::Avx_X86_CMP {
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
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx+CMP")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Unity::Burst::Intrinsics::Avx_X86_CMP {
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
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Fma+Union")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Fma_X86_Union {
    padding: quest_hook::libil2cpp::ValueTypePadding<4usize>,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Fma+Union")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Burst::Intrinsics::Fma_X86_Union {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Burst.Intrinsics";
    const CLASS_NAME: &'static str = "X86/Fma/Union";
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
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Fma+Union")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Unity::Burst::Intrinsics::Fma_X86_Union {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Fma+Union")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Unity::Burst::Intrinsics::Fma_X86_Union {
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
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Fma+Union")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Unity::Burst::Intrinsics::Fma_X86_Union {
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
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Fma+Union")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Unity::Burst::Intrinsics::Fma_X86_Union {
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
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Fma+Union")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Burst::Intrinsics::Fma_X86_Union {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Fma+Union")]
impl crate::Unity::Burst::Intrinsics::Fma_X86_Union {}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2+SIDD")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Sse4_2_X86_SIDD {
    #[default]
    BIT_MASK = 0i32,
    CMP_EQUAL_EACH = 8i32,
    CMP_EQUAL_ORDERED = 12i32,
    CMP_RANGES = 4i32,
    MASKED_NEGATIVE_POLARITY = 48i32,
    MASKED_POSITIVE_POLARITY = 32i32,
    MOST_SIGNIFICANT = 64i32,
    NEGATIVE_POLARITY = 16i32,
    SBYTE_OPS = 2i32,
    SWORD_OPS = 3i32,
    UWORD_OPS = 1i32,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2+SIDD")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Burst::Intrinsics::Sse4_2_X86_SIDD {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Burst.Intrinsics";
    const CLASS_NAME: &'static str = "X86/Sse4_2/SIDD";
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
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2+SIDD")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Unity::Burst::Intrinsics::Sse4_2_X86_SIDD {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2+SIDD")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Unity::Burst::Intrinsics::Sse4_2_X86_SIDD {
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
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2+SIDD")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Unity::Burst::Intrinsics::Sse4_2_X86_SIDD {
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
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2+SIDD")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Unity::Burst::Intrinsics::Sse4_2_X86_SIDD {
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
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2+StrBoolArray")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Sse4_2_X86_StrBoolArray {
    pub Bits: crate::Unity::Burst::Intrinsics::StrBoolArray_Sse4_2_X86__Bits_e__FixedBuffer,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2+StrBoolArray")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Burst::Intrinsics::Sse4_2_X86_StrBoolArray {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Burst.Intrinsics";
    const CLASS_NAME: &'static str = "X86/Sse4_2/StrBoolArray";
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
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2+StrBoolArray")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Unity::Burst::Intrinsics::Sse4_2_X86_StrBoolArray {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2+StrBoolArray")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Unity::Burst::Intrinsics::Sse4_2_X86_StrBoolArray {
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
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2+StrBoolArray")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Unity::Burst::Intrinsics::Sse4_2_X86_StrBoolArray {
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
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2+StrBoolArray")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Unity::Burst::Intrinsics::Sse4_2_X86_StrBoolArray {
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
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2+StrBoolArray")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Burst::Intrinsics::Sse4_2_X86_StrBoolArray {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2+StrBoolArray")]
impl crate::Unity::Burst::Intrinsics::Sse4_2_X86_StrBoolArray {
    #[cfg(
        feature = "Unity+Burst+Intrinsics+X86+Sse4_2+StrBoolArray+_Bits_e__FixedBuffer"
    )]
    pub type _Bits_e__FixedBuffer = crate::Unity::Burst::Intrinsics::StrBoolArray_Sse4_2_X86__Bits_e__FixedBuffer;
    pub fn GetBit(
        &mut self,
        aindex: i32,
        bindex: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32, i32), bool, 2usize>("GetBit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetBit", 2usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked(self, (aindex, bindex))
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetBit(
        &mut self,
        aindex: i32,
        bindex: i32,
        val: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, bool),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetBit")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetBit", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (aindex, bindex, val))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2+StrBoolArray+_Bits_e__FixedBuffer")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct StrBoolArray_Sse4_2_X86__Bits_e__FixedBuffer {
    pub FixedElementField: u16,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2+StrBoolArray+_Bits_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Burst::Intrinsics::StrBoolArray_Sse4_2_X86__Bits_e__FixedBuffer {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Burst.Intrinsics";
    const CLASS_NAME: &'static str = "X86/Sse4_2/StrBoolArray/<Bits>e__FixedBuffer";
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
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2+StrBoolArray+_Bits_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Unity::Burst::Intrinsics::StrBoolArray_Sse4_2_X86__Bits_e__FixedBuffer {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2+StrBoolArray+_Bits_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Unity::Burst::Intrinsics::StrBoolArray_Sse4_2_X86__Bits_e__FixedBuffer {
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
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2+StrBoolArray+_Bits_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Unity::Burst::Intrinsics::StrBoolArray_Sse4_2_X86__Bits_e__FixedBuffer {
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
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2+StrBoolArray+_Bits_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Unity::Burst::Intrinsics::StrBoolArray_Sse4_2_X86__Bits_e__FixedBuffer {
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
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2+StrBoolArray+_Bits_e__FixedBuffer")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Burst::Intrinsics::StrBoolArray_Sse4_2_X86__Bits_e__FixedBuffer {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2+StrBoolArray+_Bits_e__FixedBuffer")]
impl crate::Unity::Burst::Intrinsics::StrBoolArray_Sse4_2_X86__Bits_e__FixedBuffer {}
#[cfg(feature = "Unity+Burst+Intrinsics+X86")]
#[repr(C)]
#[derive(Debug)]
pub struct X86 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Burst::Intrinsics::X86 {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Burst.Intrinsics";
    const CLASS_NAME: &'static str = "X86";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86")]
impl std::ops::DerefMut for crate::Unity::Burst::Intrinsics::X86 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86")]
impl crate::Unity::Burst::Intrinsics::X86 {
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx")]
    pub type Avx = crate::Unity::Burst::Intrinsics::X86_Avx;
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx2")]
    pub type Avx2 = crate::Unity::Burst::Intrinsics::X86_Avx2;
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+Bmi1")]
    pub type Bmi1 = crate::Unity::Burst::Intrinsics::X86_Bmi1;
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+Bmi2")]
    pub type Bmi2 = crate::Unity::Burst::Intrinsics::X86_Bmi2;
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+F16C")]
    pub type F16C = crate::Unity::Burst::Intrinsics::X86_F16C;
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+Fma")]
    pub type Fma = crate::Unity::Burst::Intrinsics::X86_Fma;
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+MXCSRBits")]
    pub type MXCSRBits = crate::Unity::Burst::Intrinsics::X86_MXCSRBits;
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+Popcnt")]
    pub type Popcnt = crate::Unity::Burst::Intrinsics::X86_Popcnt;
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+RoundingMode")]
    pub type RoundingMode = crate::Unity::Burst::Intrinsics::X86_RoundingMode;
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+RoundingScope")]
    pub type RoundingScope = crate::Unity::Burst::Intrinsics::X86_RoundingScope;
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse")]
    pub type Sse = crate::Unity::Burst::Intrinsics::X86_Sse;
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse2")]
    pub type Sse2 = crate::Unity::Burst::Intrinsics::X86_Sse2;
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse3")]
    pub type Sse3 = crate::Unity::Burst::Intrinsics::X86_Sse3;
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_1")]
    pub type Sse4_1 = crate::Unity::Burst::Intrinsics::X86_Sse4_1;
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2")]
    pub type Sse4_2 = crate::Unity::Burst::Intrinsics::X86_Sse4_2;
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+Ssse3")]
    pub type Ssse3 = crate::Unity::Burst::Intrinsics::X86_Ssse3;
    pub fn BurstIntrinsicGetCSRFromManaged() -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), i32, 0usize>("BurstIntrinsicGetCSRFromManaged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "BurstIntrinsicGetCSRFromManaged", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn BurstIntrinsicSetCSRFromManaged(
        _cordl__: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("BurstIntrinsicSetCSRFromManaged")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "BurstIntrinsicSetCSRFromManaged", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (_cordl__))
        };
        Ok(__cordl_ret.into())
    }
    pub fn DoGetCSRTrampoline() -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), i32, 0usize>("DoGetCSRTrampoline")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DoGetCSRTrampoline", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn DoSetCSRTrampoline(
        bits: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("DoSetCSRTrampoline")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DoSetCSRTrampoline", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (bits))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GenericCSharpLoad(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("GenericCSharpLoad")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GenericCSharpLoad", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (ptr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GenericCSharpStore(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        val: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("GenericCSharpStore")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GenericCSharpStore", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (ptr, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsNaN_u32_0(v: u32) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u32), bool, 1usize>("IsNaN")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsNaN", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (v)) };
        Ok(__cordl_ret.into())
    }
    pub fn IsNaN_u64_1(v: u64) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), bool, 1usize>("IsNaN")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsNaN", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (v)) };
        Ok(__cordl_ret.into())
    }
    pub fn Saturate_To_Int16(val: i32) -> quest_hook::libil2cpp::Result<i16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), i16, 1usize>("Saturate_To_Int16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Saturate_To_Int16", 1usize
                )
            });
        let __cordl_ret: i16 = unsafe { method.invoke_unchecked((), (val)) };
        Ok(__cordl_ret.into())
    }
    pub fn Saturate_To_Int8(val: i32) -> quest_hook::libil2cpp::Result<i8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), i8, 1usize>("Saturate_To_Int8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Saturate_To_Int8", 1usize
                )
            });
        let __cordl_ret: i8 = unsafe { method.invoke_unchecked((), (val)) };
        Ok(__cordl_ret.into())
    }
    pub fn Saturate_To_UnsignedInt16(val: i32) -> quest_hook::libil2cpp::Result<u16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), u16, 1usize>("Saturate_To_UnsignedInt16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Saturate_To_UnsignedInt16", 1usize
                )
            });
        let __cordl_ret: u16 = unsafe { method.invoke_unchecked((), (val)) };
        Ok(__cordl_ret.into())
    }
    pub fn Saturate_To_UnsignedInt8(val: i32) -> quest_hook::libil2cpp::Result<u8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32), u8, 1usize>("Saturate_To_UnsignedInt8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Saturate_To_UnsignedInt8", 1usize
                )
            });
        let __cordl_ret: u8 = unsafe { method.invoke_unchecked((), (val)) };
        Ok(__cordl_ret.into())
    }
    pub fn get_MXCSR() -> quest_hook::libil2cpp::Result<
        crate::Unity::Burst::Intrinsics::X86_MXCSRBits,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::Unity::Burst::Intrinsics::X86_MXCSRBits,
                0usize,
            >("get_MXCSR")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_MXCSR", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::X86_MXCSRBits = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn getcsr_raw() -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), i32, 0usize>("getcsr_raw")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "getcsr_raw", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn set_MXCSR(
        value: crate::Unity::Burst::Intrinsics::X86_MXCSRBits,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::X86_MXCSRBits),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_MXCSR")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_MXCSR", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn setcsr_raw(
        bits: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                quest_hook::libil2cpp::Void,
                1usize,
            >("setcsr_raw")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "setcsr_raw", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (bits))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::Intrinsics::X86 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx")]
#[repr(C)]
#[derive(Debug)]
pub struct X86_Avx {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Burst::Intrinsics::X86_Avx {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Burst.Intrinsics";
    const CLASS_NAME: &'static str = "X86/Avx";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86_Avx {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx")]
impl std::ops::DerefMut for crate::Unity::Burst::Intrinsics::X86_Avx {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx")]
impl crate::Unity::Burst::Intrinsics::X86_Avx {
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx+CMP")]
    pub type CMP = crate::Unity::Burst::Intrinsics::Avx_X86_CMP;
    pub fn Select4(
        src1: crate::Unity::Burst::Intrinsics::v256,
        src2: crate::Unity::Burst::Intrinsics::v256,
        control: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("Select4")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Select4", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (src1, src2, control))
        };
        Ok(__cordl_ret.into())
    }
    pub fn broadcast_ss(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("broadcast_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "broadcast_ss", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (ptr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmp_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("cmp_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmp_pd", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmp_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("cmp_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmp_ps", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmp_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("cmp_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmp_sd", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmp_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("cmp_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmp_ss", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsAvxSupported() -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), bool, 0usize>("get_IsAvxSupported")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_IsAvxSupported", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn maskload_pd(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        mask: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("maskload_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "maskload_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (mem_addr, mask))
        };
        Ok(__cordl_ret.into())
    }
    pub fn maskload_ps(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        mask: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("maskload_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "maskload_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (mem_addr, mask))
        };
        Ok(__cordl_ret.into())
    }
    pub fn maskstore_pd(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        mask: crate::Unity::Burst::Intrinsics::v128,
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("maskstore_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "maskstore_pd", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (mem_addr, mask, a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn maskstore_ps(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        mask: crate::Unity::Burst::Intrinsics::v128,
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("maskstore_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "maskstore_ps", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (mem_addr, mask, a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_add_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_add_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_add_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_add_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_add_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_add_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_addsub_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_addsub_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_addsub_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_addsub_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_addsub_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_addsub_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_and_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_and_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_and_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_and_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_and_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_and_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_andnot_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_andnot_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_andnot_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_andnot_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_andnot_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_andnot_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_blend_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_blend_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_blend_pd", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_blend_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_blend_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_blend_ps", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_blendv_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        mask: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_blendv_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_blendv_pd", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b, mask))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_blendv_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        mask: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_blendv_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_blendv_ps", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b, mask))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_broadcast_pd(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_broadcast_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_broadcast_pd", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (ptr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_broadcast_ps(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_broadcast_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_broadcast_ps", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (ptr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_broadcast_sd(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_broadcast_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_broadcast_sd", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (ptr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_broadcast_ss(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_broadcast_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_broadcast_ss", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (ptr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_castpd128_pd256(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_castpd128_pd256")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_castpd128_pd256", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_castpd256_pd128(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("mm256_castpd256_pd128")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_castpd256_pd128", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_castpd_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_castpd_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_castpd_ps", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_castpd_si256(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_castpd_si256")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_castpd_si256", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_castps128_ps256(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_castps128_ps256")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_castps128_ps256", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_castps256_ps128(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("mm256_castps256_ps128")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_castps256_ps128", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_castps_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_castps_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_castps_pd", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_castps_si256(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_castps_si256")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_castps_si256", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_castsi128_si256(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_castsi128_si256")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_castsi128_si256", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_castsi256_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_castsi256_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_castsi256_pd", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_castsi256_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_castsi256_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_castsi256_ps", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_castsi256_si128(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("mm256_castsi256_si128")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_castsi256_si128", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_ceil_pd(
        val: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_ceil_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_ceil_pd", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_ceil_ps(
        val: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_ceil_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_ceil_ps", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cmp_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_cmp_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_cmp_pd", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cmp_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_cmp_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_cmp_ps", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtepi32_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_cvtepi32_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_cvtepi32_pd", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtepi32_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_cvtepi32_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_cvtepi32_ps", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtpd_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("mm256_cvtpd_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_cvtpd_epi32", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtpd_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("mm256_cvtpd_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_cvtpd_ps", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtps_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_cvtps_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_cvtps_epi32", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtps_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_cvtps_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_cvtps_pd", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtss_f32(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256),
                f32,
                1usize,
            >("mm256_cvtss_f32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_cvtss_f32", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (a)) };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvttpd_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("mm256_cvttpd_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_cvttpd_epi32", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvttps_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_cvttps_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_cvttps_epi32", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_div_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_div_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_div_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_div_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_div_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_div_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_dp_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_dp_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_dp_ps", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_extract_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256, i32),
                i32,
                2usize,
            >("mm256_extract_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_extract_epi32", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, index)) };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_extract_epi64(
        a: crate::Unity::Burst::Intrinsics::v256,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256, i32),
                i64,
                2usize,
            >("mm256_extract_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_extract_epi64", 2usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (a, index)) };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_extractf128_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256, i32),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("mm256_extractf128_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_extractf128_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_extractf128_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256, i32),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("mm256_extractf128_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_extractf128_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_extractf128_si256(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256, i32),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("mm256_extractf128_si256")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_extractf128_si256", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_floor_pd(
        val: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_floor_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_floor_pd", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_floor_ps(
        val: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_floor_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_floor_ps", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_hadd_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_hadd_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_hadd_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_hadd_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_hadd_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_hadd_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_hsub_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_hsub_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_hsub_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_hsub_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_hsub_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_hsub_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_insert_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        i: i32,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256, i32, i32),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_insert_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_insert_epi16", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, i, index))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_insert_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        i: i32,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256, i32, i32),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_insert_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_insert_epi32", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, i, index))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_insert_epi64(
        a: crate::Unity::Burst::Intrinsics::v256,
        i: i64,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256, i64, i32),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_insert_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_insert_epi64", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, i, index))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_insert_epi8(
        a: crate::Unity::Burst::Intrinsics::v256,
        i: i32,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256, i32, i32),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_insert_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_insert_epi8", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, i, index))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_insertf128_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_insertf128_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_insertf128_pd", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_insertf128_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_insertf128_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_insertf128_ps", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_insertf128_si256(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_insertf128_si256")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_insertf128_si256", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_lddqu_si256(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_lddqu_si256")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_lddqu_si256", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (mem_addr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_load_pd(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_load_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_load_pd", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (ptr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_load_ps(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_load_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_load_ps", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (ptr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_load_si256(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_load_si256")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_load_si256", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (ptr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_loadu2_m128(
        hiaddr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        loaddr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_loadu2_m128")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_loadu2_m128", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (hiaddr, loaddr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_loadu2_m128d(
        hiaddr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        loaddr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_loadu2_m128d")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_loadu2_m128d", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (hiaddr, loaddr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_loadu2_m128i(
        hiaddr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        loaddr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_loadu2_m128i")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_loadu2_m128i", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (hiaddr, loaddr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_loadu_pd(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_loadu_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_loadu_pd", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (ptr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_loadu_ps(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_loadu_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_loadu_ps", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (ptr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_loadu_si256(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_loadu_si256")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_loadu_si256", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (ptr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_maskload_pd(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        mask: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_maskload_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_maskload_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (mem_addr, mask))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_maskload_ps(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        mask: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_maskload_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_maskload_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (mem_addr, mask))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_maskstore_pd(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        mask: crate::Unity::Burst::Intrinsics::v256,
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("mm256_maskstore_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_maskstore_pd", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (mem_addr, mask, a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_maskstore_ps(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        mask: crate::Unity::Burst::Intrinsics::v256,
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("mm256_maskstore_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_maskstore_ps", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (mem_addr, mask, a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_max_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_max_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_max_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_max_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_max_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_max_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_min_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_min_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_min_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_min_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_min_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_min_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_movedup_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_movedup_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_movedup_pd", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_movehdup_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_movehdup_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_movehdup_ps", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_moveldup_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_moveldup_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_moveldup_ps", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_movemask_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256),
                i32,
                1usize,
            >("mm256_movemask_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_movemask_pd", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a)) };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_movemask_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256),
                i32,
                1usize,
            >("mm256_movemask_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_movemask_ps", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a)) };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_mul_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_mul_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_mul_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_mul_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_mul_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_mul_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_or_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_or_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_or_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_or_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_or_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_or_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_permute2f128_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_permute2f128_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_permute2f128_pd", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_permute2f128_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_permute2f128_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_permute2f128_ps", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_permute2f128_si256(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_permute2f128_si256")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_permute2f128_si256", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_permute_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256, i32),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_permute_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_permute_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_permute_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256, i32),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_permute_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_permute_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_permutevar_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_permutevar_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_permutevar_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_permutevar_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_permutevar_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_permutevar_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_rcp_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_rcp_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_rcp_ps", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_round_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        rounding: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256, i32),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_round_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_round_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, rounding))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_round_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        rounding: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256, i32),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_round_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_round_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, rounding))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_rsqrt_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_rsqrt_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_rsqrt_ps", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_set1_epi16(
        a: i16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i16),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_set1_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_set1_epi16", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_set1_epi32(
        a: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_set1_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_set1_epi32", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_set1_epi64x(
        a: i64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i64),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_set1_epi64x")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_set1_epi64x", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_set1_epi8(
        a: u8,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u8),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_set1_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_set1_epi8", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_set1_pd(
        a: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f64),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_set1_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_set1_pd", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_set1_ps(
        a: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_set1_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_set1_ps", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_set_epi16(
        e15_: i16,
        e14_: i16,
        e13_: i16,
        e12_: i16,
        e11_: i16,
        e10_: i16,
        e9_: i16,
        e8_: i16,
        e7_: i16,
        e6_: i16,
        e5_: i16,
        e4_: i16,
        e3_: i16,
        e2_: i16,
        e1_: i16,
        e0_: i16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i16,
                    i16,
                    i16,
                    i16,
                    i16,
                    i16,
                    i16,
                    i16,
                    i16,
                    i16,
                    i16,
                    i16,
                    i16,
                    i16,
                    i16,
                    i16,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                16usize,
            >("mm256_set_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_set_epi16", 16usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        e15_,
                        e14_,
                        e13_,
                        e12_,
                        e11_,
                        e10_,
                        e9_,
                        e8_,
                        e7_,
                        e6_,
                        e5_,
                        e4_,
                        e3_,
                        e2_,
                        e1_,
                        e0_,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_set_epi32(
        e7: i32,
        e6: i32,
        e5: i32,
        e4: i32,
        e3: i32,
        e2: i32,
        e1: i32,
        e0: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, i32, i32, i32, i32, i32, i32, i32),
                crate::Unity::Burst::Intrinsics::v256,
                8usize,
            >("mm256_set_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_set_epi32", 8usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (e7, e6, e5, e4, e3, e2, e1, e0))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_set_epi64x(
        e3: i64,
        e2: i64,
        e1: i64,
        e0: i64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i64, i64, i64, i64),
                crate::Unity::Burst::Intrinsics::v256,
                4usize,
            >("mm256_set_epi64x")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_set_epi64x", 4usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (e3, e2, e1, e0))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_set_epi8(
        e31_: u8,
        e30_: u8,
        e29_: u8,
        e28_: u8,
        e27_: u8,
        e26_: u8,
        e25_: u8,
        e24_: u8,
        e23_: u8,
        e22_: u8,
        e21_: u8,
        e20_: u8,
        e19_: u8,
        e18_: u8,
        e17_: u8,
        e16_: u8,
        e15_: u8,
        e14_: u8,
        e13_: u8,
        e12_: u8,
        e11_: u8,
        e10_: u8,
        e9_: u8,
        e8_: u8,
        e7_: u8,
        e6_: u8,
        e5_: u8,
        e4_: u8,
        e3_: u8,
        e2_: u8,
        e1_: u8,
        e0_: u8,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                32usize,
            >("mm256_set_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_set_epi8", 32usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        e31_,
                        e30_,
                        e29_,
                        e28_,
                        e27_,
                        e26_,
                        e25_,
                        e24_,
                        e23_,
                        e22_,
                        e21_,
                        e20_,
                        e19_,
                        e18_,
                        e17_,
                        e16_,
                        e15_,
                        e14_,
                        e13_,
                        e12_,
                        e11_,
                        e10_,
                        e9_,
                        e8_,
                        e7_,
                        e6_,
                        e5_,
                        e4_,
                        e3_,
                        e2_,
                        e1_,
                        e0_,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_set_m128(
        hi: crate::Unity::Burst::Intrinsics::v128,
        lo: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_set_m128")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_set_m128", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (hi, lo))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_set_m128d(
        hi: crate::Unity::Burst::Intrinsics::v128,
        lo: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_set_m128d")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_set_m128d", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (hi, lo))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_set_m128i(
        hi: crate::Unity::Burst::Intrinsics::v128,
        lo: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_set_m128i")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_set_m128i", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (hi, lo))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_set_pd(
        d: f64,
        c: f64,
        b: f64,
        a: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f64, f64, f64, f64),
                crate::Unity::Burst::Intrinsics::v256,
                4usize,
            >("mm256_set_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_set_pd", 4usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (d, c, b, a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_set_ps(
        e7: f32,
        e6: f32,
        e5: f32,
        e4: f32,
        e3: f32,
        e2: f32,
        e1: f32,
        e0: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, f32, f32, f32, f32, f32, f32, f32),
                crate::Unity::Burst::Intrinsics::v256,
                8usize,
            >("mm256_set_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_set_ps", 8usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (e7, e6, e5, e4, e3, e2, e1, e0))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_setr_epi16(
        e15_: i16,
        e14_: i16,
        e13_: i16,
        e12_: i16,
        e11_: i16,
        e10_: i16,
        e9_: i16,
        e8_: i16,
        e7_: i16,
        e6_: i16,
        e5_: i16,
        e4_: i16,
        e3_: i16,
        e2_: i16,
        e1_: i16,
        e0_: i16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    i16,
                    i16,
                    i16,
                    i16,
                    i16,
                    i16,
                    i16,
                    i16,
                    i16,
                    i16,
                    i16,
                    i16,
                    i16,
                    i16,
                    i16,
                    i16,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                16usize,
            >("mm256_setr_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_setr_epi16", 16usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        e15_,
                        e14_,
                        e13_,
                        e12_,
                        e11_,
                        e10_,
                        e9_,
                        e8_,
                        e7_,
                        e6_,
                        e5_,
                        e4_,
                        e3_,
                        e2_,
                        e1_,
                        e0_,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_setr_epi32(
        e7: i32,
        e6: i32,
        e5: i32,
        e4: i32,
        e3: i32,
        e2: i32,
        e1: i32,
        e0: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, i32, i32, i32, i32, i32, i32, i32),
                crate::Unity::Burst::Intrinsics::v256,
                8usize,
            >("mm256_setr_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_setr_epi32", 8usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (e7, e6, e5, e4, e3, e2, e1, e0))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_setr_epi64x(
        e3: i64,
        e2: i64,
        e1: i64,
        e0: i64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i64, i64, i64, i64),
                crate::Unity::Burst::Intrinsics::v256,
                4usize,
            >("mm256_setr_epi64x")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_setr_epi64x", 4usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (e3, e2, e1, e0))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_setr_epi8(
        e31_: u8,
        e30_: u8,
        e29_: u8,
        e28_: u8,
        e27_: u8,
        e26_: u8,
        e25_: u8,
        e24_: u8,
        e23_: u8,
        e22_: u8,
        e21_: u8,
        e20_: u8,
        e19_: u8,
        e18_: u8,
        e17_: u8,
        e16_: u8,
        e15_: u8,
        e14_: u8,
        e13_: u8,
        e12_: u8,
        e11_: u8,
        e10_: u8,
        e9_: u8,
        e8_: u8,
        e7_: u8,
        e6_: u8,
        e5_: u8,
        e4_: u8,
        e3_: u8,
        e2_: u8,
        e1_: u8,
        e0_: u8,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                    u8,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                32usize,
            >("mm256_setr_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_setr_epi8", 32usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        e31_,
                        e30_,
                        e29_,
                        e28_,
                        e27_,
                        e26_,
                        e25_,
                        e24_,
                        e23_,
                        e22_,
                        e21_,
                        e20_,
                        e19_,
                        e18_,
                        e17_,
                        e16_,
                        e15_,
                        e14_,
                        e13_,
                        e12_,
                        e11_,
                        e10_,
                        e9_,
                        e8_,
                        e7_,
                        e6_,
                        e5_,
                        e4_,
                        e3_,
                        e2_,
                        e1_,
                        e0_,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_setr_m128(
        hi: crate::Unity::Burst::Intrinsics::v128,
        lo: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_setr_m128")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_setr_m128", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (hi, lo))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_setr_m128d(
        hi: crate::Unity::Burst::Intrinsics::v128,
        lo: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_setr_m128d")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_setr_m128d", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (hi, lo))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_setr_m128i(
        hi: crate::Unity::Burst::Intrinsics::v128,
        lo: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_setr_m128i")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_setr_m128i", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (hi, lo))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_setr_pd(
        d: f64,
        c: f64,
        b: f64,
        a: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f64, f64, f64, f64),
                crate::Unity::Burst::Intrinsics::v256,
                4usize,
            >("mm256_setr_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_setr_pd", 4usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (d, c, b, a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_setr_ps(
        e7: f32,
        e6: f32,
        e5: f32,
        e4: f32,
        e3: f32,
        e2: f32,
        e1: f32,
        e0: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, f32, f32, f32, f32, f32, f32, f32),
                crate::Unity::Burst::Intrinsics::v256,
                8usize,
            >("mm256_setr_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_setr_ps", 8usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (e7, e6, e5, e4, e3, e2, e1, e0))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_setzero_pd() -> quest_hook::libil2cpp::Result<
        crate::Unity::Burst::Intrinsics::v256,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::Unity::Burst::Intrinsics::v256,
                0usize,
            >("mm256_setzero_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_setzero_pd", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_setzero_ps() -> quest_hook::libil2cpp::Result<
        crate::Unity::Burst::Intrinsics::v256,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::Unity::Burst::Intrinsics::v256,
                0usize,
            >("mm256_setzero_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_setzero_ps", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_setzero_si256() -> quest_hook::libil2cpp::Result<
        crate::Unity::Burst::Intrinsics::v256,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::Unity::Burst::Intrinsics::v256,
                0usize,
            >("mm256_setzero_si256")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_setzero_si256", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_shuffle_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_shuffle_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_shuffle_pd", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_shuffle_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_shuffle_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_shuffle_ps", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_sqrt_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_sqrt_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_sqrt_pd", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_sqrt_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_sqrt_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_sqrt_ps", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_store_pd(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("mm256_store_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_store_pd", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (ptr, a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_store_ps(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        val: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("mm256_store_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_store_ps", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (ptr, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_store_si256(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        v: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("mm256_store_si256")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_store_si256", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (ptr, v))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_storeu2_m128(
        hiaddr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        loaddr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        val: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("mm256_storeu2_m128")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_storeu2_m128", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (hiaddr, loaddr, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_storeu2_m128d(
        hiaddr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        loaddr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        val: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("mm256_storeu2_m128d")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_storeu2_m128d", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (hiaddr, loaddr, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_storeu2_m128i(
        hiaddr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        loaddr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        val: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("mm256_storeu2_m128i")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_storeu2_m128i", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (hiaddr, loaddr, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_storeu_pd(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("mm256_storeu_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_storeu_pd", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (ptr, a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_storeu_ps(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("mm256_storeu_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_storeu_ps", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (ptr, a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_storeu_si256(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        v: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("mm256_storeu_si256")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_storeu_si256", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (ptr, v))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_stream_pd(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("mm256_stream_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_stream_pd", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (mem_addr, a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_stream_ps(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("mm256_stream_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_stream_ps", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (mem_addr, a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_stream_si256(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("mm256_stream_si256")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_stream_si256", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (mem_addr, a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_sub_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_sub_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_sub_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_sub_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_sub_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_sub_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_testc_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                i32,
                2usize,
            >("mm256_testc_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_testc_pd", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_testc_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                i32,
                2usize,
            >("mm256_testc_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_testc_ps", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_testc_si256(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                i32,
                2usize,
            >("mm256_testc_si256")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_testc_si256", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_testnzc_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                i32,
                2usize,
            >("mm256_testnzc_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_testnzc_pd", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_testnzc_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                i32,
                2usize,
            >("mm256_testnzc_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_testnzc_ps", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_testnzc_si256(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                i32,
                2usize,
            >("mm256_testnzc_si256")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_testnzc_si256", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_testz_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                i32,
                2usize,
            >("mm256_testz_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_testz_pd", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_testz_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                i32,
                2usize,
            >("mm256_testz_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_testz_ps", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_testz_si256(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                i32,
                2usize,
            >("mm256_testz_si256")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_testz_si256", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_undefined_pd() -> quest_hook::libil2cpp::Result<
        crate::Unity::Burst::Intrinsics::v256,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::Unity::Burst::Intrinsics::v256,
                0usize,
            >("mm256_undefined_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_undefined_pd", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_undefined_ps() -> quest_hook::libil2cpp::Result<
        crate::Unity::Burst::Intrinsics::v256,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::Unity::Burst::Intrinsics::v256,
                0usize,
            >("mm256_undefined_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_undefined_ps", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_undefined_si256() -> quest_hook::libil2cpp::Result<
        crate::Unity::Burst::Intrinsics::v256,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::Unity::Burst::Intrinsics::v256,
                0usize,
            >("mm256_undefined_si256")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_undefined_si256", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_unpackhi_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_unpackhi_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_unpackhi_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_unpackhi_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_unpackhi_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_unpackhi_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_unpacklo_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_unpacklo_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_unpacklo_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_unpacklo_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_unpacklo_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_unpacklo_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_xor_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_xor_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_xor_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_xor_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_xor_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_xor_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_zeroall() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("mm256_zeroall")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_zeroall", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_zeroupper() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("mm256_zeroupper")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_zeroupper", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_zextpd128_pd256(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_zextpd128_pd256")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_zextpd128_pd256", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_zextps128_ps256(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_zextps128_ps256")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_zextps128_ps256", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_zextsi128_si256(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_zextsi128_si256")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_zextsi128_si256", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn permute_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128, i32),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("permute_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "permute_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn permute_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128, i32),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("permute_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "permute_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn permutevar_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("permutevar_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "permutevar_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn permutevar_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("permutevar_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "permutevar_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn testc_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                i32,
                2usize,
            >("testc_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "testc_pd", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn testc_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                i32,
                2usize,
            >("testc_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "testc_ps", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn testnzc_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                i32,
                2usize,
            >("testnzc_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "testnzc_pd", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn testnzc_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                i32,
                2usize,
            >("testnzc_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "testnzc_ps", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn testz_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                i32,
                2usize,
            >("testz_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "testz_pd", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn testz_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                i32,
                2usize,
            >("testz_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "testz_ps", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn undefined_pd() -> quest_hook::libil2cpp::Result<
        crate::Unity::Burst::Intrinsics::v128,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::Unity::Burst::Intrinsics::v128,
                0usize,
            >("undefined_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "undefined_pd", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn undefined_ps() -> quest_hook::libil2cpp::Result<
        crate::Unity::Burst::Intrinsics::v128,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::Unity::Burst::Intrinsics::v128,
                0usize,
            >("undefined_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "undefined_ps", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn undefined_si128() -> quest_hook::libil2cpp::Result<
        crate::Unity::Burst::Intrinsics::v128,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::Unity::Burst::Intrinsics::v128,
                0usize,
            >("undefined_si128")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "undefined_si128", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::Intrinsics::X86_Avx {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx2")]
#[repr(C)]
#[derive(Debug)]
pub struct X86_Avx2 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx2")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Burst::Intrinsics::X86_Avx2 {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Burst.Intrinsics";
    const CLASS_NAME: &'static str = "X86/Avx2";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx2")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86_Avx2 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx2")]
impl std::ops::DerefMut for crate::Unity::Burst::Intrinsics::X86_Avx2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx2")]
impl crate::Unity::Burst::Intrinsics::X86_Avx2 {
    pub fn EmulatedGather_Il2CppObject_Il2CppObject_Il2CppObject_i32_i32_Il2CppObject0<
        T,
        U,
    >(
        dptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        indexPtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        scale: i32,
        n: i32,
        mask: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >("EmulatedGather")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EmulatedGather", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (dptr, base_addr, indexPtr, scale, n, mask))
        };
        Ok(__cordl_ret.into())
    }
    pub fn EmulatedGather_Il2CppObject_Il2CppObject_Il2CppObject_i32_i32_Il2CppObject1<
        T,
        U,
    >(
        dptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        indexPtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        scale: i32,
        n: i32,
        mask: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        U: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    i32,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >("EmulatedGather")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "EmulatedGather", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (dptr, base_addr, indexPtr, scale, n, mask))
        };
        Ok(__cordl_ret.into())
    }
    pub fn blend_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("blend_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "blend_epi32", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn broadcastb_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("broadcastb_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "broadcastb_epi8", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn broadcastd_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("broadcastd_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "broadcastd_epi32", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn broadcastq_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("broadcastq_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "broadcastq_epi64", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn broadcastsd_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("broadcastsd_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "broadcastsd_pd", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn broadcastss_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("broadcastss_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "broadcastss_ps", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn broadcastw_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("broadcastw_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "broadcastw_epi16", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsAvx2Supported() -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), bool, 0usize>("get_IsAvx2Supported")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_IsAvx2Supported", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn i32gather_epi32(
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("i32gather_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "i32gather_epi32", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (base_addr, vindex, scale))
        };
        Ok(__cordl_ret.into())
    }
    pub fn i32gather_epi64(
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("i32gather_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "i32gather_epi64", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (base_addr, vindex, scale))
        };
        Ok(__cordl_ret.into())
    }
    pub fn i32gather_pd(
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("i32gather_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "i32gather_pd", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (base_addr, vindex, scale))
        };
        Ok(__cordl_ret.into())
    }
    pub fn i32gather_ps(
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("i32gather_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "i32gather_ps", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (base_addr, vindex, scale))
        };
        Ok(__cordl_ret.into())
    }
    pub fn i64gather_epi32(
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("i64gather_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "i64gather_epi32", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (base_addr, vindex, scale))
        };
        Ok(__cordl_ret.into())
    }
    pub fn i64gather_epi64(
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("i64gather_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "i64gather_epi64", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (base_addr, vindex, scale))
        };
        Ok(__cordl_ret.into())
    }
    pub fn i64gather_pd(
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("i64gather_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "i64gather_pd", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (base_addr, vindex, scale))
        };
        Ok(__cordl_ret.into())
    }
    pub fn i64gather_ps(
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("i64gather_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "i64gather_ps", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (base_addr, vindex, scale))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mask_i32gather_epi32(
        src: crate::Unity::Burst::Intrinsics::v128,
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        mask: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                5usize,
            >("mask_i32gather_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mask_i32gather_epi32", 5usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (src, base_addr, vindex, mask, scale))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mask_i32gather_epi64(
        src: crate::Unity::Burst::Intrinsics::v128,
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        mask: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                5usize,
            >("mask_i32gather_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mask_i32gather_epi64", 5usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (src, base_addr, vindex, mask, scale))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mask_i32gather_pd(
        src: crate::Unity::Burst::Intrinsics::v128,
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        mask: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                5usize,
            >("mask_i32gather_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mask_i32gather_pd", 5usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (src, base_addr, vindex, mask, scale))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mask_i32gather_ps(
        src: crate::Unity::Burst::Intrinsics::v128,
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        mask: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                5usize,
            >("mask_i32gather_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mask_i32gather_ps", 5usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (src, base_addr, vindex, mask, scale))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mask_i64gather_epi32(
        src: crate::Unity::Burst::Intrinsics::v128,
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        mask: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                5usize,
            >("mask_i64gather_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mask_i64gather_epi32", 5usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (src, base_addr, vindex, mask, scale))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mask_i64gather_epi64(
        src: crate::Unity::Burst::Intrinsics::v128,
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        mask: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                5usize,
            >("mask_i64gather_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mask_i64gather_epi64", 5usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (src, base_addr, vindex, mask, scale))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mask_i64gather_pd(
        src: crate::Unity::Burst::Intrinsics::v128,
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        mask: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                5usize,
            >("mask_i64gather_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mask_i64gather_pd", 5usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (src, base_addr, vindex, mask, scale))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mask_i64gather_ps(
        src: crate::Unity::Burst::Intrinsics::v128,
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        mask: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                5usize,
            >("mask_i64gather_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mask_i64gather_ps", 5usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (src, base_addr, vindex, mask, scale))
        };
        Ok(__cordl_ret.into())
    }
    pub fn maskload_epi32(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        mask: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("maskload_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "maskload_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (mem_addr, mask))
        };
        Ok(__cordl_ret.into())
    }
    pub fn maskload_epi64(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        mask: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("maskload_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "maskload_epi64", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (mem_addr, mask))
        };
        Ok(__cordl_ret.into())
    }
    pub fn maskstore_epi32(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        mask: crate::Unity::Burst::Intrinsics::v128,
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("maskstore_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "maskstore_epi32", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (mem_addr, mask, a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn maskstore_epi64(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        mask: crate::Unity::Burst::Intrinsics::v128,
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("maskstore_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "maskstore_epi64", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (mem_addr, mask, a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_abs_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_abs_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_abs_epi16", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_abs_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_abs_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_abs_epi32", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_abs_epi8(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_abs_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_abs_epi8", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_add_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_add_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_add_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_add_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_add_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_add_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_add_epi64(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_add_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_add_epi64", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_add_epi8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_add_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_add_epi8", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_adds_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_adds_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_adds_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_adds_epi8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_adds_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_adds_epi8", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_adds_epu16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_adds_epu16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_adds_epu16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_adds_epu8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_adds_epu8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_adds_epu8", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_alignr_epi8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_alignr_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_alignr_epi8", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_and_si256(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_and_si256")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_and_si256", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_andnot_si256(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_andnot_si256")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_andnot_si256", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_avg_epu16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_avg_epu16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_avg_epu16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_avg_epu8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_avg_epu8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_avg_epu8", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_blend_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_blend_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_blend_epi16", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_blend_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_blend_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_blend_epi32", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_blendv_epi8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        mask: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_blendv_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_blendv_epi8", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b, mask))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_broadcastb_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_broadcastb_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_broadcastb_epi8", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_broadcastd_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_broadcastd_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_broadcastd_epi32", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_broadcastq_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_broadcastq_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_broadcastq_epi64", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_broadcastsd_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_broadcastsd_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_broadcastsd_pd", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_broadcastsi128_si256(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_broadcastsi128_si256")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_broadcastsi128_si256", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_broadcastss_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_broadcastss_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_broadcastss_ps", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_broadcastw_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_broadcastw_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_broadcastw_epi16", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_bslli_epi128(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256, i32),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_bslli_epi128")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_bslli_epi128", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_bsrli_epi128(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256, i32),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_bsrli_epi128")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_bsrli_epi128", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cmpeq_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_cmpeq_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_cmpeq_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cmpeq_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_cmpeq_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_cmpeq_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cmpeq_epi64(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_cmpeq_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_cmpeq_epi64", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cmpeq_epi8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_cmpeq_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_cmpeq_epi8", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cmpgt_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_cmpgt_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_cmpgt_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cmpgt_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_cmpgt_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_cmpgt_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cmpgt_epi64(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_cmpgt_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_cmpgt_epi64", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cmpgt_epi8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_cmpgt_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_cmpgt_epi8", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtepi16_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_cvtepi16_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_cvtepi16_epi32", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtepi16_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_cvtepi16_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_cvtepi16_epi64", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtepi32_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_cvtepi32_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_cvtepi32_epi64", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtepi8_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_cvtepi8_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_cvtepi8_epi16", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtepi8_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_cvtepi8_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_cvtepi8_epi32", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtepi8_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_cvtepi8_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_cvtepi8_epi64", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtepu16_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_cvtepu16_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_cvtepu16_epi32", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtepu16_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_cvtepu16_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_cvtepu16_epi64", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtepu32_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_cvtepu32_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_cvtepu32_epi64", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtepu8_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_cvtepu8_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_cvtepu8_epi16", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtepu8_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_cvtepu8_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_cvtepu8_epi32", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtepu8_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_cvtepu8_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_cvtepu8_epi64", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtsd_f64(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256),
                f64,
                1usize,
            >("mm256_cvtsd_f64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_cvtsd_f64", 1usize
                )
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (a)) };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtsi256_si32(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256),
                i32,
                1usize,
            >("mm256_cvtsi256_si32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_cvtsi256_si32", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a)) };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtsi256_si64(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256),
                i64,
                1usize,
            >("mm256_cvtsi256_si64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_cvtsi256_si64", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (a)) };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_extract_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256, i32),
                i32,
                2usize,
            >("mm256_extract_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_extract_epi16", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, index)) };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_extract_epi8(
        a: crate::Unity::Burst::Intrinsics::v256,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256, i32),
                i32,
                2usize,
            >("mm256_extract_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_extract_epi8", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, index)) };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_extracti128_si256(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256, i32),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("mm256_extracti128_si256")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_extracti128_si256", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_hadd_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_hadd_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_hadd_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_hadd_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_hadd_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_hadd_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_hadds_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_hadds_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_hadds_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_hsub_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_hsub_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_hsub_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_hsub_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_hsub_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_hsub_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_hsubs_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_hsubs_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_hsubs_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_i32gather_epi32(
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v256,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v256,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_i32gather_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_i32gather_epi32", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (base_addr, vindex, scale))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_i32gather_epi64(
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_i32gather_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_i32gather_epi64", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (base_addr, vindex, scale))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_i32gather_pd(
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_i32gather_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_i32gather_pd", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (base_addr, vindex, scale))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_i32gather_ps(
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v256,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v256,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_i32gather_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_i32gather_ps", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (base_addr, vindex, scale))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_i64gather_epi32(
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v256,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v256,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("mm256_i64gather_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_i64gather_epi32", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (base_addr, vindex, scale))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_i64gather_epi64(
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v256,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v256,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_i64gather_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_i64gather_epi64", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (base_addr, vindex, scale))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_i64gather_pd(
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v256,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v256,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_i64gather_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_i64gather_pd", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (base_addr, vindex, scale))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_i64gather_ps(
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v256,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v256,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("mm256_i64gather_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_i64gather_ps", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (base_addr, vindex, scale))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_inserti128_si256(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_inserti128_si256")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_inserti128_si256", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_madd_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_madd_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_madd_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_maddubs_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_maddubs_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_maddubs_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_mask_i32gather_epi32(
        src: crate::Unity::Burst::Intrinsics::v256,
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v256,
        mask: crate::Unity::Burst::Intrinsics::v256,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                5usize,
            >("mm256_mask_i32gather_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_mask_i32gather_epi32", 5usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (src, base_addr, vindex, mask, scale))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_mask_i32gather_epi64(
        src: crate::Unity::Burst::Intrinsics::v256,
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        mask: crate::Unity::Burst::Intrinsics::v256,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v256,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                5usize,
            >("mm256_mask_i32gather_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_mask_i32gather_epi64", 5usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (src, base_addr, vindex, mask, scale))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_mask_i32gather_pd(
        src: crate::Unity::Burst::Intrinsics::v256,
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v128,
        mask: crate::Unity::Burst::Intrinsics::v256,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v256,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                5usize,
            >("mm256_mask_i32gather_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_mask_i32gather_pd", 5usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (src, base_addr, vindex, mask, scale))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_mask_i32gather_ps(
        src: crate::Unity::Burst::Intrinsics::v256,
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v256,
        mask: crate::Unity::Burst::Intrinsics::v256,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                5usize,
            >("mm256_mask_i32gather_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_mask_i32gather_ps", 5usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (src, base_addr, vindex, mask, scale))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_mask_i64gather_epi32(
        src: crate::Unity::Burst::Intrinsics::v128,
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v256,
        mask: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                5usize,
            >("mm256_mask_i64gather_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_mask_i64gather_epi32", 5usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (src, base_addr, vindex, mask, scale))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_mask_i64gather_epi64(
        src: crate::Unity::Burst::Intrinsics::v256,
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v256,
        mask: crate::Unity::Burst::Intrinsics::v256,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                5usize,
            >("mm256_mask_i64gather_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_mask_i64gather_epi64", 5usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (src, base_addr, vindex, mask, scale))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_mask_i64gather_pd(
        src: crate::Unity::Burst::Intrinsics::v256,
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v256,
        mask: crate::Unity::Burst::Intrinsics::v256,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                5usize,
            >("mm256_mask_i64gather_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_mask_i64gather_pd", 5usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (src, base_addr, vindex, mask, scale))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_mask_i64gather_ps(
        src: crate::Unity::Burst::Intrinsics::v128,
        base_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        vindex: crate::Unity::Burst::Intrinsics::v256,
        mask: crate::Unity::Burst::Intrinsics::v128,
        scale: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                5usize,
            >("mm256_mask_i64gather_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_mask_i64gather_ps", 5usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (src, base_addr, vindex, mask, scale))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_maskload_epi32(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        mask: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_maskload_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_maskload_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (mem_addr, mask))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_maskload_epi64(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        mask: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_maskload_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_maskload_epi64", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (mem_addr, mask))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_maskstore_epi32(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        mask: crate::Unity::Burst::Intrinsics::v256,
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("mm256_maskstore_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_maskstore_epi32", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (mem_addr, mask, a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_maskstore_epi64(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        mask: crate::Unity::Burst::Intrinsics::v256,
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("mm256_maskstore_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_maskstore_epi64", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (mem_addr, mask, a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_max_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_max_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_max_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_max_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_max_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_max_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_max_epi8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_max_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_max_epi8", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_max_epu16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_max_epu16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_max_epu16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_max_epu32(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_max_epu32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_max_epu32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_max_epu8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_max_epu8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_max_epu8", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_min_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_min_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_min_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_min_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_min_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_min_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_min_epi8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_min_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_min_epi8", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_min_epu16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_min_epu16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_min_epu16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_min_epu32(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_min_epu32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_min_epu32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_min_epu8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_min_epu8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_min_epu8", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_movemask_epi8(
        a: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256),
                i32,
                1usize,
            >("mm256_movemask_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_movemask_epi8", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a)) };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_mpsadbw_epu8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_mpsadbw_epu8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_mpsadbw_epu8", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_mul_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_mul_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_mul_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_mul_epu32(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_mul_epu32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_mul_epu32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_mulhi_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_mulhi_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_mulhi_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_mulhi_epu16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_mulhi_epu16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_mulhi_epu16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_mulhrs_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_mulhrs_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_mulhrs_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_mullo_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_mullo_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_mullo_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_mullo_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_mullo_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_mullo_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_or_si256(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_or_si256")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_or_si256", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_packs_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_packs_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_packs_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_packs_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_packs_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_packs_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_packus_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_packus_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_packus_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_packus_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_packus_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_packus_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_permute2x128_si256(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_permute2x128_si256")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_permute2x128_si256", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_permute4x64_epi64(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256, i32),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_permute4x64_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_permute4x64_epi64", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_permute4x64_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256, i32),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_permute4x64_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_permute4x64_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_permutevar8x32_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        idx: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_permutevar8x32_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_permutevar8x32_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, idx))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_permutevar8x32_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        idx: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_permutevar8x32_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_permutevar8x32_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, idx))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_sad_epu8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_sad_epu8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_sad_epu8", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_shuffle_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256, i32),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_shuffle_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_shuffle_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_shuffle_epi8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_shuffle_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_shuffle_epi8", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_shufflehi_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256, i32),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_shufflehi_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_shufflehi_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_shufflelo_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256, i32),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_shufflelo_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_shufflelo_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_sign_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_sign_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_sign_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_sign_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_sign_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_sign_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_sign_epi8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_sign_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_sign_epi8", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_sll_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_sll_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_sll_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, count))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_sll_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_sll_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_sll_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, count))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_sll_epi64(
        a: crate::Unity::Burst::Intrinsics::v256,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_sll_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_sll_epi64", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, count))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_slli_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256, i32),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_slli_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_slli_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_slli_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256, i32),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_slli_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_slli_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_slli_epi64(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256, i32),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_slli_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_slli_epi64", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_slli_si256(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256, i32),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_slli_si256")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_slli_si256", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_sllv_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        count: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_sllv_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_sllv_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, count))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_sllv_epi64(
        a: crate::Unity::Burst::Intrinsics::v256,
        count: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_sllv_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_sllv_epi64", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, count))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_sra_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_sra_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_sra_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, count))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_sra_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_sra_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_sra_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, count))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_srai_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256, i32),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_srai_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_srai_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_srai_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256, i32),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_srai_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_srai_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_srav_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        count: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_srav_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_srav_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, count))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_srl_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_srl_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_srl_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, count))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_srl_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_srl_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_srl_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, count))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_srl_epi64(
        a: crate::Unity::Burst::Intrinsics::v256,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_srl_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_srl_epi64", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, count))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_srli_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256, i32),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_srli_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_srli_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_srli_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256, i32),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_srli_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_srli_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_srli_epi64(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256, i32),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_srli_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_srli_epi64", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_srli_si256(
        a: crate::Unity::Burst::Intrinsics::v256,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256, i32),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_srli_si256")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_srli_si256", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_srlv_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        count: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_srlv_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_srlv_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, count))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_srlv_epi64(
        a: crate::Unity::Burst::Intrinsics::v256,
        count: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_srlv_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_srlv_epi64", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, count))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_stream_load_si256(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_stream_load_si256")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_stream_load_si256", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (mem_addr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_sub_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_sub_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_sub_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_sub_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_sub_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_sub_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_sub_epi64(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_sub_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_sub_epi64", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_sub_epi8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_sub_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_sub_epi8", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_subs_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_subs_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_subs_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_subs_epi8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_subs_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_subs_epi8", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_subs_epu16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_subs_epu16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_subs_epu16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_subs_epu8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_subs_epu8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_subs_epu8", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_unpackhi_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_unpackhi_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_unpackhi_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_unpackhi_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_unpackhi_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_unpackhi_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_unpackhi_epi64(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_unpackhi_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_unpackhi_epi64", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_unpackhi_epi8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_unpackhi_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_unpackhi_epi8", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_unpacklo_epi16(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_unpacklo_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_unpacklo_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_unpacklo_epi32(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_unpacklo_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_unpacklo_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_unpacklo_epi64(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_unpacklo_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_unpacklo_epi64", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_unpacklo_epi8(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_unpacklo_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_unpacklo_epi8", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_xor_si256(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                2usize,
            >("mm256_xor_si256")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_xor_si256", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn sllv_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("sllv_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "sllv_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, count))
        };
        Ok(__cordl_ret.into())
    }
    pub fn sllv_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("sllv_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "sllv_epi64", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, count))
        };
        Ok(__cordl_ret.into())
    }
    pub fn srav_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("srav_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "srav_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, count))
        };
        Ok(__cordl_ret.into())
    }
    pub fn srlv_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("srlv_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "srlv_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, count))
        };
        Ok(__cordl_ret.into())
    }
    pub fn srlv_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("srlv_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "srlv_epi64", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, count))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Avx2")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::Intrinsics::X86_Avx2 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Bmi1")]
#[repr(C)]
#[derive(Debug)]
pub struct X86_Bmi1 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Bmi1")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Burst::Intrinsics::X86_Bmi1 {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Burst.Intrinsics";
    const CLASS_NAME: &'static str = "X86/Bmi1";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Bmi1")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86_Bmi1 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Bmi1")]
impl std::ops::DerefMut for crate::Unity::Burst::Intrinsics::X86_Bmi1 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Bmi1")]
impl crate::Unity::Burst::Intrinsics::X86_Bmi1 {
    pub fn andn_u32(a: u32, b: u32) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u32, u32), u32, 2usize>("andn_u32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "andn_u32", 2usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn andn_u64(a: u64, b: u64) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64, u64), u64, 2usize>("andn_u64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "andn_u64", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn bextr2_u32(a: u32, control: u32) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u32, u32), u32, 2usize>("bextr2_u32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "bextr2_u32", 2usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (a, control)) };
        Ok(__cordl_ret.into())
    }
    pub fn bextr2_u64(a: u64, control: u64) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64, u64), u64, 2usize>("bextr2_u64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "bextr2_u64", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (a, control)) };
        Ok(__cordl_ret.into())
    }
    pub fn bextr_u32(
        a: u32,
        start: u32,
        len: u32,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u32, u32, u32), u32, 3usize>("bextr_u32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "bextr_u32", 3usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (a, start, len)) };
        Ok(__cordl_ret.into())
    }
    pub fn bextr_u64(
        a: u64,
        start: u32,
        len: u32,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64, u32, u32), u64, 3usize>("bextr_u64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "bextr_u64", 3usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (a, start, len)) };
        Ok(__cordl_ret.into())
    }
    pub fn blsi_u32(a: u32) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u32), u32, 1usize>("blsi_u32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "blsi_u32", 1usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (a)) };
        Ok(__cordl_ret.into())
    }
    pub fn blsi_u64(a: u64) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), u64, 1usize>("blsi_u64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "blsi_u64", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (a)) };
        Ok(__cordl_ret.into())
    }
    pub fn blsmsk_u32(a: u32) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u32), u32, 1usize>("blsmsk_u32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "blsmsk_u32", 1usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (a)) };
        Ok(__cordl_ret.into())
    }
    pub fn blsmsk_u64(a: u64) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), u64, 1usize>("blsmsk_u64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "blsmsk_u64", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (a)) };
        Ok(__cordl_ret.into())
    }
    pub fn blsr_u32(a: u32) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u32), u32, 1usize>("blsr_u32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "blsr_u32", 1usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (a)) };
        Ok(__cordl_ret.into())
    }
    pub fn blsr_u64(a: u64) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), u64, 1usize>("blsr_u64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "blsr_u64", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (a)) };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsBmi1Supported() -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), bool, 0usize>("get_IsBmi1Supported")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_IsBmi1Supported", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn tzcnt_u32(a: u32) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u32), u32, 1usize>("tzcnt_u32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "tzcnt_u32", 1usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (a)) };
        Ok(__cordl_ret.into())
    }
    pub fn tzcnt_u64(a: u64) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), u64, 1usize>("tzcnt_u64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "tzcnt_u64", 1usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (a)) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Bmi1")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::Intrinsics::X86_Bmi1 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Bmi2")]
#[repr(C)]
#[derive(Debug)]
pub struct X86_Bmi2 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Bmi2")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Burst::Intrinsics::X86_Bmi2 {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Burst.Intrinsics";
    const CLASS_NAME: &'static str = "X86/Bmi2";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Bmi2")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86_Bmi2 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Bmi2")]
impl std::ops::DerefMut for crate::Unity::Burst::Intrinsics::X86_Bmi2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Bmi2")]
impl crate::Unity::Burst::Intrinsics::X86_Bmi2 {
    pub fn bzhi_u32(a: u32, index: u32) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u32, u32), u32, 2usize>("bzhi_u32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "bzhi_u32", 2usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (a, index)) };
        Ok(__cordl_ret.into())
    }
    pub fn bzhi_u64(a: u64, index: u64) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64, u64), u64, 2usize>("bzhi_u64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "bzhi_u64", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (a, index)) };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsBmi2Supported() -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), bool, 0usize>("get_IsBmi2Supported")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_IsBmi2Supported", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn mulx_u32(
        a: u32,
        b: u32,
        hi: quest_hook::libil2cpp::ByRefMut<u32>,
    ) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u32, u32, quest_hook::libil2cpp::ByRefMut<u32>),
                u32,
                3usize,
            >("mulx_u32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mulx_u32", 3usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (a, b, hi)) };
        Ok(__cordl_ret.into())
    }
    pub fn mulx_u64(
        a: u64,
        b: u64,
        hi: quest_hook::libil2cpp::ByRefMut<u64>,
    ) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (u64, u64, quest_hook::libil2cpp::ByRefMut<u64>),
                u64,
                3usize,
            >("mulx_u64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mulx_u64", 3usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (a, b, hi)) };
        Ok(__cordl_ret.into())
    }
    pub fn pdep_u32(a: u32, mask: u32) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u32, u32), u32, 2usize>("pdep_u32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "pdep_u32", 2usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (a, mask)) };
        Ok(__cordl_ret.into())
    }
    pub fn pdep_u64(a: u64, mask: u64) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64, u64), u64, 2usize>("pdep_u64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "pdep_u64", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (a, mask)) };
        Ok(__cordl_ret.into())
    }
    pub fn pext_u32(a: u32, mask: u32) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u32, u32), u32, 2usize>("pext_u32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "pext_u32", 2usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (a, mask)) };
        Ok(__cordl_ret.into())
    }
    pub fn pext_u64(a: u64, mask: u64) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64, u64), u64, 2usize>("pext_u64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "pext_u64", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (a, mask)) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Bmi2")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::Intrinsics::X86_Bmi2 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+F16C")]
#[repr(C)]
#[derive(Debug)]
pub struct X86_F16C {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+F16C")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Burst::Intrinsics::X86_F16C {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Burst.Intrinsics";
    const CLASS_NAME: &'static str = "X86/F16C";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+F16C")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86_F16C {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+F16C")]
impl std::ops::DerefMut for crate::Unity::Burst::Intrinsics::X86_F16C {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+F16C")]
impl crate::Unity::Burst::Intrinsics::X86_F16C {
    pub fn FloatToHalf(f: u32, rounding: i32) -> quest_hook::libil2cpp::Result<u16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u32, i32), u16, 2usize>("FloatToHalf")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FloatToHalf", 2usize
                )
            });
        let __cordl_ret: u16 = unsafe { method.invoke_unchecked((), (f, rounding)) };
        Ok(__cordl_ret.into())
    }
    pub fn HalfToFloat(h: u16) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u16), u32, 1usize>("HalfToFloat")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HalfToFloat", 1usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (h)) };
        Ok(__cordl_ret.into())
    }
    pub fn cvtph_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("cvtph_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtph_ps", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cvtps_ph(
        a: crate::Unity::Burst::Intrinsics::v128,
        rounding: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128, i32),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cvtps_ph")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtps_ph", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, rounding))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsF16CSupported() -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), bool, 0usize>("get_IsF16CSupported")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_IsF16CSupported", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtph_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v256,
                1usize,
            >("mm256_cvtph_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_cvtph_ps", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_cvtps_ph(
        a: crate::Unity::Burst::Intrinsics::v256,
        rounding: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v256, i32),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("mm256_cvtps_ph")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_cvtps_ph", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, rounding))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+F16C")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::Intrinsics::X86_F16C {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Fma")]
#[repr(C)]
#[derive(Debug)]
pub struct X86_Fma {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Fma")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Burst::Intrinsics::X86_Fma {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Burst.Intrinsics";
    const CLASS_NAME: &'static str = "X86/Fma";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Fma")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86_Fma {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Fma")]
impl std::ops::DerefMut for crate::Unity::Burst::Intrinsics::X86_Fma {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Fma")]
impl crate::Unity::Burst::Intrinsics::X86_Fma {
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+Fma+Union")]
    pub type Union = crate::Unity::Burst::Intrinsics::Fma_X86_Union;
    pub fn FmaHelper(a: f32, b: f32, c: f32) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32, f32, f32), f32, 3usize>("FmaHelper")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FmaHelper", 3usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (a, b, c)) };
        Ok(__cordl_ret.into())
    }
    pub fn FnmaHelper(a: f32, b: f32, c: f32) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f32, f32, f32), f32, 3usize>("FnmaHelper")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "FnmaHelper", 3usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (a, b, c)) };
        Ok(__cordl_ret.into())
    }
    pub fn fmadd_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("fmadd_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "fmadd_pd", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, c))
        };
        Ok(__cordl_ret.into())
    }
    pub fn fmadd_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("fmadd_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "fmadd_ps", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, c))
        };
        Ok(__cordl_ret.into())
    }
    pub fn fmadd_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("fmadd_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "fmadd_sd", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, c))
        };
        Ok(__cordl_ret.into())
    }
    pub fn fmadd_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("fmadd_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "fmadd_ss", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, c))
        };
        Ok(__cordl_ret.into())
    }
    pub fn fmaddsub_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("fmaddsub_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "fmaddsub_pd", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, c))
        };
        Ok(__cordl_ret.into())
    }
    pub fn fmaddsub_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("fmaddsub_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "fmaddsub_ps", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, c))
        };
        Ok(__cordl_ret.into())
    }
    pub fn fmsub_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("fmsub_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "fmsub_pd", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, c))
        };
        Ok(__cordl_ret.into())
    }
    pub fn fmsub_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("fmsub_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "fmsub_ps", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, c))
        };
        Ok(__cordl_ret.into())
    }
    pub fn fmsub_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("fmsub_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "fmsub_sd", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, c))
        };
        Ok(__cordl_ret.into())
    }
    pub fn fmsub_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("fmsub_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "fmsub_ss", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, c))
        };
        Ok(__cordl_ret.into())
    }
    pub fn fmsubadd_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("fmsubadd_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "fmsubadd_pd", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, c))
        };
        Ok(__cordl_ret.into())
    }
    pub fn fmsubadd_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("fmsubadd_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "fmsubadd_ps", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, c))
        };
        Ok(__cordl_ret.into())
    }
    pub fn fnmadd_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("fnmadd_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "fnmadd_pd", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, c))
        };
        Ok(__cordl_ret.into())
    }
    pub fn fnmadd_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("fnmadd_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "fnmadd_ps", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, c))
        };
        Ok(__cordl_ret.into())
    }
    pub fn fnmadd_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("fnmadd_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "fnmadd_sd", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, c))
        };
        Ok(__cordl_ret.into())
    }
    pub fn fnmadd_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("fnmadd_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "fnmadd_ss", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, c))
        };
        Ok(__cordl_ret.into())
    }
    pub fn fnmsub_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("fnmsub_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "fnmsub_pd", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, c))
        };
        Ok(__cordl_ret.into())
    }
    pub fn fnmsub_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("fnmsub_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "fnmsub_ps", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, c))
        };
        Ok(__cordl_ret.into())
    }
    pub fn fnmsub_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("fnmsub_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "fnmsub_sd", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, c))
        };
        Ok(__cordl_ret.into())
    }
    pub fn fnmsub_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        c: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("fnmsub_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "fnmsub_ss", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, c))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsFmaSupported() -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), bool, 0usize>("get_IsFmaSupported")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_IsFmaSupported", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_fmadd_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        c: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_fmadd_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_fmadd_pd", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b, c))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_fmadd_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        c: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_fmadd_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_fmadd_ps", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b, c))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_fmaddsub_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        c: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_fmaddsub_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_fmaddsub_pd", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b, c))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_fmaddsub_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        c: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_fmaddsub_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_fmaddsub_ps", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b, c))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_fmsub_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        c: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_fmsub_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_fmsub_pd", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b, c))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_fmsub_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        c: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_fmsub_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_fmsub_ps", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b, c))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_fmsubadd_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        c: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_fmsubadd_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_fmsubadd_pd", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b, c))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_fmsubadd_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        c: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_fmsubadd_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_fmsubadd_ps", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b, c))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_fnmadd_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        c: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_fnmadd_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_fnmadd_pd", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b, c))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_fnmadd_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        c: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_fnmadd_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_fnmadd_ps", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b, c))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_fnmsub_pd(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        c: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_fnmsub_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_fnmsub_pd", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b, c))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mm256_fnmsub_ps(
        a: crate::Unity::Burst::Intrinsics::v256,
        b: crate::Unity::Burst::Intrinsics::v256,
        c: crate::Unity::Burst::Intrinsics::v256,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v256> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                    crate::Unity::Burst::Intrinsics::v256,
                ),
                crate::Unity::Burst::Intrinsics::v256,
                3usize,
            >("mm256_fnmsub_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mm256_fnmsub_ps", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v256 = unsafe {
            method.invoke_unchecked((), (a, b, c))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Fma")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::Intrinsics::X86_Fma {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+MXCSRBits")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum X86_MXCSRBits {
    #[default]
    DenormalFlag = 2i32,
    DenormalOperationMask = 256i32,
    DenormalsAreZeroes = 64i32,
    DivideByZeroFlag = 4i32,
    DivideByZeroMask = 512i32,
    ExceptionMask = 8064i32,
    FlagMask = 63i32,
    FlushToZero = 32768i32,
    InvalidOperationFlag = 1i32,
    InvalidOperationMask = 128i32,
    OverflowFlag = 8i32,
    OverflowMask = 1024i32,
    PrecisionFlag = 32i32,
    PrecisionMask = 4096i32,
    RoundDown = 8192i32,
    RoundToNearest = 0i32,
    RoundTowardZero = 24576i32,
    RoundUp = 16384i32,
    UnderflowFlag = 16i32,
    UnderflowMask = 2048i32,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+MXCSRBits")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Burst::Intrinsics::X86_MXCSRBits {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Burst.Intrinsics";
    const CLASS_NAME: &'static str = "X86/MXCSRBits";
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
#[cfg(feature = "Unity+Burst+Intrinsics+X86+MXCSRBits")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Unity::Burst::Intrinsics::X86_MXCSRBits {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+MXCSRBits")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Unity::Burst::Intrinsics::X86_MXCSRBits {
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
#[cfg(feature = "Unity+Burst+Intrinsics+X86+MXCSRBits")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Unity::Burst::Intrinsics::X86_MXCSRBits {
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
#[cfg(feature = "Unity+Burst+Intrinsics+X86+MXCSRBits")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Unity::Burst::Intrinsics::X86_MXCSRBits {
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
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Popcnt")]
#[repr(C)]
#[derive(Debug)]
pub struct X86_Popcnt {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Popcnt")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Burst::Intrinsics::X86_Popcnt {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Burst.Intrinsics";
    const CLASS_NAME: &'static str = "X86/Popcnt";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Popcnt")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86_Popcnt {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Popcnt")]
impl std::ops::DerefMut for crate::Unity::Burst::Intrinsics::X86_Popcnt {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Popcnt")]
impl crate::Unity::Burst::Intrinsics::X86_Popcnt {
    pub fn get_IsPopcntSupported() -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), bool, 0usize>("get_IsPopcntSupported")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_IsPopcntSupported", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn popcnt_u32(v: u32) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u32), i32, 1usize>("popcnt_u32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "popcnt_u32", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (v)) };
        Ok(__cordl_ret.into())
    }
    pub fn popcnt_u64(v: u64) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64), i32, 1usize>("popcnt_u64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "popcnt_u64", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (v)) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Popcnt")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::Intrinsics::X86_Popcnt {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+RoundingMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum X86_RoundingMode {
    #[default]
    FROUND_CEIL = 2i32,
    FROUND_CEIL_NOEXC = 10i32,
    FROUND_CUR_DIRECTION = 4i32,
    FROUND_FLOOR = 1i32,
    FROUND_FLOOR_NOEXC = 9i32,
    FROUND_NEARBYINT = 12i32,
    FROUND_NINT = 0i32,
    FROUND_NINT_NOEXC = 8i32,
    FROUND_TO_ZERO = 3i32,
    FROUND_TRUNC_NOEXC = 11i32,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+RoundingMode")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Burst::Intrinsics::X86_RoundingMode {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Burst.Intrinsics";
    const CLASS_NAME: &'static str = "X86/RoundingMode";
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
#[cfg(feature = "Unity+Burst+Intrinsics+X86+RoundingMode")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Unity::Burst::Intrinsics::X86_RoundingMode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+RoundingMode")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Unity::Burst::Intrinsics::X86_RoundingMode {
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
#[cfg(feature = "Unity+Burst+Intrinsics+X86+RoundingMode")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Unity::Burst::Intrinsics::X86_RoundingMode {
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
#[cfg(feature = "Unity+Burst+Intrinsics+X86+RoundingMode")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Unity::Burst::Intrinsics::X86_RoundingMode {
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
#[cfg(feature = "Unity+Burst+Intrinsics+X86+RoundingScope")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct X86_RoundingScope {
    pub OldBits: crate::Unity::Burst::Intrinsics::X86_MXCSRBits,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+RoundingScope")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Burst::Intrinsics::X86_RoundingScope {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Burst.Intrinsics";
    const CLASS_NAME: &'static str = "X86/RoundingScope";
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
#[cfg(feature = "Unity+Burst+Intrinsics+X86+RoundingScope")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::Unity::Burst::Intrinsics::X86_RoundingScope {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+RoundingScope")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::Unity::Burst::Intrinsics::X86_RoundingScope {
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
#[cfg(feature = "Unity+Burst+Intrinsics+X86+RoundingScope")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::Unity::Burst::Intrinsics::X86_RoundingScope {
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
#[cfg(feature = "Unity+Burst+Intrinsics+X86+RoundingScope")]
unsafe impl quest_hook::libil2cpp::Return
for crate::Unity::Burst::Intrinsics::X86_RoundingScope {
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
#[cfg(feature = "Unity+Burst+Intrinsics+X86+RoundingScope")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Burst::Intrinsics::X86_RoundingScope {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+RoundingScope")]
impl crate::Unity::Burst::Intrinsics::X86_RoundingScope {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Dispose", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        roundingMode: crate::Unity::Burst::Intrinsics::X86_MXCSRBits,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::Unity::Burst::Intrinsics::X86_MXCSRBits),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (roundingMode))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+RoundingScope")]
impl AsRef<crate::System::IDisposable>
for crate::Unity::Burst::Intrinsics::X86_RoundingScope {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+RoundingScope")]
impl AsMut<crate::System::IDisposable>
for crate::Unity::Burst::Intrinsics::X86_RoundingScope {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse")]
#[repr(C)]
#[derive(Debug)]
pub struct X86_Sse {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Burst::Intrinsics::X86_Sse {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Burst.Intrinsics";
    const CLASS_NAME: &'static str = "X86/Sse";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86_Sse {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse")]
impl std::ops::DerefMut for crate::Unity::Burst::Intrinsics::X86_Sse {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse")]
impl crate::Unity::Burst::Intrinsics::X86_Sse {
    pub fn SHUFFLE(
        d: i32,
        c: i32,
        b: i32,
        a: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32, i32, i32, i32), i32, 4usize>("SHUFFLE")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SHUFFLE", 4usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (d, c, b, a)) };
        Ok(__cordl_ret.into())
    }
    pub fn TRANSPOSE4_PS(
        row0: quest_hook::libil2cpp::ByRefMut<crate::Unity::Burst::Intrinsics::v128>,
        row1: quest_hook::libil2cpp::ByRefMut<crate::Unity::Burst::Intrinsics::v128>,
        row2: quest_hook::libil2cpp::ByRefMut<crate::Unity::Burst::Intrinsics::v128>,
        row3: quest_hook::libil2cpp::ByRefMut<crate::Unity::Burst::Intrinsics::v128>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::Unity::Burst::Intrinsics::v128,
                    >,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::Unity::Burst::Intrinsics::v128,
                    >,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::Unity::Burst::Intrinsics::v128,
                    >,
                    quest_hook::libil2cpp::ByRefMut<
                        crate::Unity::Burst::Intrinsics::v128,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("TRANSPOSE4_PS")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "TRANSPOSE4_PS", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (row0, row1, row2, row3))
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("add_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("add_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_ss", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn and_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("and_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "and_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn andnot_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("andnot_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "andnot_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpeq_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpeq_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpeq_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpeq_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpeq_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpeq_ss", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpge_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpge_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpge_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpge_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpge_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpge_ss", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpgt_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpgt_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpgt_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpgt_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpgt_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpgt_ss", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmple_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmple_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmple_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmple_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmple_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmple_ss", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmplt_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmplt_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmplt_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmplt_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmplt_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmplt_ss", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpneq_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpneq_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpneq_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpneq_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpneq_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpneq_ss", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpnge_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpnge_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpnge_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpnge_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpnge_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpnge_ss", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpngt_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpngt_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpngt_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpngt_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpngt_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpngt_ss", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpnle_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpnle_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpnle_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpnle_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpnle_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpnle_ss", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpnlt_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpnlt_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpnlt_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpnlt_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpnlt_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpnlt_ss", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpord_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpord_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpord_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpord_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpord_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpord_ss", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpunord_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpunord_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpunord_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpunord_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpunord_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpunord_ss", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn comieq_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                i32,
                2usize,
            >("comieq_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "comieq_ss", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn comige_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                i32,
                2usize,
            >("comige_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "comige_ss", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn comigt_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                i32,
                2usize,
            >("comigt_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "comigt_ss", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn comile_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                i32,
                2usize,
            >("comile_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "comile_ss", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn comilt_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                i32,
                2usize,
            >("comilt_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "comilt_ss", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn comineq_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                i32,
                2usize,
            >("comineq_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "comineq_ss", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn cvt_ss2si(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                i32,
                1usize,
            >("cvt_ss2si")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvt_ss2si", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a)) };
        Ok(__cordl_ret.into())
    }
    pub fn cvtsi32_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128, i32),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cvtsi32_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtsi32_ss", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cvtsi64_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: i64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128, i64),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cvtsi64_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtsi64_ss", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cvtss_f32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                f32,
                1usize,
            >("cvtss_f32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtss_f32", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (a)) };
        Ok(__cordl_ret.into())
    }
    pub fn cvtss_si32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                i32,
                1usize,
            >("cvtss_si32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtss_si32", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a)) };
        Ok(__cordl_ret.into())
    }
    pub fn cvtss_si64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                i64,
                1usize,
            >("cvtss_si64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtss_si64", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (a)) };
        Ok(__cordl_ret.into())
    }
    pub fn cvtt_ss2si(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                i32,
                1usize,
            >("cvtt_ss2si")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtt_ss2si", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a)) };
        Ok(__cordl_ret.into())
    }
    pub fn cvttss_si32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                i32,
                1usize,
            >("cvttss_si32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvttss_si32", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a)) };
        Ok(__cordl_ret.into())
    }
    pub fn cvttss_si64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                i64,
                1usize,
            >("cvttss_si64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvttss_si64", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (a)) };
        Ok(__cordl_ret.into())
    }
    pub fn div_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("div_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "div_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn div_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("div_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "div_ss", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsSseSupported() -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), bool, 0usize>("get_IsSseSupported")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_IsSseSupported", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn load_ps(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("load_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "load_ps", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (ptr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn loadu_ps(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("loadu_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "loadu_ps", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (ptr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn loadu_si16(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("loadu_si16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "loadu_si16", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (mem_addr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn loadu_si64(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("loadu_si64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "loadu_si64", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (mem_addr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn max_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("max_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "max_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn max_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("max_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "max_ss", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn min_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("min_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "min_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn min_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("min_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "min_ss", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn move_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("move_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "move_ss", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn movehl_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("movehl_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "movehl_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn movelh_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("movelh_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "movelh_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn movemask_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                i32,
                1usize,
            >("movemask_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "movemask_ps", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a)) };
        Ok(__cordl_ret.into())
    }
    pub fn mul_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("mul_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mul_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mul_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("mul_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mul_ss", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn or_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("or_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "or_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn rcp_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("rcp_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "rcp_ps", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn rcp_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("rcp_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "rcp_ss", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn rsqrt_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("rsqrt_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "rsqrt_ps", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn rsqrt_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("rsqrt_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "rsqrt_ss", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set1_ps(
        a: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("set1_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set1_ps", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_ps(
        e3: f32,
        e2: f32,
        e1: f32,
        e0: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, f32, f32, f32),
                crate::Unity::Burst::Intrinsics::v128,
                4usize,
            >("set_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_ps", 4usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (e3, e2, e1, e0))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_ps1(
        a: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("set_ps1")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_ps1", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_ss(
        a: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("set_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_ss", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn setr_ps(
        e3: f32,
        e2: f32,
        e1: f32,
        e0: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f32, f32, f32, f32),
                crate::Unity::Burst::Intrinsics::v128,
                4usize,
            >("setr_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "setr_ps", 4usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (e3, e2, e1, e0))
        };
        Ok(__cordl_ret.into())
    }
    pub fn setzero_ps() -> quest_hook::libil2cpp::Result<
        crate::Unity::Burst::Intrinsics::v128,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::Unity::Burst::Intrinsics::v128,
                0usize,
            >("setzero_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "setzero_ps", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("shuffle_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "shuffle_ps", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn sqrt_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("sqrt_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "sqrt_ps", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn sqrt_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("sqrt_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "sqrt_ss", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn store_ps(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        val: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("store_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "store_ps", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (ptr, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn storeu_ps(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        val: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("storeu_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "storeu_ps", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (ptr, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn storeu_si16(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("storeu_si16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "storeu_si16", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (mem_addr, a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn storeu_si64(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("storeu_si64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "storeu_si64", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (mem_addr, a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn stream_ps(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("stream_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "stream_ps", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (mem_addr, a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn sub_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("sub_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "sub_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn sub_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("sub_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "sub_ss", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ucomieq_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                i32,
                2usize,
            >("ucomieq_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ucomieq_ss", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn ucomige_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                i32,
                2usize,
            >("ucomige_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ucomige_ss", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn ucomigt_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                i32,
                2usize,
            >("ucomigt_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ucomigt_ss", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn ucomile_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                i32,
                2usize,
            >("ucomile_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ucomile_ss", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn ucomilt_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                i32,
                2usize,
            >("ucomilt_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ucomilt_ss", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn ucomineq_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                i32,
                2usize,
            >("ucomineq_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ucomineq_ss", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn unpackhi_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("unpackhi_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "unpackhi_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn unpacklo_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("unpacklo_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "unpacklo_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn xor_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("xor_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "xor_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::Intrinsics::X86_Sse {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse2")]
#[repr(C)]
#[derive(Debug)]
pub struct X86_Sse2 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse2")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Burst::Intrinsics::X86_Sse2 {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Burst.Intrinsics";
    const CLASS_NAME: &'static str = "X86/Sse2";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse2")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86_Sse2 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse2")]
impl std::ops::DerefMut for crate::Unity::Burst::Intrinsics::X86_Sse2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse2")]
impl crate::Unity::Burst::Intrinsics::X86_Sse2 {
    pub fn SHUFFLE2(x: i32, y: i32) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32, i32), i32, 2usize>("SHUFFLE2")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SHUFFLE2", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (x, y)) };
        Ok(__cordl_ret.into())
    }
    pub fn add_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("add_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("add_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("add_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_epi64", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("add_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_epi8", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("add_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("add_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_sd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn adds_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("adds_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "adds_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn adds_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("adds_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "adds_epi8", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn adds_epu16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("adds_epu16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "adds_epu16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn adds_epu8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("adds_epu8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "adds_epu8", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn and_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("and_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "and_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn and_si128(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("and_si128")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "and_si128", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn andnot_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("andnot_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "andnot_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn andnot_si128(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("andnot_si128")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "andnot_si128", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn avg_epu16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("avg_epu16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "avg_epu16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn avg_epu8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("avg_epu8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "avg_epu8", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn bslli_si128(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128, i32),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("bslli_si128")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "bslli_si128", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn bsrli_si128(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128, i32),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("bsrli_si128")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "bsrli_si128", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn clflush(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("clflush")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "clflush", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (ptr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpeq_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpeq_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpeq_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpeq_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpeq_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpeq_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpeq_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpeq_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpeq_epi8", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpeq_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpeq_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpeq_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpeq_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpeq_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpeq_sd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpge_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpge_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpge_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpge_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpge_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpge_sd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpgt_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpgt_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpgt_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpgt_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpgt_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpgt_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpgt_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpgt_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpgt_epi8", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpgt_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpgt_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpgt_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpgt_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpgt_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpgt_sd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmple_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmple_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmple_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmple_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmple_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmple_sd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmplt_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmplt_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmplt_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmplt_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmplt_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmplt_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmplt_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmplt_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmplt_epi8", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmplt_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmplt_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmplt_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmplt_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmplt_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmplt_sd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpneq_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpneq_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpneq_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpneq_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpneq_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpneq_sd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpnge_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpnge_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpnge_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpnge_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpnge_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpnge_sd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpngt_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpngt_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpngt_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpngt_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpngt_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpngt_sd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpnle_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpnle_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpnle_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpnle_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpnle_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpnle_sd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpnlt_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpnlt_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpnlt_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpnlt_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpnlt_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpnlt_sd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpord_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpord_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpord_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpord_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpord_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpord_sd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpunord_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpunord_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpunord_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpunord_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpunord_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpunord_sd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn comieq_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                i32,
                2usize,
            >("comieq_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "comieq_sd", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn comige_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                i32,
                2usize,
            >("comige_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "comige_sd", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn comigt_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                i32,
                2usize,
            >("comigt_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "comigt_sd", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn comile_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                i32,
                2usize,
            >("comile_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "comile_sd", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn comilt_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                i32,
                2usize,
            >("comilt_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "comilt_sd", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn comineq_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                i32,
                2usize,
            >("comineq_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "comineq_sd", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn cvtepi32_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("cvtepi32_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtepi32_pd", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cvtepi32_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("cvtepi32_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtepi32_ps", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cvtpd_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("cvtpd_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtpd_epi32", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cvtpd_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("cvtpd_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtpd_ps", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cvtps_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("cvtps_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtps_epi32", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cvtps_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("cvtps_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtps_pd", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cvtsd_f64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                f64,
                1usize,
            >("cvtsd_f64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtsd_f64", 1usize
                )
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (a)) };
        Ok(__cordl_ret.into())
    }
    pub fn cvtsd_si32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                i32,
                1usize,
            >("cvtsd_si32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtsd_si32", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a)) };
        Ok(__cordl_ret.into())
    }
    pub fn cvtsd_si64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                i64,
                1usize,
            >("cvtsd_si64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtsd_si64", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (a)) };
        Ok(__cordl_ret.into())
    }
    pub fn cvtsd_si64x(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                i64,
                1usize,
            >("cvtsd_si64x")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtsd_si64x", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (a)) };
        Ok(__cordl_ret.into())
    }
    pub fn cvtsd_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cvtsd_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtsd_ss", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cvtsi128_si32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                i32,
                1usize,
            >("cvtsi128_si32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtsi128_si32", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a)) };
        Ok(__cordl_ret.into())
    }
    pub fn cvtsi128_si64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                i64,
                1usize,
            >("cvtsi128_si64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtsi128_si64", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (a)) };
        Ok(__cordl_ret.into())
    }
    pub fn cvtsi128_si64x(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                i64,
                1usize,
            >("cvtsi128_si64x")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtsi128_si64x", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (a)) };
        Ok(__cordl_ret.into())
    }
    pub fn cvtsi32_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128, i32),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cvtsi32_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtsi32_sd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cvtsi32_si128(
        a: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("cvtsi32_si128")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtsi32_si128", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cvtsi64_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: i64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128, i64),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cvtsi64_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtsi64_sd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cvtsi64_si128(
        a: i64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i64),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("cvtsi64_si128")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtsi64_si128", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cvtsi64x_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: i64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128, i64),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cvtsi64x_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtsi64x_sd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cvtsi64x_si128(
        a: i64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i64),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("cvtsi64x_si128")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtsi64x_si128", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cvtss_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cvtss_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtss_sd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cvttpd_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("cvttpd_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvttpd_epi32", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cvttps_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("cvttps_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvttps_epi32", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cvttsd_si32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                i32,
                1usize,
            >("cvttsd_si32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvttsd_si32", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a)) };
        Ok(__cordl_ret.into())
    }
    pub fn cvttsd_si64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                i64,
                1usize,
            >("cvttsd_si64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvttsd_si64", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (a)) };
        Ok(__cordl_ret.into())
    }
    pub fn cvttsd_si64x(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                i64,
                1usize,
            >("cvttsd_si64x")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvttsd_si64x", 1usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (a)) };
        Ok(__cordl_ret.into())
    }
    pub fn div_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("div_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "div_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn div_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("div_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "div_sd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn extract_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<u16> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128, i32),
                u16,
                2usize,
            >("extract_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "extract_epi16", 2usize
                )
            });
        let __cordl_ret: u16 = unsafe { method.invoke_unchecked((), (a, imm8)) };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsSse2Supported() -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), bool, 0usize>("get_IsSse2Supported")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_IsSse2Supported", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn insert_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        i: i32,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128, i32, i32),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("insert_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "insert_epi16", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, i, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn load_si128(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("load_si128")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "load_si128", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (ptr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn loadu_si128(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("loadu_si128")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "loadu_si128", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (ptr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn loadu_si32(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("loadu_si32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "loadu_si32", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (mem_addr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn madd_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("madd_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "madd_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn max_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("max_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "max_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn max_epu8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("max_epu8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "max_epu8", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn max_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("max_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "max_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn max_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("max_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "max_sd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn min_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("min_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "min_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn min_epu8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("min_epu8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "min_epu8", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn min_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("min_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "min_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn min_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("min_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "min_sd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn move_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("move_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "move_epi64", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn move_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("move_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "move_sd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn movemask_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                i32,
                1usize,
            >("movemask_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "movemask_epi8", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a)) };
        Ok(__cordl_ret.into())
    }
    pub fn movemask_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                i32,
                1usize,
            >("movemask_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "movemask_pd", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a)) };
        Ok(__cordl_ret.into())
    }
    pub fn mul_epu32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("mul_epu32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mul_epu32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mul_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("mul_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mul_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mul_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("mul_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mul_sd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mulhi_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("mulhi_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mulhi_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mulhi_epu16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("mulhi_epu16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mulhi_epu16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mullo_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("mullo_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mullo_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn or_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("or_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "or_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn or_si128(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("or_si128")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "or_si128", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn packs_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("packs_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "packs_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn packs_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("packs_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "packs_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn packus_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("packus_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "packus_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn sad_epu8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("sad_epu8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "sad_epu8", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set1_epi16(
        a: i16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i16),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("set1_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set1_epi16", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set1_epi32(
        a: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("set1_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set1_epi32", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set1_epi64x(
        a: i64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i64),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("set1_epi64x")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set1_epi64x", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set1_epi8(
        a: i8,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i8),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("set1_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set1_epi8", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set1_pd(
        a: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f64),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("set1_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set1_pd", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_epi16(
        e7: i16,
        e6: i16,
        e5: i16,
        e4: i16,
        e3: i16,
        e2: i16,
        e1: i16,
        e0: i16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i16, i16, i16, i16, i16, i16, i16, i16),
                crate::Unity::Burst::Intrinsics::v128,
                8usize,
            >("set_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_epi16", 8usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (e7, e6, e5, e4, e3, e2, e1, e0))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_epi32(
        e3: i32,
        e2: i32,
        e1: i32,
        e0: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, i32, i32, i32),
                crate::Unity::Burst::Intrinsics::v128,
                4usize,
            >("set_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_epi32", 4usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (e3, e2, e1, e0))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_epi64x(
        e1: i64,
        e0: i64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i64, i64),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("set_epi64x")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_epi64x", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (e1, e0))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_epi8(
        e15_: i8,
        e14_: i8,
        e13_: i8,
        e12_: i8,
        e11_: i8,
        e10_: i8,
        e9_: i8,
        e8_: i8,
        e7_: i8,
        e6_: i8,
        e5_: i8,
        e4_: i8,
        e3_: i8,
        e2_: i8,
        e1_: i8,
        e0_: i8,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8),
                crate::Unity::Burst::Intrinsics::v128,
                16usize,
            >("set_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_epi8", 16usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        e15_,
                        e14_,
                        e13_,
                        e12_,
                        e11_,
                        e10_,
                        e9_,
                        e8_,
                        e7_,
                        e6_,
                        e5_,
                        e4_,
                        e3_,
                        e2_,
                        e1_,
                        e0_,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_pd(
        e1: f64,
        e0: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f64, f64),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("set_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (e1, e0))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_pd1(
        a: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f64),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("set_pd1")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_pd1", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_sd(
        a: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f64),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("set_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_sd", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn setr_epi16(
        e7: i16,
        e6: i16,
        e5: i16,
        e4: i16,
        e3: i16,
        e2: i16,
        e1: i16,
        e0: i16,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i16, i16, i16, i16, i16, i16, i16, i16),
                crate::Unity::Burst::Intrinsics::v128,
                8usize,
            >("setr_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "setr_epi16", 8usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (e7, e6, e5, e4, e3, e2, e1, e0))
        };
        Ok(__cordl_ret.into())
    }
    pub fn setr_epi32(
        e3: i32,
        e2: i32,
        e1: i32,
        e0: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, i32, i32, i32),
                crate::Unity::Burst::Intrinsics::v128,
                4usize,
            >("setr_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "setr_epi32", 4usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (e3, e2, e1, e0))
        };
        Ok(__cordl_ret.into())
    }
    pub fn setr_epi8(
        e15_: i8,
        e14_: i8,
        e13_: i8,
        e12_: i8,
        e11_: i8,
        e10_: i8,
        e9_: i8,
        e8_: i8,
        e7_: i8,
        e6_: i8,
        e5_: i8,
        e4_: i8,
        e3_: i8,
        e2_: i8,
        e1_: i8,
        e0_: i8,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8, i8),
                crate::Unity::Burst::Intrinsics::v128,
                16usize,
            >("setr_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "setr_epi8", 16usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        e15_,
                        e14_,
                        e13_,
                        e12_,
                        e11_,
                        e10_,
                        e9_,
                        e8_,
                        e7_,
                        e6_,
                        e5_,
                        e4_,
                        e3_,
                        e2_,
                        e1_,
                        e0_,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn setr_pd(
        e1: f64,
        e0: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (f64, f64),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("setr_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "setr_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (e1, e0))
        };
        Ok(__cordl_ret.into())
    }
    pub fn setzero_si128() -> quest_hook::libil2cpp::Result<
        crate::Unity::Burst::Intrinsics::v128,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::Unity::Burst::Intrinsics::v128,
                0usize,
            >("setzero_si128")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "setzero_si128", 0usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128, i32),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("shuffle_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "shuffle_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("shuffle_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "shuffle_pd", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn shufflehi_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128, i32),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("shufflehi_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "shufflehi_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn shufflelo_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128, i32),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("shufflelo_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "shufflelo_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn sll_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("sll_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "sll_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, count))
        };
        Ok(__cordl_ret.into())
    }
    pub fn sll_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("sll_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "sll_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, count))
        };
        Ok(__cordl_ret.into())
    }
    pub fn sll_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("sll_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "sll_epi64", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, count))
        };
        Ok(__cordl_ret.into())
    }
    pub fn slli_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128, i32),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("slli_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "slli_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn slli_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128, i32),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("slli_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "slli_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn slli_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128, i32),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("slli_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "slli_epi64", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn slli_si128(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128, i32),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("slli_si128")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "slli_si128", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn sqrt_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("sqrt_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "sqrt_pd", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn sqrt_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("sqrt_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "sqrt_sd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn sra_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("sra_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "sra_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, count))
        };
        Ok(__cordl_ret.into())
    }
    pub fn sra_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("sra_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "sra_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, count))
        };
        Ok(__cordl_ret.into())
    }
    pub fn srai_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128, i32),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("srai_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "srai_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn srai_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128, i32),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("srai_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "srai_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn srl_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("srl_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "srl_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, count))
        };
        Ok(__cordl_ret.into())
    }
    pub fn srl_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("srl_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "srl_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, count))
        };
        Ok(__cordl_ret.into())
    }
    pub fn srl_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
        count: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("srl_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "srl_epi64", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, count))
        };
        Ok(__cordl_ret.into())
    }
    pub fn srli_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128, i32),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("srli_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "srli_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn srli_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128, i32),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("srli_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "srli_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn srli_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128, i32),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("srli_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "srli_epi64", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn srli_si128(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128, i32),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("srli_si128")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "srli_si128", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn store_si128(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        val: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("store_si128")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "store_si128", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (ptr, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn storeu_si128(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        val: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("storeu_si128")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "storeu_si128", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (ptr, val))
        };
        Ok(__cordl_ret.into())
    }
    pub fn storeu_si32(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("storeu_si32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "storeu_si32", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (mem_addr, a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn stream_pd(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("stream_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "stream_pd", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (mem_addr, a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn stream_si128(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("stream_si128")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "stream_si128", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (mem_addr, a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn stream_si32(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>, i32),
                quest_hook::libil2cpp::Void,
                2usize,
            >("stream_si32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "stream_si32", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (mem_addr, a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn stream_si64(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        a: i64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>, i64),
                quest_hook::libil2cpp::Void,
                2usize,
            >("stream_si64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "stream_si64", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (mem_addr, a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn sub_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("sub_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "sub_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn sub_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("sub_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "sub_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn sub_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("sub_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "sub_epi64", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn sub_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("sub_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "sub_epi8", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn sub_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("sub_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "sub_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn sub_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("sub_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "sub_sd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn subs_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("subs_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "subs_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn subs_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("subs_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "subs_epi8", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn subs_epu16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("subs_epu16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "subs_epu16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn subs_epu8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("subs_epu8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "subs_epu8", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ucomieq_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                i32,
                2usize,
            >("ucomieq_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ucomieq_sd", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn ucomige_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                i32,
                2usize,
            >("ucomige_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ucomige_sd", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn ucomigt_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                i32,
                2usize,
            >("ucomigt_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ucomigt_sd", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn ucomile_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                i32,
                2usize,
            >("ucomile_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ucomile_sd", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn ucomilt_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                i32,
                2usize,
            >("ucomilt_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ucomilt_sd", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn ucomineq_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                i32,
                2usize,
            >("ucomineq_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ucomineq_sd", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn unpackhi_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("unpackhi_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "unpackhi_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn unpackhi_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("unpackhi_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "unpackhi_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn unpackhi_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("unpackhi_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "unpackhi_epi64", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn unpackhi_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("unpackhi_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "unpackhi_epi8", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn unpackhi_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("unpackhi_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "unpackhi_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn unpacklo_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("unpacklo_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "unpacklo_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn unpacklo_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("unpacklo_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "unpacklo_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn unpacklo_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("unpacklo_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "unpacklo_epi64", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn unpacklo_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("unpacklo_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "unpacklo_epi8", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn unpacklo_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("unpacklo_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "unpacklo_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn xor_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("xor_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "xor_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn xor_si128(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("xor_si128")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "xor_si128", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse2")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::Intrinsics::X86_Sse2 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse3")]
#[repr(C)]
#[derive(Debug)]
pub struct X86_Sse3 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse3")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Burst::Intrinsics::X86_Sse3 {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Burst.Intrinsics";
    const CLASS_NAME: &'static str = "X86/Sse3";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse3")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86_Sse3 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse3")]
impl std::ops::DerefMut for crate::Unity::Burst::Intrinsics::X86_Sse3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse3")]
impl crate::Unity::Burst::Intrinsics::X86_Sse3 {
    pub fn addsub_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("addsub_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "addsub_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn addsub_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("addsub_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "addsub_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsSse3Supported() -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), bool, 0usize>("get_IsSse3Supported")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_IsSse3Supported", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn hadd_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("hadd_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "hadd_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn hadd_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("hadd_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "hadd_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn hsub_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("hsub_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "hsub_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn hsub_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("hsub_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "hsub_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn movedup_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("movedup_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "movedup_pd", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn movehdup_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("movehdup_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "movehdup_ps", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn moveldup_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("moveldup_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "moveldup_ps", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse3")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::Intrinsics::X86_Sse3 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_1")]
#[repr(C)]
#[derive(Debug)]
pub struct X86_Sse4_1 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_1")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Burst::Intrinsics::X86_Sse4_1 {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Burst.Intrinsics";
    const CLASS_NAME: &'static str = "X86/Sse4_1";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_1")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86_Sse4_1 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_1")]
impl std::ops::DerefMut for crate::Unity::Burst::Intrinsics::X86_Sse4_1 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_1")]
impl crate::Unity::Burst::Intrinsics::X86_Sse4_1 {
    pub fn MK_INSERTPS_NDX(
        srcField: i32,
        dstField: i32,
        zeroMask: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32, i32, i32), i32, 3usize>("MK_INSERTPS_NDX")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "MK_INSERTPS_NDX", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (srcField, dstField, zeroMask))
        };
        Ok(__cordl_ret.into())
    }
    pub fn RoundDImpl(d: f64, roundingMode: i32) -> quest_hook::libil2cpp::Result<f64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(f64, i32), f64, 2usize>("RoundDImpl")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RoundDImpl", 2usize
                )
            });
        let __cordl_ret: f64 = unsafe { method.invoke_unchecked((), (d, roundingMode)) };
        Ok(__cordl_ret.into())
    }
    pub fn blend_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("blend_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "blend_epi16", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn blend_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("blend_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "blend_pd", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn blend_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("blend_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "blend_ps", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn blendv_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        mask: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("blendv_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "blendv_epi8", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, mask))
        };
        Ok(__cordl_ret.into())
    }
    pub fn blendv_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        mask: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("blendv_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "blendv_pd", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, mask))
        };
        Ok(__cordl_ret.into())
    }
    pub fn blendv_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        mask: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("blendv_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "blendv_ps", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, mask))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ceil_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("ceil_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ceil_pd", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ceil_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("ceil_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ceil_ps", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ceil_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("ceil_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ceil_sd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ceil_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("ceil_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ceil_ss", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpeq_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpeq_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpeq_epi64", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cvtepi16_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("cvtepi16_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtepi16_epi32", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cvtepi16_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("cvtepi16_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtepi16_epi64", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cvtepi32_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("cvtepi32_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtepi32_epi64", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cvtepi8_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("cvtepi8_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtepi8_epi16", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cvtepi8_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("cvtepi8_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtepi8_epi32", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cvtepi8_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("cvtepi8_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtepi8_epi64", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cvtepu16_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("cvtepu16_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtepu16_epi32", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cvtepu16_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("cvtepu16_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtepu16_epi64", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cvtepu32_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("cvtepu32_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtepu32_epi64", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cvtepu8_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("cvtepu8_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtepu8_epi16", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cvtepu8_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("cvtepu8_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtepu8_epi32", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cvtepu8_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("cvtepu8_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cvtepu8_epi64", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn dp_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("dp_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "dp_pd", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn dp_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("dp_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "dp_ps", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn extract_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128, i32),
                i32,
                2usize,
            >("extract_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "extract_epi32", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, imm8)) };
        Ok(__cordl_ret.into())
    }
    pub fn extract_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<i64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128, i32),
                i64,
                2usize,
            >("extract_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "extract_epi64", 2usize
                )
            });
        let __cordl_ret: i64 = unsafe { method.invoke_unchecked((), (a, imm8)) };
        Ok(__cordl_ret.into())
    }
    pub fn extract_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<u8> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128, i32),
                u8,
                2usize,
            >("extract_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "extract_epi8", 2usize
                )
            });
        let __cordl_ret: u8 = unsafe { method.invoke_unchecked((), (a, imm8)) };
        Ok(__cordl_ret.into())
    }
    pub fn extract_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128, i32),
                i32,
                2usize,
            >("extract_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "extract_ps", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, imm8)) };
        Ok(__cordl_ret.into())
    }
    pub fn extractf_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128, i32),
                f32,
                2usize,
            >("extractf_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "extractf_ps", 2usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked((), (a, imm8)) };
        Ok(__cordl_ret.into())
    }
    pub fn floor_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("floor_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "floor_pd", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn floor_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("floor_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "floor_ps", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn floor_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("floor_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "floor_sd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn floor_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("floor_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "floor_ss", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsSse41Supported() -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), bool, 0usize>("get_IsSse41Supported")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_IsSse41Supported", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn insert_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        i: i32,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128, i32, i32),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("insert_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "insert_epi32", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, i, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn insert_epi64(
        a: crate::Unity::Burst::Intrinsics::v128,
        i: i64,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128, i64, i32),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("insert_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "insert_epi64", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, i, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn insert_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
        i: u8,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128, u8, i32),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("insert_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "insert_epi8", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, i, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn insert_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("insert_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "insert_ps", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn max_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("max_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "max_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn max_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("max_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "max_epi8", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn max_epu16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("max_epu16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "max_epu16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn max_epu32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("max_epu32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "max_epu32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn min_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("min_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "min_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn min_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("min_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "min_epi8", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn min_epu16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("min_epu16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "min_epu16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn min_epu32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("min_epu32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "min_epu32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn minpos_epu16(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("minpos_epu16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "minpos_epu16", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mpsadbw_epu8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("mpsadbw_epu8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mpsadbw_epu8", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mul_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("mul_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mul_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mullo_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("mullo_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mullo_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn packus_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("packus_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "packus_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn round_pd(
        a: crate::Unity::Burst::Intrinsics::v128,
        rounding: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128, i32),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("round_pd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "round_pd", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, rounding))
        };
        Ok(__cordl_ret.into())
    }
    pub fn round_ps(
        a: crate::Unity::Burst::Intrinsics::v128,
        rounding: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128, i32),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("round_ps")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "round_ps", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, rounding))
        };
        Ok(__cordl_ret.into())
    }
    pub fn round_sd(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        rounding: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("round_sd")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "round_sd", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, rounding))
        };
        Ok(__cordl_ret.into())
    }
    pub fn round_ss(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        rounding: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("round_ss")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "round_ss", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, rounding))
        };
        Ok(__cordl_ret.into())
    }
    pub fn stream_load_si128(
        mem_addr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("stream_load_si128")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "stream_load_si128", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (mem_addr))
        };
        Ok(__cordl_ret.into())
    }
    pub fn test_all_ones(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                i32,
                1usize,
            >("test_all_ones")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "test_all_ones", 1usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a)) };
        Ok(__cordl_ret.into())
    }
    pub fn test_all_zeros(
        a: crate::Unity::Burst::Intrinsics::v128,
        mask: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                i32,
                2usize,
            >("test_all_zeros")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "test_all_zeros", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, mask)) };
        Ok(__cordl_ret.into())
    }
    pub fn test_mix_ones_zeroes(
        a: crate::Unity::Burst::Intrinsics::v128,
        mask: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                i32,
                2usize,
            >("test_mix_ones_zeroes")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "test_mix_ones_zeroes", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, mask)) };
        Ok(__cordl_ret.into())
    }
    pub fn testc_si128(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                i32,
                2usize,
            >("testc_si128")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "testc_si128", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn testnzc_si128(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                i32,
                2usize,
            >("testnzc_si128")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "testnzc_si128", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
    pub fn testz_si128(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                i32,
                2usize,
            >("testz_si128")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "testz_si128", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b)) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_1")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::Intrinsics::X86_Sse4_1 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2")]
#[repr(C)]
#[derive(Debug)]
pub struct X86_Sse4_2 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Burst::Intrinsics::X86_Sse4_2 {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Burst.Intrinsics";
    const CLASS_NAME: &'static str = "X86/Sse4_2";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86_Sse4_2 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2")]
impl std::ops::DerefMut for crate::Unity::Burst::Intrinsics::X86_Sse4_2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2")]
impl crate::Unity::Burst::Intrinsics::X86_Sse4_2 {
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2+SIDD")]
    pub type SIDD = crate::Unity::Burst::Intrinsics::Sse4_2_X86_SIDD;
    #[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2+StrBoolArray")]
    pub type StrBoolArray = crate::Unity::Burst::Intrinsics::Sse4_2_X86_StrBoolArray;
    pub fn ComputeStrCmpIntRes2<T>(
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        alen: i32,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        blen: i32,
        len: i32,
        imm8: i32,
        allOnes: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    i32,
                    i32,
                    i32,
                    i32,
                ),
                i32,
                7usize,
            >("ComputeStrCmpIntRes2")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ComputeStrCmpIntRes2", 7usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (a, alen, b, blen, len, imm8, allOnes))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeStriOutput(
        len: i32,
        imm8: i32,
        intRes2: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(i32, i32, i32), i32, 3usize>("ComputeStriOutput")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ComputeStriOutput", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (len, imm8, intRes2))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeStringLength<T>(
        ptr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        max: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>, i32),
                i32,
                2usize,
            >("ComputeStringLength")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ComputeStringLength", 2usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (ptr, max)) };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeStrmOutput<T>(
        len: i32,
        imm8: i32,
        allOnesT: T,
        intRes2: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, i32, T, i32),
                crate::Unity::Burst::Intrinsics::v128,
                4usize,
            >("ComputeStrmOutput")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ComputeStrmOutput", 4usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (len, imm8, allOnesT, intRes2))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpestra(
        a: crate::Unity::Burst::Intrinsics::v128,
        la: i32,
        b: crate::Unity::Burst::Intrinsics::v128,
        lb: i32,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                    i32,
                ),
                i32,
                5usize,
            >("cmpestra")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpestra", 5usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (a, la, b, lb, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpestrc(
        a: crate::Unity::Burst::Intrinsics::v128,
        la: i32,
        b: crate::Unity::Burst::Intrinsics::v128,
        lb: i32,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                    i32,
                ),
                i32,
                5usize,
            >("cmpestrc")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpestrc", 5usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (a, la, b, lb, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpestri(
        a: crate::Unity::Burst::Intrinsics::v128,
        la: i32,
        b: crate::Unity::Burst::Intrinsics::v128,
        lb: i32,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                    i32,
                ),
                i32,
                5usize,
            >("cmpestri")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpestri", 5usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (a, la, b, lb, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpestri_emulation<T>(
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        alen: i32,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        blen: i32,
        len: i32,
        imm8: i32,
        allOnes: i32,
        allOnesT: T,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    i32,
                    i32,
                    i32,
                    i32,
                    T,
                ),
                i32,
                8usize,
            >("cmpestri_emulation")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpestri_emulation", 8usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (a, alen, b, blen, len, imm8, allOnes, allOnesT))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpestrm(
        a: crate::Unity::Burst::Intrinsics::v128,
        la: i32,
        b: crate::Unity::Burst::Intrinsics::v128,
        lb: i32,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                5usize,
            >("cmpestrm")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpestrm", 5usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, la, b, lb, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpestrm_emulation<T>(
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        alen: i32,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        blen: i32,
        len: i32,
        imm8: i32,
        allOnes: i32,
        allOnesT: T,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    i32,
                    i32,
                    i32,
                    i32,
                    T,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                8usize,
            >("cmpestrm_emulation")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpestrm_emulation", 8usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, alen, b, blen, len, imm8, allOnes, allOnesT))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpestro(
        a: crate::Unity::Burst::Intrinsics::v128,
        la: i32,
        b: crate::Unity::Burst::Intrinsics::v128,
        lb: i32,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                    i32,
                ),
                i32,
                5usize,
            >("cmpestro")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpestro", 5usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (a, la, b, lb, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpestrs(
        a: crate::Unity::Burst::Intrinsics::v128,
        la: i32,
        b: crate::Unity::Burst::Intrinsics::v128,
        lb: i32,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                    i32,
                ),
                i32,
                5usize,
            >("cmpestrs")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpestrs", 5usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (a, la, b, lb, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpestrz(
        a: crate::Unity::Burst::Intrinsics::v128,
        la: i32,
        b: crate::Unity::Burst::Intrinsics::v128,
        lb: i32,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                    i32,
                ),
                i32,
                5usize,
            >("cmpestrz")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpestrz", 5usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (a, la, b, lb, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpgt_epi64(
        val1: crate::Unity::Burst::Intrinsics::v128,
        val2: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("cmpgt_epi64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpgt_epi64", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (val1, val2))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpistra(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                i32,
                3usize,
            >("cmpistra")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpistra", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b, imm8)) };
        Ok(__cordl_ret.into())
    }
    pub fn cmpistrc(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                i32,
                3usize,
            >("cmpistrc")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpistrc", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b, imm8)) };
        Ok(__cordl_ret.into())
    }
    pub fn cmpistri(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                i32,
                3usize,
            >("cmpistri")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpistri", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b, imm8)) };
        Ok(__cordl_ret.into())
    }
    pub fn cmpistri_emulation<T>(
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        len: i32,
        imm8: i32,
        allOnes: i32,
        allOnesT: T,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    i32,
                    i32,
                    i32,
                    T,
                ),
                i32,
                6usize,
            >("cmpistri_emulation")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpistri_emulation", 6usize
                )
            });
        let __cordl_ret: i32 = unsafe {
            method.invoke_unchecked((), (a, b, len, imm8, allOnes, allOnesT))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpistrm(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("cmpistrm")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpistrm", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, imm8))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpistrm_emulation<T>(
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        len: i32,
        imm8: i32,
        allOnes: i32,
        allOnesT: T,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    i32,
                    i32,
                    i32,
                    T,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                6usize,
            >("cmpistrm_emulation")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpistrm_emulation", 6usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, len, imm8, allOnes, allOnesT))
        };
        Ok(__cordl_ret.into())
    }
    pub fn cmpistro(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                i32,
                3usize,
            >("cmpistro")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpistro", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b, imm8)) };
        Ok(__cordl_ret.into())
    }
    pub fn cmpistrs(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                i32,
                3usize,
            >("cmpistrs")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpistrs", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b, imm8)) };
        Ok(__cordl_ret.into())
    }
    pub fn cmpistrz(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        imm8: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                i32,
                3usize,
            >("cmpistrz")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "cmpistrz", 3usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked((), (a, b, imm8)) };
        Ok(__cordl_ret.into())
    }
    pub fn crc32_u16(crc: u32, v: u16) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u32, u16), u32, 2usize>("crc32_u16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "crc32_u16", 2usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (crc, v)) };
        Ok(__cordl_ret.into())
    }
    pub fn crc32_u32(crc: u32, v: u32) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u32, u32), u32, 2usize>("crc32_u32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "crc32_u32", 2usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (crc, v)) };
        Ok(__cordl_ret.into())
    }
    pub fn crc32_u64_i64_0(crc_ul: u64, v: i64) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64, i64), u64, 2usize>("crc32_u64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "crc32_u64", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (crc_ul, v)) };
        Ok(__cordl_ret.into())
    }
    pub fn crc32_u64_u64_1(crc_ul: u64, v: u64) -> quest_hook::libil2cpp::Result<u64> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u64, u64), u64, 2usize>("crc32_u64")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "crc32_u64", 2usize
                )
            });
        let __cordl_ret: u64 = unsafe { method.invoke_unchecked((), (crc_ul, v)) };
        Ok(__cordl_ret.into())
    }
    pub fn crc32_u8(crc: u32, v: u8) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(u32, u8), u32, 2usize>("crc32_u8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "crc32_u8", 2usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked((), (crc, v)) };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsSse42Supported() -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), bool, 0usize>("get_IsSse42Supported")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_IsSse42Supported", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Sse4_2")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::Intrinsics::X86_Sse4_2 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Ssse3")]
#[repr(C)]
#[derive(Debug)]
pub struct X86_Ssse3 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Ssse3")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Burst::Intrinsics::X86_Ssse3 {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Burst.Intrinsics";
    const CLASS_NAME: &'static str = "X86/Ssse3";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Ssse3")]
impl std::ops::Deref for crate::Unity::Burst::Intrinsics::X86_Ssse3 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Ssse3")]
impl std::ops::DerefMut for crate::Unity::Burst::Intrinsics::X86_Ssse3 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Ssse3")]
impl crate::Unity::Burst::Intrinsics::X86_Ssse3 {
    pub fn abs_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("abs_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "abs_epi16", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn abs_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("abs_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "abs_epi32", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn abs_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::Unity::Burst::Intrinsics::v128),
                crate::Unity::Burst::Intrinsics::v128,
                1usize,
            >("abs_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "abs_epi8", 1usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a))
        };
        Ok(__cordl_ret.into())
    }
    pub fn alignr_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                    i32,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                3usize,
            >("alignr_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "alignr_epi8", 3usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b, count))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_IsSsse3Supported() -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<(), bool, 0usize>("get_IsSsse3Supported")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_IsSsse3Supported", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn hadd_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("hadd_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "hadd_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn hadd_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("hadd_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "hadd_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn hadds_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("hadds_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "hadds_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn hsub_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("hsub_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "hsub_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn hsub_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("hsub_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "hsub_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn hsubs_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("hsubs_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "hsubs_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn maddubs_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("maddubs_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "maddubs_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn mulhrs_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("mulhrs_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "mulhrs_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn shuffle_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("shuffle_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "shuffle_epi8", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn sign_epi16(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("sign_epi16")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "sign_epi16", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn sign_epi32(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("sign_epi32")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "sign_epi32", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
    pub fn sign_epi8(
        a: crate::Unity::Burst::Intrinsics::v128,
        b: crate::Unity::Burst::Intrinsics::v128,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Burst::Intrinsics::v128> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Burst::Intrinsics::v128,
                    crate::Unity::Burst::Intrinsics::v128,
                ),
                crate::Unity::Burst::Intrinsics::v128,
                2usize,
            >("sign_epi8")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "sign_epi8", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Burst::Intrinsics::v128 = unsafe {
            method.invoke_unchecked((), (a, b))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Burst+Intrinsics+X86+Ssse3")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Burst::Intrinsics::X86_Ssse3 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
