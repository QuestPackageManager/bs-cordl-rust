#[cfg(feature = "OVRVirtualKeyboard+BaseInputSource")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRVirtualKeyboard_BaseInputSource {
    __cordl_parent: crate::System::Object,
    pub _operatingWithoutOVRCameraRig: bool,
    pub _rig: *mut OVRCameraRig,
}
#[cfg(feature = "OVRVirtualKeyboard+BaseInputSource")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRVirtualKeyboard_BaseInputSource => ""
    ."OVRVirtualKeyboard/BaseInputSource"
);
#[cfg(feature = "OVRVirtualKeyboard+BaseInputSource")]
impl std::ops::Deref for crate::GlobalNamespace::OVRVirtualKeyboard_BaseInputSource {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRVirtualKeyboard+BaseInputSource")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRVirtualKeyboard_BaseInputSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRVirtualKeyboard+BaseInputSource")]
impl crate::GlobalNamespace::OVRVirtualKeyboard_BaseInputSource {
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
    pub fn OnUpdatedAnchors(
        &mut self,
        obj: *mut OVRCameraRig,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnUpdatedAnchors", (obj))?;
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
    pub fn UpdateInput(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateInput", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
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
#[cfg(feature = "OVRVirtualKeyboard+BaseInputSource")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRVirtualKeyboard_BaseInputSource {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRVirtualKeyboard+ControllerInputSource")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRVirtualKeyboard_ControllerInputSource {
    __cordl_parent: crate::GlobalNamespace::OVRVirtualKeyboard_BaseInputSource,
    pub _rootTransform: *mut crate::UnityEngine::Transform,
    pub _directTransform: *mut crate::UnityEngine::Transform,
    pub _inputSource: crate::GlobalNamespace::OVRVirtualKeyboard_InputSource,
    pub _controllerType: crate::GlobalNamespace::OVRInput_Controller,
    pub _keyboard: *mut OVRVirtualKeyboard,
    pub _lastFrameCount: i32,
    pub _triggerButton: crate::GlobalNamespace::OVRInput_RawButton,
}
#[cfg(feature = "OVRVirtualKeyboard+ControllerInputSource")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRVirtualKeyboard_ControllerInputSource => ""
    ."OVRVirtualKeyboard/ControllerInputSource"
);
#[cfg(feature = "OVRVirtualKeyboard+ControllerInputSource")]
impl std::ops::Deref
for crate::GlobalNamespace::OVRVirtualKeyboard_ControllerInputSource {
    type Target = crate::GlobalNamespace::OVRVirtualKeyboard_BaseInputSource;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRVirtualKeyboard+ControllerInputSource")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OVRVirtualKeyboard_ControllerInputSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRVirtualKeyboard+ControllerInputSource")]
impl crate::GlobalNamespace::OVRVirtualKeyboard_ControllerInputSource {
    pub fn _ctor(
        &mut self,
        keyboard: *mut OVRVirtualKeyboard,
        inputSource: crate::GlobalNamespace::OVRVirtualKeyboard_InputSource,
        controllerType: crate::GlobalNamespace::OVRInput_Controller,
        rootTransform: *mut crate::UnityEngine::Transform,
        directTransform: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (keyboard, inputSource, controllerType, rootTransform, directTransform),
            )?;
        Ok(__cordl_ret)
    }
    pub fn UpdateInput(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateInput", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TriggerIsPressed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_TriggerIsPressed", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        keyboard: *mut OVRVirtualKeyboard,
        inputSource: crate::GlobalNamespace::OVRVirtualKeyboard_InputSource,
        controllerType: crate::GlobalNamespace::OVRInput_Controller,
        rootTransform: *mut crate::UnityEngine::Transform,
        directTransform: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (keyboard, inputSource, controllerType, rootTransform, directTransform),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "OVRVirtualKeyboard+ControllerInputSource")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRVirtualKeyboard_ControllerInputSource {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRVirtualKeyboard+HandInputSource")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRVirtualKeyboard_HandInputSource {
    __cordl_parent: crate::GlobalNamespace::OVRVirtualKeyboard_BaseInputSource,
    pub _hand: *mut OVRHand,
    pub _inputSource: crate::GlobalNamespace::OVRVirtualKeyboard_InputSource,
    pub _keyboard: *mut OVRVirtualKeyboard,
    pub _skeleton: *mut OVRSkeleton,
    pub _lastFrameCount: i32,
}
#[cfg(feature = "OVRVirtualKeyboard+HandInputSource")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRVirtualKeyboard_HandInputSource => ""
    ."OVRVirtualKeyboard/HandInputSource"
);
#[cfg(feature = "OVRVirtualKeyboard+HandInputSource")]
impl std::ops::Deref for crate::GlobalNamespace::OVRVirtualKeyboard_HandInputSource {
    type Target = crate::GlobalNamespace::OVRVirtualKeyboard_BaseInputSource;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRVirtualKeyboard+HandInputSource")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRVirtualKeyboard_HandInputSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRVirtualKeyboard+HandInputSource")]
