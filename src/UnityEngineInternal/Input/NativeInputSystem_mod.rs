#[cfg(feature = "UnityEngineInternal+Input+NativeInputSystem")]
#[repr(C)]
#[derive(Debug)]
pub struct NativeInputSystem {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "UnityEngineInternal+Input+NativeInputSystem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngineInternal::Input::NativeInputSystem =>
    "UnityEngineInternal.Input"."NativeInputSystem"
);
#[cfg(feature = "UnityEngineInternal+Input+NativeInputSystem")]
impl std::ops::Deref for crate::UnityEngineInternal::Input::NativeInputSystem {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngineInternal+Input+NativeInputSystem")]
impl std::ops::DerefMut for crate::UnityEngineInternal::Input::NativeInputSystem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngineInternal+Input+NativeInputSystem")]
impl crate::UnityEngineInternal::Input::NativeInputSystem {}
#[cfg(feature = "UnityEngineInternal+Input+NativeInputSystem")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngineInternal::Input::NativeInputSystem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
