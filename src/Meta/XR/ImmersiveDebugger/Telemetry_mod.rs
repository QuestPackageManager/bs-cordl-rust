#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+Telemetry")]
#[repr(C)]
#[derive(Debug)]
pub struct Telemetry {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+Telemetry")]
unsafe impl quest_hook::libil2cpp::Type for crate::Meta::XR::ImmersiveDebugger::Telemetry {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Meta.XR.ImmersiveDebugger";
    const CLASS_NAME: &'static str = "Telemetry";
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
#[cfg(feature = "Meta+XR+ImmersiveDebugger+Telemetry")]
impl std::ops::Deref for crate::Meta::XR::ImmersiveDebugger::Telemetry {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+ImmersiveDebugger+Telemetry")]
impl std::ops::DerefMut for crate::Meta::XR::ImmersiveDebugger::Telemetry {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+ImmersiveDebugger+Telemetry")]
impl crate::Meta::XR::ImmersiveDebugger::Telemetry {
    #[cfg(feature = "Meta+XR+ImmersiveDebugger+Telemetry+AnnotationType")]
    pub type AnnotationType = crate::Meta::XR::ImmersiveDebugger::Telemetry_AnnotationType;
    #[cfg(feature = "Meta+XR+ImmersiveDebugger+Telemetry+MarkerId")]
    pub type MarkerId = crate::Meta::XR::ImmersiveDebugger::Telemetry_MarkerId;
    #[cfg(feature = "Meta+XR+ImmersiveDebugger+Telemetry+Method")]
    pub type Method = crate::Meta::XR::ImmersiveDebugger::Telemetry_Method;
    #[cfg(feature = "Meta+XR+ImmersiveDebugger+Telemetry+State")]
    pub type State = crate::Meta::XR::ImmersiveDebugger::Telemetry_State;
    #[cfg(feature = "Meta+XR+ImmersiveDebugger+Telemetry+TelemetryTracker")]
    pub type TelemetryTracker = crate::Meta::XR::ImmersiveDebugger::Telemetry_TelemetryTracker;
    pub fn FetchPanel(
        controller: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::Controller,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::Panel,
        >,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::Gc<
                        crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::Controller,
                    >), quest_hook::libil2cpp::Gc<
                        crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::Panel,
                    >, 1usize>("FetchPanel")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "FetchPanel",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::Panel,
        > = unsafe { cordl_method_info.invoke_unchecked((), (controller))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetTypeHash(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        1usize,
                    >("GetTypeHash")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetTypeHash", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked((), (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn IsTypeCustom(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (quest_hook::libil2cpp::Gc<crate::System::Type>),
                        bool,
                        1usize,
                    >("IsTypeCustom")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "IsTypeCustom", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (_cordl_type))? };
        Ok(__cordl_ret.into())
    }
    pub fn OnButtonClicked(
        button: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::Button,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::Gc<
                        crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::Button,
                    >), quest_hook::libil2cpp::Void, 1usize>("OnButtonClicked")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnButtonClicked",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (button))? };
        Ok(__cordl_ret.into())
    }
    pub fn OnPanelActiveStateChanged(
        panel: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::Panel,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(quest_hook::libil2cpp::Gc<
                        crate::Meta::XR::ImmersiveDebugger::UserInterface::Generic::Panel,
                    >), quest_hook::libil2cpp::Void, 1usize>(
                        "OnPanelActiveStateChanged"
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnPanelActiveStateChanged",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked((), (panel))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+Telemetry")]
impl quest_hook::libil2cpp::ObjectType for crate::Meta::XR::ImmersiveDebugger::Telemetry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+Telemetry+AnnotationType")]
#[repr(C)]
#[derive(Debug)]
pub struct Telemetry_AnnotationType {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+Telemetry+AnnotationType")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Meta::XR::ImmersiveDebugger::Telemetry_AnnotationType
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Meta.XR.ImmersiveDebugger";
    const CLASS_NAME: &'static str = "Telemetry/AnnotationType";
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
#[cfg(feature = "Meta+XR+ImmersiveDebugger+Telemetry+AnnotationType")]
impl std::ops::Deref for crate::Meta::XR::ImmersiveDebugger::Telemetry_AnnotationType {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+ImmersiveDebugger+Telemetry+AnnotationType")]
impl std::ops::DerefMut for crate::Meta::XR::ImmersiveDebugger::Telemetry_AnnotationType {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+ImmersiveDebugger+Telemetry+AnnotationType")]
impl crate::Meta::XR::ImmersiveDebugger::Telemetry_AnnotationType {
    pub const Action: &'static str = "action";
    pub const ActionType: &'static str = "action_type";
    pub const Actions: &'static str = "Actions";
    pub const Gizmos: &'static str = "Gizmos";
    pub const Instances: &'static str = "Instances";
    pub const IsCustom: &'static str = "IsCustom";
    pub const Method: &'static str = "Method";
    pub const Origin: &'static str = "origin";
    pub const OriginType: &'static str = "origin_type";
    pub const Platform: &'static str = "platform";
    pub const State: &'static str = "State";
    pub const Tweaks: &'static str = "Tweaks";
    pub const Type: &'static str = "Type";
    pub const Watches: &'static str = "Watches";
}
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+Telemetry+AnnotationType")]
impl quest_hook::libil2cpp::ObjectType
    for crate::Meta::XR::ImmersiveDebugger::Telemetry_AnnotationType
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+Telemetry+MarkerId")]
#[repr(C)]
#[derive(Debug)]
pub struct Telemetry_MarkerId {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+Telemetry+MarkerId")]
unsafe impl quest_hook::libil2cpp::Type for crate::Meta::XR::ImmersiveDebugger::Telemetry_MarkerId {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Meta.XR.ImmersiveDebugger";
    const CLASS_NAME: &'static str = "Telemetry/MarkerId";
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
#[cfg(feature = "Meta+XR+ImmersiveDebugger+Telemetry+MarkerId")]
impl std::ops::Deref for crate::Meta::XR::ImmersiveDebugger::Telemetry_MarkerId {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+ImmersiveDebugger+Telemetry+MarkerId")]
impl std::ops::DerefMut for crate::Meta::XR::ImmersiveDebugger::Telemetry_MarkerId {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+ImmersiveDebugger+Telemetry+MarkerId")]
impl crate::Meta::XR::ImmersiveDebugger::Telemetry_MarkerId {
    pub const ComponentTracked: i32 = 163059554i32;
    pub const FrameUpdate: i32 = 163056655i32;
    pub const PanelClose: i32 = 163059919i32;
    pub const PanelInteraction: i32 = 163058794i32;
    pub const PanelOpen: i32 = 163057243i32;
    pub const Run: i32 = 163061656i32;
}
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+Telemetry+MarkerId")]
impl quest_hook::libil2cpp::ObjectType for crate::Meta::XR::ImmersiveDebugger::Telemetry_MarkerId {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+Telemetry+Method")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum Telemetry_Method {
    #[default]
    Attributes = 0i32,
    DebugInspector = 1i32,
    Hierarchy = 2i32,
}
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+Telemetry+Method")]
unsafe impl quest_hook::libil2cpp::Type for crate::Meta::XR::ImmersiveDebugger::Telemetry_Method {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Meta.XR.ImmersiveDebugger";
    const CLASS_NAME: &'static str = "Telemetry/Method";
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
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+Telemetry+Method")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::Meta::XR::ImmersiveDebugger::Telemetry_Method
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+Telemetry+Method")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::Meta::XR::ImmersiveDebugger::Telemetry_Method
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
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+Telemetry+Method")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::Meta::XR::ImmersiveDebugger::Telemetry_Method
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
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+Telemetry+Method")]
unsafe impl quest_hook::libil2cpp::Return for crate::Meta::XR::ImmersiveDebugger::Telemetry_Method {
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
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+Telemetry+State")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(i32)]
pub enum Telemetry_State {
    #[default]
    OnDisable = 2i32,
    OnFocusLost = 1i32,
    OnStart = 0i32,
}
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+Telemetry+State")]
unsafe impl quest_hook::libil2cpp::Type for crate::Meta::XR::ImmersiveDebugger::Telemetry_State {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Meta.XR.ImmersiveDebugger";
    const CLASS_NAME: &'static str = "Telemetry/State";
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
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+Telemetry+State")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::Meta::XR::ImmersiveDebugger::Telemetry_State
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+Telemetry+State")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::Meta::XR::ImmersiveDebugger::Telemetry_State
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
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+Telemetry+State")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::Meta::XR::ImmersiveDebugger::Telemetry_State
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
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+Telemetry+State")]
unsafe impl quest_hook::libil2cpp::Return for crate::Meta::XR::ImmersiveDebugger::Telemetry_State {
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
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+Telemetry+TelemetryTracker")]
#[repr(C)]
#[derive(Debug)]
pub struct Telemetry_TelemetryTracker {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _method: crate::Meta::XR::ImmersiveDebugger::Telemetry_Method,
    pub _cache: quest_hook::libil2cpp::Gc<crate::Meta::XR::ImmersiveDebugger::Utils::InstanceCache>,
    pub _managers: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IEnumerable_1<
            quest_hook::libil2cpp::Gc<crate::Meta::XR::ImmersiveDebugger::Manager::IDebugManager>,
        >,
    >,
    pub _runTelemetryMarker: crate::GlobalNamespace::OVRTelemetryMarker,
}
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+Telemetry+TelemetryTracker")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::Meta::XR::ImmersiveDebugger::Telemetry_TelemetryTracker
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Meta.XR.ImmersiveDebugger";
    const CLASS_NAME: &'static str = "Telemetry/TelemetryTracker";
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
#[cfg(feature = "Meta+XR+ImmersiveDebugger+Telemetry+TelemetryTracker")]
impl std::ops::Deref for crate::Meta::XR::ImmersiveDebugger::Telemetry_TelemetryTracker {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+ImmersiveDebugger+Telemetry+TelemetryTracker")]
impl std::ops::DerefMut for crate::Meta::XR::ImmersiveDebugger::Telemetry_TelemetryTracker {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Meta+XR+ImmersiveDebugger+Telemetry+TelemetryTracker")]
impl crate::Meta::XR::ImmersiveDebugger::Telemetry_TelemetryTracker {
    pub fn Init(
        method: crate::Meta::XR::ImmersiveDebugger::Telemetry_Method,
        managers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::Meta::XR::ImmersiveDebugger::Manager::IDebugManager,
                >,
            >,
        >,
        cache: quest_hook::libil2cpp::Gc<crate::Meta::XR::ImmersiveDebugger::Utils::InstanceCache>,
        debugManager: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::Manager::DebugManager,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Meta::XR::ImmersiveDebugger::Telemetry_TelemetryTracker>,
    > {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::Meta::XR::ImmersiveDebugger::Telemetry_Method,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::Meta::XR::ImmersiveDebugger::Manager::IDebugManager,
                                >,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::Meta::XR::ImmersiveDebugger::Utils::InstanceCache,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::Meta::XR::ImmersiveDebugger::Manager::DebugManager,
                        >,
                    ), quest_hook::libil2cpp::Gc<
                        crate::Meta::XR::ImmersiveDebugger::Telemetry_TelemetryTracker,
                    >, 4usize>("Init")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Init",
                            4usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Meta::XR::ImmersiveDebugger::Telemetry_TelemetryTracker,
        > = unsafe {
            cordl_method_info.invoke_unchecked((), (method, managers, cache, debugManager))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn New(
        method: crate::Meta::XR::ImmersiveDebugger::Telemetry_Method,
        managers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::Meta::XR::ImmersiveDebugger::Manager::IDebugManager,
                >,
            >,
        >,
        cache: quest_hook::libil2cpp::Gc<crate::Meta::XR::ImmersiveDebugger::Utils::InstanceCache>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (method, managers, cache))?;
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
    pub fn OnFocusLost(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnFocusLost")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnFocusLost",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn OnStart(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnStart")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "OnStart",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn SendComponentTracked(
        &mut self,
        state: crate::Meta::XR::ImmersiveDebugger::Telemetry_State,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (crate::Meta::XR::ImmersiveDebugger::Telemetry_State),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("SendComponentTracked")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SendComponentTracked", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (state))? };
        Ok(__cordl_ret.into())
    }
    pub fn SendStart(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("SendStart")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "SendStart",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        method: crate::Meta::XR::ImmersiveDebugger::Telemetry_Method,
        managers: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<
                    crate::Meta::XR::ImmersiveDebugger::Manager::IDebugManager,
                >,
            >,
        >,
        cache: quest_hook::libil2cpp::Gc<crate::Meta::XR::ImmersiveDebugger::Utils::InstanceCache>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(
                        crate::Meta::XR::ImmersiveDebugger::Telemetry_Method,
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::Generic::IEnumerable_1<
                                quest_hook::libil2cpp::Gc<
                                    crate::Meta::XR::ImmersiveDebugger::Manager::IDebugManager,
                                >,
                            >,
                        >,
                        quest_hook::libil2cpp::Gc<
                            crate::Meta::XR::ImmersiveDebugger::Utils::InstanceCache,
                        >,
                    ), quest_hook::libil2cpp::Void, 3usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (method, managers, cache))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Meta+XR+ImmersiveDebugger+Telemetry+TelemetryTracker")]
impl quest_hook::libil2cpp::ObjectType
    for crate::Meta::XR::ImmersiveDebugger::Telemetry_TelemetryTracker
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
