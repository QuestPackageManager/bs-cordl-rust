#[cfg(feature = "UnityEngine+ClosestPointCommand")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ClosestPointCommand {
    pub _point_k__BackingField: crate::UnityEngine::Vector3,
    pub _colliderInstanceID_k__BackingField: i32,
    pub _position_k__BackingField: crate::UnityEngine::Vector3,
    pub _rotation_k__BackingField: crate::UnityEngine::Quaternion,
    pub _scale_k__BackingField: crate::UnityEngine::Vector3,
}
#[cfg(feature = "UnityEngine+ClosestPointCommand")]
unsafe impl quest_hook::libil2cpp::Type for crate::UnityEngine::ClosestPointCommand {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine";
    const CLASS_NAME: &'static str = "ClosestPointCommand";
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
#[cfg(feature = "UnityEngine+ClosestPointCommand")]
unsafe impl quest_hook::libil2cpp::Argument for crate::UnityEngine::ClosestPointCommand {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+ClosestPointCommand")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::ClosestPointCommand {
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
#[cfg(feature = "UnityEngine+ClosestPointCommand")]
unsafe impl quest_hook::libil2cpp::Returned for crate::UnityEngine::ClosestPointCommand {
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
#[cfg(feature = "UnityEngine+ClosestPointCommand")]
unsafe impl quest_hook::libil2cpp::Return for crate::UnityEngine::ClosestPointCommand {
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
    pub fn ScheduleBatch(
        commands: crate::Unity::Collections::NativeArray_1<
            crate::UnityEngine::ClosestPointCommand,
        >,
        results: crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Vector3>,
        minCommandsPerJob: i32,
        dependsOn: crate::Unity::Jobs::JobHandle,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        let __cordl_ret: crate::Unity::Jobs::JobHandle = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ScheduleBatch", (commands, results, minCommandsPerJob, dependsOn))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleClosestPointCommandBatch(
        parameters: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_JobScheduleParameters,
        >,
        commands: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        commandLen: i32,
        result: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        resultLen: i32,
        minCommandsPerJob: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        let __cordl_ret: crate::Unity::Jobs::JobHandle = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ScheduleClosestPointCommandBatch",
                (parameters, commands, commandLen, result, resultLen, minCommandsPerJob),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleClosestPointCommandBatch_Injected(
        parameters: quest_hook::libil2cpp::ByRefMut<
            crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_JobScheduleParameters,
        >,
        commands: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        commandLen: i32,
        result: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        resultLen: i32,
        minCommandsPerJob: i32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::JobHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ScheduleClosestPointCommandBatch_Injected",
                (
                    parameters,
                    commands,
                    commandLen,
                    result,
                    resultLen,
                    minCommandsPerJob,
                    ret,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Collider1(
        &mut self,
        point: crate::UnityEngine::Vector3,
        collider: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
        position: crate::UnityEngine::Vector3,
        rotation: crate::UnityEngine::Quaternion,
        scale: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (point, collider, position, rotation, scale),
        )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn get_colliderInstanceID(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_colliderInstanceID",
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
    pub fn get_position(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_position",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_rotation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Quaternion> {
        let __cordl_ret: crate::UnityEngine::Quaternion = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_rotation",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_scale(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_ret: crate::UnityEngine::Vector3 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_scale",
            (),
        )?;
        Ok(__cordl_ret.into())
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
    pub fn set_position(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "set_position",
            (value),
        )?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
}
