#[cfg(feature = "UnityEngine+UIElements+UIR+DetachedAllocator")]
#[repr(C)]
#[derive(Debug)]
pub struct DetachedAllocator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_VertsPool: *mut crate::UnityEngine::UIElements::UIR::TempAllocator_1<
        crate::UnityEngine::UIElements::Vertex,
    >,
    pub m_IndexPool: *mut crate::UnityEngine::UIElements::UIR::TempAllocator_1<u16>,
    pub m_MeshWriteDataPool: *mut crate::System::Collections::Generic::List_1<
        *mut crate::UnityEngine::UIElements::MeshWriteData,
    >,
    pub m_MeshWriteDataCount: i32,
    pub m_Disposed: bool,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+DetachedAllocator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIR::DetachedAllocator
    => "UnityEngine.UIElements.UIR"."DetachedAllocator"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+DetachedAllocator")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIR::DetachedAllocator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+DetachedAllocator")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIR::DetachedAllocator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+DetachedAllocator")]
impl crate::UnityEngine::UIElements::UIR::DetachedAllocator {
    pub fn Dispose_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn Dispose__cordl_bool1(
        &mut self,
        disposing: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", (disposing))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+DetachedAllocator")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIR::DetachedAllocator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
