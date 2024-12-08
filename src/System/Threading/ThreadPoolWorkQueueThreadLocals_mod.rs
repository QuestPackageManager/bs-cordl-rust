#[cfg(feature = "System+Threading+ThreadPoolWorkQueueThreadLocals")]
#[repr(C)]
#[derive(Debug)]
pub struct ThreadPoolWorkQueueThreadLocals {
    __cordl_parent: crate::System::Object,
    pub workQueue: *mut crate::System::Threading::ThreadPoolWorkQueue,
    pub workStealingQueue: *mut crate::System::Threading::ThreadPoolWorkQueue_WorkStealingQueue,
    pub random: *mut crate::System::Random,
}
#[cfg(feature = "System+Threading+ThreadPoolWorkQueueThreadLocals")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Threading::ThreadPoolWorkQueueThreadLocals => "System.Threading"
    ."ThreadPoolWorkQueueThreadLocals"
);
#[cfg(feature = "System+Threading+ThreadPoolWorkQueueThreadLocals")]
impl std::ops::Deref for crate::System::Threading::ThreadPoolWorkQueueThreadLocals {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+ThreadPoolWorkQueueThreadLocals")]
impl std::ops::DerefMut for crate::System::Threading::ThreadPoolWorkQueueThreadLocals {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Threading+ThreadPoolWorkQueueThreadLocals")]
impl crate::System::Threading::ThreadPoolWorkQueueThreadLocals {
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret)
    }
    pub fn CleanUp(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CleanUp", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        tpq: *mut crate::System::Threading::ThreadPoolWorkQueue,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (tpq))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        tpq: *mut crate::System::Threading::ThreadPoolWorkQueue,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tpq))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Threading+ThreadPoolWorkQueueThreadLocals")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Threading::ThreadPoolWorkQueueThreadLocals {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
