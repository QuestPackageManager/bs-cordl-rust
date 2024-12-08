#[cfg(feature = "UnityEngine+UIElements+MouseManipulator")]
#[repr(C)]
#[derive(Debug)]
pub struct MouseManipulator {
    __cordl_parent: crate::UnityEngine::UIElements::Manipulator,
    pub _activators_k__BackingField: *mut crate::System::Collections::Generic::List_1<
        crate::UnityEngine::UIElements::ManipulatorActivationFilter,
    >,
    pub m_currentActivator: crate::UnityEngine::UIElements::ManipulatorActivationFilter,
}
#[cfg(feature = "UnityEngine+UIElements+MouseManipulator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::MouseManipulator =>
    "UnityEngine.UIElements"."MouseManipulator"
);
#[cfg(feature = "UnityEngine+UIElements+MouseManipulator")]
impl std::ops::Deref for crate::UnityEngine::UIElements::MouseManipulator {
    type Target = crate::UnityEngine::UIElements::Manipulator;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MouseManipulator")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::MouseManipulator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+MouseManipulator")]
impl crate::UnityEngine::UIElements::MouseManipulator {
    pub fn CanStartManipulation(
        &mut self,
        e: *mut crate::UnityEngine::UIElements::IMouseEvent,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanStartManipulation", (e))?;
        Ok(__cordl_ret)
    }
    pub fn CanStopManipulation(
        &mut self,
        e: *mut crate::UnityEngine::UIElements::IMouseEvent,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("CanStopManipulation", (e))?;
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
    pub fn set_activators(
        &mut self,
        value: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::ManipulatorActivationFilter,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_activators", (value))?;
        Ok(__cordl_ret)
    }
    pub fn get_activators(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::ManipulatorActivationFilter,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            crate::UnityEngine::UIElements::ManipulatorActivationFilter,
        > = __cordl_object.invoke("get_activators", ())?;
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
#[cfg(feature = "UnityEngine+UIElements+MouseManipulator")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::MouseManipulator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
