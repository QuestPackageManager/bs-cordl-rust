#[cfg(feature = "UnityEngine+ClosestPointCommand")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct ClosestPointCommand {
    pub _point_k__BackingField: crate::UnityEngine::Vector3,
    pub _colliderInstanceID_k__BackingField: i32,
    pub _position_k__BackingField: crate::UnityEngine::Vector3,
    pub _rotation_k__BackingField: crate::UnityEngine::Quaternion,
    pub _scale_k__BackingField: crate::UnityEngine::Vector3,
}
#[cfg(feature = "UnityEngine+ClosestPointCommand")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ClosestPointCommand =>
    "UnityEngine"."ClosestPointCommand"
);
#[cfg(feature = "UnityEngine+ClosestPointCommand")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ClosestPointCommand {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ClosestPointCommand")]
impl crate::UnityEngine::ClosestPointCommand {
    pub fn _ctor_Collider1(
        &mut self,
        point: crate::UnityEngine::Vector3,
        collider: *mut crate::UnityEngine::Collider,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        scale: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (point, collider, position, rotation, scale),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_0(
        &mut self,
        point: crate::UnityEngine::Vector3,
        colliderInstanceID: i32,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        scale: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (point, colliderInstanceID, position, rotation, scale),
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
    pub fn get_position(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_position",
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
    pub fn get_scale(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_scale",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_colliderInstanceID(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_colliderInstanceID",
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
    pub fn set_position(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_position",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_rotation(
        &mut self,
        value: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_rotation",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_scale(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_scale",
            (value),
        )?;
        Ok(__cordl_ret)
    }
}
