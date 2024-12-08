#[cfg(feature = "UnityEngine+OverlapCapsuleCommand")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OverlapCapsuleCommand {
    pub _point0_k__BackingField: crate::UnityEngine::Vector3,
    pub _point1_k__BackingField: crate::UnityEngine::Vector3,
    pub _radius_k__BackingField: f32,
    pub _physicsScene_k__BackingField: crate::UnityEngine::PhysicsScene,
    pub queryParameters: crate::UnityEngine::QueryParameters,
}
#[cfg(feature = "UnityEngine+OverlapCapsuleCommand")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::OverlapCapsuleCommand =>
    "UnityEngine"."OverlapCapsuleCommand"
);
#[cfg(feature = "UnityEngine+OverlapCapsuleCommand")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::OverlapCapsuleCommand {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+OverlapCapsuleCommand")]
impl crate::UnityEngine::OverlapCapsuleCommand {
    pub fn get_radius(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_radius",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_point0(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_point0",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_physicsScene(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::PhysicsScene> {
        let __cordl_ret: crate::UnityEngine::PhysicsScene = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_physicsScene",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_physicsScene(
        &mut self,
        value: crate::UnityEngine::PhysicsScene,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_physicsScene",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_point0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_point0",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_radius(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_radius",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Vector3_f32_QueryParameters0(
        &mut self,
        point0: crate::UnityEngine::Vector3,
        point1: crate::UnityEngine::Vector3,
        radius: f32,
        queryParameters: crate::UnityEngine::QueryParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (point0, point1, radius, queryParameters),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_PhysicsScene_Vector3_f32_QueryParameters1(
        &mut self,
        physicsScene: crate::UnityEngine::PhysicsScene,
        point0: crate::UnityEngine::Vector3,
        point1: crate::UnityEngine::Vector3,
        radius: f32,
        queryParameters: crate::UnityEngine::QueryParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (physicsScene, point0, point1, radius, queryParameters),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_point1(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_point1",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_point1(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_point1",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
