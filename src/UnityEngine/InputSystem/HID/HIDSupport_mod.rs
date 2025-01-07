#[cfg(feature = "UnityEngine+InputSystem+HID+HIDSupport")]
#[repr(C)]
#[derive(Debug)]
pub struct HIDSupport {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDSupport")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::HID::HIDSupport {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.HID";
    const CLASS_NAME: &'static str = "HIDSupport";
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
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDSupport")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::HID::HIDSupport {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDSupport")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::HID::HIDSupport {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDSupport")]
impl crate::UnityEngine::InputSystem::HID::HIDSupport {
    #[cfg(feature = "UnityEngine+InputSystem+HID+HIDSupport+HIDPageUsage")]
    pub type HIDPageUsage = crate::UnityEngine::InputSystem::HID::HIDSupport_HIDPageUsage;
    pub fn Initialize() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Initialize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_supportedHIDUsages() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::HID::HIDSupport_HIDPageUsage,
        >,
    > {
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::HID::HIDSupport_HIDPageUsage,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_supportedHIDUsages", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_supportedHIDUsages(
        value: crate::UnityEngine::InputSystem::Utilities::ReadOnlyArray_1<
            crate::UnityEngine::InputSystem::HID::HIDSupport_HIDPageUsage,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_supportedHIDUsages", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDSupport")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::HID::HIDSupport {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDSupport+HIDPageUsage")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct HIDSupport_HIDPageUsage {
    pub page: crate::UnityEngine::InputSystem::HID::HID_UsagePage,
    pub usage: i32,
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDSupport+HIDPageUsage")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::HID::HIDSupport_HIDPageUsage {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.HID";
    const CLASS_NAME: &'static str = "HIDPageUsage";
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
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDSupport+HIDPageUsage")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::HID::HIDSupport_HIDPageUsage {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDSupport+HIDPageUsage")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::HID::HIDSupport_HIDPageUsage {
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
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDSupport+HIDPageUsage")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::HID::HIDSupport_HIDPageUsage {
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
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDSupport+HIDPageUsage")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::HID::HIDSupport_HIDPageUsage {
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
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDSupport+HIDPageUsage")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::HID::HIDSupport_HIDPageUsage {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+HID+HIDSupport+HIDPageUsage")]
impl crate::UnityEngine::InputSystem::HID::HIDSupport_HIDPageUsage {
    pub fn _ctor_HID_GenericDesktop1(
        &mut self,
        usage: crate::UnityEngine::InputSystem::HID::HID_GenericDesktop,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (usage),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_HID_UsagePage_i32_0(
        &mut self,
        page: crate::UnityEngine::InputSystem::HID::HID_UsagePage,
        usage: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (page, usage),
        )?;
        Ok(__cordl_ret.into())
    }
}
