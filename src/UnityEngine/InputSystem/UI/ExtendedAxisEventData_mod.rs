#[cfg(feature = "UnityEngine+InputSystem+UI+ExtendedAxisEventData")]
#[repr(C)]
#[derive(Debug)]
pub struct ExtendedAxisEventData {
    __cordl_parent: crate::UnityEngine::EventSystems::AxisEventData,
}
#[cfg(feature = "UnityEngine+InputSystem+UI+ExtendedAxisEventData")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::UI::ExtendedAxisEventData => "UnityEngine.InputSystem.UI"
    ."ExtendedAxisEventData"
);
#[cfg(feature = "UnityEngine+InputSystem+UI+ExtendedAxisEventData")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::UI::ExtendedAxisEventData {
    type Target = crate::UnityEngine::EventSystems::AxisEventData;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+UI+ExtendedAxisEventData")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::UI::ExtendedAxisEventData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+UI+ExtendedAxisEventData")]
impl crate::UnityEngine::InputSystem::UI::ExtendedAxisEventData {
    pub fn New(
        eventSystem: *mut crate::UnityEngine::EventSystems::EventSystem,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (eventSystem))?;
        Ok(__cordl_object)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        eventSystem: *mut crate::UnityEngine::EventSystems::EventSystem,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (eventSystem))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+UI+ExtendedAxisEventData")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::UI::ExtendedAxisEventData {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}