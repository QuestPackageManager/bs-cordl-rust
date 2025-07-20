#[cfg(feature = "VRUIControls+MouseState")]
#[repr(C)]
#[derive(Debug)]
pub struct MouseState {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _trackedButtons: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<crate::VRUIControls::ButtonState>,
        >,
    >,
}
#[cfg(feature = "VRUIControls+MouseState")]
unsafe impl quest_hook::libil2cpp::Type for crate::VRUIControls::MouseState {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "VRUIControls";
    const CLASS_NAME: &'static str = "MouseState";
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
#[cfg(feature = "VRUIControls+MouseState")]
impl std::ops::Deref for crate::VRUIControls::MouseState {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "VRUIControls+MouseState")]
impl std::ops::DerefMut for crate::VRUIControls::MouseState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "VRUIControls+MouseState")]
impl crate::VRUIControls::MouseState {
    pub fn AnyPressesThisFrame(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::VRUIControls::MouseState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("AnyPressesThisFrame")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::VRUIControls::MouseState as quest_hook::libil2cpp::Type >
                    ::class(), "AnyPressesThisFrame", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn AnyReleasesThisFrame(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::VRUIControls::MouseState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("AnyReleasesThisFrame")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::VRUIControls::MouseState as quest_hook::libil2cpp::Type >
                    ::class(), "AnyReleasesThisFrame", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetButtonState(
        &mut self,
        button: crate::UnityEngine::EventSystems::PointerEventData_InputButton,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::VRUIControls::ButtonState>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::VRUIControls::MouseState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::UnityEngine::EventSystems::PointerEventData_InputButton),
                quest_hook::libil2cpp::Gc<crate::VRUIControls::ButtonState>,
                1usize,
            >("GetButtonState")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::VRUIControls::MouseState as quest_hook::libil2cpp::Type >
                    ::class(), "GetButtonState", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::VRUIControls::ButtonState> = unsafe {
            method.invoke_unchecked(self, (button))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn SetButtonState(
        &mut self,
        button: crate::UnityEngine::EventSystems::PointerEventData_InputButton,
        stateForMouseButton: crate::UnityEngine::EventSystems::PointerEventData_FramePressState,
        data: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::EventSystems::PointerEventData,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::VRUIControls::MouseState as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::EventSystems::PointerEventData_InputButton,
                    crate::UnityEngine::EventSystems::PointerEventData_FramePressState,
                    quest_hook::libil2cpp::Gc<
                        crate::UnityEngine::EventSystems::PointerEventData,
                    >,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("SetButtonState")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::VRUIControls::MouseState as quest_hook::libil2cpp::Type >
                    ::class(), "SetButtonState", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (button, stateForMouseButton, data))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::VRUIControls::MouseState as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::VRUIControls::MouseState as quest_hook::libil2cpp::Type >
                    ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "VRUIControls+MouseState")]
impl quest_hook::libil2cpp::ObjectType for crate::VRUIControls::MouseState {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
