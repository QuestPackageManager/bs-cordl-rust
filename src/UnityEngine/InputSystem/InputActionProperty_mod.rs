#[cfg(feature = "UnityEngine+InputSystem+InputActionProperty")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InputActionProperty {
    pub m_UseReference: bool,
    pub m_Action: *mut crate::UnityEngine::InputSystem::InputAction,
    pub m_Reference: *mut crate::UnityEngine::InputSystem::InputActionReference,
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionProperty")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::InputActionProperty =>
    "UnityEngine.InputSystem"."InputActionProperty"
);
#[cfg(feature = "UnityEngine+InputSystem+InputActionProperty")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::InputActionProperty {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+InputActionProperty")]
impl crate::UnityEngine::InputSystem::InputActionProperty {
    pub fn _ctor_InputAction0(
        &mut self,
        action: *mut crate::UnityEngine::InputSystem::InputAction,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (action),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_InputActionReference1(
        &mut self,
        reference: *mut crate::UnityEngine::InputSystem::InputActionReference,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (reference),
        )?;
        Ok(__cordl_ret)
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "GetHashCode",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_serializedAction(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputAction,
    > {
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputAction = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_serializedAction",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_action(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputAction,
    > {
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputAction = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_action",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_reference(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionReference,
    > {
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionReference = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_reference",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_serializedReference(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputActionReference,
    > {
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputActionReference = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_serializedReference",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_InputActionProperty0(
        &mut self,
        other: crate::UnityEngine::InputSystem::InputActionProperty,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_InputAction1(
        &mut self,
        other: *mut crate::UnityEngine::InputSystem::InputAction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_InputActionReference2(
        &mut self,
        other: *mut crate::UnityEngine::InputSystem::InputActionReference,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (other),
        )?;
        Ok(__cordl_ret)
    }
    pub fn Equals_Object3(
        &mut self,
        obj: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Equals",
            (obj),
        )?;
        Ok(__cordl_ret)
    }
}
