#[cfg(feature = "UnityEngine+UIElements+UIR+JobManager")]
#[repr(C)]
#[derive(Debug)]
pub struct JobManager {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_NudgeJobs: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::NativePagedList_1<
            crate::UnityEngine::UIElements::UIR::NudgeJobData,
        >,
    >,
    pub m_ConvertMeshJobs: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::NativePagedList_1<
            crate::UnityEngine::UIElements::UIR::ConvertMeshJobData,
        >,
    >,
    pub m_CopyClosingMeshJobs: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::NativePagedList_1<
            crate::UnityEngine::UIElements::UIR::CopyClosingMeshJobData,
        >,
    >,
    pub m_JobMerger: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UIElements::UIR::JobMerger,
    >,
    pub _disposed_k__BackingField: bool,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+JobManager")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIR::JobManager =>
    "UnityEngine.UIElements.UIR"."JobManager"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+JobManager")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIR::JobManager {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+JobManager")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIR::JobManager {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+JobManager")]
impl crate::UnityEngine::UIElements::UIR::JobManager {
    pub fn Add_ByRefMut0(
        &mut self,
        job: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::NudgeJobData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (job))?;
        Ok(__cordl_ret.into())
    }
    pub fn Add_ByRefMut1(
        &mut self,
        job: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::ConvertMeshJobData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (job))?;
        Ok(__cordl_ret.into())
    }
    pub fn Add_ByRefMut2(
        &mut self,
        job: quest_hook::libil2cpp::ByRefMut<
            crate::UnityEngine::UIElements::UIR::CopyClosingMeshJobData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (job))?;
        Ok(__cordl_ret.into())
    }
    pub fn CompleteClosingMeshJobs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompleteClosingMeshJobs", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CompleteConvertMeshJobs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompleteConvertMeshJobs", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CompleteNudgeJobs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CompleteNudgeJobs", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose__cordl_bool1(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_disposed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_disposed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_disposed(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_disposed", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+JobManager")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIR::JobManager {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+JobManager")]
impl AsRef<crate::System::IDisposable>
for crate::UnityEngine::UIElements::UIR::JobManager {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+JobManager")]
impl AsMut<crate::System::IDisposable>
for crate::UnityEngine::UIElements::UIR::JobManager {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
