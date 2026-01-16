#[cfg(
    feature = "cordl_class_Meta+XR+BuildingBlocks+ControllerButtonsMapper+ButtonClickAction+ButtonClickMode"
)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum ButtonClickAction_ControllerButtonsMapper_ButtonClickMode {
    #[default]
    OnButton = 2i32,
    OnButtonDown = 1i32,
    OnButtonUp = 0i32,
}
#[cfg(
    feature = "cordl_class_Meta+XR+BuildingBlocks+ControllerButtonsMapper+ButtonClickAction+ButtonClickMode"
)]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Meta::XR::BuildingBlocks::ButtonClickAction_ControllerButtonsMapper_ButtonClickMode
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Meta.XR.BuildingBlocks";
    const CLASS_NAME: &'static str = "ControllerButtonsMapper/ButtonClickAction/ButtonClickMode";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(
    feature = "cordl_class_Meta+XR+BuildingBlocks+ControllerButtonsMapper+ButtonClickAction+ButtonClickMode"
)]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::Meta::XR::BuildingBlocks::ButtonClickAction_ControllerButtonsMapper_ButtonClickMode
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(
    feature = "cordl_class_Meta+XR+BuildingBlocks+ControllerButtonsMapper+ButtonClickAction+ButtonClickMode"
)]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::Meta::XR::BuildingBlocks::ButtonClickAction_ControllerButtonsMapper_ButtonClickMode
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(
    feature = "cordl_class_Meta+XR+BuildingBlocks+ControllerButtonsMapper+ButtonClickAction+ButtonClickMode"
)]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::Meta::XR::BuildingBlocks::ButtonClickAction_ControllerButtonsMapper_ButtonClickMode
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(
    feature = "cordl_class_Meta+XR+BuildingBlocks+ControllerButtonsMapper+ButtonClickAction+ButtonClickMode"
)]
unsafe impl quest_hook::libil2cpp::Return
    for crate::Meta::XR::BuildingBlocks::ButtonClickAction_ControllerButtonsMapper_ButtonClickMode
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_Meta+XR+BuildingBlocks+ControllerButtonsMapper")]
#[repr(C)]
#[derive(Debug)]
pub struct ControllerButtonsMapper {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _buttonClickActions: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            crate::Meta::XR::BuildingBlocks::ControllerButtonsMapper_ButtonClickAction,
        >,
    >,
}
#[cfg(feature = "cordl_class_Meta+XR+BuildingBlocks+ControllerButtonsMapper")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Meta::XR::BuildingBlocks::ControllerButtonsMapper
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Meta.XR.BuildingBlocks";
    const CLASS_NAME: &'static str = "ControllerButtonsMapper";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "Meta+XR+BuildingBlocks+ControllerButtonsMapper")]
