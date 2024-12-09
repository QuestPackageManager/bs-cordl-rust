#[cfg(feature = "System+Data+RBTree_1")]
#[repr(C)]
#[derive(Debug)]
pub struct RBTree_1<K: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _pageTable: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::System::Data::RBTree_1_TreePage<K>,
    >,
    pub _pageTableMap: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub _inUsePageCount: i32,
    pub _nextFreePageLine: i32,
    pub root: i32,
    pub _version: i32,
    pub _inUseNodeCount: i32,
    pub _inUseSatelliteTreeCount: i32,
    pub _accessMethod: crate::System::Data::TreeAccessMethod,
    __cordl_phantom_K: std::marker::PhantomData<K>,
}
#[cfg(feature = "System+Data+RBTree_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::RBTree_1 < K > => "System.Data"
    ."RBTree`1" < K >
);
#[cfg(feature = "System+Data+RBTree_1")]
impl<K: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Data::RBTree_1<K> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+RBTree_1")]
impl<K: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Data::RBTree_1<K> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+RBTree_1")]
impl<K: quest_hook::libil2cpp::Type> crate::System::Data::RBTree_1<K> {
    #[cfg(feature = "System+Data+RBTree_1+Node")]
    pub type Node = crate::System::Data::RBTree_1_Node<K>;
    #[cfg(feature = "System+Data+RBTree_1+NodeColor")]
    pub type NodeColor = crate::System::Data::RBTree_1_NodeColor;
    #[cfg(feature = "System+Data+RBTree_1+NodePath")]
    pub type NodePath = crate::System::Data::RBTree_1_NodePath<K>;
    #[cfg(feature = "System+Data+RBTree_1+RBTreeEnumerator")]
    pub type RBTreeEnumerator = crate::System::Data::RBTree_1_RBTreeEnumerator<K>;
    #[cfg(feature = "System+Data+RBTree_1+TreePage")]
    pub type TreePage = crate::System::Data::RBTree_1_TreePage<K>;
    pub fn Add(&mut self, item: K) -> quest_hook::libil2cpp::Result<i32>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Add", (item))?;
        Ok(__cordl_ret)
    }
    pub fn AllocPage(
        &mut self,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Data::RBTree_1_TreePage<K>>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Data::RBTree_1_TreePage<K> = __cordl_object
            .invoke("AllocPage", (_cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn Clear(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret)
    }
    pub fn CompareNode(
        &mut self,
        record1: K,
        record2: K,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("CompareNode", (record1, record2))?;
        Ok(__cordl_ret)
    }
    pub fn CompareSateliteTreeNode(
        &mut self,
        record1: K,
        record2: K,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("CompareSateliteTreeNode", (record1, record2))?;
        Ok(__cordl_ret)
    }
    pub fn ComputeIndexByNode(
        &mut self,
        nodeId: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("ComputeIndexByNode", (nodeId))?;
        Ok(__cordl_ret)
    }
    pub fn ComputeIndexWithSatelliteByNode(
        &mut self,
        nodeId: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ComputeIndexWithSatelliteByNode", (nodeId))?;
        Ok(__cordl_ret)
    }
    pub fn ComputeNodeByIndex_ByRefMut0(
        &mut self,
        index: i32,
        satelliteRootId: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ComputeNodeByIndex", (index, satelliteRootId))?;
        Ok(__cordl_ret)
    }
    pub fn ComputeNodeByIndex_i32_1(
        &mut self,
        x_id: i32,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("ComputeNodeByIndex", (x_id, index))?;
        Ok(__cordl_ret)
    }
    pub fn CopyTo_Array0(
        &mut self,
        array: *mut crate::System::Array,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyTo", (array, index))?;
        Ok(__cordl_ret)
    }
    pub fn CopyTo_Il2CppArray1(
        &mut self,
        array: *mut quest_hook::libil2cpp::Il2CppArray<K>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyTo", (array, index))?;
        Ok(__cordl_ret)
    }
    pub fn DecreaseSize(
        &mut self,
        nodeId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DecreaseSize", (nodeId))?;
        Ok(__cordl_ret)
    }
    pub fn DeleteByIndex(&mut self, i: i32) -> quest_hook::libil2cpp::Result<K>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: K = __cordl_object.invoke("DeleteByIndex", (i))?;
        Ok(__cordl_ret)
    }
    pub fn FreeNode(
        &mut self,
        nodeId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FreeNode", (nodeId))?;
        Ok(__cordl_ret)
    }
    pub fn FreePage(
        &mut self,
        page: *mut crate::System::Data::RBTree_1_TreePage<K>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FreePage", (page))?;
        Ok(__cordl_ret)
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Collections::IEnumerator>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::IEnumerator = __cordl_object
            .invoke("GetEnumerator", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetIndexByKey(&mut self, key: K) -> quest_hook::libil2cpp::Result<i32>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetIndexByKey", (key))?;
        Ok(__cordl_ret)
    }
    pub fn GetIndexByNode(&mut self, node: i32) -> quest_hook::libil2cpp::Result<i32>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetIndexByNode", (node))?;
        Ok(__cordl_ret)
    }
    pub fn GetIndexByNodePath(
        &mut self,
        path: crate::System::Data::RBTree_1_NodePath<K>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetIndexByNodePath", (path))?;
        Ok(__cordl_ret)
    }
    pub fn GetIndexOfPageWithFreeSlot(
        &mut self,
        allocatedPage: bool,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("GetIndexOfPageWithFreeSlot", (allocatedPage))?;
        Ok(__cordl_ret)
    }
    pub fn GetNewNode(&mut self, key: K) -> quest_hook::libil2cpp::Result<i32>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetNewNode", (key))?;
        Ok(__cordl_ret)
    }
    pub fn GetNodeByIndex(
        &mut self,
        userIndex: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::RBTree_1_NodePath<K>>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Data::RBTree_1_NodePath<K> = __cordl_object
            .invoke("GetNodeByIndex", (userIndex))?;
        Ok(__cordl_ret)
    }
    pub fn GetNodeByKey(
        &mut self,
        key: K,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::RBTree_1_NodePath<K>>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Data::RBTree_1_NodePath<K> = __cordl_object
            .invoke("GetNodeByKey", (key))?;
        Ok(__cordl_ret)
    }
    pub fn IncreaseSize(
        &mut self,
        nodeId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("IncreaseSize", (nodeId))?;
        Ok(__cordl_ret)
    }
    pub fn IndexOf(&mut self, nodeId: i32, item: K) -> quest_hook::libil2cpp::Result<i32>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("IndexOf", (nodeId, item))?;
        Ok(__cordl_ret)
    }
    pub fn InitTree(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InitTree", ())?;
        Ok(__cordl_ret)
    }
    pub fn InsertAt(
        &mut self,
        position: i32,
        item: K,
        append: bool,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("InsertAt", (position, item, append))?;
        Ok(__cordl_ret)
    }
    pub fn Insert_K0(&mut self, item: K) -> quest_hook::libil2cpp::Result<i32>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Insert", (item))?;
        Ok(__cordl_ret)
    }
    pub fn Insert_i32_K1(
        &mut self,
        position: i32,
        item: K,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Insert", (position, item))?;
        Ok(__cordl_ret)
    }
    pub fn Key(&mut self, nodeId: i32) -> quest_hook::libil2cpp::Result<K>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: K = __cordl_object.invoke("Key", (nodeId))?;
        Ok(__cordl_ret)
    }
    pub fn Left(&mut self, nodeId: i32) -> quest_hook::libil2cpp::Result<i32>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Left", (nodeId))?;
        Ok(__cordl_ret)
    }
    pub fn LeftRotate(
        &mut self,
        root_id: i32,
        x_id: i32,
        mainTreeNode: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("LeftRotate", (root_id, x_id, mainTreeNode))?;
        Ok(__cordl_ret)
    }
    pub fn MarkPageFree(
        &mut self,
        page: *mut crate::System::Data::RBTree_1_TreePage<K>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkPageFree", (page))?;
        Ok(__cordl_ret)
    }
    pub fn MarkPageFull(
        &mut self,
        page: *mut crate::System::Data::RBTree_1_TreePage<K>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("MarkPageFull", (page))?;
        Ok(__cordl_ret)
    }
    pub fn Minimum(&mut self, x_id: i32) -> quest_hook::libil2cpp::Result<i32>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Minimum", (x_id))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        accessMethod: crate::System::Data::TreeAccessMethod,
    ) -> quest_hook::libil2cpp::Result<*mut Self>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (accessMethod))?;
        Ok(__cordl_object)
    }
    pub fn Next(&mut self, nodeId: i32) -> quest_hook::libil2cpp::Result<i32>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Next", (nodeId))?;
        Ok(__cordl_ret)
    }
    pub fn Parent(&mut self, nodeId: i32) -> quest_hook::libil2cpp::Result<i32>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Parent", (nodeId))?;
        Ok(__cordl_ret)
    }
    pub fn RBDelete(&mut self, z_id: i32) -> quest_hook::libil2cpp::Result<i32>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("RBDelete", (z_id))?;
        Ok(__cordl_ret)
    }
    pub fn RBDeleteFixup(
        &mut self,
        root_id: i32,
        x_id: i32,
        px_id: i32,
        mainTreeNodeID: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("RBDeleteFixup", (root_id, x_id, px_id, mainTreeNodeID))?;
        Ok(__cordl_ret)
    }
    pub fn RBDeleteX(
        &mut self,
        root_id: i32,
        z_id: i32,
        mainTreeNodeID: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("RBDeleteX", (root_id, z_id, mainTreeNodeID))?;
        Ok(__cordl_ret)
    }
    pub fn RBInsert(
        &mut self,
        root_id: i32,
        x_id: i32,
        mainTreeNodeID: i32,
        position: i32,
        append: bool,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("RBInsert", (root_id, x_id, mainTreeNodeID, position, append))?;
        Ok(__cordl_ret)
    }
    pub fn RecomputeSize(
        &mut self,
        nodeId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RecomputeSize", (nodeId))?;
        Ok(__cordl_ret)
    }
    pub fn RemoveAt(
        &mut self,
        position: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveAt", (position))?;
        Ok(__cordl_ret)
    }
    pub fn Right(&mut self, nodeId: i32) -> quest_hook::libil2cpp::Result<i32>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Right", (nodeId))?;
        Ok(__cordl_ret)
    }
    pub fn RightRotate(
        &mut self,
        root_id: i32,
        x_id: i32,
        mainTreeNode: i32,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object
            .invoke("RightRotate", (root_id, x_id, mainTreeNode))?;
        Ok(__cordl_ret)
    }
    pub fn SearchSubTree(
        &mut self,
        root_id: i32,
        key: K,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("SearchSubTree", (root_id, key))?;
        Ok(__cordl_ret)
    }
    pub fn SetColor(
        &mut self,
        nodeId: i32,
        color: crate::System::Data::RBTree_1_NodeColor<K>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetColor", (nodeId, color))?;
        Ok(__cordl_ret)
    }
    pub fn SetKey(
        &mut self,
        nodeId: i32,
        key: K,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetKey", (nodeId, key))?;
        Ok(__cordl_ret)
    }
    pub fn SetLeft(
        &mut self,
        nodeId: i32,
        leftNodeId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetLeft", (nodeId, leftNodeId))?;
        Ok(__cordl_ret)
    }
    pub fn SetNext(
        &mut self,
        nodeId: i32,
        nextNodeId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetNext", (nodeId, nextNodeId))?;
        Ok(__cordl_ret)
    }
    pub fn SetParent(
        &mut self,
        nodeId: i32,
        parentNodeId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetParent", (nodeId, parentNodeId))?;
        Ok(__cordl_ret)
    }
    pub fn SetRight(
        &mut self,
        nodeId: i32,
        rightNodeId: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetRight", (nodeId, rightNodeId))?;
        Ok(__cordl_ret)
    }
    pub fn SetSubTreeSize(
        &mut self,
        nodeId: i32,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetSubTreeSize", (nodeId, _cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn SubTreeSize(&mut self, nodeId: i32) -> quest_hook::libil2cpp::Result<i32>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("SubTreeSize", (nodeId))?;
        Ok(__cordl_ret)
    }
    pub fn Successor_ByRefMut_ByRefMut1(
        &mut self,
        nodeId: quest_hook::libil2cpp::ByRefMut<i32>,
        mainTreeNodeId: quest_hook::libil2cpp::ByRefMut<i32>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("Successor", (nodeId, mainTreeNodeId))?;
        Ok(__cordl_ret)
    }
    pub fn Successor_i32_0(&mut self, x_id: i32) -> quest_hook::libil2cpp::Result<i32>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("Successor", (x_id))?;
        Ok(__cordl_ret)
    }
    pub fn UpdateNodeKey(
        &mut self,
        currentKey: K,
        newKey: K,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdateNodeKey", (currentKey, newKey))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        accessMethod: crate::System::Data::TreeAccessMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (accessMethod))?;
        Ok(__cordl_ret)
    }
    pub fn color(
        &mut self,
        nodeId: i32,
    ) -> quest_hook::libil2cpp::Result<crate::System::Data::RBTree_1_NodeColor<K>>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Data::RBTree_1_NodeColor<K> = __cordl_object
            .invoke("color", (nodeId))?;
        Ok(__cordl_ret)
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_HasDuplicates(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_HasDuplicates", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Item(&mut self, index: i32) -> quest_hook::libil2cpp::Result<K>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: K = __cordl_object.invoke("get_Item", (index))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Data+RBTree_1")]
impl<K: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Data::RBTree_1<K> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Data+RBTree_1+Node")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct RBTree_1_Node<K: quest_hook::libil2cpp::Type> {
    pub _selfId: i32,
    pub _leftId: i32,
    pub _rightId: i32,
    pub _parentId: i32,
    pub _nextId: i32,
    pub _subTreeSize: i32,
    pub _keyOfNode: K,
    pub _nodeColor: crate::System::Data::RBTree_1_NodeColor<K>,
    __cordl_phantom_K: std::marker::PhantomData<K>,
}
#[cfg(feature = "System+Data+RBTree_1+Node")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Data::RBTree_1_Node < K > =>
    "System.Data"."RBTree`1/Node<K>" < K >
);
#[cfg(feature = "System+Data+RBTree_1+Node")]
unsafe impl<K: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::System::Data::RBTree_1_Node<K> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Data+RBTree_1+Node")]
impl<K: quest_hook::libil2cpp::Type> crate::System::Data::RBTree_1_Node<K> {}
#[cfg(feature = "System+Data+RBTree_1+NodeColor")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RBTree_1_NodeColor {
    black = 1i32,
    red = 0i32,
}
#[cfg(feature = "System+Data+RBTree_1+NodeColor")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Data::RBTree_1_NodeColor =>
    "System.Data"."RBTree`1/NodeColor<K>"
);
#[cfg(feature = "System+Data+RBTree_1+NodePath")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct RBTree_1_NodePath<K: quest_hook::libil2cpp::Type> {
    pub _nodeID: i32,
    pub _mainTreeNodeID: i32,
    __cordl_phantom_K: std::marker::PhantomData<K>,
}
#[cfg(feature = "System+Data+RBTree_1+NodePath")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Data::RBTree_1_NodePath < K > =>
    "System.Data"."RBTree`1/NodePath<K>" < K >
);
#[cfg(feature = "System+Data+RBTree_1+NodePath")]
unsafe impl<K: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::System::Data::RBTree_1_NodePath<K> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Data+RBTree_1+NodePath")]
impl<K: quest_hook::libil2cpp::Type> crate::System::Data::RBTree_1_NodePath<K> {
    pub fn _ctor(
        &mut self,
        nodeID: i32,
        mainTreeNodeID: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (nodeID, mainTreeNodeID),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Data+RBTree_1+RBTreeEnumerator")]
#[repr(C)]
#[derive(Debug, Clone)]
pub struct RBTree_1_RBTreeEnumerator<K: quest_hook::libil2cpp::Type> {
    pub _tree: *mut crate::System::Data::RBTree_1<K>,
    pub _version: i32,
    pub _index: i32,
    pub _mainTreeNodeId: i32,
    pub _current: K,
    __cordl_phantom_K: std::marker::PhantomData<K>,
}
#[cfg(feature = "System+Data+RBTree_1+RBTreeEnumerator")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::System::Data::RBTree_1_RBTreeEnumerator < K > =>
    "System.Data"."RBTree`1/RBTreeEnumerator<K>" < K >
);
#[cfg(feature = "System+Data+RBTree_1+RBTreeEnumerator")]
unsafe impl<K: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::System::Data::RBTree_1_RBTreeEnumerator<K> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Data+RBTree_1+RBTreeEnumerator")]
impl<K: quest_hook::libil2cpp::Type> crate::System::Data::RBTree_1_RBTreeEnumerator<K> {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "Dispose",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn MoveNext(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "MoveNext",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IEnumerator_Reset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.IEnumerator.Reset",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn System_Collections_IEnumerator_get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.IEnumerator.get_Current",
            (),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_RBTree_1_0(
        &mut self,
        tree: *mut crate::System::Data::RBTree_1<K>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (tree),
        )?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_1(
        &mut self,
        tree: *mut crate::System::Data::RBTree_1<K>,
        position: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (tree, position),
        )?;
        Ok(__cordl_ret)
    }
    pub fn get_Current(&mut self) -> quest_hook::libil2cpp::Result<K>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: K = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Current",
            (),
        )?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Data+RBTree_1+TreePage")]
#[repr(C)]
#[derive(Debug)]
pub struct RBTree_1_TreePage<K: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _slots: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::System::Data::RBTree_1_Node<K>,
    >,
    pub _slotMap: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
    pub _inUseCount: i32,
    pub _pageId: i32,
    pub _nextFreeSlotLine: i32,
    __cordl_phantom_K: std::marker::PhantomData<K>,
}
#[cfg(feature = "System+Data+RBTree_1+TreePage")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Data::RBTree_1_TreePage < K > =>
    "System.Data"."RBTree`1/TreePage" < K >
);
#[cfg(feature = "System+Data+RBTree_1+TreePage")]
impl<K: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Data::RBTree_1_TreePage<K> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+RBTree_1+TreePage")]
impl<K: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Data::RBTree_1_TreePage<K> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Data+RBTree_1+TreePage")]
impl<K: quest_hook::libil2cpp::Type> crate::System::Data::RBTree_1_TreePage<K> {
    pub fn AllocSlot(
        &mut self,
        tree: *mut crate::System::Data::RBTree_1<K>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("AllocSlot", (tree))?;
        Ok(__cordl_ret)
    }
    pub fn New(_cordl_size: i32) -> quest_hook::libil2cpp::Result<*mut Self>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_size))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_size))?;
        Ok(__cordl_ret)
    }
    pub fn get_InUseCount(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_InUseCount", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PageId(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_PageId", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_InUseCount(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_InUseCount", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_PageId(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_PageId", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Data+RBTree_1+TreePage")]
impl<K: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Data::RBTree_1_TreePage<K> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
