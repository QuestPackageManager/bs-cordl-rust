#[cfg(feature = "UnityEngine+RaycastHit2D")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct RaycastHit2D {
    pub m_Centroid: crate::UnityEngine::Vector2,
    pub m_Point: crate::UnityEngine::Vector2,
    pub m_Normal: crate::UnityEngine::Vector2,
    pub m_Distance: f32,
    pub m_Fraction: f32,
    pub m_Collider: i32,
}
#[cfg(feature = "UnityEngine+RaycastHit2D")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::RaycastHit2D => "UnityEngine"
    ."RaycastHit2D"
);
#[cfg(feature = "UnityEngine+RaycastHit2D")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::RaycastHit2D {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+RaycastHit2D")]
impl crate::UnityEngine::RaycastHit2D {
    pub fn get_collider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider2D>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider2D> = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_collider",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_distance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_distance",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_fraction(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_fraction",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_normal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_normal",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_point(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_point",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
