#[cfg(feature = "UnityEngine+InputSystem+IInputActionCollection")]
#[repr(C)]
#[derive(Debug)]
pub struct IInputActionCollection {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+IInputActionCollection")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::IInputActionCollection {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "IInputActionCollection";
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
#[cfg(feature = "UnityEngine+InputSystem+IInputActionCollection")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::IInputActionCollection {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+IInputActionCollection")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::IInputActionCollection {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+IInputActionCollection")]
impl crate::UnityEngine::InputSystem::IInputActionCollection {
    pub fn Contains(
        &mut self,
        action: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Contains", (action))?;
        Ok(__cordl_ret.into())
    }
    pub fn Disable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Disable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Enable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Enable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_bindingMask(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<crate::UnityEngine::InputSystem::InputBinding>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::InputBinding,
        > = __cordl_object.invoke("get_bindingMask", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_controlSchemes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::InputControlScheme,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::InputControlScheme,
        > = __cordl_object.invoke("get_controlSchemes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_devices(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
            >,
        > = __cordl_object.invoke("get_devices", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_bindingMask(
        &mut self,
        value: crate::System::Nullable_1<crate::UnityEngine::InputSystem::InputBinding>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_bindingMask", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_devices(
        &mut self,
        value: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputDevice>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_devices", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+IInputActionCollection")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::IInputActionCollection {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+IInputActionCollection")]
impl AsRef<
    crate::System::Collections::Generic::IEnumerable_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    >,
> for crate::UnityEngine::InputSystem::IInputActionCollection {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEnumerable_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+IInputActionCollection")]
impl AsMut<
    crate::System::Collections::Generic::IEnumerable_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    >,
> for crate::UnityEngine::InputSystem::IInputActionCollection {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerable_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+IInputActionCollection")]
impl AsRef<crate::System::Collections::IEnumerable>
for crate::UnityEngine::InputSystem::IInputActionCollection {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+IInputActionCollection")]
impl AsMut<crate::System::Collections::IEnumerable>
for crate::UnityEngine::InputSystem::IInputActionCollection {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
