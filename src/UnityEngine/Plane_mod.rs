#[cfg(feature = "UnityEngine+Plane")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct Plane {
    pub m_Normal: crate::UnityEngine::Vector3,
    pub m_Distance: f32,
}
#[cfg(feature = "UnityEngine+Plane")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::Plane => "UnityEngine"."Plane"
);
#[cfg(feature = "UnityEngine+Plane")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::Plane {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+Plane")]
impl crate::UnityEngine::Plane {
    pub const _cordl_size: i32 = 16i32;
    pub fn ClosestPointOnPlane(
        &mut self,
        point: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ClosestPointOnPlane",
            (point),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetDistanceToPoint(
        &mut self,
        point: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetDistanceToPoint",
            (point),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Raycast(
        &mut self,
        ray: crate::UnityEngine::Ray,
        enter: quest_hook::libil2cpp::ByRefMut<f32>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Raycast",
            (ray, enter),
        )?;
        Ok(__cordl_ret)
    }
    pub fn SameSide(
        &mut self,
        inPt0: crate::UnityEngine::Vector3,
        inPt1: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SameSide",
            (inPt0, inPt1),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn ToString_String_IFormatProvider1(
        &mut self,
        format: *mut crate::System::String,
        formatProvider: *mut crate::System::IFormatProvider,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_ret: *mut crate::System::String = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "ToString",
            (format, formatProvider),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Vector3_1(
        &mut self,
        a: crate::UnityEngine::Vector3,
        b: crate::UnityEngine::Vector3,
        c: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (a, b, c),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Vector3_Vector3_0(
        &mut self,
        inNormal: crate::UnityEngine::Vector3,
        inPoint: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (inNormal, inPoint),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_distance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_distance",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_normal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_normal",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
