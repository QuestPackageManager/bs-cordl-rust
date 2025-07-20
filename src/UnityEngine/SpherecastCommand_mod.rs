#[cfg(feature = "UnityEngine+SpherecastCommand")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct SpherecastCommand {
    pub _origin_k__BackingField: crate::UnityEngine::Vector3,
    pub _radius_k__BackingField: f32,
    pub _direction_k__BackingField: crate::UnityEngine::Vector3,
    pub _distance_k__BackingField: f32,
    pub _physicsScene_k__BackingField: crate::UnityEngine::PhysicsScene,
    pub queryParameters: crate::UnityEngine::QueryParameters,
}
#[cfg(feature = "UnityEngine+SpherecastCommand")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::SpherecastCommand {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "SpherecastCommand";
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
#[cfg(feature = "UnityEngine+SpherecastCommand")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::SpherecastCommand {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+SpherecastCommand")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::SpherecastCommand {
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
#[cfg(feature = "UnityEngine+SpherecastCommand")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::SpherecastCommand {
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
#[cfg(feature = "UnityEngine+SpherecastCommand")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::SpherecastCommand {
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
#[cfg(feature = "UnityEngine+SpherecastCommand")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::SpherecastCommand {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+SpherecastCommand")]
impl crate::UnityEngine::SpherecastCommand {
    pub fn ScheduleBatch_JobHandle1(
        commands: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::SpherecastCommand,
        >,
        results: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::RaycastHit,
        >,
        minCommandsPerJob: i32,
        dependsOn: crate::Unity::Jobs::JobHandle,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Collections::NativeArray_1<
                        crate::UnityEngine::SpherecastCommand,
                    >,
                    crate::Unity::Collections::NativeArray_1<
                        crate::UnityEngine::RaycastHit,
                    >,
                    i32,
                    crate::Unity::Jobs::JobHandle,
                ),
                crate::Unity::Jobs::JobHandle,
                4usize,
            >("ScheduleBatch")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type
                    > ::class(), "ScheduleBatch", 4usize
                )
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle = unsafe {
            method
                .invoke_unchecked((), (commands, results, minCommandsPerJob, dependsOn))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleBatch_i32_JobHandle0(
        commands: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::SpherecastCommand,
        >,
        results: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::RaycastHit,
        >,
        minCommandsPerJob: i32,
        maxHits: i32,
        dependsOn: crate::Unity::Jobs::JobHandle,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Collections::NativeArray_1<
                        crate::UnityEngine::SpherecastCommand,
                    >,
                    crate::Unity::Collections::NativeArray_1<
                        crate::UnityEngine::RaycastHit,
                    >,
                    i32,
                    i32,
                    crate::Unity::Jobs::JobHandle,
                ),
                crate::Unity::Jobs::JobHandle,
                5usize,
            >("ScheduleBatch")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type
                    > ::class(), "ScheduleBatch", 5usize
                )
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (commands, results, minCommandsPerJob, maxHits, dependsOn),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleSpherecastBatch(
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_JobScheduleParameters,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    i32,
                    i32,
                    i32,
                ),
                crate::Unity::Jobs::JobHandle,
                7usize,
            >("ScheduleSpherecastBatch")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type
                    > ::class(), "ScheduleSpherecastBatch", 7usize
                )
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (
                        parameters,
                        commands,
                        commandLen,
                        result,
                        resultLen,
                        minCommandsPerJob,
                        maxHits,
                    ),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleSpherecastBatch_Injected(
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::ByRefMut<
                        crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_JobScheduleParameters,
                    >,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    i32,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    i32,
                    i32,
                    i32,
                    quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::JobHandle>,
                ),
                quest_hook::libil2cpp::Void,
                8usize,
            >("ScheduleSpherecastBatch_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type
                    > ::class(), "ScheduleSpherecastBatch_Injected", 8usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    (),
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
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_PhysicsScene_Vector3_f32_Vector3_QueryParameters_f32_1(
        &mut self,
        physicsScene: crate::UnityEngine::PhysicsScene,
        origin: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        queryParameters: crate::UnityEngine::QueryParameters,
        distance: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::PhysicsScene,
                    crate::UnityEngine::Vector3,
                    f32,
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::QueryParameters,
                    f32,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type
                    > ::class(), ".ctor", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (physicsScene, origin, radius, direction, queryParameters, distance),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_PhysicsScene_Vector3_f32_Vector3_f32_i32_3(
        &mut self,
        physicsScene: crate::UnityEngine::PhysicsScene,
        origin: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        distance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::PhysicsScene,
                    crate::UnityEngine::Vector3,
                    f32,
                    crate::UnityEngine::Vector3,
                    f32,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type
                    > ::class(), ".ctor", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (physicsScene, origin, radius, direction, distance, layerMask),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Vector3_f32_Vector3_QueryParameters_f32_0(
        &mut self,
        origin: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        queryParameters: crate::UnityEngine::QueryParameters,
        distance: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Vector3,
                    f32,
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::QueryParameters,
                    f32,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type
                    > ::class(), ".ctor", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (origin, radius, direction, queryParameters, distance),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Vector3_f32_Vector3_f32_i32_2(
        &mut self,
        origin: crate::UnityEngine::Vector3,
        radius: f32,
        direction: crate::UnityEngine::Vector3,
        distance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Vector3,
                    f32,
                    crate::UnityEngine::Vector3,
                    f32,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type
                    > ::class(), ".ctor", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (origin, radius, direction, distance, layerMask),
                )?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_direction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::UnityEngine::Vector3, 0usize>("get_direction")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type
                    > ::class(), "get_direction", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_distance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_distance")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type
                    > ::class(), "get_distance", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_layerMask(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_layerMask")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type
                    > ::class(), "get_layerMask", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_origin(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::UnityEngine::Vector3, 0usize>("get_origin")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type
                    > ::class(), "get_origin", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_physicsScene(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::PhysicsScene> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::PhysicsScene,
                0usize,
            >("get_physicsScene")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type
                    > ::class(), "get_physicsScene", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::PhysicsScene = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_radius(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_radius")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type
                    > ::class(), "get_radius", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_direction(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Vector3),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_direction")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type
                    > ::class(), "set_direction", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_distance(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f32), quest_hook::libil2cpp::Void, 1usize>("set_distance")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type
                    > ::class(), "set_distance", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_layerMask(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("set_layerMask")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type
                    > ::class(), "set_layerMask", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_origin(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Vector3),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_origin")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type
                    > ::class(), "set_origin", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_physicsScene(
        &mut self,
        value: crate::UnityEngine::PhysicsScene,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::PhysicsScene),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_physicsScene")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type
                    > ::class(), "set_physicsScene", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_radius(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f32), quest_hook::libil2cpp::Void, 1usize>("set_radius")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::UnityEngine::SpherecastCommand as quest_hook::libil2cpp::Type
                    > ::class(), "set_radius", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
