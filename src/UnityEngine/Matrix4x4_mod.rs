#[cfg(feature = "UnityEngine+Matrix4x4")]
#[repr(C)]
#[derive(Debug, Clone)]
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
    pub fn Equals_Il2CppObject0(
        &mut self,
        other: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetLossyScale(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetLossyScale",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetRotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetRotation",
            (),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn ToString_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString_Il2CppString_IFormatProvider1(
        &mut self,
        format: *mut quest_hook::libil2cpp::Il2CppString,
        formatProvider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (format, formatProvider),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
    pub fn get_Item_i32_1(&mut self, index: i32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Item",
            (index),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_inverse(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_inverse",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_lossyScale(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_lossyScale",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_rotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_rotation",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_transpose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Matrix4x4> {
        let __cordl_ret: crate::UnityEngine::Matrix4x4 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_transpose",
            (),
        )?;
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
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
        Ok(__cordl_ret)
    }
}
