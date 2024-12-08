#[cfg(feature = "Zenject+SignalExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct SignalExtensions {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Zenject+SignalExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::SignalExtensions => "Zenject"
    ."SignalExtensions"
);
#[cfg(feature = "Zenject+SignalExtensions")]
impl std::ops::Deref for crate::Zenject::SignalExtensions {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SignalExtensions")]
impl std::ops::DerefMut for crate::Zenject::SignalExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SignalExtensions")]
impl crate::Zenject::SignalExtensions {}
#[cfg(feature = "Zenject+SignalExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::SignalExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
