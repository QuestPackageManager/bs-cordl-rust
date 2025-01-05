#[cfg(feature = "UnityEngine+ProBuilder+KdTree+KdTree_2")]
#[repr(C)]
#[derive(Debug)]
pub struct KdTree_2<
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub dimensions: i32,
    pub typeMath: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ProBuilder::KdTree::ITypeMath_1<TKey>,
    >,
    pub root: quest_hook::libil2cpp::Gc<
        crate::UnityEngine::ProBuilder::KdTree::KdTreeNode_2<TKey, TValue>,
    >,
    pub _AddDuplicateBehavior_k__BackingField: crate::UnityEngine::ProBuilder::KdTree::AddDuplicateBehavior,
    pub _Count_k__BackingField: i32,
    __cordl_phantom_TKey: std::marker::PhantomData<TKey>,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+KdTree_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::KdTree::KdTree_2 <
    TKey, TValue > => "UnityEngine.ProBuilder.KdTree"."KdTree`2" < TKey, TValue >
);
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+KdTree_2")]
impl<
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> std::ops::Deref for crate::UnityEngine::ProBuilder::KdTree::KdTree_2<TKey, TValue> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+KdTree_2")]
impl<
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> std::ops::DerefMut for crate::UnityEngine::ProBuilder::KdTree::KdTree_2<TKey, TValue> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+KdTree_2")]
impl<
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> crate::UnityEngine::ProBuilder::KdTree::KdTree_2<TKey, TValue> {
    pub fn Add(
        &mut self,
        point: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TKey>>,
        value: TValue,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Add", (point, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddNearestNeighbours(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::KdTree::KdTreeNode_2<TKey, TValue>,
        >,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TKey>>,
        rect: crate::UnityEngine::ProBuilder::KdTree::HyperRect_1<TKey>,
        depth: i32,
        nearestNeighbours: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::KdTree::NearestNeighbourList_2<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::KdTree::KdTreeNode_2<TKey, TValue>,
                >,
                TKey,
            >,
        >,
        maxSearchRadiusSquared: TKey,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "AddNearestNeighbours",
                (node, target, rect, depth, nearestNeighbours, maxSearchRadiusSquared),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AddNodeToStringBuilder(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::KdTree::KdTreeNode_2<TKey, TValue>,
        >,
        sb: quest_hook::libil2cpp::Gc<crate::System::Text::StringBuilder>,
        depth: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddNodeToStringBuilder", (node, sb, depth))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddNodesBalanced(
        &mut self,
        nodes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::UnityEngine::ProBuilder::KdTree::KdTreeNode_2<TKey, TValue>,
            >,
        >,
        byDimension: i32,
        fromIndex: i32,
        toIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddNodesBalanced", (nodes, byDimension, fromIndex, toIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddNodesToList(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::KdTree::KdTreeNode_2<TKey, TValue>,
        >,
        nodes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::KdTree::KdTreeNode_2<TKey, TValue>,
                >,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddNodesToList", (node, nodes))?;
        Ok(__cordl_ret.into())
    }
    pub fn Balance(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Balance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Clear(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FindValue(
        &mut self,
        value: TValue,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TKey>>,
    >
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<TKey>,
        > = __cordl_object.invoke("FindValue", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn FindValueAt(
        &mut self,
        point: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TKey>>,
    ) -> quest_hook::libil2cpp::Result<TValue>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: TValue = __cordl_object.invoke("FindValueAt", (point))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::KdTree::KdTreeNode_2<TKey, TValue>,
                >,
            >,
        >,
    >
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<
                quest_hook::libil2cpp::Gc<
                    crate::UnityEngine::ProBuilder::KdTree::KdTreeNode_2<TKey, TValue>,
                >,
            >,
        > = __cordl_object.invoke("GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetNearestNeighbours(
        &mut self,
        point: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TKey>>,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::UnityEngine::ProBuilder::KdTree::KdTreeNode_2<TKey, TValue>,
            >,
        >,
    >
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::UnityEngine::ProBuilder::KdTree::KdTreeNode_2<TKey, TValue>,
            >,
        > = __cordl_object.invoke("GetNearestNeighbours", (point, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn LoadFromFile(
        filename: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::KdTree::KdTree_2<TKey, TValue>,
        >,
    >
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::KdTree::KdTree_2<TKey, TValue>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LoadFromFile", (filename))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_AddDuplicateBehavior1(
        dimensions: i32,
        typeMath: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::KdTree::ITypeMath_1<TKey>,
        >,
        addDuplicateBehavior: crate::UnityEngine::ProBuilder::KdTree::AddDuplicateBehavior,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dimensions, typeMath, addDuplicateBehavior))?;
        Ok(__cordl_object.into())
    }
    pub fn New_i32_ITypeMath_1_0(
        dimensions: i32,
        typeMath: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::KdTree::ITypeMath_1<TKey>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (dimensions, typeMath))?;
        Ok(__cordl_object.into())
    }
    pub fn RadialSearch(
        &mut self,
        center: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TKey>>,
        radius: TKey,
        count: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::UnityEngine::ProBuilder::KdTree::KdTreeNode_2<TKey, TValue>,
            >,
        >,
    >
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::UnityEngine::ProBuilder::KdTree::KdTreeNode_2<TKey, TValue>,
            >,
        > = __cordl_object.invoke("RadialSearch", (center, radius, count))?;
        Ok(__cordl_ret.into())
    }
    pub fn ReadChildNodes(
        &mut self,
        removedNode: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::KdTree::KdTreeNode_2<TKey, TValue>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ReadChildNodes", (removedNode))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveAt(
        &mut self,
        point: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TKey>>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveAt", (point))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveChildNodes(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::KdTree::KdTreeNode_2<TKey, TValue>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveChildNodes", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn SaveToFile(
        &mut self,
        filename: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SaveToFile", (filename))?;
        Ok(__cordl_ret.into())
    }
    pub fn SortNodesArray(
        &mut self,
        nodes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::UnityEngine::ProBuilder::KdTree::KdTreeNode_2<TKey, TValue>,
            >,
        >,
        byDimension: i32,
        fromIndex: i32,
        toIndex: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SortNodesArray", (nodes, byDimension, fromIndex, toIndex))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    >
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::IEnumerator,
        > = __cordl_object.invoke("System.Collections.IEnumerable.GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    >
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TryFindValue(
        &mut self,
        value: TValue,
        point: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TKey>>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryFindValue", (value, point))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryFindValueAt(
        &mut self,
        point: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<TKey>>,
        value: quest_hook::libil2cpp::ByRefMut<TValue>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryFindValueAt", (point, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_AddDuplicateBehavior1(
        &mut self,
        dimensions: i32,
        typeMath: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::KdTree::ITypeMath_1<TKey>,
        >,
        addDuplicateBehavior: crate::UnityEngine::ProBuilder::KdTree::AddDuplicateBehavior,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (dimensions, typeMath, addDuplicateBehavior))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_i32_ITypeMath_1_0(
        &mut self,
        dimensions: i32,
        typeMath: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::KdTree::ITypeMath_1<TKey>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (dimensions, typeMath))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_AddDuplicateBehavior(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::UnityEngine::ProBuilder::KdTree::AddDuplicateBehavior,
    >
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ProBuilder::KdTree::AddDuplicateBehavior = __cordl_object
            .invoke("get_AddDuplicateBehavior", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_AddDuplicateBehavior(
        &mut self,
        value: crate::UnityEngine::ProBuilder::KdTree::AddDuplicateBehavior,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AddDuplicateBehavior", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Count(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Count", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+KdTree_2")]
impl<
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::KdTree::KdTree_2<TKey, TValue> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+KdTree_2")]
impl<
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsRef<
    crate::System::Collections::Generic::IEnumerable_1<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::KdTree::KdTreeNode_2<TKey, TValue>,
        >,
    >,
> for crate::UnityEngine::ProBuilder::KdTree::KdTree_2<TKey, TValue> {
    fn as_ref(
        &self,
    ) -> &crate::System::Collections::Generic::IEnumerable_1<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::KdTree::KdTreeNode_2<TKey, TValue>,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+KdTree_2")]
impl<
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsMut<
    crate::System::Collections::Generic::IEnumerable_1<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::KdTree::KdTreeNode_2<TKey, TValue>,
        >,
    >,
> for crate::UnityEngine::ProBuilder::KdTree::KdTree_2<TKey, TValue> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerable_1<
        quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::KdTree::KdTreeNode_2<TKey, TValue>,
        >,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+KdTree_2")]
impl<
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::IEnumerable>
for crate::UnityEngine::ProBuilder::KdTree::KdTree_2<TKey, TValue> {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+KdTree_2")]
impl<
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::IEnumerable>
for crate::UnityEngine::ProBuilder::KdTree::KdTree_2<TKey, TValue> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+KdTree_2")]
impl<
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsRef<crate::UnityEngine::ProBuilder::KdTree::IKdTree_2<TKey, TValue>>
for crate::UnityEngine::ProBuilder::KdTree::KdTree_2<TKey, TValue> {
    fn as_ref(
        &self,
    ) -> &crate::UnityEngine::ProBuilder::KdTree::IKdTree_2<TKey, TValue> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+KdTree_2")]
impl<
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> AsMut<crate::UnityEngine::ProBuilder::KdTree::IKdTree_2<TKey, TValue>>
for crate::UnityEngine::ProBuilder::KdTree::KdTree_2<TKey, TValue> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::UnityEngine::ProBuilder::KdTree::IKdTree_2<TKey, TValue> {
        unsafe { std::mem::transmute(self) }
    }
}
