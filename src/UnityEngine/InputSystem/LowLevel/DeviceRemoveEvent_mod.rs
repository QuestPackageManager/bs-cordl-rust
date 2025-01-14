#[cfg(feature = "UnityEngine+InputSystem+LowLevel+DeviceRemoveEvent")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct DeviceRemoveEvent {
    padding: quest_hook::libil2cpp::ValueTypePadding<20usize>,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+DeviceRemoveEvent")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::LowLevel::DeviceRemoveEvent {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.LowLevel";
    const CLASS_NAME: &'static str = "DeviceRemoveEvent";
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
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+DeviceRemoveEvent")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::LowLevel::DeviceRemoveEvent {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+DeviceRemoveEvent")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::LowLevel::DeviceRemoveEvent {
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
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+DeviceRemoveEvent")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::LowLevel::DeviceRemoveEvent {
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
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+DeviceRemoveEvent")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::LowLevel::DeviceRemoveEvent {
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
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+DeviceRemoveEvent")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::DeviceRemoveEvent {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+DeviceRemoveEvent")]
impl crate::UnityEngine::InputSystem::LowLevel::DeviceRemoveEvent {
    pub const Type: i32 = 1146242381i32;
    pub fn Create(
        deviceId: i32,
        _cordl_time: f64,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::DeviceRemoveEvent,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (i32, f64),
                crate::UnityEngine::InputSystem::LowLevel::DeviceRemoveEvent,
                2usize,
            >("Create")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Create", 2usize
                )
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::DeviceRemoveEvent = unsafe {
            method.invoke_unchecked((), (deviceId, _cordl_time))
        };
        Ok(__cordl_ret.into())
    }
    pub fn ToEventPtr(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::InputSystem::LowLevel::InputEventPtr,
                0usize,
            >("ToEventPtr")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "ToEventPtr", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::InputEventPtr = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_typeStatic(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::Utilities::FourCC,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::UnityEngine::InputSystem::Utilities::FourCC,
                0usize,
            >("get_typeStatic")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_typeStatic", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::Utilities::FourCC = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+DeviceRemoveEvent")]
impl AsRef<crate::UnityEngine::InputSystem::LowLevel::IInputEventTypeInfo>
for crate::UnityEngine::InputSystem::LowLevel::DeviceRemoveEvent {
    fn as_ref(&self) -> &crate::UnityEngine::InputSystem::LowLevel::IInputEventTypeInfo {
        todo!()
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+DeviceRemoveEvent")]
impl AsMut<crate::UnityEngine::InputSystem::LowLevel::IInputEventTypeInfo>
for crate::UnityEngine::InputSystem::LowLevel::DeviceRemoveEvent {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::InputSystem::LowLevel::IInputEventTypeInfo {
        todo!()
    }
}
