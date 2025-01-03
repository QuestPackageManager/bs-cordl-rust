#[cfg(feature = "UnityEngine+UIElements+UIR+BestFitAllocator")]
#[repr(C)]
#[derive(Debug)]
pub struct BestFitAllocator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _totalSize_k__BackingField: u32,
    pub m_FirstBlock: *mut crate::UnityEngine::UIElements::UIR::BestFitAllocator_Block,
    pub m_FirstAvailableBlock: *mut crate::UnityEngine::UIElements::UIR::BestFitAllocator_Block,
    pub m_BlockPool: *mut crate::UnityEngine::UIElements::UIR::BestFitAllocator_BlockPool,
    pub m_HighWatermark: u32,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+BestFitAllocator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIR::BestFitAllocator
    => "UnityEngine.UIElements.UIR"."BestFitAllocator"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+BestFitAllocator")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIR::BestFitAllocator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+BestFitAllocator")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIR::BestFitAllocator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+BestFitAllocator")]
impl crate::UnityEngine::UIElements::UIR::BestFitAllocator {
    #[cfg(feature = "UnityEngine+UIElements+UIR+BestFitAllocator+Block")]
    pub type Block = crate::UnityEngine::UIElements::UIR::BestFitAllocator_Block;
    #[cfg(feature = "UnityEngine+UIElements+UIR+BestFitAllocator+BlockPool")]
    pub type BlockPool = crate::UnityEngine::UIElements::UIR::BestFitAllocator_BlockPool;
    pub fn Allocate(
        &mut self,
        _cordl_size: u32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::UIElements::UIR::Alloc> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::UIElements::UIR::Alloc = __cordl_object
            .invoke("Allocate", (_cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn BestFitFindAvailableBlock(
        &mut self,
        _cordl_size: u32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::BestFitAllocator_Block,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::BestFitAllocator_Block,
        > = __cordl_object.invoke("BestFitFindAvailableBlock", (_cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn CoalesceBlockWithPrevious(
        &mut self,
        block: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::BestFitAllocator_Block,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::BestFitAllocator_Block,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::BestFitAllocator_Block,
        > = __cordl_object.invoke("CoalesceBlockWithPrevious", (block))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn New(
        _cordl_size: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_size))?;
        Ok(__cordl_object.into())
    }
    pub fn SplitBlock(
        &mut self,
        block: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::BestFitAllocator_Block,
        >,
        _cordl_size: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SplitBlock", (block, _cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_size: u32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_size))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_highWatermark(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("get_highWatermark", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_totalSize(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("get_totalSize", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+BestFitAllocator")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIR::BestFitAllocator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+BestFitAllocator+Block")]
#[repr(C)]
#[derive(Debug)]
pub struct BestFitAllocator_Block {
    __cordl_parent: crate::UnityEngine::UIElements::UIR::LinkedPoolItem_1<
        *mut crate::UnityEngine::UIElements::UIR::BestFitAllocator_Block,
    >,
    pub start: u32,
    pub end: u32,
    pub prev: *mut crate::UnityEngine::UIElements::UIR::BestFitAllocator_Block,
    pub next: *mut crate::UnityEngine::UIElements::UIR::BestFitAllocator_Block,
    pub prevAvailable: *mut crate::UnityEngine::UIElements::UIR::BestFitAllocator_Block,
    pub nextAvailable: *mut crate::UnityEngine::UIElements::UIR::BestFitAllocator_Block,
    pub allocated: bool,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+BestFitAllocator+Block")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIR::BestFitAllocator_Block =>
    "UnityEngine.UIElements.UIR"."BestFitAllocator/Block"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+BestFitAllocator+Block")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIR::BestFitAllocator_Block {
    type Target = crate::UnityEngine::UIElements::UIR::LinkedPoolItem_1<
        *mut crate::UnityEngine::UIElements::UIR::BestFitAllocator_Block,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+BestFitAllocator+Block")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIR::BestFitAllocator_Block {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+BestFitAllocator+Block")]
impl crate::UnityEngine::UIElements::UIR::BestFitAllocator_Block {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_size(&mut self) -> quest_hook::libil2cpp::Result<u32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: u32 = __cordl_object.invoke("get_size", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+BestFitAllocator+Block")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIR::BestFitAllocator_Block {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+BestFitAllocator+BlockPool")]
#[repr(C)]
#[derive(Debug)]
pub struct BestFitAllocator_BlockPool {
    __cordl_parent: crate::UnityEngine::UIElements::UIR::LinkedPool_1<
        *mut crate::UnityEngine::UIElements::UIR::BestFitAllocator_Block,
    >,
}
#[cfg(feature = "UnityEngine+UIElements+UIR+BestFitAllocator+BlockPool")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIR::BestFitAllocator_BlockPool =>
    "UnityEngine.UIElements.UIR"."BestFitAllocator/BlockPool"
);
#[cfg(feature = "UnityEngine+UIElements+UIR+BestFitAllocator+BlockPool")]
impl std::ops::Deref
for crate::UnityEngine::UIElements::UIR::BestFitAllocator_BlockPool {
    type Target = crate::UnityEngine::UIElements::UIR::LinkedPool_1<
        *mut crate::UnityEngine::UIElements::UIR::BestFitAllocator_Block,
    >;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+BestFitAllocator+BlockPool")]
impl std::ops::DerefMut
for crate::UnityEngine::UIElements::UIR::BestFitAllocator_BlockPool {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+BestFitAllocator+BlockPool")]
impl crate::UnityEngine::UIElements::UIR::BestFitAllocator_BlockPool {
    pub fn CreateBlock() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::BestFitAllocator_Block,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::BestFitAllocator_Block,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("CreateBlock", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn ResetBlock(
        block: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIR::BestFitAllocator_Block,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ResetBlock", (block))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIR+BestFitAllocator+BlockPool")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIR::BestFitAllocator_BlockPool {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
