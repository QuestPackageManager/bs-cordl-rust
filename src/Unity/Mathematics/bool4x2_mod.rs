#[cfg(feature = "Unity+Mathematics+bool4x2")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct bool4x2 {
    pub c0: crate::Unity::Mathematics::bool4,
    pub c1: crate::Unity::Mathematics::bool4,
}
#[cfg(feature = "Unity+Mathematics+bool4x2")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Mathematics::bool4x2 =>
    "Unity.Mathematics"."bool4x2"
);
#[cfg(feature = "Unity+Mathematics+bool4x2")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::Unity::Mathematics::bool4x2 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Mathematics+bool4x2")]
impl crate::Unity::Mathematics::bool4x2 {
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
    pub fn Equals_bool4x2_0(
        &mut self,
        rhs: crate::Unity::Mathematics::bool4x2,
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
        m10: bool,
        m11: bool,
        m20: bool,
        m21: bool,
        m30: bool,
        m31: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (m00, m01, m10, m11, m20, m21, m30, m31),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_bool4_bool4_0(
        &mut self,
        c0: crate::Unity::Mathematics::bool4,
        c1: crate::Unity::Mathematics::bool4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (c0, c1),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::bool4>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Mathematics::bool4,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Item", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseAnd__cordl_bool_bool4x2_2(
        lhs: bool,
        rhs: crate::Unity::Mathematics::bool4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_BitwiseAnd", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseAnd_bool4x2__cordl_bool1(
        lhs: crate::Unity::Mathematics::bool4x2,
        rhs: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_BitwiseAnd", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseAnd_bool4x2_bool4x2_0(
        lhs: crate::Unity::Mathematics::bool4x2,
        rhs: crate::Unity::Mathematics::bool4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_BitwiseAnd", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseOr__cordl_bool_bool4x2_2(
        lhs: bool,
        rhs: crate::Unity::Mathematics::bool4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_BitwiseOr", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseOr_bool4x2__cordl_bool1(
        lhs: crate::Unity::Mathematics::bool4x2,
        rhs: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_BitwiseOr", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseOr_bool4x2_bool4x2_0(
        lhs: crate::Unity::Mathematics::bool4x2,
        rhs: crate::Unity::Mathematics::bool4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_BitwiseOr", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality__cordl_bool_bool4x2_2(
        lhs: bool,
        rhs: crate::Unity::Mathematics::bool4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_bool4x2__cordl_bool1(
        lhs: crate::Unity::Mathematics::bool4x2,
        rhs: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_bool4x2_bool4x2_0(
        lhs: crate::Unity::Mathematics::bool4x2,
        rhs: crate::Unity::Mathematics::bool4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_ExclusiveOr__cordl_bool_bool4x2_2(
        lhs: bool,
        rhs: crate::Unity::Mathematics::bool4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_ExclusiveOr", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_ExclusiveOr_bool4x2__cordl_bool1(
        lhs: crate::Unity::Mathematics::bool4x2,
        rhs: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_ExclusiveOr", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_ExclusiveOr_bool4x2_bool4x2_0(
        lhs: crate::Unity::Mathematics::bool4x2,
        rhs: crate::Unity::Mathematics::bool4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_ExclusiveOr", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality__cordl_bool_bool4x2_2(
        lhs: bool,
        rhs: crate::Unity::Mathematics::bool4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_bool4x2__cordl_bool1(
        lhs: crate::Unity::Mathematics::bool4x2,
        rhs: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_bool4x2_bool4x2_0(
        lhs: crate::Unity::Mathematics::bool4x2,
        rhs: crate::Unity::Mathematics::bool4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LogicalNot(
        val: crate::Unity::Mathematics::bool4x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool4x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool4x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LogicalNot", (val))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Mathematics+bool4x2")]
impl AsRef<crate::System::IEquatable_1<crate::Unity::Mathematics::bool4x2>>
for crate::Unity::Mathematics::bool4x2 {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::Unity::Mathematics::bool4x2> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+bool4x2")]
impl AsMut<crate::System::IEquatable_1<crate::Unity::Mathematics::bool4x2>>
for crate::Unity::Mathematics::bool4x2 {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::Unity::Mathematics::bool4x2> {
        todo!()
    }
}
