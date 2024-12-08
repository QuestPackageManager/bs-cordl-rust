#[cfg(feature = "UnityEngine+RaycastHit")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct RaycastHit {
    pub m_Point: crate::UnityEngine::Vector3,
    pub m_Normal: crate::UnityEngine::Vector3,
    pub m_FaceID: u32,
    pub m_Distance: f32,
    pub m_UV: crate::UnityEngine::Vector2,
    pub m_Collider: i32,
}
#[cfg(feature = "UnityEngine+RaycastHit")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::RaycastHit => "UnityEngine"
    ."RaycastHit"
);
#[cfg(feature = "UnityEngine+RaycastHit")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::RaycastHit {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+RaycastHit")]
impl crate::UnityEngine::RaycastHit {
    pub fn get_articulationBody(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::ArticulationBody> {
        let __cordl_ret: *mut crate::UnityEngine::ArticulationBody = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_articulationBody",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_barycentricCoordinate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_barycentricCoordinate",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_collider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Collider> {
        let __cordl_ret: *mut crate::UnityEngine::Collider = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_collider",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_colliderInstanceID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_colliderInstanceID",
            (),
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
    pub fn get_lightmapCoord(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_lightmapCoord",
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
    pub fn get_rigidbody(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Rigidbody> {
        let __cordl_ret: *mut crate::UnityEngine::Rigidbody = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_rigidbody",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_textureCoord(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_textureCoord",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_textureCoord1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_textureCoord1",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_textureCoord2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_ret: crate::UnityEngine::Vector2 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_textureCoord2",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_transform(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Transform> {
        let __cordl_ret: *mut crate::UnityEngine::Transform = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_transform",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_triangleIndex(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_triangleIndex",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_barycentricCoordinate(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_barycentricCoordinate",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_distance(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_distance",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_normal(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_normal",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_point(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_point",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
