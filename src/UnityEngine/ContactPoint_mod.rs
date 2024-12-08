#[cfg(feature = "UnityEngine+ContactPoint")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ContactPoint {
    pub m_Point: crate::UnityEngine::Vector3,
    pub m_Normal: crate::UnityEngine::Vector3,
    pub m_Impulse: crate::UnityEngine::Vector3,
    pub m_ThisColliderInstanceID: i32,
    pub m_OtherColliderInstanceID: i32,
    pub m_Separation: f32,
}
#[cfg(feature = "UnityEngine+ContactPoint")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ContactPoint => "UnityEngine"
    ."ContactPoint"
);
#[cfg(feature = "UnityEngine+ContactPoint")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::ContactPoint {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ContactPoint")]
impl crate::UnityEngine::ContactPoint {
    pub fn get_otherCollider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Collider> {
        let __cordl_ret: *mut crate::UnityEngine::Collider = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_otherCollider",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_thisCollider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Collider> {
        let __cordl_ret: *mut crate::UnityEngine::Collider = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_thisCollider",
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
    pub fn get_separation(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_separation",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        point: crate::UnityEngine::Vector3,
        normal: crate::UnityEngine::Vector3,
        impulse: crate::UnityEngine::Vector3,
        separation: f32,
        thisInstanceID: i32,
        otherInstenceID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (point, normal, impulse, separation, thisInstanceID, otherInstenceID),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_point(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_point",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_impulse(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_impulse",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
