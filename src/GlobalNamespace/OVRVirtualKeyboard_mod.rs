#[cfg(
    feature = "OVRVirtualKeyboard+InteractorRootTransformOverride+InteractorRootOverrideData"
)]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct InteractorRootTransformOverride_OVRVirtualKeyboard_InteractorRootOverrideData {
    pub root: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    pub originalPose: crate::GlobalNamespace::OVRPose,
    pub targetPose: crate::GlobalNamespace::OVRPose,
}
#[cfg(
    feature = "OVRVirtualKeyboard+InteractorRootTransformOverride+InteractorRootOverrideData"
)]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::InteractorRootTransformOverride_OVRVirtualKeyboard_InteractorRootOverrideData {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRVirtualKeyboard/InteractorRootTransformOverride/InteractorRootOverrideData";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "OVRVirtualKeyboard+InteractorRootTransformOverride+InteractorRootOverrideData"
)]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::InteractorRootTransformOverride_OVRVirtualKeyboard_InteractorRootOverrideData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "OVRVirtualKeyboard+InteractorRootTransformOverride+InteractorRootOverrideData"
)]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::InteractorRootTransformOverride_OVRVirtualKeyboard_InteractorRootOverrideData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(
    feature = "OVRVirtualKeyboard+InteractorRootTransformOverride+InteractorRootOverrideData"
)]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::InteractorRootTransformOverride_OVRVirtualKeyboard_InteractorRootOverrideData {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(
    feature = "OVRVirtualKeyboard+InteractorRootTransformOverride+InteractorRootOverrideData"
)]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::InteractorRootTransformOverride_OVRVirtualKeyboard_InteractorRootOverrideData {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRVirtualKeyboard {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRVirtualKeyboard";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ApplyHideFlags")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ApplyHideFlags", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (t))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Awake(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Awake")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Awake", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn ChangeTextContext(
        &mut self,
        textContext: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ChangeTextContext")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ChangeTextContext", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (textContext))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ChangeTextContextInternal(
        &mut self,
        textContext: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("ChangeTextContextInternal")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ChangeTextContextInternal", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (textContext))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ComputeLocation(
        &mut self,
        transform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::OVRPlugin_VirtualKeyboardLocationInfo,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>),
                crate::GlobalNamespace::OVRPlugin_VirtualKeyboardLocationInfo,
                1usize,
            >("ComputeLocation")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ComputeLocation", 1usize
                )
            });
        let __cordl_ret: crate::GlobalNamespace::OVRPlugin_VirtualKeyboardLocationInfo = unsafe {
            method.invoke_unchecked(self, (transform))
        };
        Ok(__cordl_ret.into())
    }
    pub fn DestroyKeyboard(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("DestroyKeyboard")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "DestroyKeyboard", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn HideKeyboard(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("HideKeyboard")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "HideKeyboard", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn LateUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("LateUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LateUpdate", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn LoadRuntimeVirtualKeyboardMesh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("LoadRuntimeVirtualKeyboardMesh")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LoadRuntimeVirtualKeyboardMesh", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn MaxElement(
        &mut self,
        vec: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(crate::UnityEngine::Vector3), f32, 1usize>("MaxElement")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "MaxElement", 1usize
                )
            });
        let __cordl_ret: f32 = unsafe { method.invoke_unchecked(self, (vec)) };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnBackspace")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnBackspace", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnCommitText(
        &mut self,
        text: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnCommitText")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnCommitText", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (text))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnDestroy")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnDestroy", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnDisable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnDisable", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnEnable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnEnable", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnEnter(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnEnter")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnEnter", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnEvent(
        &mut self,
        eventDataBuffer: crate::GlobalNamespace::OVRPlugin_EventDataBuffer,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::OVRPlugin_EventDataBuffer),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnEvent")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnEvent", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (eventDataBuffer))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnKeyboardHidden(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnKeyboardHidden")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnKeyboardHidden", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnKeyboardShown(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnKeyboardShown")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnKeyboardShown", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnTextCommitFieldChange(
        &mut self,
        textContext: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnTextCommitFieldChange")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnTextCommitFieldChange", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (textContext))
        };
        Ok(__cordl_ret.into())
    }
    pub fn PopulateCollision(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("PopulateCollision")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "PopulateCollision", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn SendVirtualKeyboardDirectInput(
        &mut self,
        position: crate::UnityEngine::Vector3,
        source: crate::GlobalNamespace::OVRVirtualKeyboard_InputSource,
        isPressed: bool,
        interactorRootTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::UnityEngine::Vector3,
                    crate::GlobalNamespace::OVRVirtualKeyboard_InputSource,
                    bool,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("SendVirtualKeyboardDirectInput")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SendVirtualKeyboardDirectInput", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (position, source, isPressed, interactorRootTransform),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn SendVirtualKeyboardInput(
        &mut self,
        inputSource: crate::GlobalNamespace::OVRPlugin_VirtualKeyboardInputSource,
        pose: crate::GlobalNamespace::OVRPose,
        isPressed: bool,
        interactorRootTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    crate::GlobalNamespace::OVRPlugin_VirtualKeyboardInputSource,
                    crate::GlobalNamespace::OVRPose,
                    bool,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("SendVirtualKeyboardInput")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SendVirtualKeyboardInput", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (inputSource, pose, isPressed, interactorRootTransform),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn SendVirtualKeyboardRayInput(
        &mut self,
        inputTransform: quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
        source: crate::GlobalNamespace::OVRVirtualKeyboard_InputSource,
        isPressed: bool,
        useRaycastMask: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                    crate::GlobalNamespace::OVRVirtualKeyboard_InputSource,
                    bool,
                    bool,
                ),
                quest_hook::libil2cpp::Void,
                4usize,
            >("SendVirtualKeyboardRayInput")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SendVirtualKeyboardRayInput", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (inputTransform, source, isPressed, useRaycastMask),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn SetKeyboardVisibility(
        &mut self,
        visible: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (bool),
                quest_hook::libil2cpp::Void,
                1usize,
            >("SetKeyboardVisibility")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SetKeyboardVisibility", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (visible))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ShowKeyboard(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("ShowKeyboard")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ShowKeyboard", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn SyncKeyboardLocation(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("SyncKeyboardLocation")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "SyncKeyboardLocation", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Update")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Update", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateAnimationState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("UpdateAnimationState")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UpdateAnimationState", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateInputs(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("UpdateInputs")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UpdateInputs", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateVisibleState(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("UpdateVisibleState")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UpdateVisibleState", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn UseSuggestedLocation(
        &mut self,
        position: crate::GlobalNamespace::OVRVirtualKeyboard_KeyboardPosition,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::GlobalNamespace::OVRVirtualKeyboard_KeyboardPosition),
                quest_hook::libil2cpp::Void,
                1usize,
            >("UseSuggestedLocation")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UseSuggestedLocation", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (position))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _LoadRuntimeVirtualKeyboardMesh_b__68_1(
        &mut self,
        rawUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mat: quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Material>,
                ),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D>,
                2usize,
            >("<LoadRuntimeVirtualKeyboardMesh>b__68_1")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "<LoadRuntimeVirtualKeyboardMesh>b__68_1", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Texture2D> = unsafe {
            method.invoke_unchecked(self, (rawUri, mat))
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_Backspace(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_Backspace")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_Backspace", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_CommitText")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_CommitText", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_Enter(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_Enter")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_Enter", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_KeyboardHidden(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_KeyboardHidden")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_KeyboardHidden", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn add_KeyboardShown(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("add_KeyboardShown")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "add_KeyboardShown", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_Collider(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
                0usize,
            >("get_Collider")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Collider", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider> = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_TextCommitField(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::InputField>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::InputField>,
                0usize,
            >("get_TextCommitField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_TextCommitField", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::InputField> = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_Backspace(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_Backspace")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "remove_Backspace", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::System::Action_1<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_CommitText")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "remove_CommitText", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_Enter(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_Enter")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "remove_Enter", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_KeyboardHidden(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_KeyboardHidden")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "remove_KeyboardHidden", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn remove_KeyboardShown(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Action>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Action>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("remove_KeyboardShown")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "remove_KeyboardShown", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_Collider(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::Collider>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_Collider")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_Collider", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_TextCommitField(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::InputField>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::UI::InputField>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_TextCommitField")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_TextCommitField", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
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
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRVirtualKeyboard_BaseInputSource {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRVirtualKeyboard/BaseInputSource";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Dispose")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Dispose", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
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
    pub fn OnUpdatedAnchors(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRCameraRig>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRCameraRig>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnUpdatedAnchors")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnUpdatedAnchors", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (obj))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Update")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Update", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn UpdateInput(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("UpdateInput")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UpdateInput", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
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
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRVirtualKeyboard_ControllerInputSource {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRVirtualKeyboard/ControllerInputSource";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("UpdateInput")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UpdateInput", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::OVRVirtualKeyboard,
                    >,
                    crate::GlobalNamespace::OVRVirtualKeyboard_InputSource,
                    crate::GlobalNamespace::OVRInput_Controller,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                ),
                quest_hook::libil2cpp::Void,
                5usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 5usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (
                        keyboard,
                        inputSource,
                        controllerType,
                        rootTransform,
                        directTransform,
                    ),
                )
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_TriggerIsPressed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), bool, 0usize>("get_TriggerIsPressed")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_TriggerIsPressed", 0usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked(self, ()) };
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
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRVirtualKeyboard_HandInputSource {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRVirtualKeyboard/HandInputSource";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("UpdateInput")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "UpdateInput", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        keyboard: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRVirtualKeyboard>,
        inputSource: crate::GlobalNamespace::OVRVirtualKeyboard_InputSource,
        hand: quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRHand>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::GlobalNamespace::OVRVirtualKeyboard,
                    >,
                    crate::GlobalNamespace::OVRVirtualKeyboard_InputSource,
                    quest_hook::libil2cpp::Gc<crate::GlobalNamespace::OVRHand>,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (keyboard, inputSource, hand))
        };
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
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRVirtualKeyboard_IInputSource {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRVirtualKeyboard/IInputSource";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Update")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Update", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
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
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRVirtualKeyboard_InputSource {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRVirtualKeyboard/InputSource";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "OVRVirtualKeyboard+InputSource")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRVirtualKeyboard_InputSource {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVRVirtualKeyboard+InputSource")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRVirtualKeyboard_InputSource {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "OVRVirtualKeyboard+InputSource")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRVirtualKeyboard_InputSource {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "OVRVirtualKeyboard+InputSource")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRVirtualKeyboard_InputSource {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRVirtualKeyboard_InteractorRootTransformOverride {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRVirtualKeyboard/InteractorRootTransformOverride";
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::GlobalNamespace::InteractorRootTransformOverride_OVRVirtualKeyboard_InteractorRootOverrideData),
                bool,
                1usize,
            >("ApplyOverride")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ApplyOverride", 1usize
                )
            });
        let __cordl_ret: bool = unsafe {
            method.invoke_unchecked((), (interactorOverride))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Enqueue(
        &mut self,
        interactorRootTransform: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::Transform,
        >,
        interactorRootPose: crate::GlobalNamespace::OVRPlugin_Posef,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::UnityEngine::Transform>,
                    crate::GlobalNamespace::OVRPlugin_Posef,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Enqueue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Enqueue", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (interactorRootTransform, interactorRootPose))
        };
        Ok(__cordl_ret.into())
    }
    pub fn LateApply(
        &mut self,
        coroutineRunner: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>),
                quest_hook::libil2cpp::Void,
                1usize,
            >("LateApply")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "LateApply", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (coroutineRunner))
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
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Reset")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Reset", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn RevertInteractorOverrides(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
                0usize,
            >("RevertInteractorOverrides")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "RevertInteractorOverrides", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
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
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::OVRVirtualKeyboard_KeyboardPosition {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRVirtualKeyboard/KeyboardPosition";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
#[cfg(feature = "OVRVirtualKeyboard+KeyboardPosition")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::OVRVirtualKeyboard_KeyboardPosition {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "OVRVirtualKeyboard+KeyboardPosition")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::OVRVirtualKeyboard_KeyboardPosition {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
#[cfg(feature = "OVRVirtualKeyboard+KeyboardPosition")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::OVRVirtualKeyboard_KeyboardPosition {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
#[cfg(feature = "OVRVirtualKeyboard+KeyboardPosition")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::OVRVirtualKeyboard_KeyboardPosition {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
