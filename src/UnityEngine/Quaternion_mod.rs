#[cfg(feature = "UnityEngine+Quaternion")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
#[cfg(feature = "UnityEngine+Quaternion")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Quaternion => "UnityEngine"
    ."Quaternion"
);
#[cfg(feature = "UnityEngine+Quaternion")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::Quaternion {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Quaternion")]
impl crate::UnityEngine::Quaternion {
    pub const kEpsilon: f32 = 0.000001f32;
    pub fn Angle(
        a: crate::UnityEngine::Quaternion,
        b: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Angle", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn AngleAxis(
        angle: f32,
        axis: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AngleAxis", (angle, axis))?;
        Ok(__cordl_ret.into())
    }
    pub fn AngleAxis_Injected(
        angle: f32,
        axis: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AngleAxis_Injected", (angle, axis, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn Dot(
        a: crate::UnityEngine::Quaternion,
        b: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Dot", (a, b))?;
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
    pub fn Equals_Quaternion1(
        &mut self,
        other: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Euler_Vector3_1(
        euler: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Euler", (euler))?;
        Ok(__cordl_ret.into())
    }
    pub fn Euler_f32_f32_f32_0(
        x: f32,
        y: f32,
        z: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Euler", (x, y, z))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromToRotation(
        fromDirection: crate::UnityEngine::Vector3,
        toDirection: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromToRotation", (fromDirection, toDirection))?;
        Ok(__cordl_ret.into())
    }
    pub fn FromToRotation_Injected(
        fromDirection: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        toDirection: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("FromToRotation_Injected", (fromDirection, toDirection, ret))?;
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
    pub fn Internal_FromEulerRad(
        euler: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_FromEulerRad", (euler))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_FromEulerRad_Injected(
        euler: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_FromEulerRad_Injected", (euler, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_MakePositive(
        euler: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_MakePositive", (euler))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_ToEulerRad(
        rotation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_ToEulerRad", (rotation))?;
        Ok(__cordl_ret.into())
    }
    pub fn Internal_ToEulerRad_Injected(
        rotation: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Internal_ToEulerRad_Injected", (rotation, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn Inverse(
        rotation: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Inverse", (rotation))?;
        Ok(__cordl_ret.into())
    }
    pub fn Inverse_Injected(
        rotation: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Inverse_Injected", (rotation, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsEqualUsingDot(dot: f32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsEqualUsingDot", (dot))?;
        Ok(__cordl_ret.into())
    }
    pub fn Lerp(
        a: crate::UnityEngine::Quaternion,
        b: crate::UnityEngine::Quaternion,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Lerp", (a, b, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn Lerp_Injected(
        a: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
        b: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
        t: f32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Lerp_Injected", (a, b, t, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn LookRotation_Injected(
        forward: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        upwards: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LookRotation_Injected", (forward, upwards, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn LookRotation_Vector3_0(
        forward: crate::UnityEngine::Vector3,
        upwards: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LookRotation", (forward, upwards))?;
        Ok(__cordl_ret.into())
    }
    pub fn LookRotation_Vector3_1(
        forward: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LookRotation", (forward))?;
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
    pub fn Normalize_Quaternion0(
        q: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Normalize", (q))?;
        Ok(__cordl_ret.into())
    }
    pub fn RotateTowards(
        from: crate::UnityEngine::Quaternion,
        to: crate::UnityEngine::Quaternion,
        maxDegreesDelta: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RotateTowards", (from, to, maxDegreesDelta))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLookRotation_Vector3_0(
        &mut self,
        view: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetLookRotation",
            (view),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetLookRotation_Vector3_1(
        &mut self,
        view: crate::UnityEngine::Vector3,
        up: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetLookRotation",
            (view, up),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Slerp(
        a: crate::UnityEngine::Quaternion,
        b: crate::UnityEngine::Quaternion,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Slerp", (a, b, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn SlerpUnclamped(
        a: crate::UnityEngine::Quaternion,
        b: crate::UnityEngine::Quaternion,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SlerpUnclamped", (a, b, t))?;
        Ok(__cordl_ret.into())
    }
    pub fn SlerpUnclamped_Injected(
        a: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
        b: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
        t: f32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SlerpUnclamped_Injected", (a, b, t, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn Slerp_Injected(
        a: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
        b: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
        t: f32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
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
    pub fn _ctor(
        &mut self,
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (x, y, z, w),
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
    pub fn get_eulerAngles(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_eulerAngles",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_identity() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Quaternion,
    > {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_identity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        lhs: crate::UnityEngine::Quaternion,
        rhs: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        lhs: crate::UnityEngine::Quaternion,
        rhs: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_Quaternion0(
        lhs: crate::UnityEngine::Quaternion,
        rhs: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_Vector3_1(
        rotation: crate::UnityEngine::Quaternion,
        point: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (rotation, point))?;
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
    pub fn set_eulerAngles(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_eulerAngles",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Quaternion")]
impl AsRef<crate::System::IEquatable_1<crate::UnityEngine::Quaternion>>
for crate::UnityEngine::Quaternion {
    fn as_ref(&self) -> &crate::System::IEquatable_1<crate::UnityEngine::Quaternion> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Quaternion")]
impl AsMut<crate::System::IEquatable_1<crate::UnityEngine::Quaternion>>
for crate::UnityEngine::Quaternion {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<crate::UnityEngine::Quaternion> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Quaternion")]
impl AsRef<crate::System::IFormattable> for crate::UnityEngine::Quaternion {
    fn as_ref(&self) -> &crate::System::IFormattable {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Quaternion")]
impl AsMut<crate::System::IFormattable> for crate::UnityEngine::Quaternion {
    fn as_mut(&mut self) -> &mut crate::System::IFormattable {
        todo!()
    }
}
