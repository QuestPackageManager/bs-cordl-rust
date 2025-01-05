#[cfg(
    feature = "OVRVirtualKeyboard+InteractorRootTransformOverride+InteractorRootOverrideData"
)]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct InteractorRootTransformOverride_OVRVirtualKeyboard_InteractorRootOverrideData {
    pub root: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub originalPose: crate::GlobalNamespace::OVRPose,
    pub targetPose: crate::GlobalNamespace::OVRPose,
}
#[cfg(
    feature = "OVRVirtualKeyboard+InteractorRootTransformOverride+InteractorRootOverrideData"
)]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::InteractorRootTransformOverride_OVRVirtualKeyboard_InteractorRootOverrideData
    => ""."OVRVirtualKeyboard/InteractorRootTransformOverride/InteractorRootOverrideData"
);
#[cfg(
    feature = "OVRVirtualKeyboard+InteractorRootTransformOverride+InteractorRootOverrideData"
)]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::InteractorRootTransformOverride_OVRVirtualKeyboard_InteractorRootOverrideData {
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
impl crate::GlobalNamespace::InteractorRootTransformOverride_OVRVirtualKeyboard_InteractorRootOverrideData {}
#[cfg(feature = "OVRVirtualKeyboard")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRVirtualKeyboard {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub CommitText: quest_hook::libil2cpp::Gc<
        crate::System::Action_1<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        >,
    >,
    pub Backspace: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub Enter: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub KeyboardShown: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub KeyboardHidden: quest_hook::libil2cpp::Gc<crate::System::Action>,
    pub _Collider_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Collider,
    >,
    pub InitialPosition: crate::GlobalNamespace::OVRVirtualKeyboard_KeyboardPosition,
    pub textCommitField: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::InputField>,
    pub leftControllerRootTransform: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Transform,
    >,
    pub leftControllerDirectTransform: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Transform,
    >,
    pub rightControllerRootTransform: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Transform,
    >,
    pub rightControllerDirectTransform: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Transform,
    >,
    pub controllerDirectInteraction: bool,
    pub controllerRayInteraction: bool,
    pub controllerRaycaster: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::OVRPhysicsRaycaster,
    >,
    pub handLeft: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRHand>,
    pub handRight: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRHand>,
    pub handDirectInteraction: bool,
    pub handRayInteraction: bool,
    pub handRaycaster: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::EventSystems::OVRPhysicsRaycaster,
    >,
    pub keyboardModelShader: quest_hook::libil2cpp::Gc<crate::UnityEngine::Shader>,
    pub keyboardModelAlphaBlendShader: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Shader,
    >,
    pub InputEnabled: bool,
    pub isKeyboardCreated_: bool,
    pub keyboardSpace_: u64,
    pub virtualKeyboardTextures_: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Dictionary_2<
            u64,
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                >,
            >,
        >,
    >,
    pub virtualKeyboardScene_: crate::GlobalNamespace::OVRGLTFScene,
    pub virtualKeyboardModelKey_: u64,
    pub modelInitialized_: bool,
    pub modelAvailable_: bool,
    pub keyboardVisible_: bool,
    pub _interactorRootTransformOverride: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::OVRVirtualKeyboard_InteractorRootTransformOverride,
    >,
    pub _inputSources: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            quest_hook::libil2cpp::Gc<
                crate::GlobalNamespace::OVRVirtualKeyboard_IInputSource,
            >,
        >,
    >,
    pub ignoreTextCommmitFieldOnValueChanged_: bool,
    pub runtimeInputField_: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::UI::InputField,
    >,
}
#[cfg(feature = "OVRVirtualKeyboard")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::OVRVirtualKeyboard => ""
    ."OVRVirtualKeyboard"
);
#[cfg(feature = "OVRVirtualKeyboard")]
impl std::ops::Deref for crate::GlobalNamespace::OVRVirtualKeyboard {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRVirtualKeyboard")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRVirtualKeyboard {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRVirtualKeyboard")]
impl crate::GlobalNamespace::OVRVirtualKeyboard {
    #[cfg(feature = "OVRVirtualKeyboard+BaseInputSource")]
    pub type BaseInputSource = crate::GlobalNamespace::OVRVirtualKeyboard_BaseInputSource;
    #[cfg(feature = "OVRVirtualKeyboard+ControllerInputSource")]
    pub type ControllerInputSource = crate::GlobalNamespace::OVRVirtualKeyboard_ControllerInputSource;
    #[cfg(feature = "OVRVirtualKeyboard+HandInputSource")]
    pub type HandInputSource = crate::GlobalNamespace::OVRVirtualKeyboard_HandInputSource;
    #[cfg(feature = "OVRVirtualKeyboard+IInputSource")]
    type IInputSource = crate::GlobalNamespace::OVRVirtualKeyboard_IInputSource;
    #[cfg(feature = "OVRVirtualKeyboard+InputSource")]
    pub type InputSource = crate::GlobalNamespace::OVRVirtualKeyboard_InputSource;
    #[cfg(feature = "OVRVirtualKeyboard+InteractorRootTransformOverride")]
    pub type InteractorRootTransformOverride = crate::GlobalNamespace::OVRVirtualKeyboard_InteractorRootTransformOverride;
    #[cfg(feature = "OVRVirtualKeyboard+KeyboardPosition")]
    pub type KeyboardPosition = crate::GlobalNamespace::OVRVirtualKeyboard_KeyboardPosition;
    pub fn ApplyHideFlags(
        t: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyHideFlags", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Awake", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ChangeTextContext(
        &mut self,
        textContext: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ChangeTextContext", (textContext))?;
        Ok(__cordl_ret.into())
    }
    pub fn ChangeTextContextInternal(
        &mut self,
        textContext: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ChangeTextContextInternal", (textContext))?;
        Ok(__cordl_ret.into())
    }
    pub fn ComputeLocation(
        &mut self,
        transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_VirtualKeyboardLocationInfo,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_VirtualKeyboardLocationInfo = __cordl_object
            .invoke("ComputeLocation", (transform))?;
        Ok(__cordl_ret.into())
    }
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
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateUpdate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadRuntimeVirtualKeyboardMesh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("LoadRuntimeVirtualKeyboardMesh", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MaxElement(
        &mut self,
        vec: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("MaxElement", (vec))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnBackspace(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnBackspace", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnCommitText(
        &mut self,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnCommitText", (text))?;
        Ok(__cordl_ret.into())
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
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnEnter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnter", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn OnKeyboardHidden(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnKeyboardHidden", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnKeyboardShown(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnKeyboardShown", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnTextCommitFieldChange(
        &mut self,
        textContext: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnTextCommitFieldChange", (textContext))?;
        Ok(__cordl_ret.into())
    }
    pub fn PopulateCollision(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("PopulateCollision", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SendVirtualKeyboardDirectInput(
        &mut self,
        position: crate::UnityEngine::Vector3,
        source: crate::GlobalNamespace::OVRVirtualKeyboard_InputSource,
        isPressed: bool,
        interactorRootTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SendVirtualKeyboardDirectInput",
                (position, source, isPressed, interactorRootTransform),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SendVirtualKeyboardInput(
        &mut self,
        inputSource: crate::GlobalNamespace::OVRPlugin_VirtualKeyboardInputSource,
        pose: crate::GlobalNamespace::OVRPose,
        isPressed: bool,
        interactorRootTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "SendVirtualKeyboardInput",
                (inputSource, pose, isPressed, interactorRootTransform),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn SendVirtualKeyboardRayInput(
        &mut self,
        inputTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
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
        Ok(__cordl_ret.into())
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
    pub fn SyncKeyboardLocation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SyncKeyboardLocation", ())?;
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
    pub fn UpdateAnimationState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateAnimationState", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateInputs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateInputs", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UpdateVisibleState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateVisibleState", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn _LoadRuntimeVirtualKeyboardMesh_b__68_1(
        &mut self,
        rawUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = __cordl_object
            .invoke("<LoadRuntimeVirtualKeyboardMesh>b__68_1", (rawUri, mat))?;
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
    pub fn add_Backspace(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_Backspace", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_CommitText(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_CommitText", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_Enter(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_Enter", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_KeyboardHidden(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_KeyboardHidden", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn add_KeyboardShown(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("add_KeyboardShown", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Collider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider> = __cordl_object
            .invoke("get_Collider", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TextCommitField(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::InputField>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::InputField> = __cordl_object
            .invoke("get_TextCommitField", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_Backspace(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_Backspace", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_CommitText(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Action_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_CommitText", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_Enter(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_Enter", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_KeyboardHidden(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_KeyboardHidden", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn remove_KeyboardShown(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("remove_KeyboardShown", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Collider(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Collider", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_TextCommitField(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::InputField>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_TextCommitField", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "OVRVirtualKeyboard")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRVirtualKeyboard {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "OVRVirtualKeyboard")]
impl AsRef<crate::GlobalNamespace::OVRManager_EventListener>
for crate::GlobalNamespace::OVRVirtualKeyboard {
    fn as_ref(&self) -> &crate::GlobalNamespace::OVRManager_EventListener {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OVRVirtualKeyboard")]
impl AsMut<crate::GlobalNamespace::OVRManager_EventListener>
for crate::GlobalNamespace::OVRVirtualKeyboard {
    fn as_mut(&mut self) -> &mut crate::GlobalNamespace::OVRManager_EventListener {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OVRVirtualKeyboard+BaseInputSource")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRVirtualKeyboard_BaseInputSource {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _operatingWithoutOVRCameraRig: bool,
    pub _rig: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRCameraRig>,
}
#[cfg(feature = "OVRVirtualKeyboard+BaseInputSource")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::OVRVirtualKeyboard_BaseInputSource => ""
    ."OVRVirtualKeyboard/BaseInputSource"
);
#[cfg(feature = "OVRVirtualKeyboard+BaseInputSource")]
impl std::ops::Deref for crate::GlobalNamespace::OVRVirtualKeyboard_BaseInputSource {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnUpdatedAnchors(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRCameraRig>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnUpdatedAnchors", (obj))?;
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
    pub fn UpdateInput(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateInput", ())?;
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
#[cfg(feature = "OVRVirtualKeyboard+BaseInputSource")]
impl AsRef<crate::GlobalNamespace::OVRVirtualKeyboard_IInputSource>
for crate::GlobalNamespace::OVRVirtualKeyboard_BaseInputSource {
    fn as_ref(&self) -> &crate::GlobalNamespace::OVRVirtualKeyboard_IInputSource {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OVRVirtualKeyboard+BaseInputSource")]
impl AsMut<crate::GlobalNamespace::OVRVirtualKeyboard_IInputSource>
for crate::GlobalNamespace::OVRVirtualKeyboard_BaseInputSource {
    fn as_mut(
        &mut self,
    ) -> &mut crate::GlobalNamespace::OVRVirtualKeyboard_IInputSource {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OVRVirtualKeyboard+BaseInputSource")]
impl AsRef<crate::System::IDisposable>
for crate::GlobalNamespace::OVRVirtualKeyboard_BaseInputSource {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OVRVirtualKeyboard+BaseInputSource")]
impl AsMut<crate::System::IDisposable>
for crate::GlobalNamespace::OVRVirtualKeyboard_BaseInputSource {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "OVRVirtualKeyboard+ControllerInputSource")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRVirtualKeyboard_ControllerInputSource {
    __cordl_parent: crate::GlobalNamespace::OVRVirtualKeyboard_BaseInputSource,
    pub _rootTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _directTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub _inputSource: crate::GlobalNamespace::OVRVirtualKeyboard_InputSource,
    pub _controllerType: crate::GlobalNamespace::OVRInput_Controller,
    pub _keyboard: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRVirtualKeyboard>,
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
    pub fn New(
        keyboard: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRVirtualKeyboard>,
        inputSource: crate::GlobalNamespace::OVRVirtualKeyboard_InputSource,
        controllerType: crate::GlobalNamespace::OVRInput_Controller,
        rootTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        directTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (keyboard, inputSource, controllerType, rootTransform, directTransform),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn UpdateInput(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateInput", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        keyboard: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRVirtualKeyboard>,
        inputSource: crate::GlobalNamespace::OVRVirtualKeyboard_InputSource,
        controllerType: crate::GlobalNamespace::OVRInput_Controller,
        rootTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        directTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (keyboard, inputSource, controllerType, rootTransform, directTransform),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TriggerIsPressed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_TriggerIsPressed", ())?;
        Ok(__cordl_ret.into())
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
    pub _hand: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRHand>,
    pub _inputSource: crate::GlobalNamespace::OVRVirtualKeyboard_InputSource,
    pub _keyboard: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRVirtualKeyboard>,
    pub _skeleton: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRSkeleton>,
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
    pub fn New(
        keyboard: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRVirtualKeyboard>,
        inputSource: crate::GlobalNamespace::OVRVirtualKeyboard_InputSource,
        hand: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRHand>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (keyboard, inputSource, hand))?;
        Ok(__cordl_object.into())
    }
    pub fn UpdateInput(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateInput", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        keyboard: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRVirtualKeyboard>,
        inputSource: crate::GlobalNamespace::OVRVirtualKeyboard_InputSource,
        hand: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRHand>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (keyboard, inputSource, hand))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRVirtualKeyboard_InputSource {
    #[default]
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
#[cfg(feature = "OVRVirtualKeyboard+InteractorRootTransformOverride")]
#[repr(C)]
#[derive(Debug)]
pub struct OVRVirtualKeyboard_InteractorRootTransformOverride {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub applyQueue: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Queue_1<
            crate::GlobalNamespace::InteractorRootTransformOverride_OVRVirtualKeyboard_InteractorRootOverrideData,
        >,
    >,
    pub revertQueue: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::Queue_1<
            crate::GlobalNamespace::InteractorRootTransformOverride_OVRVirtualKeyboard_InteractorRootOverrideData,
        >,
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
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub type InteractorRootOverrideData = crate::GlobalNamespace::InteractorRootTransformOverride_OVRVirtualKeyboard_InteractorRootOverrideData;
    pub fn ApplyOverride(
        interactorOverride: crate::GlobalNamespace::InteractorRootTransformOverride_OVRVirtualKeyboard_InteractorRootOverrideData,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ApplyOverride", (interactorOverride))?;
        Ok(__cordl_ret.into())
    }
    pub fn Enqueue(
        &mut self,
        interactorRootTransform: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Transform,
        >,
        interactorRootPose: crate::GlobalNamespace::OVRPlugin_Posef,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Enqueue", (interactorRootTransform, interactorRootPose))?;
        Ok(__cordl_ret.into())
    }
    pub fn LateApply(
        &mut self,
        coroutineRunner: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("LateApply", (coroutineRunner))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RevertInteractorOverrides(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("RevertInteractorOverrides", ())?;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OVRVirtualKeyboard_KeyboardPosition {
    #[default]
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
