#[cfg(feature = "UnityEngine+UIElements+TimerEventScheduler")]
#[repr(C)]
#[derive(Debug)]
pub struct TimerEventScheduler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_ScheduledItems: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::ScheduledItem>,
        >,
    >,
    pub m_TransactionMode: bool,
    pub m_ScheduleTransactions: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::ScheduledItem>,
        >,
    >,
    pub m_UnscheduleTransactions: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::ScheduledItem>,
        >,
    >,
    pub disableThrottling: bool,
    pub m_LastUpdatedIndex: i32,
}
#[cfg(feature = "UnityEngine+UIElements+TimerEventScheduler")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::TimerEventScheduler {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "TimerEventScheduler";
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
#[cfg(feature = "UnityEngine+UIElements+TimerEventScheduler")]
impl std::ops::Deref for crate::UnityEngine::UIElements::TimerEventScheduler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PrivateUnSchedule(
        &mut self,
        sItem: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::ScheduledItem>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("PrivateUnSchedule", (sItem))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn Schedule(
        &mut self,
        item: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::ScheduledItem>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Schedule", (item))?;
        Ok(__cordl_ret.into())
    }
    pub fn Unschedule(
        &mut self,
        item: quest_hook::libil2cpp::Gc<crate::UnityEngine::UIElements::ScheduledItem>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Unschedule", (item))?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateScheduledEvents(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateScheduledEvents", ())?;
        Ok(__cordl_ret.into())
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
#[cfg(feature = "UnityEngine+UIElements+TimerEventScheduler")]
impl AsRef<crate::UnityEngine::UIElements::IScheduler>
for crate::UnityEngine::UIElements::TimerEventScheduler {
    fn as_ref(&self) -> &crate::UnityEngine::UIElements::IScheduler {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+TimerEventScheduler")]
impl AsMut<crate::UnityEngine::UIElements::IScheduler>
for crate::UnityEngine::UIElements::TimerEventScheduler {
    fn as_mut(&mut self) -> &mut crate::UnityEngine::UIElements::IScheduler {
        unsafe { std::mem::transmute(self) }
    }
}
