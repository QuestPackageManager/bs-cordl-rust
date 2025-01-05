#[cfg(feature = "UnityEngine+UIElements+UIR+JobProcessor")]
#[repr(C)]
#[derive(Debug)]
pub struct JobProcessor {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+JobProcessor")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIR::JobProcessor =>
    "UnityEngine.UIElements.UIR"."JobProcessor"
);
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
        let __cordl_ret: crate::Unity::Jobs::JobHandle = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ScheduleConvertMeshJobs", (buffer, jobCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleConvertMeshJobs_Injected(
        buffer: crate::System::IntPtr,
        jobCount: i32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::JobHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ScheduleConvertMeshJobs_Injected", (buffer, jobCount, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleCopyClosingMeshJobs(
        buffer: crate::System::IntPtr,
        jobCount: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        let __cordl_ret: crate::Unity::Jobs::JobHandle = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ScheduleCopyClosingMeshJobs", (buffer, jobCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleCopyClosingMeshJobs_Injected(
        buffer: crate::System::IntPtr,
        jobCount: i32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::JobHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ScheduleCopyClosingMeshJobs_Injected", (buffer, jobCount, ret))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleNudgeJobs(
        buffer: crate::System::IntPtr,
        jobCount: i32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Jobs::JobHandle> {
        let __cordl_ret: crate::Unity::Jobs::JobHandle = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ScheduleNudgeJobs", (buffer, jobCount))?;
        Ok(__cordl_ret.into())
    }
    pub fn ScheduleNudgeJobs_Injected(
        buffer: crate::System::IntPtr,
        jobCount: i32,
        ret: quest_hook::libil2cpp::ByRefMut<crate::Unity::Jobs::JobHandle>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ScheduleNudgeJobs_Injected", (buffer, jobCount, ret))?;
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
