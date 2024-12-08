#[cfg(feature = "UnityEngine+ProBuilder+WingedEdgeEnumerator")]
#[repr(C)]
#[derive(Debug)]
pub struct WingedEdgeEnumerator {
    __cordl_parent: crate::System::Object,
    pub m_Start: *mut crate::UnityEngine::ProBuilder::WingedEdge,
    pub m_Current: *mut crate::UnityEngine::ProBuilder::WingedEdge,
}
#[cfg(feature = "UnityEngine+ProBuilder+WingedEdgeEnumerator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::WingedEdgeEnumerator =>
    "UnityEngine.ProBuilder"."WingedEdgeEnumerator"
);
#[cfg(feature = "UnityEngine+ProBuilder+WingedEdgeEnumerator")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::WingedEdgeEnumerator {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+WingedEdgeEnumerator")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::WingedEdgeEnumerator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+WingedEdgeEnumerator")]
impl crate::UnityEngine::ProBuilder::WingedEdgeEnumerator {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret)
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("MoveNext", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        start: *mut crate::UnityEngine::ProBuilder::WingedEdge,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (start))?;
        Ok(__cordl_object)
    }
    pub fn Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Reset", ())?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IEnumerator_get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
            .invoke("System.Collections.IEnumerator.get_Current", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        start: *mut crate::UnityEngine::ProBuilder::WingedEdge,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (start))?;
        Ok(__cordl_ret)
    }
    pub fn get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::ProBuilder::WingedEdge> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::ProBuilder::WingedEdge = __cordl_object
            .invoke("get_Current", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+WingedEdgeEnumerator")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::WingedEdgeEnumerator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}