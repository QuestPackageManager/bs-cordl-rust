#[cfg(feature = "Unity+Mathematics+bool2x4")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct bool2x4 {
    pub c0: crate::Unity::Mathematics::bool2,
    pub c1: crate::Unity::Mathematics::bool2,
    pub c2: crate::Unity::Mathematics::bool2,
    pub c3: crate::Unity::Mathematics::bool2,
}
#[cfg(feature = "Unity+Mathematics+bool2x4")]
unsafe impl quest_hook::libil2cpp::Type for crate::Unity::Mathematics::bool2x4 {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Mathematics";
    const CLASS_NAME: &'static str = "bool2x4";
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
unsafe impl quest_hook::libil2cpp::Argument for crate::Unity::Mathematics::bool2x4 {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter for crate::Unity::Mathematics::bool2x4 {
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
unsafe impl quest_hook::libil2cpp::Returned for crate::Unity::Mathematics::bool2x4 {
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
unsafe impl quest_hook::libil2cpp::Return for crate::Unity::Mathematics::bool2x4 {
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
#[cfg(feature = "Unity+Mathematics+bool2x4")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::Unity::Mathematics::bool2x4 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Mathematics+bool2x4")]
impl crate::Unity::Mathematics::bool2x4 {
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
    pub fn Equals_bool2x4_0(
        &mut self,
        rhs: crate::Unity::Mathematics::bool2x4,
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
    pub fn _ctor__cordl_bool2(
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
    pub fn _ctor__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool1(
        &mut self,
        m00: bool,
        m01: bool,
        m02: bool,
        m03: bool,
        m10: bool,
        m11: bool,
        m12: bool,
        m13: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (m00, m01, m02, m03, m10, m11, m12, m13),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_bool2_bool2_bool2_bool2_0(
        &mut self,
        c0: crate::Unity::Mathematics::bool2,
        c1: crate::Unity::Mathematics::bool2,
        c2: crate::Unity::Mathematics::bool2,
        c3: crate::Unity::Mathematics::bool2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (c0, c1, c2, c3),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::bool2>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Mathematics::bool2,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Item", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseAnd__cordl_bool_bool2x4_2(
        lhs: bool,
        rhs: crate::Unity::Mathematics::bool2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_BitwiseAnd", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseAnd_bool2x4__cordl_bool1(
        lhs: crate::Unity::Mathematics::bool2x4,
        rhs: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_BitwiseAnd", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseAnd_bool2x4_bool2x4_0(
        lhs: crate::Unity::Mathematics::bool2x4,
        rhs: crate::Unity::Mathematics::bool2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_BitwiseAnd", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseOr__cordl_bool_bool2x4_2(
        lhs: bool,
        rhs: crate::Unity::Mathematics::bool2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_BitwiseOr", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseOr_bool2x4__cordl_bool1(
        lhs: crate::Unity::Mathematics::bool2x4,
        rhs: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_BitwiseOr", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseOr_bool2x4_bool2x4_0(
        lhs: crate::Unity::Mathematics::bool2x4,
        rhs: crate::Unity::Mathematics::bool2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_BitwiseOr", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality__cordl_bool_bool2x4_2(
        lhs: bool,
        rhs: crate::Unity::Mathematics::bool2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_bool2x4__cordl_bool1(
        lhs: crate::Unity::Mathematics::bool2x4,
        rhs: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_bool2x4_bool2x4_0(
        lhs: crate::Unity::Mathematics::bool2x4,
        rhs: crate::Unity::Mathematics::bool2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_ExclusiveOr__cordl_bool_bool2x4_2(
        lhs: bool,
        rhs: crate::Unity::Mathematics::bool2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_ExclusiveOr", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_ExclusiveOr_bool2x4__cordl_bool1(
        lhs: crate::Unity::Mathematics::bool2x4,
        rhs: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_ExclusiveOr", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_ExclusiveOr_bool2x4_bool2x4_0(
        lhs: crate::Unity::Mathematics::bool2x4,
        rhs: crate::Unity::Mathematics::bool2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_ExclusiveOr", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality__cordl_bool_bool2x4_2(
        lhs: bool,
        rhs: crate::Unity::Mathematics::bool2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_bool2x4__cordl_bool1(
        lhs: crate::Unity::Mathematics::bool2x4,
        rhs: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_bool2x4_bool2x4_0(
        lhs: crate::Unity::Mathematics::bool2x4,
        rhs: crate::Unity::Mathematics::bool2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LogicalNot(
        val: crate::Unity::Mathematics::bool2x4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x4> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LogicalNot", (val))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Mathematics+bool2x4")]
impl AsRef<crate::System::IEquatable_1<crate::Unity::Mathematics::bool2x4>>
for crate::Unity::Mathematics::bool2x4 {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::Unity::Mathematics::bool2x4> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+bool2x4")]
impl AsMut<crate::System::IEquatable_1<crate::Unity::Mathematics::bool2x4>>
for crate::Unity::Mathematics::bool2x4 {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::Unity::Mathematics::bool2x4> {
        todo!()
    }
}
