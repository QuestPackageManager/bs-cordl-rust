#[cfg(feature = "UnityEngine+UIElements+UIR+GPUBufferAllocator")]
#[repr(C)]
#[derive(Debug)]
pub struct GPUBufferAllocator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Low: *mut crate::UnityEngine::UIElements::UIR::BestFitAllocator,
    pub m_High: *mut crate::UnityEngine::UIElements::UIR::BestFitAllocator,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+GPUBufferAllocator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIR::GPUBufferAllocator
    => "UnityEngine.UIElements.UIR"."GPUBufferAllocator"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+GPUBufferAllocator")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIR::GPUBufferAllocator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+GPUBufferAllocator")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIR::GPUBufferAllocator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+GPUBufferAllocator")]
impl crate::UnityEngine::UIElements::UIR::GPUBufferAllocator {
    pub fn Allocate(
        &mut self,
        _cordl_size: u32,
        shortLived: bool,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::UIR::Alloc> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::UIR::Alloc = __cordl_object
            .invoke("Allocate", (_cordl_size, shortLived))?;
        Ok(__cordl_ret)
    }
    pub fn Free(
        &mut self,
        alloc: crate::UnityEngine::UIElements::UIR::Alloc,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Free", (alloc))?;
        Ok(__cordl_ret)
    }
    pub fn HighLowCollide(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HighLowCollide", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(maxSize: u32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (maxSize))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        maxSize: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (maxSize))?;
        Ok(__cordl_ret)
    }
    pub fn get_isEmpty(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_isEmpty", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+GPUBufferAllocator")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIR::GPUBufferAllocator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
