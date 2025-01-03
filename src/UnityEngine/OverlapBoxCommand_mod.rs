#[cfg(feature = "UnityEngine+OverlapBoxCommand")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct OverlapBoxCommand {
    pub _center_k__BackingField: crate::UnityEngine::Vector3,
    pub _halfExtents_k__BackingField: crate::UnityEngine::Vector3,
    pub _orientation_k__BackingField: crate::UnityEngine::Quaternion,
    pub _physicsScene_k__BackingField: crate::UnityEngine::PhysicsScene,
    pub queryParameters: crate::UnityEngine::QueryParameters,
}
#[cfg(feature = "UnityEngine+OverlapBoxCommand")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::OverlapBoxCommand => "UnityEngine"
    ."OverlapBoxCommand"
);
#[cfg(feature = "UnityEngine+OverlapBoxCommand")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::OverlapBoxCommand {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+OverlapBoxCommand")]
impl crate::UnityEngine::OverlapBoxCommand {
    pub fn ScheduleBatch(
        commands: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::OverlapBoxCommand,
        >,
        results: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::ColliderHit,
        >,
        minCommandsPerJob: i32,
        maxHits: i32,
        dependsOn: crate::Unity::Jobs::JobHandle,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        let __cordl_ret: crate::Unity::Jobs::JobHandle = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ScheduleBatch",
                (commands, results, minCommandsPerJob, maxHits, dependsOn),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleOverlapBoxBatch(
        parameters: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_JobScheduleParameters,
        >,
        commands: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        commandLen: i32,
        result: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        resultLen: i32,
        minCommandsPerJob: i32,
        maxHits: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        let __cordl_ret: crate::Unity::Jobs::JobHandle = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ScheduleOverlapBoxBatch",
                (
                    parameters,
                    commands,
                    commandLen,
                    result,
                    resultLen,
                    minCommandsPerJob,
                    maxHits,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleOverlapBoxBatch_Injected(
        parameters: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_JobScheduleParameters,
        >,
        commands: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        commandLen: i32,
        result: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        resultLen: i32,
        minCommandsPerJob: i32,
        maxHits: i32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::JobHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ScheduleOverlapBoxBatch_Injected",
                (
                    parameters,
                    commands,
                    commandLen,
                    result,
                    resultLen,
                    minCommandsPerJob,
                    maxHits,
                    ret,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_PhysicsScene_Vector3_Quaternion_QueryParameters1(
        &mut self,
        physicsScene: crate::UnityEngine::PhysicsScene,
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
        queryParameters: crate::UnityEngine::QueryParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (physicsScene, center, halfExtents, orientation, queryParameters),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Vector3_Quaternion_QueryParameters0(
        &mut self,
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
        queryParameters: crate::UnityEngine::QueryParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (center, halfExtents, orientation, queryParameters),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_center(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_center",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_halfExtents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_halfExtents",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_orientation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_orientation",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_physicsScene(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::PhysicsScene> {
        let __cordl_ret: crate::UnityEngine::PhysicsScene = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_physicsScene",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_center(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_center",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_halfExtents(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_halfExtents",
            (value),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn set_orientation(
        &mut self,
        value: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_orientation",
            (value),
        )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
}
