#[cfg(feature = "UnityEngine+UIElements+UIRAtlasAllocator")]
#[repr(C)]
#[derive(Debug)]
pub struct UIRAtlasAllocator {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _maxAtlasSize_k__BackingField: i32,
    pub _maxImageWidth_k__BackingField: i32,
    pub _maxImageHeight_k__BackingField: i32,
    pub _virtualWidth_k__BackingField: i32,
    pub _virtualHeight_k__BackingField: i32,
    pub _physicalWidth_k__BackingField: i32,
    pub _physicalHeight_k__BackingField: i32,
    pub m_FirstUnpartitionedArea: *mut crate::UnityEngine::UIElements::UIRAtlasAllocator_AreaNode,
    pub m_OpenRows: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::UIElements::UIRAtlasAllocator_Row,
    >,
    pub m_1SidePadding: i32,
    pub m_2SidePadding: i32,
    pub _disposed_k__BackingField: bool,
}
#[cfg(feature = "UnityEngine+UIElements+UIRAtlasAllocator")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIRAtlasAllocator =>
    "UnityEngine.UIElements"."UIRAtlasAllocator"
);
#[cfg(feature = "UnityEngine+UIElements+UIRAtlasAllocator")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIRAtlasAllocator {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIRAtlasAllocator")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIRAtlasAllocator {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIRAtlasAllocator")]
impl crate::UnityEngine::UIElements::UIRAtlasAllocator {
    #[cfg(feature = "UnityEngine+UIElements+UIRAtlasAllocator+AreaNode")]
    pub type AreaNode = crate::UnityEngine::UIElements::UIRAtlasAllocator_AreaNode;
    #[cfg(feature = "UnityEngine+UIElements+UIRAtlasAllocator+Row")]
    pub type Row = crate::UnityEngine::UIElements::UIRAtlasAllocator_Row;
    pub fn BuildAreas(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("BuildAreas", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Dispose_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Dispose", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn New(
        initialAtlasSize: i32,
        maxAtlasSize: i32,
        sidePadding: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (initialAtlasSize, maxAtlasSize, sidePadding))?;
        Ok(__cordl_object.into())
    }
    pub fn TryAllocate(
        &mut self,
        width: i32,
        height: i32,
        location: quest_hook::libil2cpp::ByRefMut<crate::UnityEngine::RectInt>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryAllocate", (width, height, location))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryPartitionArea(
        &mut self,
        areaNode: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIRAtlasAllocator_AreaNode,
        >,
        rowIndex: i32,
        rowHeight: i32,
        minWidth: i32,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryPartitionArea", (areaNode, rowIndex, rowHeight, minWidth))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        initialAtlasSize: i32,
        maxAtlasSize: i32,
        sidePadding: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (initialAtlasSize, maxAtlasSize, sidePadding))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_disposed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_disposed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maxAtlasSize(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_maxAtlasSize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maxImageHeight(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_maxImageHeight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_maxImageWidth(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_maxImageWidth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_physicalHeight(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_physicalHeight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_physicalWidth(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_physicalWidth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_virtualHeight(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_virtualHeight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_virtualWidth(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_virtualWidth", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_disposed(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_disposed", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_physicalHeight(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_physicalHeight", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_physicalWidth(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_physicalWidth", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_virtualHeight(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_virtualHeight", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_virtualWidth(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_virtualWidth", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIRAtlasAllocator")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIRAtlasAllocator {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIRAtlasAllocator")]
impl AsRef<crate::System::IDisposable>
for crate::UnityEngine::UIElements::UIRAtlasAllocator {
    fn as_ref(&self) -> &crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIRAtlasAllocator")]
impl AsMut<crate::System::IDisposable>
for crate::UnityEngine::UIElements::UIRAtlasAllocator {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIRAtlasAllocator+AreaNode")]
#[repr(C)]
#[derive(Debug)]
pub struct UIRAtlasAllocator_AreaNode {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub rect: crate::UnityEngine::RectInt,
    pub previous: *mut crate::UnityEngine::UIElements::UIRAtlasAllocator_AreaNode,
    pub next: *mut crate::UnityEngine::UIElements::UIRAtlasAllocator_AreaNode,
}
#[cfg(feature = "UnityEngine+UIElements+UIRAtlasAllocator+AreaNode")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::UIElements::UIRAtlasAllocator_AreaNode => "UnityEngine.UIElements"
    ."UIRAtlasAllocator/AreaNode"
);
#[cfg(feature = "UnityEngine+UIElements+UIRAtlasAllocator+AreaNode")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIRAtlasAllocator_AreaNode {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIRAtlasAllocator+AreaNode")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIRAtlasAllocator_AreaNode {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIRAtlasAllocator+AreaNode")]
impl crate::UnityEngine::UIElements::UIRAtlasAllocator_AreaNode {
    #[cfg(feature = "UnityEngine+UIElements+UIRAtlasAllocator+AreaNode+__c")]
    pub type __c = crate::UnityEngine::UIElements::AreaNode_UIRAtlasAllocator___c;
    pub fn AddAfter(
        &mut self,
        previous: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::UIElements::UIRAtlasAllocator_AreaNode,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddAfter", (previous))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Release(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveFromChain(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveFromChain", ())?;
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
#[cfg(feature = "UnityEngine+UIElements+UIRAtlasAllocator+AreaNode")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIRAtlasAllocator_AreaNode {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIRAtlasAllocator+Row")]
#[repr(C)]
#[derive(Debug)]
pub struct UIRAtlasAllocator_Row {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _offsetX_k__BackingField: i32,
    pub _offsetY_k__BackingField: i32,
    pub _width_k__BackingField: i32,
    pub _height_k__BackingField: i32,
    pub Cursor: i32,
}
#[cfg(feature = "UnityEngine+UIElements+UIRAtlasAllocator+Row")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::UIElements::UIRAtlasAllocator_Row
    => "UnityEngine.UIElements"."UIRAtlasAllocator/Row"
);
#[cfg(feature = "UnityEngine+UIElements+UIRAtlasAllocator+Row")]
impl std::ops::Deref for crate::UnityEngine::UIElements::UIRAtlasAllocator_Row {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIRAtlasAllocator+Row")]
impl std::ops::DerefMut for crate::UnityEngine::UIElements::UIRAtlasAllocator_Row {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIRAtlasAllocator+Row")]
impl crate::UnityEngine::UIElements::UIRAtlasAllocator_Row {
    #[cfg(feature = "UnityEngine+UIElements+UIRAtlasAllocator+Row+__c")]
    pub type __c = crate::UnityEngine::UIElements::Row_UIRAtlasAllocator___c;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn Release(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", ())?;
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
    pub fn get_offsetX(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_offsetX", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_offsetY(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_offsetY", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_width(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_width", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_height(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_height", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_offsetX(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_offsetX", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_offsetY(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_offsetY", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_width(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_width", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+UIElements+UIRAtlasAllocator+Row")]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::UIElements::UIRAtlasAllocator_Row {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
