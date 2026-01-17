#[cfg(feature = "cordl_class_UnityEngine+InputSystem+IInputInteraction_1")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[repr(C)]
pub struct IInputInteraction_1<TValue: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+IInputInteraction_1")]
unsafe impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
    for crate::UnityEngine::InputSystem::IInputInteraction_1<TValue>
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.InputSystem";
    const CLASS_NAME: &'static str = "IInputInteraction`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find(
                "UnityEngine.InputSystem",
                "IInputInteraction`1",
            )
            .unwrap()
            .make_generic::<(TValue)>()
            .unwrap()
            .unwrap()
        })
    }
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
#[cfg(feature = "UnityEngine+InputSystem+IInputInteraction_1")]
impl<TValue: quest_hook::libil2cpp::Type> std::ops::Deref
    for crate::UnityEngine::InputSystem::IInputInteraction_1<TValue>
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+IInputInteraction_1")]
impl<TValue: quest_hook::libil2cpp::Type> std::ops::DerefMut
    for crate::UnityEngine::InputSystem::IInputInteraction_1<TValue>
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+IInputInteraction_1")]
impl<TValue: quest_hook::libil2cpp::Type>
    crate::UnityEngine::InputSystem::IInputInteraction_1<TValue>
{
    pub fn from_object_mut(object_param: *mut quest_hook::libil2cpp::Il2CppObject) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+InputSystem+IInputInteraction_1")]
impl<TValue: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
    for crate::UnityEngine::InputSystem::IInputInteraction_1<TValue>
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+InputSystem+IInputInteraction_1")]
impl<TValue: quest_hook::libil2cpp::Type>
    AsRef<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>>
    for crate::UnityEngine::InputSystem::IInputInteraction_1<TValue>
{
    fn as_ref(&self) -> &quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+InputSystem+IInputInteraction_1")]
impl<TValue: quest_hook::libil2cpp::Type>
    AsMut<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>>
    for crate::UnityEngine::InputSystem::IInputInteraction_1<TValue>
{
    fn as_mut(&mut self) -> &mut quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject> {
        unsafe { std::mem::transmute(self) }
    }
}