impl std::ops::Deref for crate::Meta::XR::BuildingBlocks::ControllerButtonsMapper {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+BuildingBlocks+ControllerButtonsMapper")]
impl std::ops::DerefMut for crate::Meta::XR::BuildingBlocks::ControllerButtonsMapper {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+BuildingBlocks+ControllerButtonsMapper")]
impl crate::Meta::XR::BuildingBlocks::ControllerButtonsMapper {
    pub const UseLegacyInputSystem: bool = true;
    pub const UseNewInputSystem: bool = true;
    #[cfg(feature = "Meta+XR+BuildingBlocks+ControllerButtonsMapper+ButtonClickAction")]
    pub type ButtonClickAction =
        crate::Meta::XR::BuildingBlocks::ControllerButtonsMapper_ButtonClickAction;
    pub fn IsActionTriggered(
        buttonClickAction: crate::Meta::XR::BuildingBlocks::ControllerButtonsMapper_ButtonClickAction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Meta::XR::BuildingBlocks::ControllerButtonsMapper_ButtonClickAction),
                        bool,
                        1usize,
                    >("IsActionTriggered")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsActionTriggered", 1usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (buttonClickAction))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsLegacyInputActionTriggered(
        buttonMode: crate::Meta::XR::BuildingBlocks::ButtonClickAction_ControllerButtonsMapper_ButtonClickMode,
        button: crate::GlobalNamespace::OVRInput_Button,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (
                            crate::Meta::XR::BuildingBlocks::ButtonClickAction_ControllerButtonsMapper_ButtonClickMode,
                            crate::GlobalNamespace::OVRInput_Button,
                        ),
                        bool,
                        2usize,
                    >("IsLegacyInputActionTriggered")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsLegacyInputActionTriggered", 2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (buttonMode, button))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsNewInputSystemActionTriggered(
        buttonClickAction: crate::Meta::XR::BuildingBlocks::ControllerButtonsMapper_ButtonClickAction,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::Meta::XR::BuildingBlocks::ControllerButtonsMapper_ButtonClickAction),
                        bool,
                        1usize,
                    >("IsNewInputSystemActionTriggered")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsNewInputSystemActionTriggered", 1usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (buttonClickAction))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDisable(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnDisable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnDisable",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn OnEnable(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnEnable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnEnable",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn Update(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("Update")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Update",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ButtonClickActions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::Meta::XR::BuildingBlocks::ControllerButtonsMapper_ButtonClickAction,
            >,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::Meta::XR::BuildingBlocks::ControllerButtonsMapper_ButtonClickAction,
                            >,
                        >,
                        0usize,
                    >("get_ButtonClickActions")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_ButtonClickActions", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::Meta::XR::BuildingBlocks::ControllerButtonsMapper_ButtonClickAction,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_ButtonClickActions(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                crate::Meta::XR::BuildingBlocks::ControllerButtonsMapper_ButtonClickAction,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::List_1<
                                crate::Meta::XR::BuildingBlocks::ControllerButtonsMapper_ButtonClickAction,
                            >,
                        >),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("set_ButtonClickActions")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "set_ButtonClickActions", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (value))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Meta+XR+BuildingBlocks+ControllerButtonsMapper")]
impl quest_hook::libil2cpp::ObjectType
    for crate::Meta::XR::BuildingBlocks::ControllerButtonsMapper
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_Meta+XR+BuildingBlocks+ControllerButtonsMapper+ButtonClickAction")]
#[derive(Debug, Clone, Default, PartialEq)]
#[repr(C)]
pub struct ControllerButtonsMapper_ButtonClickAction {
    pub Title: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub Button: crate::GlobalNamespace::OVRInput_Button,
    pub ButtonMode:
        crate::Meta::XR::BuildingBlocks::ButtonClickAction_ControllerButtonsMapper_ButtonClickMode,
    pub InputActionReference:
        quest_hook::libil2cpp::Gc<crate::UnityEngine::InputSystem::InputActionReference>,
    pub CallbackWithContext: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::Events::UnityEvent_1<
            crate::UnityEngine::InputSystem::InputAction_CallbackContext,
        >,
    >,
    pub Callback: quest_hook::libil2cpp::Gc<crate::UnityEngine::Events::UnityEvent>,
}
#[cfg(feature = "cordl_class_Meta+XR+BuildingBlocks+ControllerButtonsMapper+ButtonClickAction")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Meta::XR::BuildingBlocks::ControllerButtonsMapper_ButtonClickAction
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Meta.XR.BuildingBlocks";
    const CLASS_NAME: &'static str = "ControllerButtonsMapper/ButtonClickAction";
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_Meta+XR+BuildingBlocks+ControllerButtonsMapper+ButtonClickAction")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::Meta::XR::BuildingBlocks::ControllerButtonsMapper_ButtonClickAction
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Meta+XR+BuildingBlocks+ControllerButtonsMapper+ButtonClickAction")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::Meta::XR::BuildingBlocks::ControllerButtonsMapper_ButtonClickAction
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_Meta+XR+BuildingBlocks+ControllerButtonsMapper+ButtonClickAction")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::Meta::XR::BuildingBlocks::ControllerButtonsMapper_ButtonClickAction
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_Meta+XR+BuildingBlocks+ControllerButtonsMapper+ButtonClickAction")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::Meta::XR::BuildingBlocks::ControllerButtonsMapper_ButtonClickAction
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_Meta+XR+BuildingBlocks+ControllerButtonsMapper+ButtonClickAction")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::Meta::XR::BuildingBlocks::ControllerButtonsMapper_ButtonClickAction
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Meta+XR+BuildingBlocks+ControllerButtonsMapper+ButtonClickAction")]
impl crate::Meta::XR::BuildingBlocks::ControllerButtonsMapper_ButtonClickAction {
    #[cfg(
        feature = "Meta+XR+BuildingBlocks+ControllerButtonsMapper+ButtonClickAction+ButtonClickMode"
    )]
    pub type ButtonClickMode =
        crate::Meta::XR::BuildingBlocks::ButtonClickAction_ControllerButtonsMapper_ButtonClickMode;
    pub fn OnCallbackWithContext(
        &mut self,
        callbackContext: crate::UnityEngine::InputSystem::InputAction_CallbackContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::UnityEngine::InputSystem::InputAction_CallbackContext),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("OnCallbackWithContext")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "OnCallbackWithContext", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (callbackContext))? };
        Ok(__cordl_ret.into())
    }
}
