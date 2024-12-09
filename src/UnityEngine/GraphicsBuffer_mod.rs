#[cfg(feature = "UnityEngine+GraphicsBuffer")]
#[repr(C)]
#[derive(Debug)]
pub struct GraphicsBuffer {
    __cordl_parent: crate::System::Object,
    pub m_Ptr: crate::System::IntPtr,
}
#[cfg(feature = "UnityEngine+GraphicsBuffer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::GraphicsBuffer => "UnityEngine"
    ."GraphicsBuffer"
);
#[cfg(feature = "UnityEngine+GraphicsBuffer")]
impl std::ops::Deref for crate::UnityEngine::GraphicsBuffer {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GraphicsBuffer")]
impl std::ops::DerefMut for crate::UnityEngine::GraphicsBuffer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+GraphicsBuffer")]
impl crate::UnityEngine::GraphicsBuffer {
    #[cfg(feature = "UnityEngine+GraphicsBuffer+Target")]
    pub type TargetType = crate::UnityEngine::GraphicsBuffer_Target;
}
#[cfg(feature = "UnityEngine+GraphicsBuffer")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::GraphicsBuffer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+GraphicsBuffer+Target")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphicsBuffer_Target {
    Append = 64i32,
    Constant = 512i32,
    CopyDestination = 8i32,
    CopySource = 4i32,
    Counter = 128i32,
    Index = 2i32,
    IndirectArguments = 256i32,
    Raw = 32i32,
    Structured = 16i32,
    Vertex = 1i32,
}
#[cfg(feature = "UnityEngine+GraphicsBuffer+Target")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::GraphicsBuffer_Target =>
    "UnityEngine"."GraphicsBuffer/Target"
);
