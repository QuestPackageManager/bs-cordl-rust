#[cfg(feature = "UnityEngine+InputSystem+IInputActionCollection2")]
#[repr(C)]
#[derive(Debug)]
pub struct IInputActionCollection2 {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+IInputActionCollection2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::IInputActionCollection2 => "UnityEngine.InputSystem"
    ."IInputActionCollection2"
);
#[cfg(feature = "UnityEngine+InputSystem+IInputActionCollection2")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::IInputActionCollection2 {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+IInputActionCollection2")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::IInputActionCollection2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+IInputActionCollection2")]
impl crate::UnityEngine::InputSystem::IInputActionCollection2 {
    pub fn FindAction(
        &mut self,
        actionNameOrId: *mut crate::System::String,
        throwIfNotFound: bool,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::UnityEngine::InputSystem::InputAction,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::InputSystem::InputAction = __cordl_object
            .invoke("FindAction", (actionNameOrId, throwIfNotFound))?;
        Ok(__cordl_ret)
    }
    pub fn FindBinding(
        &mut self,
        mask: crate::UnityEngine::InputSystem::InputBinding,
        action: quest_hook::libil2cpp::ByRefMut<
            *mut crate::UnityEngine::InputSystem::InputAction,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("FindBinding", (mask, action))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_bindings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            crate::UnityEngine::InputSystem::InputBinding,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            crate::UnityEngine::InputSystem::InputBinding,
        > = __cordl_object.invoke("get_bindings", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+IInputActionCollection2")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::IInputActionCollection2 {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
