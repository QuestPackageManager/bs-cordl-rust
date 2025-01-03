#[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultInputActions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _asset_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputActionAsset,
    >,
    pub m_Player: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputActionMap,
    >,
    pub m_PlayerActionsCallbackInterface: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::DefaultInputActions_IPlayerActions,
    >,
    pub m_Player_Move: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputAction,
    >,
    pub m_Player_Look: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputAction,
    >,
    pub m_Player_Fire: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputAction,
    >,
    pub m_UI: quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionMap>,
    pub m_UIActionsCallbackInterface: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::DefaultInputActions_IUIActions,
    >,
    pub m_UI_Navigate: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputAction,
    >,
    pub m_UI_Submit: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputAction,
    >,
    pub m_UI_Cancel: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputAction,
    >,
    pub m_UI_Point: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputAction,
    >,
    pub m_UI_Click: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputAction,
    >,
    pub m_UI_ScrollWheel: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputAction,
    >,
    pub m_UI_MiddleClick: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputAction,
    >,
    pub m_UI_RightClick: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputAction,
    >,
    pub m_UI_TrackedDevicePosition: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputAction,
    >,
    pub m_UI_TrackedDeviceOrientation: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::InputAction,
    >,
    pub m_KeyboardMouseSchemeIndex: i32,
    pub m_GamepadSchemeIndex: i32,
    pub m_TouchSchemeIndex: i32,
    pub m_JoystickSchemeIndex: i32,
    pub m_XRSchemeIndex: i32,
}
#[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::InputSystem::DefaultInputActions =>
    "UnityEngine.InputSystem"."DefaultInputActions"
);
#[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::DefaultInputActions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::DefaultInputActions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions")]
impl crate::UnityEngine::InputSystem::DefaultInputActions {
    #[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions+IPlayerActions")]
    type IPlayerActions = crate::UnityEngine::InputSystem::DefaultInputActions_IPlayerActions;
    #[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions+IUIActions")]
    type IUIActions = crate::UnityEngine::InputSystem::DefaultInputActions_IUIActions;
    #[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions+PlayerActions")]
    pub type PlayerActions = crate::UnityEngine::InputSystem::DefaultInputActions_PlayerActions;
    #[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions+UIActions")]
    pub type UIActions = crate::UnityEngine::InputSystem::DefaultInputActions_UIActions;
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
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
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
    pub fn FindAction(
        &mut self,
        actionNameOrId: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        throwIfNotFound: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputAction,
        > = __cordl_object.invoke("FindAction", (actionNameOrId, throwIfNotFound))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindBinding(
        &mut self,
        bindingMask: crate::UnityEngine::InputSystem::InputBinding,
        action: quest_hook::libil2cpp::ByRefMut<
            *mut crate::UnityEngine::InputSystem::InputAction,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("FindBinding", (bindingMask, action))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                *mut crate::UnityEngine::InputSystem::InputAction,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                *mut crate::UnityEngine::InputSystem::InputAction,
            >,
        > = __cordl_object.invoke("GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("System.Collections.IEnumerable.GetEnumerator", ())?;
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
    pub fn get_GamepadScheme(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlScheme,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlScheme = __cordl_object
            .invoke("get_GamepadScheme", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_JoystickScheme(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlScheme,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlScheme = __cordl_object
            .invoke("get_JoystickScheme", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_KeyboardMouseScheme(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlScheme,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlScheme = __cordl_object
            .invoke("get_KeyboardMouseScheme", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Player(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::DefaultInputActions_PlayerActions,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::DefaultInputActions_PlayerActions = __cordl_object
            .invoke("get_Player", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TouchScheme(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlScheme,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlScheme = __cordl_object
            .invoke("get_TouchScheme", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_UI(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::DefaultInputActions_UIActions,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::DefaultInputActions_UIActions = __cordl_object
            .invoke("get_UI", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_XRScheme(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::InputControlScheme,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::InputSystem::InputControlScheme = __cordl_object
            .invoke("get_XRScheme", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_asset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionAsset>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionAsset,
        > = __cordl_object.invoke("get_asset", ())?;
        Ok(__cordl_ret.into())
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
    pub fn get_bindings(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::InputSystem::InputBinding,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::UnityEngine::InputSystem::InputBinding,
            >,
        > = __cordl_object.invoke("get_bindings", ())?;
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
                *mut crate::UnityEngine::InputSystem::InputDevice,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Nullable_1<
            crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
                *mut crate::UnityEngine::InputSystem::InputDevice,
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
                *mut crate::UnityEngine::InputSystem::InputDevice,
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
#[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::DefaultInputActions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions")]
impl AsRef<
    crate::System::Collections::Generic::IEnumerable_1<
        *mut crate::UnityEngine::InputSystem::InputAction,
    >,
> for crate::UnityEngine::InputSystem::DefaultInputActions {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEnumerable_1<
        *mut crate::UnityEngine::InputSystem::InputAction,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions")]
impl AsMut<
    crate::System::Collections::Generic::IEnumerable_1<
        *mut crate::UnityEngine::InputSystem::InputAction,
    >,
> for crate::UnityEngine::InputSystem::DefaultInputActions {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerable_1<
        *mut crate::UnityEngine::InputSystem::InputAction,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions")]
impl AsRef<crate::System::Collections::IEnumerable>
for crate::UnityEngine::InputSystem::DefaultInputActions {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions")]
impl AsMut<crate::System::Collections::IEnumerable>
for crate::UnityEngine::InputSystem::DefaultInputActions {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions")]
impl AsRef<crate::System::IDisposable>
for crate::UnityEngine::InputSystem::DefaultInputActions {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions")]
impl AsMut<crate::System::IDisposable>
for crate::UnityEngine::InputSystem::DefaultInputActions {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions")]
impl AsRef<crate::UnityEngine::InputSystem::IInputActionCollection>
for crate::UnityEngine::InputSystem::DefaultInputActions {
    fn as_ref(&self) -> &crate::UnityEngine::InputSystem::IInputActionCollection {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions")]
impl AsMut<crate::UnityEngine::InputSystem::IInputActionCollection>
for crate::UnityEngine::InputSystem::DefaultInputActions {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::InputSystem::IInputActionCollection {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions")]
impl AsRef<crate::UnityEngine::InputSystem::IInputActionCollection2>
for crate::UnityEngine::InputSystem::DefaultInputActions {
    fn as_ref(&self) -> &crate::UnityEngine::InputSystem::IInputActionCollection2 {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions")]
impl AsMut<crate::UnityEngine::InputSystem::IInputActionCollection2>
for crate::UnityEngine::InputSystem::DefaultInputActions {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::InputSystem::IInputActionCollection2 {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions+IPlayerActions")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultInputActions_IPlayerActions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions+IPlayerActions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::DefaultInputActions_IPlayerActions =>
    "UnityEngine.InputSystem"."DefaultInputActions/IPlayerActions"
);
#[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions+IPlayerActions")]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::DefaultInputActions_IPlayerActions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions+IPlayerActions")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::DefaultInputActions_IPlayerActions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions+IPlayerActions")]
impl crate::UnityEngine::InputSystem::DefaultInputActions_IPlayerActions {
    pub fn OnFire(
        &mut self,
        context: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnFire", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnLook(
        &mut self,
        context: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnLook", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnMove(
        &mut self,
        context: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnMove", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions+IPlayerActions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::DefaultInputActions_IPlayerActions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions+IUIActions")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultInputActions_IUIActions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions+IUIActions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::DefaultInputActions_IUIActions =>
    "UnityEngine.InputSystem"."DefaultInputActions/IUIActions"
);
#[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions+IUIActions")]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::DefaultInputActions_IUIActions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions+IUIActions")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::DefaultInputActions_IUIActions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions+IUIActions")]
impl crate::UnityEngine::InputSystem::DefaultInputActions_IUIActions {
    pub fn OnCancel(
        &mut self,
        context: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnCancel", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnClick(
        &mut self,
        context: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnClick", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnMiddleClick(
        &mut self,
        context: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnMiddleClick", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnNavigate(
        &mut self,
        context: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnNavigate", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnPoint(
        &mut self,
        context: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnPoint", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnRightClick(
        &mut self,
        context: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnRightClick", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnScrollWheel(
        &mut self,
        context: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnScrollWheel", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnSubmit(
        &mut self,
        context: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnSubmit", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnTrackedDeviceOrientation(
        &mut self,
        context: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnTrackedDeviceOrientation", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn OnTrackedDevicePosition(
        &mut self,
        context: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnTrackedDevicePosition", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions+IUIActions")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::DefaultInputActions_IUIActions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions+PlayerActions")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DefaultInputActions_PlayerActions {
    pub m_Wrapper: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::DefaultInputActions,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions+PlayerActions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::DefaultInputActions_PlayerActions =>
    "UnityEngine.InputSystem"."DefaultInputActions/PlayerActions"
);
#[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions+PlayerActions")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::DefaultInputActions_PlayerActions {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions+PlayerActions")]
impl crate::UnityEngine::InputSystem::DefaultInputActions_PlayerActions {
    pub fn Disable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Disable",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Enable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Enable",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Get(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionMap>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionMap,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "Get", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetCallbacks(
        &mut self,
        instance: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::DefaultInputActions_IPlayerActions,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetCallbacks",
            (instance),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        wrapper: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::DefaultInputActions,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (wrapper),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Fire(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputAction,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Fire", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Look(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputAction,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Look", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Move(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputAction,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Move", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_enabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_enabled",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        set: crate::UnityEngine::InputSystem::DefaultInputActions_PlayerActions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionMap>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionMap,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("op_Implicit", (set))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions+UIActions")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct DefaultInputActions_UIActions {
    pub m_Wrapper: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::InputSystem::DefaultInputActions,
    >,
}
#[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions+UIActions")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::InputSystem::DefaultInputActions_UIActions =>
    "UnityEngine.InputSystem"."DefaultInputActions/UIActions"
);
#[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions+UIActions")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::DefaultInputActions_UIActions {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+DefaultInputActions+UIActions")]
impl crate::UnityEngine::InputSystem::DefaultInputActions_UIActions {
    pub fn Disable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Disable",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Enable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Enable",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn Get(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionMap>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionMap,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "Get", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetCallbacks(
        &mut self,
        instance: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::DefaultInputActions_IUIActions,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "SetCallbacks",
            (instance),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        wrapper: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::DefaultInputActions,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (wrapper),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Cancel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputAction,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Cancel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Click(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputAction,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Click", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MiddleClick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputAction,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_MiddleClick", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Navigate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputAction,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Navigate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Point(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputAction,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Point", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_RightClick(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputAction,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_RightClick", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ScrollWheel(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputAction,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_ScrollWheel", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Submit(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputAction,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(self, "get_Submit", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TrackedDeviceOrientation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputAction,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_TrackedDeviceOrientation",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TrackedDevicePosition(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputAction>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputAction,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_TrackedDevicePosition",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_enabled(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_enabled",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Implicit(
        set: crate::UnityEngine::InputSystem::DefaultInputActions_UIActions,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionMap>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::InputSystem::InputActionMap,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("op_Implicit", (set))?;
        Ok(__cordl_ret.into())
    }
}
