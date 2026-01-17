#[cfg(feature = "cordl_class_OVRKeyboard")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct OVRKeyboard {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "cordl_class_OVRKeyboard")]
unsafe impl quest_hook::libil2cpp::Type for crate::GlobalNamespace::OVRKeyboard {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRKeyboard";
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
#[cfg(feature = "OVRKeyboard")]
impl std::ops::Deref for crate::GlobalNamespace::OVRKeyboard {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "OVRKeyboard")]
impl std::ops::DerefMut for crate::GlobalNamespace::OVRKeyboard {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "OVRKeyboard")]
impl crate::GlobalNamespace::OVRKeyboard {
    #[cfg(feature = "OVRKeyboard+TrackedKeyboardInfo")]
    pub type TrackedKeyboardInfo = crate::GlobalNamespace::OVRKeyboard_TrackedKeyboardInfo;
    #[cfg(feature = "OVRKeyboard+TrackedKeyboardState")]
    pub type TrackedKeyboardState = crate::GlobalNamespace::OVRKeyboard_TrackedKeyboardState;
    pub fn GetKeyboardState(
    ) -> quest_hook::libil2cpp::Result<crate::GlobalNamespace::OVRKeyboard_TrackedKeyboardState>
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (),
                        crate::GlobalNamespace::OVRKeyboard_TrackedKeyboardState,
                        0usize,
                    >("GetKeyboardState")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetKeyboardState", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::GlobalNamespace::OVRKeyboard_TrackedKeyboardState =
            unsafe { cordl_method_info.invoke_unchecked((), ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetSystemKeyboardInfo(
        keyboardQueryFlags: crate::GlobalNamespace::OVRPlugin_TrackedKeyboardQueryFlags,
        keyboardInfo: quest_hook::libil2cpp::ByRefMut<
            crate::GlobalNamespace::OVRKeyboard_TrackedKeyboardInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<(
                        crate::GlobalNamespace::OVRPlugin_TrackedKeyboardQueryFlags,
                        quest_hook::libil2cpp::ByRefMut<
                            crate::GlobalNamespace::OVRKeyboard_TrackedKeyboardInfo,
                        >,
                    ), bool, 2usize>("GetSystemKeyboardInfo")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetSystemKeyboardInfo",
                            2usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked((), (keyboardQueryFlags, keyboardInfo))? };
        Ok(__cordl_ret.into())
    }
    pub fn StopKeyboardTracking(
        keyboardInfo: crate::GlobalNamespace::OVRKeyboard_TrackedKeyboardInfo,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_static_method::<
                        (crate::GlobalNamespace::OVRKeyboard_TrackedKeyboardInfo),
                        bool,
                        1usize,
                    >("StopKeyboardTracking")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "StopKeyboardTracking", 1usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked((), (keyboardInfo))? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_OVRKeyboard")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::OVRKeyboard {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "cordl_class_OVRKeyboard+TrackedKeyboardInfo")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[repr(C)]
pub struct OVRKeyboard_TrackedKeyboardInfo {
    pub Name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub Identifier: u64,
    pub Dimensions: crate::UnityEngine::Vector3,
    pub KeyboardFlags: crate::GlobalNamespace::OVRPlugin_TrackedKeyboardFlags,
    pub SupportedPresentationStyles:
        crate::GlobalNamespace::OVRPlugin_TrackedKeyboardPresentationStyles,
}
#[cfg(feature = "cordl_class_OVRKeyboard+TrackedKeyboardInfo")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::OVRKeyboard_TrackedKeyboardInfo
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRKeyboard/TrackedKeyboardInfo";
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
#[cfg(feature = "cordl_class_OVRKeyboard+TrackedKeyboardInfo")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::GlobalNamespace::OVRKeyboard_TrackedKeyboardInfo
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRKeyboard+TrackedKeyboardInfo")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::GlobalNamespace::OVRKeyboard_TrackedKeyboardInfo
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
#[cfg(feature = "cordl_class_OVRKeyboard+TrackedKeyboardInfo")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::GlobalNamespace::OVRKeyboard_TrackedKeyboardInfo
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
#[cfg(feature = "cordl_class_OVRKeyboard+TrackedKeyboardInfo")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::GlobalNamespace::OVRKeyboard_TrackedKeyboardInfo
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
#[cfg(feature = "cordl_class_OVRKeyboard+TrackedKeyboardInfo")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::GlobalNamespace::OVRKeyboard_TrackedKeyboardInfo
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRKeyboard+TrackedKeyboardInfo")]
impl crate::GlobalNamespace::OVRKeyboard_TrackedKeyboardInfo {}
#[cfg(feature = "cordl_class_OVRKeyboard+TrackedKeyboardState")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[repr(C)]
pub struct OVRKeyboard_TrackedKeyboardState {
    pub isPositionValid: bool,
    pub isPositionTracked: bool,
    pub isOrientationValid: bool,
    pub isOrientationTracked: bool,
    pub position: crate::UnityEngine::Vector3,
    pub rotation: crate::UnityEngine::Quaternion,
    pub timeInSeconds: f64,
}
#[cfg(feature = "cordl_class_OVRKeyboard+TrackedKeyboardState")]
unsafe impl quest_hook::libil2cpp::Type
    for crate::GlobalNamespace::OVRKeyboard_TrackedKeyboardState
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "OVRKeyboard/TrackedKeyboardState";
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
#[cfg(feature = "cordl_class_OVRKeyboard+TrackedKeyboardState")]
unsafe impl quest_hook::libil2cpp::Argument
    for crate::GlobalNamespace::OVRKeyboard_TrackedKeyboardState
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_OVRKeyboard+TrackedKeyboardState")]
unsafe impl quest_hook::libil2cpp::Parameter
    for crate::GlobalNamespace::OVRKeyboard_TrackedKeyboardState
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
#[cfg(feature = "cordl_class_OVRKeyboard+TrackedKeyboardState")]
unsafe impl quest_hook::libil2cpp::Returned
    for crate::GlobalNamespace::OVRKeyboard_TrackedKeyboardState
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
#[cfg(feature = "cordl_class_OVRKeyboard+TrackedKeyboardState")]
unsafe impl quest_hook::libil2cpp::Return
    for crate::GlobalNamespace::OVRKeyboard_TrackedKeyboardState
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
#[cfg(feature = "cordl_class_OVRKeyboard+TrackedKeyboardState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
    for crate::GlobalNamespace::OVRKeyboard_TrackedKeyboardState
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "OVRKeyboard+TrackedKeyboardState")]
impl crate::GlobalNamespace::OVRKeyboard_TrackedKeyboardState {}
