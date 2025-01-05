#[cfg(feature = "UnityEngine+ComputeBuffer")]
#[repr(C)]
#[derive(Debug)]
pub struct ComputeBuffer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Ptr: crate::System::IntPtr,
}
#[cfg(feature = "UnityEngine+ComputeBuffer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ComputeBuffer => "UnityEngine"
    ."ComputeBuffer"
);
#[cfg(feature = "UnityEngine+ComputeBuffer")]
impl std::ops::Deref for crate::UnityEngine::ComputeBuffer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ComputeBuffer")]
impl std::ops::DerefMut for crate::UnityEngine::ComputeBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ComputeBuffer")]
impl crate::UnityEngine::ComputeBuffer {}
#[cfg(feature = "UnityEngine+ComputeBuffer")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ComputeBuffer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