impl crate::GlobalNamespace::OVRVirtualKeyboard_HandInputSource {
    #[cfg(feature = "OVRVirtualKeyboard+HandInputSource+__c")]
    pub type __c = crate::GlobalNamespace::HandInputSource___c;
    pub fn _ctor(
        &mut self,
        keyboard: *mut OVRVirtualKeyboard,
        inputSource: crate::GlobalNamespace::OVRVirtualKeyboard_InputSource,
        hand: *mut OVRHand,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (keyboard, inputSource, hand))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateInput(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateInput", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        keyboard: *mut OVRVirtualKeyboard,
        inputSource: crate::GlobalNamespace::OVRVirtualKeyboard_InputSource,
        hand: *mut OVRHand,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (keyboard, inputSource, hand))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "OVRVirtualKeyboard+HandInputSource")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRVirtualKeyboard_HandInputSource {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRVirtualKeyboard+IInputSource")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRVirtualKeyboard_IInputSource {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "OVRVirtualKeyboard+IInputSource")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRVirtualKeyboard_IInputSource
    => ""."OVRVirtualKeyboard/IInputSource"
);
#[cfg(feature = "OVRVirtualKeyboard+IInputSource")]
impl std::ops::Deref for crate::GlobalNamespace::OVRVirtualKeyboard_IInputSource {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRVirtualKeyboard+IInputSource")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRVirtualKeyboard_IInputSource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRVirtualKeyboard+IInputSource")]
impl crate::GlobalNamespace::OVRVirtualKeyboard_IInputSource {
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
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "OVRVirtualKeyboard+IInputSource")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRVirtualKeyboard_IInputSource {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRVirtualKeyboard+InputSource")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRVirtualKeyboard_InputSource {
    ControllerLeft = 0i32,
    ControllerRight = 1i32,
    HandLeft = 2i32,
    HandRight = 3i32,
}
#[cfg(feature = "OVRVirtualKeyboard+InputSource")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRVirtualKeyboard_InputSource
    => ""."OVRVirtualKeyboard/InputSource"
);
#[cfg(
    feature = "OVRVirtualKeyboard+InteractorRootTransformOverride+InteractorRootOverrideData"
)]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct InteractorRootTransformOverride_InteractorRootOverrideData {
    pub root: *mut crate::UnityEngine::Transform,
    pub originalPose: OVRPose,
    pub targetPose: OVRPose,
}
#[cfg(
    feature = "OVRVirtualKeyboard+InteractorRootTransformOverride+InteractorRootOverrideData"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::InteractorRootTransformOverride_InteractorRootOverrideData => ""
    ."OVRVirtualKeyboard/InteractorRootTransformOverride/InteractorRootOverrideData"
);
#[cfg(
    feature = "OVRVirtualKeyboard+InteractorRootTransformOverride+InteractorRootOverrideData"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::InteractorRootTransformOverride_InteractorRootOverrideData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(
    feature = "OVRVirtualKeyboard+InteractorRootTransformOverride+InteractorRootOverrideData"
)]
impl crate::GlobalNamespace::InteractorRootTransformOverride_InteractorRootOverrideData {}
#[cfg(feature = "OVRVirtualKeyboard+InteractorRootTransformOverride")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRVirtualKeyboard_InteractorRootTransformOverride {
    __cordl_parent: crate::System::Object,
    pub applyQueue: *mut crate::System::Collections::Generic::Queue_1<
        crate::GlobalNamespace::InteractorRootTransformOverride_InteractorRootOverrideData,
    >,
    pub revertQueue: *mut crate::System::Collections::Generic::Queue_1<
        crate::GlobalNamespace::InteractorRootTransformOverride_InteractorRootOverrideData,
    >,
}
#[cfg(feature = "OVRVirtualKeyboard+InteractorRootTransformOverride")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRVirtualKeyboard_InteractorRootTransformOverride => ""
    ."OVRVirtualKeyboard/InteractorRootTransformOverride"
);
#[cfg(feature = "OVRVirtualKeyboard+InteractorRootTransformOverride")]
impl std::ops::Deref
for crate::GlobalNamespace::OVRVirtualKeyboard_InteractorRootTransformOverride {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRVirtualKeyboard+InteractorRootTransformOverride")]
impl std::ops::DerefMut
for crate::GlobalNamespace::OVRVirtualKeyboard_InteractorRootTransformOverride {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRVirtualKeyboard+InteractorRootTransformOverride")]
impl crate::GlobalNamespace::OVRVirtualKeyboard_InteractorRootTransformOverride {
    #[cfg(
        feature = "OVRVirtualKeyboard+InteractorRootTransformOverride+InteractorRootOverrideData"
    )]
    pub type InteractorRootOverrideData = crate::GlobalNamespace::InteractorRootTransformOverride_InteractorRootOverrideData;
    #[cfg(
        feature = "OVRVirtualKeyboard+InteractorRootTransformOverride+_RevertInteractorOverrides_d__6"
    )]
    pub type _RevertInteractorOverrides_d__6 = crate::GlobalNamespace::InteractorRootTransformOverride__RevertInteractorOverrides_d__6;
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn Enqueue(
        &mut self,
        interactorRootTransform: *mut crate::UnityEngine::Transform,
        interactorRootPose: crate::GlobalNamespace::OVRPlugin_Posef,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Enqueue", (interactorRootTransform, interactorRootPose))?;
        Ok(__cordl_ret)
    }
    pub fn RevertInteractorOverrides(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("RevertInteractorOverrides", ())?;
        Ok(__cordl_ret)
    }
    pub fn LateApply(
        &mut self,
        coroutineRunner: *mut crate::UnityEngine::MonoBehaviour,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateApply", (coroutineRunner))?;
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "OVRVirtualKeyboard+InteractorRootTransformOverride")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::OVRVirtualKeyboard_InteractorRootTransformOverride {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRVirtualKeyboard+KeyboardPosition")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OVRVirtualKeyboard_KeyboardPosition {
    Custom = 2i32,
    Direct = 1i32,
    Far = 0i32,
}
#[cfg(feature = "OVRVirtualKeyboard+KeyboardPosition")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRVirtualKeyboard_KeyboardPosition => ""
    ."OVRVirtualKeyboard/KeyboardPosition"
);
#[cfg(feature = "OVRVirtualKeyboard")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRVirtualKeyboard {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub CommitText: *mut crate::System::Action_1<*mut crate::System::String>,
    pub Backspace: *mut crate::System::Action,
    pub Enter: *mut crate::System::Action,
    pub KeyboardShown: *mut crate::System::Action,
    pub KeyboardHidden: *mut crate::System::Action,
    pub _Collider_k__BackingField: *mut crate::UnityEngine::Collider,
    pub InitialPosition: crate::GlobalNamespace::OVRVirtualKeyboard_KeyboardPosition,
    pub textCommitField: *mut crate::UnityEngine::UI::InputField,
    pub leftControllerRootTransform: *mut crate::UnityEngine::Transform,
    pub leftControllerDirectTransform: *mut crate::UnityEngine::Transform,
    pub rightControllerRootTransform: *mut crate::UnityEngine::Transform,
    pub rightControllerDirectTransform: *mut crate::UnityEngine::Transform,
    pub controllerDirectInteraction: bool,
    pub controllerRayInteraction: bool,
    pub controllerRaycaster: *mut crate::UnityEngine::EventSystems::OVRPhysicsRaycaster,
    pub handLeft: *mut OVRHand,
    pub handRight: *mut OVRHand,
    pub handDirectInteraction: bool,
    pub handRayInteraction: bool,
    pub handRaycaster: *mut crate::UnityEngine::EventSystems::OVRPhysicsRaycaster,
    pub keyboardModelShader: *mut crate::UnityEngine::Shader,
    pub keyboardModelAlphaBlendShader: *mut crate::UnityEngine::Shader,
    pub InputEnabled: bool,
    pub isKeyboardCreated_: bool,
    pub keyboardSpace_: u64,
    pub virtualKeyboardTextures_: *mut crate::System::Collections::Generic::Dictionary_2<
        u64,
        *mut crate::System::Collections::Generic::List_1<
            *mut crate::UnityEngine::Material,
        >,
    >,
    pub virtualKeyboardScene_: OVRGLTFScene,
    pub virtualKeyboardModelKey_: u64,
    pub modelInitialized_: bool,
    pub modelAvailable_: bool,
    pub keyboardVisible_: bool,
    pub _interactorRootTransformOverride: *mut crate::GlobalNamespace::OVRVirtualKeyboard_InteractorRootTransformOverride,
    pub _inputSources: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::OVRVirtualKeyboard_IInputSource,
    >,
    pub ignoreTextCommmitFieldOnValueChanged_: bool,
    pub runtimeInputField_: *mut crate::UnityEngine::UI::InputField,
}
#[cfg(feature = "OVRVirtualKeyboard")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for OVRVirtualKeyboard => ""."OVRVirtualKeyboard"
);
#[cfg(feature = "OVRVirtualKeyboard")]
impl std::ops::Deref for OVRVirtualKeyboard {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRVirtualKeyboard")]
impl std::ops::DerefMut for OVRVirtualKeyboard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRVirtualKeyboard")]
impl OVRVirtualKeyboard {
    #[cfg(feature = "OVRVirtualKeyboard+InputSource")]
    pub type InputSource = crate::GlobalNamespace::OVRVirtualKeyboard_InputSource;
    #[cfg(feature = "OVRVirtualKeyboard+ControllerInputSource")]
    pub type ControllerInputSource = crate::GlobalNamespace::OVRVirtualKeyboard_ControllerInputSource;
    #[cfg(feature = "OVRVirtualKeyboard+HandInputSource")]
    pub type HandInputSource = crate::GlobalNamespace::OVRVirtualKeyboard_HandInputSource;
    #[cfg(feature = "OVRVirtualKeyboard+InteractorRootTransformOverride")]
    pub type InteractorRootTransformOverride = crate::GlobalNamespace::OVRVirtualKeyboard_InteractorRootTransformOverride;
    #[cfg(feature = "OVRVirtualKeyboard+KeyboardPosition")]
    pub type KeyboardPosition = crate::GlobalNamespace::OVRVirtualKeyboard_KeyboardPosition;
    #[cfg(feature = "OVRVirtualKeyboard+IInputSource")]
    type IInputSource = crate::GlobalNamespace::OVRVirtualKeyboard_IInputSource;
    #[cfg(feature = "OVRVirtualKeyboard+BaseInputSource")]
    pub type BaseInputSource = crate::GlobalNamespace::OVRVirtualKeyboard_BaseInputSource;
    #[cfg(feature = "OVRVirtualKeyboard+__c")]
    pub type __c = crate::GlobalNamespace::OVRVirtualKeyboard___c;
    pub fn SyncKeyboardLocation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SyncKeyboardLocation", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateAnimationState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateAnimationState", ())?;
        Ok(__cordl_ret)
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret)
    }
    pub fn ChangeTextContextInternal(
        &mut self,
        textContext: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ChangeTextContextInternal", (textContext))?;
        Ok(__cordl_ret)
    }
    pub fn add_KeyboardHidden(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_KeyboardHidden", (value))?;
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
    pub fn ChangeTextContext(
        &mut self,
        textContext: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ChangeTextContext", (textContext))?;
        Ok(__cordl_ret)
    }
    pub fn add_KeyboardShown(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_KeyboardShown", (value))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateVisibleState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateVisibleState", ())?;
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
    pub fn OnKeyboardHidden(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnKeyboardHidden", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Collider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Collider> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Collider = __cordl_object
            .invoke("get_Collider", ())?;
        Ok(__cordl_ret)
    }
    pub fn SetKeyboardVisibility(
        &mut self,
        visible: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetKeyboardVisibility", (visible))?;
        Ok(__cordl_ret)
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret)
    }
    pub fn SendVirtualKeyboardRayInput(
        &mut self,
        inputTransform: *mut crate::UnityEngine::Transform,
        source: crate::GlobalNamespace::OVRVirtualKeyboard_InputSource,
        isPressed: bool,
        useRaycastMask: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SendVirtualKeyboardRayInput",
                (inputTransform, source, isPressed, useRaycastMask),
            )?;
        Ok(__cordl_ret)
    }
    pub fn OnTextCommitFieldChange(
        &mut self,
        textContext: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnTextCommitFieldChange", (textContext))?;
        Ok(__cordl_ret)
    }
    pub fn OnCommitText(
        &mut self,
        text: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnCommitText", (text))?;
        Ok(__cordl_ret)
    }
    pub fn set_Collider(
        &mut self,
        value: *mut crate::UnityEngine::Collider,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Collider", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_Backspace(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_Backspace", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SendVirtualKeyboardDirectInput(
        &mut self,
        position: crate::UnityEngine::Vector3,
        source: crate::GlobalNamespace::OVRVirtualKeyboard_InputSource,
        isPressed: bool,
        interactorRootTransform: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SendVirtualKeyboardDirectInput",
                (position, source, isPressed, interactorRootTransform),
            )?;
        Ok(__cordl_ret)
    }
    pub fn LoadRuntimeVirtualKeyboardMesh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("LoadRuntimeVirtualKeyboardMesh", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_TextCommitField(
        &mut self,
        value: *mut crate::UnityEngine::UI::InputField,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_TextCommitField", (value))?;
        Ok(__cordl_ret)
    }
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
    pub fn remove_KeyboardShown(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_KeyboardShown", (value))?;
        Ok(__cordl_ret)
    }
    pub fn PopulateCollision(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopulateCollision", ())?;
        Ok(__cordl_ret)
    }
    pub fn ComputeLocation(
        &mut self,
        transform: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_VirtualKeyboardLocationInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_VirtualKeyboardLocationInfo = __cordl_object
            .invoke("ComputeLocation", (transform))?;
        Ok(__cordl_ret)
    }
    pub fn MaxElement(
        &mut self,
        vec: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("MaxElement", (vec))?;
        Ok(__cordl_ret)
    }
    pub fn remove_CommitText(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_CommitText", (value))?;
        Ok(__cordl_ret)
    }
    pub fn add_Enter(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_Enter", (value))?;
        Ok(__cordl_ret)
    }
    pub fn SendVirtualKeyboardInput(
        &mut self,
        inputSource: crate::GlobalNamespace::OVRPlugin_VirtualKeyboardInputSource,
        pose: OVRPose,
        isPressed: bool,
        interactorRootTransform: *mut crate::UnityEngine::Transform,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SendVirtualKeyboardInput",
                (inputSource, pose, isPressed, interactorRootTransform),
            )?;
        Ok(__cordl_ret)
    }
    pub fn OnEnter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnter", ())?;
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
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_Enter(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_Enter", (value))?;
        Ok(__cordl_ret)
    }
    pub fn UseSuggestedLocation(
        &mut self,
        position: crate::GlobalNamespace::OVRVirtualKeyboard_KeyboardPosition,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UseSuggestedLocation", (position))?;
        Ok(__cordl_ret)
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdate", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TextCommitField(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::UI::InputField> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::UI::InputField = __cordl_object
            .invoke("get_TextCommitField", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_Backspace(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_Backspace", (value))?;
        Ok(__cordl_ret)
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
    pub fn OnKeyboardShown(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnKeyboardShown", ())?;
        Ok(__cordl_ret)
    }
    pub fn remove_KeyboardHidden(
        &mut self,
        value: *mut crate::System::Action,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_KeyboardHidden", (value))?;
        Ok(__cordl_ret)
    }
    pub fn OnEvent(
        &mut self,
        eventDataBuffer: crate::GlobalNamespace::OVRPlugin_EventDataBuffer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEvent", (eventDataBuffer))?;
        Ok(__cordl_ret)
    }
    pub fn _LoadRuntimeVirtualKeyboardMesh_b__68_1(
        &mut self,
        rawUri: *mut crate::System::String,
        mat: *mut crate::UnityEngine::Material,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Texture2D> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Texture2D = __cordl_object
            .invoke("<LoadRuntimeVirtualKeyboardMesh>b__68_1", (rawUri, mat))?;
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
    pub fn add_CommitText(
        &mut self,
        value: *mut crate::System::Action_1<*mut crate::System::String>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_CommitText", (value))?;
        Ok(__cordl_ret)
    }
    pub fn OnBackspace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnBackspace", ())?;
        Ok(__cordl_ret)
    }
    pub fn UpdateInputs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateInputs", ())?;
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
#[cfg(feature = "OVRVirtualKeyboard")]
impl quest_hook::libil2cpp::ObjectType for OVRVirtualKeyboard {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
