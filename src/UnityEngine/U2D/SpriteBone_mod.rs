#[cfg(feature = "UnityEngine+U2D+SpriteBone")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct SpriteBone {
    pub m_Name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_Guid: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub m_Position: crate::UnityEngine::Vector3,
    pub m_Rotation: crate::UnityEngine::Quaternion,
    pub m_Length: f32,
    pub m_ParentId: i32,
    pub m_Color: crate::UnityEngine::Color32,
}
#[cfg(feature = "UnityEngine+U2D+SpriteBone")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::U2D::SpriteBone =>
    "UnityEngine.U2D"."SpriteBone"
);
#[cfg(feature = "UnityEngine+U2D+SpriteBone")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::U2D::SpriteBone {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+U2D+SpriteBone")]
impl crate::UnityEngine::U2D::SpriteBone {}
