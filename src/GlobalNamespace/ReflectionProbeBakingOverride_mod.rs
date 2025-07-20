#[cfg(feature = "ReflectionProbeBakingOverride")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectionProbeBakingOverride {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _stateHandling: crate::GlobalNamespace::ReflectionProbeBakingOverride_ActiveStateHandling,
    pub _setPosition: bool,
    pub _localPosition: crate::UnityEngine::Vector3,
    pub _setRotation: bool,
    pub _localRotation: crate::UnityEngine::Vector3,
    pub _setScale: bool,
    pub _localScale: crate::UnityEngine::Vector3,
}
#[cfg(feature = "ReflectionProbeBakingOverride")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ReflectionProbeBakingOverride {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ReflectionProbeBakingOverride";
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
#[cfg(feature = "ReflectionProbeBakingOverride")]
impl std::ops::Deref for crate::GlobalNamespace::ReflectionProbeBakingOverride {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ReflectionProbeBakingOverride")]
impl std::ops::DerefMut for crate::GlobalNamespace::ReflectionProbeBakingOverride {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ReflectionProbeBakingOverride")]
impl crate::GlobalNamespace::ReflectionProbeBakingOverride {
    #[cfg(feature = "ReflectionProbeBakingOverride+ActiveStateHandling")]
    pub type ActiveStateHandling = crate::GlobalNamespace::ReflectionProbeBakingOverride_ActiveStateHandling;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn UpdateForProbeBaking(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::ReflectionProbeBakingOverride as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Void,
                0usize,
            >("UpdateForProbeBaking")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::ReflectionProbeBakingOverride as
                    quest_hook::libil2cpp::Type > ::class(), "UpdateForProbeBaking",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::GlobalNamespace::ReflectionProbeBakingOverride as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::GlobalNamespace::ReflectionProbeBakingOverride as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ReflectionProbeBakingOverride")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::ReflectionProbeBakingOverride {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "ReflectionProbeBakingOverride+ActiveStateHandling")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ReflectionProbeBakingOverride_ActiveStateHandling {
    #[default]
    Disable = 2i32,
    Enable = 1i32,
    LeaveAsIs = 0i32,
}
#[cfg(feature = "ReflectionProbeBakingOverride+ActiveStateHandling")]
unsafe impl quest_hook::libil2cpp::Type
for crate::GlobalNamespace::ReflectionProbeBakingOverride_ActiveStateHandling {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "";
    const CLASS_NAME: &'static str = "ReflectionProbeBakingOverride/ActiveStateHandling";
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
#[cfg(feature = "ReflectionProbeBakingOverride+ActiveStateHandling")]
unsafe impl quest_hook::libil2cpp::Argument
for crate::GlobalNamespace::ReflectionProbeBakingOverride_ActiveStateHandling {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "ReflectionProbeBakingOverride+ActiveStateHandling")]
unsafe impl quest_hook::libil2cpp::Parameter
for crate::GlobalNamespace::ReflectionProbeBakingOverride_ActiveStateHandling {
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
#[cfg(feature = "ReflectionProbeBakingOverride+ActiveStateHandling")]
unsafe impl quest_hook::libil2cpp::Returned
for crate::GlobalNamespace::ReflectionProbeBakingOverride_ActiveStateHandling {
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
#[cfg(feature = "ReflectionProbeBakingOverride+ActiveStateHandling")]
unsafe impl quest_hook::libil2cpp::Return
for crate::GlobalNamespace::ReflectionProbeBakingOverride_ActiveStateHandling {
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
