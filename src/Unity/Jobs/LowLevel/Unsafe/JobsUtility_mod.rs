#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobsUtility")]
#[repr(C)]
#[derive(Debug)]
pub struct JobsUtility {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobsUtility")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Jobs::LowLevel::Unsafe::JobsUtility =>
    "Unity.Jobs.LowLevel.Unsafe"."JobsUtility"
);
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobsUtility")]
impl std::ops::Deref for crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobsUtility")]
impl std::ops::DerefMut for crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobsUtility")]
impl crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility {
    #[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobsUtility+JobScheduleParameters")]
    pub type JobScheduleParameters = crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_JobScheduleParameters;
    #[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobsUtility+PanicFunction_")]
    pub type PanicFunction_ = crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_PanicFunction_;
}
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobsUtility")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobsUtility+JobScheduleParameters")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct JobsUtility_JobScheduleParameters {
    pub Dependency: crate::Unity::Jobs::JobHandle,
    pub ScheduleMode: i32,
    pub ReflectionData: crate::System::IntPtr,
    pub JobDataPtr: crate::System::IntPtr,
}
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobsUtility+JobScheduleParameters")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Jobs::LowLevel::Unsafe::JobsUtility_JobScheduleParameters =>
    "Unity.Jobs.LowLevel.Unsafe"."JobsUtility/JobScheduleParameters"
);
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobsUtility+JobScheduleParameters")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_JobScheduleParameters {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobsUtility+JobScheduleParameters")]
impl crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_JobScheduleParameters {
    pub fn _ctor(
        &mut self,
        i_jobData: *mut quest_hook::libil2cpp::Il2CppObject,
        i_reflectionData: crate::System::IntPtr,
        i_dependency: crate::Unity::Jobs::JobHandle,
        i_scheduleMode: crate::Unity::Jobs::LowLevel::Unsafe::ScheduleMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (i_jobData, i_reflectionData, i_dependency, i_scheduleMode),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobsUtility+PanicFunction_")]
#[repr(C)]
#[derive(Debug)]
pub struct JobsUtility_PanicFunction_ {
    __cordl_parent: crate::System::MulticastDelegate,
}
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobsUtility+PanicFunction_")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Unity::Jobs::LowLevel::Unsafe::JobsUtility_PanicFunction_ =>
    "Unity.Jobs.LowLevel.Unsafe"."JobsUtility/PanicFunction_"
);
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobsUtility+PanicFunction_")]
impl std::ops::Deref
for crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_PanicFunction_ {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobsUtility+PanicFunction_")]
impl std::ops::DerefMut
for crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_PanicFunction_ {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobsUtility+PanicFunction_")]
impl crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_PanicFunction_ {
    pub fn Invoke(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Invoke", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        object: *mut quest_hook::libil2cpp::Il2CppObject,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        object: *mut quest_hook::libil2cpp::Il2CppObject,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Unity+Jobs+LowLevel+Unsafe+JobsUtility+PanicFunction_")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Jobs::LowLevel::Unsafe::JobsUtility_PanicFunction_ {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
