#[cfg(feature = "Unity+Mathematics+bool2x2")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct bool2x2 {
    pub c0: crate::Unity::Mathematics::bool2,
    pub c1: crate::Unity::Mathematics::bool2,
}
#[cfg(feature = "Unity+Mathematics+bool2x2")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Mathematics::bool2x2 =>
    "Unity.Mathematics"."bool2x2"
);
#[cfg(feature = "Unity+Mathematics+bool2x2")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::Unity::Mathematics::bool2x2 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Mathematics+bool2x2")]
impl crate::Unity::Mathematics::bool2x2 {
    pub fn Equals_Gc1(
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
    pub fn Equals_bool2x2_0(
        &mut self,
        rhs: crate::Unity::Mathematics::bool2x2,
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
    pub fn _ctor__cordl_bool__cordl_bool__cordl_bool__cordl_bool1(
        &mut self,
        m00: bool,
        m01: bool,
        m10: bool,
        m11: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (m00, m01, m10, m11),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_bool2_bool2_0(
        &mut self,
        c0: crate::Unity::Mathematics::bool2,
        c1: crate::Unity::Mathematics::bool2,
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
        quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::bool2>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Mathematics::bool2,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Item", (index))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseAnd__cordl_bool_bool2x2_2(
        lhs: bool,
        rhs: crate::Unity::Mathematics::bool2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_BitwiseAnd", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseAnd_bool2x2__cordl_bool1(
        lhs: crate::Unity::Mathematics::bool2x2,
        rhs: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_BitwiseAnd", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseAnd_bool2x2_bool2x2_0(
        lhs: crate::Unity::Mathematics::bool2x2,
        rhs: crate::Unity::Mathematics::bool2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_BitwiseAnd", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseOr__cordl_bool_bool2x2_2(
        lhs: bool,
        rhs: crate::Unity::Mathematics::bool2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_BitwiseOr", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseOr_bool2x2__cordl_bool1(
        lhs: crate::Unity::Mathematics::bool2x2,
        rhs: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_BitwiseOr", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_BitwiseOr_bool2x2_bool2x2_0(
        lhs: crate::Unity::Mathematics::bool2x2,
        rhs: crate::Unity::Mathematics::bool2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_BitwiseOr", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality__cordl_bool_bool2x2_2(
        lhs: bool,
        rhs: crate::Unity::Mathematics::bool2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_bool2x2__cordl_bool1(
        lhs: crate::Unity::Mathematics::bool2x2,
        rhs: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality_bool2x2_bool2x2_0(
        lhs: crate::Unity::Mathematics::bool2x2,
        rhs: crate::Unity::Mathematics::bool2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_ExclusiveOr__cordl_bool_bool2x2_2(
        lhs: bool,
        rhs: crate::Unity::Mathematics::bool2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_ExclusiveOr", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_ExclusiveOr_bool2x2__cordl_bool1(
        lhs: crate::Unity::Mathematics::bool2x2,
        rhs: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_ExclusiveOr", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_ExclusiveOr_bool2x2_bool2x2_0(
        lhs: crate::Unity::Mathematics::bool2x2,
        rhs: crate::Unity::Mathematics::bool2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_ExclusiveOr", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        v: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality__cordl_bool_bool2x2_2(
        lhs: bool,
        rhs: crate::Unity::Mathematics::bool2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_bool2x2__cordl_bool1(
        lhs: crate::Unity::Mathematics::bool2x2,
        rhs: bool,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality_bool2x2_bool2x2_0(
        lhs: crate::Unity::Mathematics::bool2x2,
        rhs: crate::Unity::Mathematics::bool2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_LogicalNot(
        val: crate::Unity::Mathematics::bool2x2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::bool2x2> {
        let __cordl_ret: crate::Unity::Mathematics::bool2x2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_LogicalNot", (val))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Mathematics+bool2x2")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::Unity::Mathematics::bool2x2>>
for crate::Unity::Mathematics::bool2x2 {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::Unity::Mathematics::bool2x2> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+bool2x2")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::Unity::Mathematics::bool2x2>>
for crate::Unity::Mathematics::bool2x2 {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::Unity::Mathematics::bool2x2> {
        todo!()
    }
}
