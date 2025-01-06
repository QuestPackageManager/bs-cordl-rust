#[cfg(feature = "UnityEngine+ColliderHit")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ColliderHit {
    pub m_ColliderInstanceID: i32,
}
#[cfg(feature = "UnityEngine+ColliderHit")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ColliderHit => "UnityEngine"
    ."ColliderHit"
);
#[cfg(feature = "UnityEngine+ColliderHit")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::ColliderHit {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ColliderHit")]
impl crate::UnityEngine::ColliderHit {
    pub fn get_collider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_collider",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_instanceID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_instanceID",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
