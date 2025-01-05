#[cfg(feature = "System+Threading+ThreadPoolWorkQueueThreadLocals")]
#[repr(C)]
#[derive(Debug)]
pub struct ThreadPoolWorkQueueThreadLocals {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub workQueue: quest_hook::libil2cpp::Gc<
        crate::System::Threading::ThreadPoolWorkQueue,
    >,
    pub workStealingQueue: quest_hook::libil2cpp::Gc<
        crate::System::Threading::ThreadPoolWorkQueue_WorkStealingQueue,
    >,
    pub random: quest_hook::libil2cpp::Gc<crate::System::Random>,
}
#[cfg(feature = "System+Threading+ThreadPoolWorkQueueThreadLocals")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Threading::ThreadPoolWorkQueueThreadLocals => "System.Threading"
    ."ThreadPoolWorkQueueThreadLocals"
);
#[cfg(feature = "System+Threading+ThreadPoolWorkQueueThreadLocals")]
impl std::ops::Deref for crate::System::Threading::ThreadPoolWorkQueueThreadLocals {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn CleanUp(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CleanUp", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Finalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Finalize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        tpq: quest_hook::libil2cpp::Gc<crate::System::Threading::ThreadPoolWorkQueue>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tpq))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        tpq: quest_hook::libil2cpp::Gc<crate::System::Threading::ThreadPoolWorkQueue>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (tpq))?;
        Ok(__cordl_ret.into())
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
