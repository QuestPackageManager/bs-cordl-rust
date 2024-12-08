#[cfg(feature = "UnityEngine+UIElements+IVisualElementScheduler")]
#[repr(C)]
#[derive(Debug)]
pub struct IVisualElementScheduler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+UIElements+IVisualElementScheduler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::IVisualElementScheduler
    => "UnityEngine.UIElements"."IVisualElementScheduler"
);
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
    pub fn Execute_Action_1_0(
        &mut self,
        timerUpdateEvent: *mut crate::System::Action_1<
            crate::UnityEngine::UIElements::TimerState,
        >,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::IVisualElementScheduledItem,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::IVisualElementScheduledItem = __cordl_object
            .invoke("Execute", (timerUpdateEvent))?;
        Ok(__cordl_ret)
    }
    pub fn Execute_Action1(
        &mut self,
        updateEvent: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::UIElements::IVisualElementScheduledItem,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UIElements::IVisualElementScheduledItem = __cordl_object
            .invoke("Execute", (updateEvent))?;
        Ok(__cordl_ret)
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
