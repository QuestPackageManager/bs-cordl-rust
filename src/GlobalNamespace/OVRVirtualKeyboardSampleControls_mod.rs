#[cfg(feature = "OVRVirtualKeyboardSampleControls")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRVirtualKeyboardSampleControls {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub ShowButton: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    pub MoveButton: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    pub HideButton: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    pub MoveNearButton: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    pub MoveFarButton: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    pub DestroyKeyboardButton: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::Button>,
    pub keyboard: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRVirtualKeyboard>,
    pub inputHandler: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::OVRVirtualKeyboardSampleInputHandler,
    >,
    pub isMovingKeyboard_: bool,
    pub isMovingKeyboardFinished_: bool,
    pub keyboardMoveDistance_: f32,
    pub keyboardScale_: f32,
    pub keyboardBackup: crate::GlobalNamespace::OVRVirtualKeyboardSampleControls_OVRVirtualKeyboardBackup,
}
#[cfg(feature = "OVRVirtualKeyboardSampleControls")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRVirtualKeyboardSampleControls => ""
    ."OVRVirtualKeyboardSampleControls"
);
#[cfg(feature = "OVRVirtualKeyboardSampleControls")]
impl std::ops::Deref for crate::GlobalNamespace::OVRVirtualKeyboardSampleControls {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRVirtualKeyboardSampleControls")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRVirtualKeyboardSampleControls {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRVirtualKeyboardSampleControls")]
impl crate::GlobalNamespace::OVRVirtualKeyboardSampleControls {
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
        Ok(__cordl_ret.into())
    }
    pub fn HideKeyboard(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("HideKeyboard", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveKeyboard(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveKeyboard", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveKeyboardFar(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveKeyboardFar", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveKeyboardNear(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MoveKeyboardNear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnHideKeyboard(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnHideKeyboard", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ShowKeyboard(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ShowKeyboard", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateButtonInteractable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateButtonInteractable", ())?;
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
}
#[cfg(feature = "OVRVirtualKeyboardSampleControls")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRVirtualKeyboardSampleControls {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRVirtualKeyboardSampleControls+OVRVirtualKeyboardBackup")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct OVRVirtualKeyboardSampleControls_OVRVirtualKeyboardBackup {
    pub _textCommitField: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::InputField>,
    pub _position: crate::UnityEngine::Vector3,
    pub _rotation: crate::UnityEngine::Quaternion,
    pub _scale: crate::UnityEngine::Vector3,
    pub _rightControllerDirectTransform: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Transform,
    >,
    pub _rightControllerRootTransform: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Transform,
    >,
    pub _leftControllerDirectTransform: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Transform,
    >,
    pub _leftControllerRootTransform: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Transform,
    >,
    pub _controllerRayInteraction: bool,
    pub _controllerDirectInteraction: bool,
    pub _handLeft: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRHand>,
    pub _handRight: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRHand>,
    pub _handRayInteraction: bool,
    pub _handDirectInteraction: bool,
    pub _controllerRaycaster: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::OVRPhysicsRaycaster,
    >,
    pub _handRaycaster: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::OVRPhysicsRaycaster,
    >,
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
        keyboard: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRVirtualKeyboard>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "RestoreTo",
            (keyboard),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        keyboard: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRVirtualKeyboard>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (keyboard),
        )?;
        Ok(__cordl_ret.into())
    }
}
