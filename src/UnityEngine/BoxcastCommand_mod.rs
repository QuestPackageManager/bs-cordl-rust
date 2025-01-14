#[cfg(feature = "UnityEngine+BoxcastCommand")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct BoxcastCommand {
    pub _center_k__BackingField: crate::UnityEngine::Vector3,
    pub _halfExtents_k__BackingField: crate::UnityEngine::Vector3,
    pub _orientation_k__BackingField: crate::UnityEngine::Quaternion,
    pub _direction_k__BackingField: crate::UnityEngine::Vector3,
    pub _distance_k__BackingField: f32,
    pub _physicsScene_k__BackingField: crate::UnityEngine::PhysicsScene,
    pub queryParameters: crate::UnityEngine::QueryParameters,
}
#[cfg(feature = "UnityEngine+BoxcastCommand")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::BoxcastCommand {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "BoxcastCommand";
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
#[cfg(feature = "UnityEngine+BoxcastCommand")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::BoxcastCommand {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+BoxcastCommand")]
unsafe impl quest_hook::libil2cpp::Parameter for crate::UnityEngine::BoxcastCommand {
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
#[cfg(feature = "UnityEngine+BoxcastCommand")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::BoxcastCommand {
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
#[cfg(feature = "UnityEngine+BoxcastCommand")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::BoxcastCommand {
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
#[cfg(feature = "UnityEngine+BoxcastCommand")]
unsafe impl quest_hook::libil2cpp::ThisArgument for crate::UnityEngine::BoxcastCommand {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+BoxcastCommand")]
impl crate::UnityEngine::BoxcastCommand {
    pub fn ScheduleBatch_JobHandle1(
        commands: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::BoxcastCommand,
        >,
        results: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::RaycastHit,
        >,
        minCommandsPerJob: i32,
        dependsOn: crate::Unity::Jobs::JobHandle,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Collections::NativeArray_1<
                        crate::UnityEngine::BoxcastCommand,
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
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ScheduleBatch", 4usize
                )
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle = unsafe {
            method
                .invoke_unchecked((), (commands, results, minCommandsPerJob, dependsOn))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleBatch_i32_JobHandle0(
        commands: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::BoxcastCommand,
        >,
        results: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::RaycastHit,
        >,
        minCommandsPerJob: i32,
        maxHits: i32,
        dependsOn: crate::Unity::Jobs::JobHandle,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::Unity::Collections::NativeArray_1<
                        crate::UnityEngine::BoxcastCommand,
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
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ScheduleBatch", 5usize
                )
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle = unsafe {
            method
                .invoke_unchecked(
                    (),
                    (commands, results, minCommandsPerJob, maxHits, dependsOn),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleBoxcastBatch(
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
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
            >("ScheduleBoxcastBatch")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ScheduleBoxcastBatch", 7usize
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
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleBoxcastBatch_Injected(
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
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
            >("ScheduleBoxcastBatch_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ScheduleBoxcastBatch_Injected", 8usize
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
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_PhysicsScene_Vector3_Quaternion_Vector3_QueryParameters_f32_1(
        &mut self,
        physicsScene: crate::UnityEngine::PhysicsScene,
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
        direction: crate::UnityEngine::Vector3,
        queryParameters: crate::UnityEngine::QueryParameters,
        distance: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::PhysicsScene,
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Quaternion,
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::QueryParameters,
                    f32,
                ),
                quest_hook::libil2cpp::Void,
                7usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 7usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        physicsScene,
                        center,
                        halfExtents,
                        orientation,
                        direction,
                        queryParameters,
                        distance,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_PhysicsScene_Vector3_Quaternion_Vector3_f32_i32_3(
        &mut self,
        physicsScene: crate::UnityEngine::PhysicsScene,
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
        direction: crate::UnityEngine::Vector3,
        distance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::PhysicsScene,
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Quaternion,
                    crate::UnityEngine::Vector3,
                    f32,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                7usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 7usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        physicsScene,
                        center,
                        halfExtents,
                        orientation,
                        direction,
                        distance,
                        layerMask,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Vector3_Quaternion_Vector3_QueryParameters_f32_0(
        &mut self,
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
        direction: crate::UnityEngine::Vector3,
        queryParameters: crate::UnityEngine::QueryParameters,
        distance: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Quaternion,
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::QueryParameters,
                    f32,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        center,
                        halfExtents,
                        orientation,
                        direction,
                        queryParameters,
                        distance,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Vector3_Quaternion_Vector3_f32_i32_2(
        &mut self,
        center: crate::UnityEngine::Vector3,
        halfExtents: crate::UnityEngine::Vector3,
        orientation: crate::UnityEngine::Quaternion,
        direction: crate::UnityEngine::Vector3,
        distance: f32,
        layerMask: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Vector3,
                    crate::UnityEngine::Quaternion,
                    crate::UnityEngine::Vector3,
                    f32,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                6usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 6usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (center, halfExtents, orientation, direction, distance, layerMask),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_center(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::UnityEngine::Vector3, 0usize>("get_center")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_center", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_direction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::UnityEngine::Vector3, 0usize>("get_direction")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_direction", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_distance(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), f32, 0usize>("get_distance")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_distance", 0usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_halfExtents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::UnityEngine::Vector3, 0usize>("get_halfExtents")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_halfExtents", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Vector3 = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_layerMask(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_layerMask")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_layerMask", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn get_orientation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), crate::UnityEngine::Quaternion, 0usize>("get_orientation")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_orientation", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::Quaternion = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_physicsScene(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::PhysicsScene> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::PhysicsScene,
                0usize,
            >("get_physicsScene")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_physicsScene", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::PhysicsScene = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_center(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Vector3),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_center")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_center", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_direction(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Vector3),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_direction")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_direction", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_distance(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(f32), quest_hook::libil2cpp::Void, 1usize>("set_distance")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_distance", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_halfExtents(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Vector3),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_halfExtents")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_halfExtents", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_layerMask(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("set_layerMask")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_layerMask", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_orientation(
        &mut self,
        value: crate::UnityEngine::Quaternion,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::Quaternion),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_orientation")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_orientation", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_physicsScene(
        &mut self,
        value: crate::UnityEngine::PhysicsScene,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::PhysicsScene),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_physicsScene")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_physicsScene", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
}
