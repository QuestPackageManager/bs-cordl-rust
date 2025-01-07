#[cfg(feature = "System+Data+RBTree_1")]
#[repr(C)]
#[derive(Debug)]
pub struct RBTree_1<K: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _pageTable: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<
            quest_hook::libil2cpp::Gc<crate::System::Data::RBTree_1_TreePage<K>>,
        >,
    >,
    pub _pageTableMap: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<i32>,
    >,
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
unsafe impl<K: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::System::Data::RBTree_1<K> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Data";
    const CLASS_NAME: &'static str = "RBTree`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find("System.Data", "RBTree`1")
                    .unwrap()
                    .make_generic::<(K)>()
                    .unwrap()
                    .unwrap()
            })
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
        Ok(__cordl_ret.into())
    }
    pub fn AllocPage(
        &mut self,
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Data::RBTree_1_TreePage<K>>,
    >
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Data::RBTree_1_TreePage<K>,
        > = __cordl_object.invoke("AllocPage", (_cordl_size))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn CopyTo_Array0(
        &mut self,
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
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
        Ok(__cordl_ret.into())
    }
    pub fn CopyTo_Il2CppArray1(
        &mut self,
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<K>>,
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn FreePage(
        &mut self,
        page: quest_hook::libil2cpp::Gc<crate::System::Data::RBTree_1_TreePage<K>>,
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
        Ok(__cordl_ret.into())
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    >
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("GetEnumerator", ())?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn GetIntValueFromBitMap(bitMap: u32) -> quest_hook::libil2cpp::Result<i32>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetIntValueFromBitMap", (bitMap))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn MarkPageFree(
        &mut self,
        page: quest_hook::libil2cpp::Gc<crate::System::Data::RBTree_1_TreePage<K>>,
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
        Ok(__cordl_ret.into())
    }
    pub fn MarkPageFull(
        &mut self,
        page: quest_hook::libil2cpp::Gc<crate::System::Data::RBTree_1_TreePage<K>>,
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn New(
        accessMethod: crate::System::Data::TreeAccessMethod,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (accessMethod))?;
        Ok(__cordl_object.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
#[cfg(feature = "System+Data+RBTree_1")]
impl<K: quest_hook::libil2cpp::Type> AsRef<crate::System::Collections::IEnumerable>
for crate::System::Data::RBTree_1<K> {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Data+RBTree_1")]
impl<K: quest_hook::libil2cpp::Type> AsMut<crate::System::Collections::IEnumerable>
for crate::System::Data::RBTree_1<K> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Data+RBTree_1+Node")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
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
unsafe impl<K: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::System::Data::RBTree_1_Node<K> {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Data";
    const CLASS_NAME: &'static str = "Node";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find("System.Data", "Node")
                    .unwrap()
                    .make_generic::<(K)>()
                    .unwrap()
                    .unwrap()
            })
    }
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
unsafe impl<K: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Argument
for crate::System::Data::RBTree_1_Node<K> {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl<K: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Parameter
for crate::System::Data::RBTree_1_Node<K> {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
unsafe impl<K: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Returned
for crate::System::Data::RBTree_1_Node<K> {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
unsafe impl<K: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Return
for crate::System::Data::RBTree_1_Node<K> {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum RBTree_1_NodeColor {
    #[default]
    black = 1i32,
    red = 0i32,
}
#[cfg(feature = "System+Data+RBTree_1+NodeColor")]
unsafe impl quest_hook::libil2cpp::Type for crate::System::Data::RBTree_1_NodeColor {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Data";
    const CLASS_NAME: &'static str = "NodeColor";
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
unsafe impl quest_hook::libil2cpp::Argument for crate::System::Data::RBTree_1_NodeColor {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl quest_hook::libil2cpp::Parameter
for crate::System::Data::RBTree_1_NodeColor {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
unsafe impl quest_hook::libil2cpp::Returned for crate::System::Data::RBTree_1_NodeColor {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
unsafe impl quest_hook::libil2cpp::Return for crate::System::Data::RBTree_1_NodeColor {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "System+Data+RBTree_1+NodePath")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RBTree_1_NodePath<K: quest_hook::libil2cpp::Type> {
    pub _nodeID: i32,
    pub _mainTreeNodeID: i32,
    __cordl_phantom_K: std::marker::PhantomData<K>,
}
#[cfg(feature = "System+Data+RBTree_1+NodePath")]
unsafe impl<K: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::System::Data::RBTree_1_NodePath<K> {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Data";
    const CLASS_NAME: &'static str = "NodePath";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find("System.Data", "NodePath")
                    .unwrap()
                    .make_generic::<(K)>()
                    .unwrap()
                    .unwrap()
            })
    }
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
unsafe impl<K: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Argument
for crate::System::Data::RBTree_1_NodePath<K> {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl<K: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Parameter
for crate::System::Data::RBTree_1_NodePath<K> {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
unsafe impl<K: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Returned
for crate::System::Data::RBTree_1_NodePath<K> {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
unsafe impl<K: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Return
for crate::System::Data::RBTree_1_NodePath<K> {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Data+RBTree_1+RBTreeEnumerator")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct RBTree_1_RBTreeEnumerator<K: quest_hook::libil2cpp::Type> {
    pub _tree: quest_hook::libil2cpp::Gc<crate::System::Data::RBTree_1<K>>,
    pub _version: i32,
    pub _index: i32,
    pub _mainTreeNodeId: i32,
    pub _current: K,
    __cordl_phantom_K: std::marker::PhantomData<K>,
}
#[cfg(feature = "System+Data+RBTree_1+RBTreeEnumerator")]
unsafe impl<K: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::System::Data::RBTree_1_RBTreeEnumerator<K> {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "System.Data";
    const CLASS_NAME: &'static str = "RBTreeEnumerator";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "System.Data",
                        "RBTreeEnumerator",
                    )
                    .unwrap()
                    .make_generic::<(K)>()
                    .unwrap()
                    .unwrap()
            })
    }
    fn matches_value_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && ty
                .class()
                .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        !ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref()
            && <Self as quest_hook::libil2cpp::Type>::class()
                .is_assignable_from(ty.class())
    }
}
unsafe impl<K: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Argument
for crate::System::Data::RBTree_1_RBTreeEnumerator<K> {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
unsafe impl<K: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Parameter
for crate::System::Data::RBTree_1_RBTreeEnumerator<K> {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
}
unsafe impl<K: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Returned
for crate::System::Data::RBTree_1_RBTreeEnumerator<K> {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(
                quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()),
            )
        }
    }
}
unsafe impl<K: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Return
for crate::System::Data::RBTree_1_RBTreeEnumerator<K> {
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> Self::Actual {
        self
    }
    fn from_actual(actual: Self::Actual) -> Self {
        actual
    }
}
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerator_get_Current(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Collections.IEnumerator.get_Current",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_RBTree_1_0(
        &mut self,
        tree: quest_hook::libil2cpp::Gc<crate::System::Data::RBTree_1<K>>,
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
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_1(
        &mut self,
        tree: quest_hook::libil2cpp::Gc<crate::System::Data::RBTree_1<K>>,
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Data+RBTree_1+RBTreeEnumerator")]
impl<
    K: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IEnumerator_1<K>>
for crate::System::Data::RBTree_1_RBTreeEnumerator<K> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::IEnumerator_1<K> {
        todo!()
    }
}
#[cfg(feature = "System+Data+RBTree_1+RBTreeEnumerator")]
impl<
    K: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IEnumerator_1<K>>
for crate::System::Data::RBTree_1_RBTreeEnumerator<K> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::Generic::IEnumerator_1<K> {
        todo!()
    }
}
#[cfg(feature = "System+Data+RBTree_1+RBTreeEnumerator")]
impl<K: quest_hook::libil2cpp::Type> AsRef<crate::System::Collections::IEnumerator>
for crate::System::Data::RBTree_1_RBTreeEnumerator<K> {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerator {
        todo!()
    }
}
#[cfg(feature = "System+Data+RBTree_1+RBTreeEnumerator")]
impl<K: quest_hook::libil2cpp::Type> AsMut<crate::System::Collections::IEnumerator>
for crate::System::Data::RBTree_1_RBTreeEnumerator<K> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerator {
        todo!()
    }
}
#[cfg(feature = "System+Data+RBTree_1+RBTreeEnumerator")]
impl<K: quest_hook::libil2cpp::Type> AsRef<crate::System::IDisposable>
for crate::System::Data::RBTree_1_RBTreeEnumerator<K> {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "System+Data+RBTree_1+RBTreeEnumerator")]
impl<K: quest_hook::libil2cpp::Type> AsMut<crate::System::IDisposable>
for crate::System::Data::RBTree_1_RBTreeEnumerator<K> {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "System+Data+RBTree_1+TreePage")]
#[repr(C)]
#[derive(Debug)]
pub struct RBTree_1_TreePage<K: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _slots: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppArray<crate::System::Data::RBTree_1_Node<K>>,
    >,
    pub _slotMap: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    pub _inUseCount: i32,
    pub _pageId: i32,
    pub _nextFreeSlotLine: i32,
    __cordl_phantom_K: std::marker::PhantomData<K>,
}
#[cfg(feature = "System+Data+RBTree_1+TreePage")]
unsafe impl<K: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::System::Data::RBTree_1_TreePage<K> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Data";
    const CLASS_NAME: &'static str = "TreePage";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find("System.Data", "TreePage")
                    .unwrap()
                    .make_generic::<(K)>()
                    .unwrap()
                    .unwrap()
            })
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
        tree: quest_hook::libil2cpp::Gc<crate::System::Data::RBTree_1<K>>,
    ) -> quest_hook::libil2cpp::Result<i32>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("AllocSlot", (tree))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        _cordl_size: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        K: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_size))?;
        Ok(__cordl_object.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
