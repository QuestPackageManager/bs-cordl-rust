#[cfg(feature = "UnityEngine+RaycastCommand")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct RaycastCommand {
    pub _from_k__BackingField: crate::UnityEngine::Vector3,
    pub _direction_k__BackingField: crate::UnityEngine::Vector3,
    pub _physicsScene_k__BackingField: crate::UnityEngine::PhysicsScene,
    pub _distance_k__BackingField: f32,
    pub queryParameters: crate::UnityEngine::QueryParameters,
}
#[cfg(feature = "UnityEngine+RaycastCommand")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::RaycastCommand => "UnityEngine"
    ."RaycastCommand"
);
#[cfg(feature = "UnityEngine+RaycastCommand")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::RaycastCommand {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+RaycastCommand")]
impl crate::UnityEngine::RaycastCommand {
    pub fn _ctor_PhysicsScene_Vector3_QueryParameters_f32_1(
        &mut self,
        physicsScene: crate::UnityEngine::PhysicsScene,
        from: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        queryParameters: crate::UnityEngine::QueryParameters,
        distance: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (physicsScene, from, direction, queryParameters, distance),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_PhysicsScene_Vector3_f32_i32_i32_3(
        &mut self,
        physicsScene: crate::UnityEngine::PhysicsScene,
        from: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        distance: f32,
        layerMask: i32,
        maxHits: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (physicsScene, from, direction, distance, layerMask, maxHits),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Vector3_QueryParameters_f32_0(
        &mut self,
        from: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        queryParameters: crate::UnityEngine::QueryParameters,
        distance: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (from, direction, queryParameters, distance),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_Vector3_f32_i32_i32_2(
        &mut self,
        from: crate::UnityEngine::Vector3,
        direction: crate::UnityEngine::Vector3,
        distance: f32,
        layerMask: i32,
        maxHits: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (from, direction, distance, layerMask, maxHits),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_direction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_direction",
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
    pub fn get_from(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_from",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_layerMask(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_layerMask",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_maxHits(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_maxHits",
            (),
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
    pub fn set_direction(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_direction",
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
    pub fn set_from(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_from",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_layerMask(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_layerMask",
            (value),
        )?;
        Ok(__cordl_ret)
    }
    pub fn set_maxHits(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_maxHits",
            (value),
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
}
