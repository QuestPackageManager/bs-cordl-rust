#[cfg(feature = "UnityEngine+ObjectDispatcher")]
#[repr(C)]
#[derive(Debug)]
pub struct ObjectDispatcher {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub m_Ptr: crate::System::IntPtr,
}
#[cfg(feature = "UnityEngine+ObjectDispatcher")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ObjectDispatcher => "UnityEngine"
    ."ObjectDispatcher"
);
#[cfg(feature = "UnityEngine+ObjectDispatcher")]
impl std::ops::Deref for crate::UnityEngine::ObjectDispatcher {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ObjectDispatcher")]
impl std::ops::DerefMut for crate::UnityEngine::ObjectDispatcher {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ObjectDispatcher")]
impl crate::UnityEngine::ObjectDispatcher {}
#[cfg(feature = "UnityEngine+ObjectDispatcher")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ObjectDispatcher {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
