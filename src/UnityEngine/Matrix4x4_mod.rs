#[cfg(feature = "UnityEngine+Matrix4x4")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct Matrix4x4 {
    pub m00: f32,
    pub m10: f32,
    pub m20: f32,
    pub m30: f32,
    pub m01: f32,
    pub m11: f32,
    pub m21: f32,
    pub m31: f32,
    pub m02: f32,
    pub m12: f32,
    pub m22: f32,
    pub m32: f32,
    pub m03: f32,
    pub m13: f32,
    pub m23: f32,
    pub m33: f32,
}
#[cfg(feature = "UnityEngine+Matrix4x4")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Matrix4x4 => "UnityEngine"
    ."Matrix4x4"
);
#[cfg(feature = "UnityEngine+Matrix4x4")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::Matrix4x4 {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Matrix4x4")]
impl crate::UnityEngine::Matrix4x4 {
    pub fn Equals_Gc0(
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
    pub fn Equals_Matrix4x4_1(
        &mut self,
        other: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetColumn(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_ret: crate::UnityEngine::Vector4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetColumn",
            (index),
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
    pub fn GetLossyScale(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetLossyScale",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetLossyScale_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetLossyScale_Injected", (_unity_self, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetRotation",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRotation_Injected(
        _unity_self: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetRotation_Injected", (_unity_self, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRow(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_ret: crate::UnityEngine::Vector4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetRow",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Inverse(
        m: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Inverse", (m))?;
        Ok(__cordl_ret.into())
    }
    pub fn Inverse3DAffine(
        input: crate::UnityEngine::Matrix4x4,
        result: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Inverse3DAffine", (input, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Inverse3DAffine_Injected(
        input: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        result: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Inverse3DAffine_Injected", (input, result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Inverse_Injected(
        m: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Inverse_Injected", (m, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn MultiplyPoint(
        &mut self,
        point: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MultiplyPoint",
            (point),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn MultiplyPoint3x4(
        &mut self,
        point: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MultiplyPoint3x4",
            (point),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn MultiplyVector(
        &mut self,
        vector: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MultiplyVector",
            (vector),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Ortho(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        zNear: f32,
        zFar: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Ortho", (left, right, bottom, top, zNear, zFar))?;
        Ok(__cordl_ret.into())
    }
    pub fn Ortho_Injected(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        zNear: f32,
        zFar: f32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Ortho_Injected", (left, right, bottom, top, zNear, zFar, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn Perspective(
        fov: f32,
        aspect: f32,
        zNear: f32,
        zFar: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Perspective", (fov, aspect, zNear, zFar))?;
        Ok(__cordl_ret.into())
    }
    pub fn Perspective_Injected(
        fov: f32,
        aspect: f32,
        zNear: f32,
        zFar: f32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Perspective_Injected", (fov, aspect, zNear, zFar, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn Rotate(
        q: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Rotate", (q))?;
        Ok(__cordl_ret.into())
    }
    pub fn Scale(
        vector: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Scale", (vector))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetColumn(
        &mut self,
        index: i32,
        column: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetColumn",
            (index, column),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn SetTRS(
        &mut self,
        pos: crate::UnityEngine::Vector3,
        q: crate::UnityEngine::Quaternion,
        s: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetTRS",
            (pos, q, s),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn TRS(
        pos: crate::UnityEngine::Vector3,
        q: crate::UnityEngine::Quaternion,
        s: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TRS", (pos, q, s))?;
        Ok(__cordl_ret.into())
    }
    pub fn TRS_Injected(
        pos: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        q: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Quaternion>,
        s: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Vector3>,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TRS_Injected", (pos, q, s, ret))?;
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
    pub fn ToString_Gc_Gc1(
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
    pub fn Transpose(
        m: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Transpose", (m))?;
        Ok(__cordl_ret.into())
    }
    pub fn Transpose_Injected(
        m: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
        ret: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::Matrix4x4>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Transpose_Injected", (m, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        column0: crate::UnityEngine::Vector4,
        column1: crate::UnityEngine::Vector4,
        column2: crate::UnityEngine::Vector4,
        column3: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (column0, column1, column2, column3),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item_i32_0(
        &mut self,
        row: i32,
        column: i32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Item",
            (row, column),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Item_i32_1(&mut self, index: i32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Item",
            (index),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_identity() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::Matrix4x4,
    > {
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_identity", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_inverse(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_inverse",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lossyScale(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_lossyScale",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_rotation",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_transpose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_transpose",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_zero() -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_zero", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        lhs: crate::UnityEngine::Matrix4x4,
        rhs: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        lhs: crate::UnityEngine::Matrix4x4,
        rhs: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_Matrix4x4_0(
        lhs: crate::UnityEngine::Matrix4x4,
        rhs: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (lhs, rhs))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply_Vector4_1(
        lhs: crate::UnityEngine::Matrix4x4,
        vector: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_ret: crate::UnityEngine::Vector4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (lhs, vector))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Item_f32_1(
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
    pub fn set_Item_i32_f32_0(
        &mut self,
        row: i32,
        column: i32,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_Item",
            (row, column, value),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+Matrix4x4")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::IFormattable>>
for crate::UnityEngine::Matrix4x4 {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::System::IFormattable> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Matrix4x4")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::IFormattable>>
for crate::UnityEngine::Matrix4x4 {
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<crate::System::IFormattable> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Matrix4x4")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::UnityEngine::Matrix4x4>>
for crate::UnityEngine::Matrix4x4 {
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<crate::UnityEngine::Matrix4x4> {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+Matrix4x4")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::UnityEngine::Matrix4x4>>
for crate::UnityEngine::Matrix4x4 {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::UnityEngine::Matrix4x4> {
        todo!()
    }
}
