#[cfg(feature = "Unity+Mathematics+half")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct half {
    pub value: u16,
}
#[cfg(feature = "Unity+Mathematics+half")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Unity::Mathematics::half => "Unity.Mathematics"
    ."half"
);
#[cfg(feature = "Unity+Mathematics+half")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::Unity::Mathematics::half {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Mathematics+half")]
impl crate::Unity::Mathematics::half {
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
    pub fn Equals_half0(
        &mut self,
        rhs: crate::Unity::Mathematics::half,
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
    pub fn _ctor_f32_1(
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
    pub fn _ctor_f64_2(
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
    pub fn _ctor_half0(
        &mut self,
        x: crate::Unity::Mathematics::half,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (x),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MaxValue() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_MaxValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MaxValueAsHalf() -> quest_hook::libil2cpp::Result<
        crate::Unity::Mathematics::half,
    > {
        let __cordl_ret: crate::Unity::Mathematics::half = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_MaxValueAsHalf", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MinValue() -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_MinValue", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MinValueAsHalf() -> quest_hook::libil2cpp::Result<
        crate::Unity::Mathematics::half,
    > {
        let __cordl_ret: crate::Unity::Mathematics::half = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_MinValueAsHalf", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        lhs: crate::Unity::Mathematics::half,
        rhs: crate::Unity::Mathematics::half,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_f32_0(
        v: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half> {
        let __cordl_ret: crate::Unity::Mathematics::half = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Explicit_f64_1(
        v: f64,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::half> {
        let __cordl_ret: crate::Unity::Mathematics::half = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Explicit", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_half0(
        d: crate::Unity::Mathematics::half,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (d))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit_half1(
        d: crate::Unity::Mathematics::half,
    ) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_ret: f64 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Implicit", (d))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        lhs: crate::Unity::Mathematics::half,
        rhs: crate::Unity::Mathematics::half,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Mathematics+half")]
impl AsRef<crate::System::IEquatable_1<crate::Unity::Mathematics::half>>
for crate::Unity::Mathematics::half {
    fn as_ref(&self) -> &crate::System::IEquatable_1<crate::Unity::Mathematics::half> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+half")]
impl AsMut<crate::System::IEquatable_1<crate::Unity::Mathematics::half>>
for crate::Unity::Mathematics::half {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::Unity::Mathematics::half> {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+half")]
impl AsRef<crate::System::IFormattable> for crate::Unity::Mathematics::half {
    fn as_ref(&self) -> &crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "Unity+Mathematics+half")]
impl AsMut<crate::System::IFormattable> for crate::Unity::Mathematics::half {
    fn as_mut(&mut self) -> &mut crate::System::IFormattable {
        todo!()
    }
}
