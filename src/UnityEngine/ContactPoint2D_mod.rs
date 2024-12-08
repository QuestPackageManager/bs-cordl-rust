#[cfg(feature = "UnityEngine+ContactPoint2D")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ContactPoint2D {
    pub m_Point: crate::UnityEngine::Vector2,
    pub m_Normal: crate::UnityEngine::Vector2,
    pub m_RelativeVelocity: crate::UnityEngine::Vector2,
    pub m_Separation: f32,
    pub m_NormalImpulse: f32,
    pub m_TangentImpulse: f32,
    pub m_Collider: i32,
    pub m_OtherCollider: i32,
    pub m_Rigidbody: i32,
    pub m_OtherRigidbody: i32,
    pub m_Enabled: i32,
}
#[cfg(feature = "UnityEngine+ContactPoint2D")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ContactPoint2D => "UnityEngine"
    ."ContactPoint2D"
);
#[cfg(feature = "UnityEngine+ContactPoint2D")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::ContactPoint2D {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ContactPoint2D")]
impl crate::UnityEngine::ContactPoint2D {}
