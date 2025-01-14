#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate")]
#[repr(C)]
#[derive(Debug)]
pub struct InputUpdate {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::LowLevel::InputUpdate {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.LowLevel";
    const CLASS_NAME: &'static str = "InputUpdate";
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
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate")]
impl std::ops::Deref for crate::UnityEngine::InputSystem::LowLevel::InputUpdate {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate")]
impl std::ops::DerefMut for crate::UnityEngine::InputSystem::LowLevel::InputUpdate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate")]
impl crate::UnityEngine::InputSystem::LowLevel::InputUpdate {
    #[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate+SerializedState")]
    pub type SerializedState = crate::UnityEngine::InputSystem::LowLevel::InputUpdate_SerializedState;
    #[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate+UpdateStepCount")]
    pub type UpdateStepCount = crate::UnityEngine::InputSystem::LowLevel::InputUpdate_UpdateStepCount;
    pub fn GetUpdateTypeForPlayer(
        mask: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::InputSystem::LowLevel::InputUpdateType),
                crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
                1usize,
            >("GetUpdateTypeForPlayer")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetUpdateTypeForPlayer", 1usize
                )
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType = unsafe {
            method.invoke_unchecked((), (mask))
        };
        Ok(__cordl_ret.into())
    }
    pub fn IsPlayerUpdate(
        updateType: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
    ) -> quest_hook::libil2cpp::Result<bool> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::InputSystem::LowLevel::InputUpdateType),
                bool,
                1usize,
            >("IsPlayerUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "IsPlayerUpdate", 1usize
                )
            });
        let __cordl_ret: bool = unsafe { method.invoke_unchecked((), (updateType)) };
        Ok(__cordl_ret.into())
    }
    pub fn OnBeforeUpdate(
        _cordl_type: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::InputSystem::LowLevel::InputUpdateType),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnBeforeUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnBeforeUpdate", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (_cordl_type))
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnUpdate(
        _cordl_type: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::InputSystem::LowLevel::InputUpdateType),
                quest_hook::libil2cpp::Void,
                1usize,
            >("OnUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnUpdate", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (_cordl_type))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Restore(
        state: crate::UnityEngine::InputSystem::LowLevel::InputUpdate_SerializedState,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (crate::UnityEngine::InputSystem::LowLevel::InputUpdate_SerializedState),
                quest_hook::libil2cpp::Void,
                1usize,
            >("Restore")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Restore", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (state))
        };
        Ok(__cordl_ret.into())
    }
    pub fn Save() -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::InputSystem::LowLevel::InputUpdate_SerializedState,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                crate::UnityEngine::InputSystem::LowLevel::InputUpdate_SerializedState,
                0usize,
            >("Save")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Save", 0usize
                )
            });
        let __cordl_ret: crate::UnityEngine::InputSystem::LowLevel::InputUpdate_SerializedState = unsafe {
            method.invoke_unchecked((), ())
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::LowLevel::InputUpdate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate+SerializedState")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct InputUpdate_SerializedState {
    pub lastUpdateType: crate::UnityEngine::InputSystem::LowLevel::InputUpdateType,
    pub playerUpdateStepCount: crate::UnityEngine::InputSystem::LowLevel::InputUpdate_UpdateStepCount,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate+SerializedState")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::LowLevel::InputUpdate_SerializedState {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.LowLevel";
    const CLASS_NAME: &'static str = "InputUpdate/SerializedState";
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
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate+SerializedState")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::LowLevel::InputUpdate_SerializedState {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate+SerializedState")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::LowLevel::InputUpdate_SerializedState {
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
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate+SerializedState")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::LowLevel::InputUpdate_SerializedState {
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
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate+SerializedState")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::LowLevel::InputUpdate_SerializedState {
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
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate+SerializedState")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::InputUpdate_SerializedState {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate+SerializedState")]
impl crate::UnityEngine::InputSystem::LowLevel::InputUpdate_SerializedState {}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate+UpdateStepCount")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct InputUpdate_UpdateStepCount {
    pub m_WasUpdated: bool,
    pub _value_k__BackingField: u32,
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate+UpdateStepCount")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::LowLevel::InputUpdate_UpdateStepCount {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.LowLevel";
    const CLASS_NAME: &'static str = "InputUpdate/UpdateStepCount";
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
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate+UpdateStepCount")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::LowLevel::InputUpdate_UpdateStepCount {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate+UpdateStepCount")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::LowLevel::InputUpdate_UpdateStepCount {
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
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate+UpdateStepCount")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::LowLevel::InputUpdate_UpdateStepCount {
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
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate+UpdateStepCount")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::LowLevel::InputUpdate_UpdateStepCount {
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
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate+UpdateStepCount")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::InputSystem::LowLevel::InputUpdate_UpdateStepCount {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+LowLevel+InputUpdate+UpdateStepCount")]
impl crate::UnityEngine::InputSystem::LowLevel::InputUpdate_UpdateStepCount {
    pub fn OnBeforeUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnBeforeUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnBeforeUpdate", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn OnUpdate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>("OnUpdate")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "OnUpdate", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_value(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), u32, 0usize>("get_value")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_value", 0usize
                )
            });
        let __cordl_ret: u32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn set_value(
        &mut self,
        value: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(u32), quest_hook::libil2cpp::Void, 1usize>("set_value")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_value", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
}
