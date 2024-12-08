#[cfg(feature = "System+Threading+IThreadPoolWorkItem")]
#[repr(C)]
#[derive(Debug)]
pub struct IThreadPoolWorkItem {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Threading+IThreadPoolWorkItem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Threading::IThreadPoolWorkItem =>
    "System.Threading"."IThreadPoolWorkItem"
);
#[cfg(feature = "System+Threading+IThreadPoolWorkItem")]
impl std::ops::Deref for crate::System::Threading::IThreadPoolWorkItem {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+IThreadPoolWorkItem")]
impl std::ops::DerefMut for crate::System::Threading::IThreadPoolWorkItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+IThreadPoolWorkItem")]
impl crate::System::Threading::IThreadPoolWorkItem {
    pub fn ExecuteWorkItem(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ExecuteWorkItem", ())?;
        Ok(__cordl_ret)
    }
    pub fn MarkAborted(
        &mut self,
        tae: *mut crate::System::Threading::ThreadAbortException,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkAborted", (tae))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Threading+IThreadPoolWorkItem")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::IThreadPoolWorkItem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
