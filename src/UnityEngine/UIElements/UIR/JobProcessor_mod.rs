#[cfg(feature = "UnityEngine+UIElements+UIR+JobProcessor")]
#[repr(C)]
#[derive(Debug)]
pub struct JobProcessor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+JobProcessor")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::UIR::JobProcessor {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements.UIR";
    const CLASS_NAME: &'static str = "JobProcessor";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+JobProcessor")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIR::JobProcessor {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+JobProcessor")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIR::JobProcessor {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+JobProcessor")]
impl crate::UnityEngine::UIElements::UIR::JobProcessor {
    pub fn ScheduleConvertMeshJobs(
        buffer: crate::System::IntPtr,
        jobCount: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, i32),
                crate::Unity::Jobs::JobHandle,
                2usize,
            >("ScheduleConvertMeshJobs")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ScheduleConvertMeshJobs", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle = unsafe {
            method.invoke_unchecked((), (buffer, jobCount))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleConvertMeshJobs_Injected(
        buffer: crate::System::IntPtr,
        jobCount: i32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::JobHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    i32,
                    quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::JobHandle>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("ScheduleConvertMeshJobs_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ScheduleConvertMeshJobs_Injected", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (buffer, jobCount, ret))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleCopyClosingMeshJobs(
        buffer: crate::System::IntPtr,
        jobCount: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, i32),
                crate::Unity::Jobs::JobHandle,
                2usize,
            >("ScheduleCopyClosingMeshJobs")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ScheduleCopyClosingMeshJobs", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle = unsafe {
            method.invoke_unchecked((), (buffer, jobCount))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleCopyClosingMeshJobs_Injected(
        buffer: crate::System::IntPtr,
        jobCount: i32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::JobHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    i32,
                    quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::JobHandle>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("ScheduleCopyClosingMeshJobs_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ScheduleCopyClosingMeshJobs_Injected", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (buffer, jobCount, ret))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleNudgeJobs(
        buffer: crate::System::IntPtr,
        jobCount: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::System::IntPtr, i32),
                crate::Unity::Jobs::JobHandle,
                2usize,
            >("ScheduleNudgeJobs")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ScheduleNudgeJobs", 2usize
                )
            });
        let __cordl_ret: crate::Unity::Jobs::JobHandle = unsafe {
            method.invoke_unchecked((), (buffer, jobCount))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleNudgeJobs_Injected(
        buffer: crate::System::IntPtr,
        jobCount: i32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::JobHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    crate::System::IntPtr,
                    i32,
                    quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::JobHandle>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("ScheduleNudgeJobs_Injected")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ScheduleNudgeJobs_Injected", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (buffer, jobCount, ret))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+JobProcessor")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIR::JobProcessor {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
