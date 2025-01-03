#[cfg(feature = "UnityEngine+Vector3")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[cfg(feature = "UnityEngine+Vector3")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Vector3 => "UnityEngine"."Vector3"
);
#[cfg(feature = "UnityEngine+Vector3")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::Vector3 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Vector3")]
impl crate::UnityEngine::Vector3 {
    pub const kEpsilon: f32 = 0.00001f32;
    pub const kEpsilonNormalSqrt: f32 = 0.000000000000001f32;
    pub fn Angle(
        from: crate::UnityEngine::Vector3,
        to: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Angle", (from, to))?;
        Ok(__cordl_ret.into())
    }
    pub fn Cross(
        lhs: crate::UnityEngine::Vector3,
        rhs: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Cross", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn Distance(
        a: crate::UnityEngine::Vector3,
        b: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Distance", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn Dot(
        lhs: crate::UnityEngine::Vector3,
        rhs: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Dot", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject0(
        &mut self,
        other: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Vector3_1(
        &mut self,
        other: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
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
    pub fn Lerp(
        a: crate::UnityEngine::Vector3,
        b: crate::UnityEngine::Vector3,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Lerp", (a, b, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn LerpUnclamped(
        a: crate::UnityEngine::Vector3,
        b: crate::UnityEngine::Vector3,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LerpUnclamped", (a, b, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn Magnitude(
        vector: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Magnitude", (vector))?;
        Ok(__cordl_ret.into())
    }
    pub fn Max(
        lhs: crate::UnityEngine::Vector3,
        rhs: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Max", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn Min(
        lhs: crate::UnityEngine::Vector3,
        rhs: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Min", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn Normalize_1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Normalize",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Normalize_Vector3_0(
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Normalize", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn OrthoNormalize(
        normal: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        tangent: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OrthoNormalize", (normal, tangent))?;
        Ok(__cordl_ret.into())
    }
    pub fn OrthoNormalize2(
        a: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        b: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("OrthoNormalize2", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn Project(
        vector: crate::UnityEngine::Vector3,
        onNormal: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Project", (vector, onNormal))?;
        Ok(__cordl_ret.into())
    }
    pub fn ProjectOnPlane(
        vector: crate::UnityEngine::Vector3,
        planeNormal: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ProjectOnPlane", (vector, planeNormal))?;
        Ok(__cordl_ret.into())
    }
    pub fn Reflect(
        inDirection: crate::UnityEngine::Vector3,
        inNormal: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Reflect", (inDirection, inNormal))?;
        Ok(__cordl_ret.into())
    }
    pub fn Scale_Vector3_0(
        a: crate::UnityEngine::Vector3,
        b: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Scale", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn Scale_Vector3_1(
        &mut self,
        scale: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Scale",
            (scale),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Slerp(
        a: crate::UnityEngine::Vector3,
        b: crate::UnityEngine::Vector3,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Slerp", (a, b, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn Slerp_Injected(
        a: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        b: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        t: f32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Slerp_Injected", (a, b, t, ret))?;
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
    pub fn ToString_Il2CppString1(
        &mut self,
        format: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "ToString", (format))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString_Il2CppString_IFormatProvider2(
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
    pub fn _ctor_f32_0(
        &mut self,
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (x, y, z),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_f32_f32_1(
        &mut self,
        x: f32,
        y: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (x, y),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item(&mut self, index: i32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Item",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_back() -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_back", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_down() -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_down", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_forward() -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_forward", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_left() -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_left", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_magnitude(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_magnitude",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_normalized(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_normalized",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_one() -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_one", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_positiveInfinity() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Vector3,
    > {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_positiveInfinity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_right() -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_right", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_sqrMagnitude(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_sqrMagnitude",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_up() -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_up", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zero() -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_zero", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition(
        a: crate::UnityEngine::Vector3,
        b: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Division(
        a: crate::UnityEngine::Vector3,
        d: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Division", (a, d))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        lhs: crate::UnityEngine::Vector3,
        rhs: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        lhs: crate::UnityEngine::Vector3,
        rhs: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_Vector3_f32_0(
        a: crate::UnityEngine::Vector3,
        d: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (a, d))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_f32_Vector3_1(
        d: f32,
        a: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (d, a))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction(
        a: crate::UnityEngine::Vector3,
        b: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_UnaryNegation(
        a: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_UnaryNegation", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Item(
        &mut self,
        index: i32,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Item",
            (index, value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Vector3")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::Vector3>>
for crate::UnityEngine::Vector3 {
    fn as_ref(&self) -> &crate::System::IEquatable_1<crate::UnityEngine::Vector3> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Vector3")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::Vector3>>
for crate::UnityEngine::Vector3 {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::UnityEngine::Vector3> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Vector3")]
impl AsRef<crate::System::IFormattable> for crate::UnityEngine::Vector3 {
    fn as_ref(&self) -> &crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Vector3")]
impl AsMut<crate::System::IFormattable> for crate::UnityEngine::Vector3 {
    fn as_mut(&mut self) -> &mut crate::System::IFormattable {
        todo!()
    }
}
