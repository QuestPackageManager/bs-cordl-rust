#[cfg(feature = "UnityEngine+UIElements+IVisualElementScheduler")]
#[repr(C)]
#[derive(Debug)]
pub struct IVisualElementScheduler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+IVisualElementScheduler")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::UIElements::IVisualElementScheduler {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.UIElements";
    const CLASS_NAME: &'static str = "IVisualElementScheduler";
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
#[cfg(feature = "UnityEngine+UIElements+IVisualElementScheduler")]
impl std::ops::Deref for crate::UnityEngine::UIElements::IVisualElementScheduler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IVisualElementScheduler")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::IVisualElementScheduler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IVisualElementScheduler")]
impl crate::UnityEngine::UIElements::IVisualElementScheduler {
    pub fn Execute_Action1(
        &mut self,
        updateEvent: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IVisualElementScheduledItem,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::IVisualElementScheduledItem,
                >,
                1usize,
            >("Execute")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Execute", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IVisualElementScheduledItem,
        > = unsafe { method.invoke_unchecked(self, (updateEvent)) };
        Ok(__cordl_ret.into())
    }
    pub fn Execute_Action_1_0(
        &mut self,
        timerUpdateEvent: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<crate::UnityEngine::UIElements::TimerState>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IVisualElementScheduledItem,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<crate::UnityEngine::UIElements::TimerState>,
                >),
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::UIElements::IVisualElementScheduledItem,
                >,
                1usize,
            >("Execute")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Execute", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::IVisualElementScheduledItem,
        > = unsafe { method.invoke_unchecked(self, (timerUpdateEvent)) };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+IVisualElementScheduler")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::IVisualElementScheduler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
