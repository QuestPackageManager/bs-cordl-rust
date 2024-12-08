#[cfg(feature = "OVRVirtualKeyboardSampleControls+OVRVirtualKeyboardBackup")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct OVRVirtualKeyboardSampleControls_OVRVirtualKeyboardBackup {
    pub _textCommitField: *mut crate::UnityEngine::UI::InputField,
    pub _position: crate::UnityEngine::Vector3,
    pub _rotation: crate::UnityEngine::Quaternion,
    pub _scale: crate::UnityEngine::Vector3,
    pub _rightControllerDirectTransform: *mut crate::UnityEngine::Transform,
    pub _rightControllerRootTransform: *mut crate::UnityEngine::Transform,
    pub _leftControllerDirectTransform: *mut crate::UnityEngine::Transform,
    pub _leftControllerRootTransform: *mut crate::UnityEngine::Transform,
    pub _controllerRayInteraction: bool,
    pub _controllerDirectInteraction: bool,
    pub _handLeft: *mut OVRHand,
    pub _handRight: *mut OVRHand,
    pub _handRayInteraction: bool,
    pub _handDirectInteraction: bool,
    pub _controllerRaycaster: *mut crate::UnityEngine::EventSystems::OVRPhysicsRaycaster,
    pub _handRaycaster: *mut crate::UnityEngine::EventSystems::OVRPhysicsRaycaster,
}
#[cfg(feature = "OVRVirtualKeyboardSampleControls+OVRVirtualKeyboardBackup")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRVirtualKeyboardSampleControls_OVRVirtualKeyboardBackup => ""
    ."OVRVirtualKeyboardSampleControls/OVRVirtualKeyboardBackup"
);
#[cfg(feature = "OVRVirtualKeyboardSampleControls+OVRVirtualKeyboardBackup")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::OVRVirtualKeyboardSampleControls_OVRVirtualKeyboardBackup {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRVirtualKeyboardSampleControls+OVRVirtualKeyboardBackup")]
impl crate::GlobalNamespace::OVRVirtualKeyboardSampleControls_OVRVirtualKeyboardBackup {
    pub fn RestoreTo(
        &mut self,
        keyboard: *mut OVRVirtualKeyboard,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "RestoreTo",
            (keyboard),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        keyboard: *mut OVRVirtualKeyboard,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (keyboard),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "OVRVirtualKeyboardSampleControls")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRVirtualKeyboardSampleControls {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub ShowButton: *mut crate::UnityEngine::UI::Button,
    pub MoveButton: *mut crate::UnityEngine::UI::Button,
    pub HideButton: *mut crate::UnityEngine::UI::Button,
    pub MoveNearButton: *mut crate::UnityEngine::UI::Button,
    pub MoveFarButton: *mut crate::UnityEngine::UI::Button,
    pub DestroyKeyboardButton: *mut crate::UnityEngine::UI::Button,
    pub keyboard: *mut OVRVirtualKeyboard,
    pub inputHandler: *mut OVRVirtualKeyboardSampleInputHandler,
    pub isMovingKeyboard_: bool,
    pub isMovingKeyboardFinished_: bool,
    pub keyboardMoveDistance_: f32,
    pub keyboardScale_: f32,
    pub keyboardBackup: crate::GlobalNamespace::OVRVirtualKeyboardSampleControls_OVRVirtualKeyboardBackup,
}
#[cfg(feature = "OVRVirtualKeyboardSampleControls")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for OVRVirtualKeyboardSampleControls => ""
    ."OVRVirtualKeyboardSampleControls"
);
#[cfg(feature = "OVRVirtualKeyboardSampleControls")]
impl std::ops::Deref for OVRVirtualKeyboardSampleControls {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRVirtualKeyboardSampleControls")]
impl std::ops::DerefMut for OVRVirtualKeyboardSampleControls {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRVirtualKeyboardSampleControls")]
impl OVRVirtualKeyboardSampleControls {
    pub const THUMBSTICK_DEADZONE: f32 = 0.2f32;
    #[cfg(feature = "OVRVirtualKeyboardSampleControls+OVRVirtualKeyboardBackup")]
    pub type OVRVirtualKeyboardBackup = crate::GlobalNamespace::OVRVirtualKeyboardSampleControls_OVRVirtualKeyboardBackup;
    pub fn DestroyKeyboard(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DestroyKeyboard", ())?;
        Ok(__cordl_ret)
    }
    pub fn HideKeyboard(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HideKeyboard", ())?;
        Ok(__cordl_ret)
    }
    pub fn MoveKeyboard(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveKeyboard", ())?;
        Ok(__cordl_ret)
    }
    pub fn MoveKeyboardFar(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveKeyboardFar", ())?;
        Ok(__cordl_ret)
    }
    pub fn MoveKeyboardNear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveKeyboardNear", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnHideKeyboard(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnHideKeyboard", ())?;
        Ok(__cordl_ret)
    }
    pub fn ShowKeyboard(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowKeyboard", ())?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateButtonInteractable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateButtonInteractable", ())?;
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
}
#[cfg(feature = "OVRVirtualKeyboardSampleControls")]
impl quest_hook::libil2cpp::ObjectType for OVRVirtualKeyboardSampleControls {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
