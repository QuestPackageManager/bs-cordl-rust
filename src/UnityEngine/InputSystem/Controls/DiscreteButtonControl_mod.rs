#[cfg(feature = "UnityEngine+InputSystem+Controls+DiscreteButtonControl")]
#[repr(C)]
#[derive(Debug)]
pub struct DiscreteButtonControl {
    __cordl_parent: crate::UnityEngine::InputSystem::Controls::ButtonControl,
    pub minValue: i32,
    pub maxValue: i32,
    pub wrapAtValue: i32,
    pub nullValue: i32,
    pub writeMode: crate::UnityEngine::InputSystem::Controls::DiscreteButtonControl_WriteMode,
}
#[cfg(feature = "UnityEngine+InputSystem+Controls+DiscreteButtonControl")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::Controls::DiscreteButtonControl {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Controls";
    const CLASS_NAME: &'static str = "DiscreteButtonControl";
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
#[cfg(feature = "UnityEngine+InputSystem+Controls+DiscreteButtonControl")]
impl std::ops::Deref
for crate::UnityEngine::InputSystem::Controls::DiscreteButtonControl {
    type Target = crate::UnityEngine::InputSystem::Controls::ButtonControl;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Controls+DiscreteButtonControl")]
impl std::ops::DerefMut
for crate::UnityEngine::InputSystem::Controls::DiscreteButtonControl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Controls+DiscreteButtonControl")]
impl crate::UnityEngine::InputSystem::Controls::DiscreteButtonControl {
    #[cfg(feature = "UnityEngine+InputSystem+Controls+DiscreteButtonControl+WriteMode")]
    pub type WriteMode = crate::UnityEngine::InputSystem::Controls::DiscreteButtonControl_WriteMode;
    pub fn FinishSetup(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinishSetup", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ReadUnprocessedValueFromState(
        &mut self,
        statePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object
            .invoke("ReadUnprocessedValueFromState", (statePtr))?;
        Ok(__cordl_ret.into())
    }
    pub fn WriteValueIntoState(
        &mut self,
        value: f32,
        statePtr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WriteValueIntoState", (value, statePtr))?;
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
#[cfg(feature = "UnityEngine+InputSystem+Controls+DiscreteButtonControl")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::InputSystem::Controls::DiscreteButtonControl {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Controls+DiscreteButtonControl+WriteMode")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum DiscreteButtonControl_WriteMode {
    #[default]
    WriteDisabled = 0i32,
    WriteNullAndMaxValue = 1i32,
}
#[cfg(feature = "UnityEngine+InputSystem+Controls+DiscreteButtonControl+WriteMode")]
unsafe impl quest_hook::libil2cpp::Type
for crate::UnityEngine::InputSystem::Controls::DiscreteButtonControl_WriteMode {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem.Controls";
    const CLASS_NAME: &'static str = "DiscreteButtonControl/WriteMode";
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
#[cfg(feature = "UnityEngine+InputSystem+Controls+DiscreteButtonControl+WriteMode")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::UnityEngine::InputSystem::Controls::DiscreteButtonControl_WriteMode {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "UnityEngine+InputSystem+Controls+DiscreteButtonControl+WriteMode")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::UnityEngine::InputSystem::Controls::DiscreteButtonControl_WriteMode {
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
#[cfg(feature = "UnityEngine+InputSystem+Controls+DiscreteButtonControl+WriteMode")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::UnityEngine::InputSystem::Controls::DiscreteButtonControl_WriteMode {
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
#[cfg(feature = "UnityEngine+InputSystem+Controls+DiscreteButtonControl+WriteMode")]
unsafe impl quest_hook::libil2cpp::Return
for crate::UnityEngine::InputSystem::Controls::DiscreteButtonControl_WriteMode {
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
