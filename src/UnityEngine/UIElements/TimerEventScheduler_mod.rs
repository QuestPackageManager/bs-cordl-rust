#[cfg(feature = "UnityEngine+UIElements+TimerEventScheduler")]
#[repr(C)]
#[derive(Debug)]
pub struct TimerEventScheduler {
    __cordl_parent: crate::System::Object,
    pub m_ScheduledItems: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::UIElements::ScheduledItem,
    >,
    pub m_TransactionMode: bool,
    pub m_ScheduleTransactions: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::UIElements::ScheduledItem,
    >,
    pub m_UnscheduleTransactions: *mut crate::System::Collections::Generic::HashSet_1<
        *mut crate::UnityEngine::UIElements::ScheduledItem,
    >,
    pub disableThrottling: bool,
    pub m_LastUpdatedIndex: i32,
}
#[cfg(feature = "UnityEngine+UIElements+TimerEventScheduler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::TimerEventScheduler =>
    "UnityEngine.UIElements"."TimerEventScheduler"
);
#[cfg(feature = "UnityEngine+UIElements+TimerEventScheduler")]
impl std::ops::Deref for crate::UnityEngine::UIElements::TimerEventScheduler {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TimerEventScheduler")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::TimerEventScheduler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TimerEventScheduler")]
impl crate::UnityEngine::UIElements::TimerEventScheduler {
    pub fn PrivateUnSchedule(
        &mut self,
        sItem: *mut crate::UnityEngine::UIElements::ScheduledItem,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("PrivateUnSchedule", (sItem))?;
        Ok(__cordl_ret)
    }
    pub fn Schedule(
        &mut self,
        item: *mut crate::UnityEngine::UIElements::ScheduledItem,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Schedule", (item))?;
        Ok(__cordl_ret)
    }
    pub fn RemovedScheduledItemAt(
        &mut self,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("RemovedScheduledItemAt", (index))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateScheduledEvents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateScheduledEvents", ())?;
        Ok(__cordl_ret)
    }
    pub fn Unschedule(
        &mut self,
        item: *mut crate::UnityEngine::UIElements::ScheduledItem,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Unschedule", (item))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "UnityEngine+UIElements+TimerEventScheduler")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::TimerEventScheduler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
