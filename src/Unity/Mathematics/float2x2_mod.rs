#[cfg(feature = "Unity+Mathematics+float2x2")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct float2x2 {
    pub c0: crate::Unity::Mathematics::float2,
    pub c1: crate::Unity::Mathematics::float2,
}
#[cfg(feature = "Unity+Mathematics+float2x2")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Mathematics::float2x2 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Mathematics";
    const CLASS_NAME: &'static str = "float2x2";
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
unsafe impl quest_hook::libil2cpp::Argument for crate::Unity::Mathematics::float2x2 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter for crate::Unity::Mathematics::float2x2 {
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
unsafe impl quest_hook::libil2cpp::Returned for crate::Unity::Mathematics::float2x2 {
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
unsafe impl quest_hook::libil2cpp::Return for crate::Unity::Mathematics::float2x2 {
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
#[cfg(feature = "Unity+Mathematics+float2x2")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::Unity::Mathematics::float2x2 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Mathematics+float2x2")]
impl crate::Unity::Mathematics::float2x2 {
    pub fn Equals_Il2CppObject1(
        &mut self,
        o: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (o),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_float2x2_0(
        &mut self,
        rhs: crate::Unity::Mathematics::float2x2,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (rhs),
        )?;
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
    pub fn Rotate(
        angle: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Rotate", (angle))?;
        Ok(__cordl_ret.into())
    }
    pub fn Scale_f32_0(
        s: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Scale", (s))?;
        Ok(__cordl_ret.into())
    }
    pub fn Scale_f32_f32_1(
        x: f32,
        y: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Scale", (x, y))?;
        Ok(__cordl_ret.into())
    }
    pub fn Scale_float2_2(
        v: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Scale", (v))?;
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
    pub fn ToString_Il2CppString_IFormatProvider1(
        &mut self,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        formatProvider: quest_hook::libil2cpp::Gc<crate::System::IFormatProvider>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (format, formatProvider),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool3(
        &mut self,
        v: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_bool2x2_4(
        &mut self,
        v: crate::Unity::Mathematics::bool2x2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_double2x2_10(
        &mut self,
        v: crate::Unity::Mathematics::double2x2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f32_2(
        &mut self,
        v: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f32_f32_f32_f32_1(
        &mut self,
        m00: f32,
        m01: f32,
        m10: f32,
        m11: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (m00, m01, m10, m11),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f64_9(
        &mut self,
        v: f64,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_float2_float2_0(
        &mut self,
        c0: crate::Unity::Mathematics::float2,
        c1: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (c0, c1),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_5(
        &mut self,
        v: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_int2x2_6(
        &mut self,
        v: crate::Unity::Mathematics::int2x2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_u32_7(
        &mut self,
        v: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_uint2x2_8(
        &mut self,
        v: crate::Unity::Mathematics::uint2x2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (v),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::float2>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Mathematics::float2,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Item", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition_f32_float2x2_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition_float2x2_f32_1(
        lhs: crate::Unity::Mathematics::float2x2,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition_float2x2_float2x2_0(
        lhs: crate::Unity::Mathematics::float2x2,
        rhs: crate::Unity::Mathematics::float2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Decrement(
        val: crate::Unity::Mathematics::float2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Decrement", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Division_f32_float2x2_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Division", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Division_float2x2_f32_1(
        lhs: crate::Unity::Mathematics::float2x2,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Division", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Division_float2x2_float2x2_0(
        lhs: crate::Unity::Mathematics::float2x2,
        rhs: crate::Unity::Mathematics::float2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Division", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_f32_float2x2_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_float2x2_f32_1(
        lhs: crate::Unity::Mathematics::float2x2,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_float2x2_float2x2_0(
        lhs: crate::Unity::Mathematics::float2x2,
        rhs: crate::Unity::Mathematics::float2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit__cordl_bool0(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_bool2x2_1(
        v: crate::Unity::Mathematics::bool2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_double2x2_3(
        v: crate::Unity::Mathematics::double2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_f64_2(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual_f32_float2x2_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual_float2x2_f32_1(
        lhs: crate::Unity::Mathematics::float2x2,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThanOrEqual_float2x2_float2x2_0(
        lhs: crate::Unity::Mathematics::float2x2,
        rhs: crate::Unity::Mathematics::float2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan_f32_float2x2_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan_float2x2_f32_1(
        lhs: crate::Unity::Mathematics::float2x2,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_GreaterThan_float2x2_float2x2_0(
        lhs: crate::Unity::Mathematics::float2x2,
        rhs: crate::Unity::Mathematics::float2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_GreaterThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_f32_0(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_i32_1(
        v: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_int2x2_2(
        v: crate::Unity::Mathematics::int2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_u32_3(
        v: u32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_uint2x2_4(
        v: crate::Unity::Mathematics::uint2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Increment(
        val: crate::Unity::Mathematics::float2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Increment", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_f32_float2x2_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_float2x2_f32_1(
        lhs: crate::Unity::Mathematics::float2x2,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_float2x2_float2x2_0(
        lhs: crate::Unity::Mathematics::float2x2,
        rhs: crate::Unity::Mathematics::float2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_f32_float2x2_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_float2x2_f32_1(
        lhs: crate::Unity::Mathematics::float2x2,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThanOrEqual_float2x2_float2x2_0(
        lhs: crate::Unity::Mathematics::float2x2,
        rhs: crate::Unity::Mathematics::float2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThanOrEqual", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_f32_float2x2_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_float2x2_f32_1(
        lhs: crate::Unity::Mathematics::float2x2,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LessThan_float2x2_float2x2_0(
        lhs: crate::Unity::Mathematics::float2x2,
        rhs: crate::Unity::Mathematics::float2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LessThan", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_f32_float2x2_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Modulus", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_float2x2_f32_1(
        lhs: crate::Unity::Mathematics::float2x2,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Modulus", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Modulus_float2x2_float2x2_0(
        lhs: crate::Unity::Mathematics::float2x2,
        rhs: crate::Unity::Mathematics::float2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Modulus", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_f32_float2x2_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_float2x2_f32_1(
        lhs: crate::Unity::Mathematics::float2x2,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_float2x2_float2x2_0(
        lhs: crate::Unity::Mathematics::float2x2,
        rhs: crate::Unity::Mathematics::float2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_f32_float2x2_2(
        lhs: f32,
        rhs: crate::Unity::Mathematics::float2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_float2x2_f32_1(
        lhs: crate::Unity::Mathematics::float2x2,
        rhs: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction_float2x2_float2x2_0(
        lhs: crate::Unity::Mathematics::float2x2,
        rhs: crate::Unity::Mathematics::float2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_UnaryNegation(
        val: crate::Unity::Mathematics::float2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_UnaryNegation", (val))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_UnaryPlus(
        val: crate::Unity::Mathematics::float2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2x2> {
        let __cordl_ret: crate::Unity::Mathematics::float2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_UnaryPlus", (val))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Mathematics+float2x2")]
impl AsRef<crate::System::IEquatable_1<crate::Unity::Mathematics::float2x2>>
for crate::Unity::Mathematics::float2x2 {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::Unity::Mathematics::float2x2> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+float2x2")]
impl AsMut<crate::System::IEquatable_1<crate::Unity::Mathematics::float2x2>>
for crate::Unity::Mathematics::float2x2 {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::Unity::Mathematics::float2x2> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+float2x2")]
impl AsRef<crate::System::IFormattable> for crate::Unity::Mathematics::float2x2 {
    fn as_ref(&self) -> &crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+float2x2")]
impl AsMut<crate::System::IFormattable> for crate::Unity::Mathematics::float2x2 {
    fn as_mut(&mut self) -> &mut crate::System::IFormattable {
        todo!()
    }
}
