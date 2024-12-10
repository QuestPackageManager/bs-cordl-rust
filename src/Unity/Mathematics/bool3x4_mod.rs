#[cfg(feature = "Unity+Mathematics+bool3x4")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct bool3x4 {
    pub c0: crate::Unity::Mathematics::bool3,
    pub c1: crate::Unity::Mathematics::bool3,
    pub c2: crate::Unity::Mathematics::bool3,
    pub c3: crate::Unity::Mathematics::bool3,
}
#[cfg(feature = "Unity+Mathematics+bool3x4")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Mathematics::bool3x4 =>
    "Unity.Mathematics"."bool3x4"
);
#[cfg(feature = "Unity+Mathematics+bool3x4")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::Unity::Mathematics::bool3x4 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Mathematics+bool3x4")]
impl crate::Unity::Mathematics::bool3x4 {
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
    pub fn Equals_bool3x4_0(
        &mut self,
        rhs: crate::Unity::Mathematics::bool3x4,
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
    pub fn _ctor__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool__cordl_bool1(
        &mut self,
        m00: bool,
        m01: bool,
        m02: bool,
        m03: bool,
        m10: bool,
        m11: bool,
        m12: bool,
        m13: bool,
        m20: bool,
        m21: bool,
        m22: bool,
        m23: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (m00, m01, m02, m03, m10, m11, m12, m13, m20, m21, m22, m23),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_bool3_bool3_bool3_bool3_0(
        &mut self,
        c0: crate::Unity::Mathematics::bool3,
        c1: crate::Unity::Mathematics::bool3,
        c2: crate::Unity::Mathematics::bool3,
        c3: crate::Unity::Mathematics::bool3,
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
        quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::bool3>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Mathematics::bool3,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Item", (index))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Mathematics+bool3x4")]
impl AsRef<crate::System::IEquatable_1<crate::Unity::Mathematics::bool3x4>>
for crate::Unity::Mathematics::bool3x4 {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<crate::Unity::Mathematics::bool3x4> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+bool3x4")]
impl AsMut<crate::System::IEquatable_1<crate::Unity::Mathematics::bool3x4>>
for crate::Unity::Mathematics::bool3x4 {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::Unity::Mathematics::bool3x4> {
        todo!()
    }
}
