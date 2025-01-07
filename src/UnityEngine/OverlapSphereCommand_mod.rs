#[cfg(feature = "UnityEngine+OverlapSphereCommand")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OverlapSphereCommand {
    pub _point_k__BackingField: crate::UnityEngine::Vector3,
    pub _radius_k__BackingField: f32,
    pub _physicsScene_k__BackingField: crate::UnityEngine::PhysicsScene,
    pub queryParameters: crate::UnityEngine::QueryParameters,
}
#[cfg(feature = "UnityEngine+OverlapSphereCommand")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::OverlapSphereCommand {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "OverlapSphereCommand";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::OverlapSphereCommand {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::OverlapSphereCommand {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::OverlapSphereCommand {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::OverlapSphereCommand {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "UnityEngine+OverlapSphereCommand")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::OverlapSphereCommand {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+OverlapSphereCommand")]
impl crate::UnityEngine::OverlapSphereCommand {
    pub fn ScheduleBatch(
        commands: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::OverlapSphereCommand,
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
    pub fn ScheduleOverlapSphereBatch(
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
                "ScheduleOverlapSphereBatch",
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
    pub fn ScheduleOverlapSphereBatch_Injected(
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
                "ScheduleOverlapSphereBatch_Injected",
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
    pub fn _ctor_PhysicsScene_Vector3_f32_QueryParameters1(
        &mut self,
        physicsScene: crate::UnityEngine::PhysicsScene,
        point: crate::UnityEngine::Vector3,
        radius: f32,
        queryParameters: crate::UnityEngine::QueryParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (physicsScene, point, radius, queryParameters),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Vector3_f32_QueryParameters0(
        &mut self,
        point: crate::UnityEngine::Vector3,
        radius: f32,
        queryParameters: crate::UnityEngine::QueryParameters,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (point, radius, queryParameters),
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
    pub fn get_point(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_point",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_radius(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_radius",
            (),
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
    pub fn set_point(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_point",
            (value),
        )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
}
