#[cfg(feature = "GameplayServerFiniteStateMachineFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct GameplayServerFiniteStateMachineFactory {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "GameplayServerFiniteStateMachineFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::GameplayServerFiniteStateMachineFactory => ""
    ."GameplayServerFiniteStateMachineFactory"
);
#[cfg(feature = "GameplayServerFiniteStateMachineFactory")]
impl std::ops::Deref
for crate::GlobalNamespace::GameplayServerFiniteStateMachineFactory {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayServerFiniteStateMachineFactory")]
impl std::ops::DerefMut
for crate::GlobalNamespace::GameplayServerFiniteStateMachineFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "GameplayServerFiniteStateMachineFactory")]
impl crate::GlobalNamespace::GameplayServerFiniteStateMachineFactory {}
#[cfg(feature = "GameplayServerFiniteStateMachineFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::GameplayServerFiniteStateMachineFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
