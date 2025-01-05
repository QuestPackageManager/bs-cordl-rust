#[cfg(feature = "System+Collections+Generic+LinkedList_1")]
#[repr(C)]
#[derive(Debug)]
pub struct LinkedList_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub head: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::LinkedListNode_1<T>,
    >,
    pub count: i32,
    pub version: i32,
    pub _syncRoot: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub _siInfo: quest_hook::libil2cpp::Gc<
        crate::System::Runtime::Serialization::SerializationInfo,
    >,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "System+Collections+Generic+LinkedList_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Collections::Generic::LinkedList_1 < T >
    => "System.Collections.Generic"."LinkedList`1" < T >
);
#[cfg(feature = "System+Collections+Generic+LinkedList_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Collections::Generic::LinkedList_1<T> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Generic+LinkedList_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Collections::Generic::LinkedList_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Generic+LinkedList_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::System::Collections::Generic::LinkedList_1<T> {
    pub const CountName: &'static str = "Count";
    pub const ValuesName: &'static str = "Data";
    pub const VersionName: &'static str = "Version";
    #[cfg(feature = "System+Collections+Generic+LinkedList_1+Enumerator")]
    pub type Enumerator = crate::System::Collections::Generic::LinkedList_1_Enumerator<
        T,
    >;
    pub fn AddAfter(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<T>,
        >,
        newNode: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddAfter", (node, newNode))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddBefore_LinkedListNode_1_1(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<T>,
        >,
        newNode: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddBefore", (node, newNode))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddBefore_T0(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<T>,
        >,
        value: T,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<T>,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<T>,
        > = __cordl_object.invoke("AddBefore", (node, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddFirst_LinkedListNode_1_1(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddFirst", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddFirst_T0(
        &mut self,
        value: T,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<T>,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<T>,
        > = __cordl_object.invoke("AddFirst", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddLast_LinkedListNode_1_1(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddLast", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn AddLast_T0(
        &mut self,
        value: T,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<T>,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<T>,
        > = __cordl_object.invoke("AddLast", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Clear(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Clear", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Contains(&mut self, value: T) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Contains", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyTo(
        &mut self,
        array: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CopyTo", (array, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn Find(
        &mut self,
        value: T,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<T>,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<T>,
        > = __cordl_object.invoke("Find", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Collections::Generic::LinkedList_1_Enumerator<T>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Collections::Generic::LinkedList_1_Enumerator<
            T,
        > = __cordl_object.invoke("GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectData(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("GetObjectData", (info, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalInsertNodeBefore(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<T>,
        >,
        newNode: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalInsertNodeBefore", (node, newNode))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalInsertNodeToEmptyList(
        &mut self,
        newNode: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalInsertNodeToEmptyList", (newNode))?;
        Ok(__cordl_ret.into())
    }
    pub fn InternalRemoveNode(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InternalRemoveNode", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_SerializationInfo_StreamingContext1(
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (info, context))?;
        Ok(__cordl_object.into())
    }
    pub fn OnDeserialization(
        &mut self,
        sender: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDeserialization", (sender))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveFirst(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveFirst", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveLast(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RemoveLast", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Remove_LinkedListNode_1_1(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Remove", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn Remove_T0(&mut self, value: T) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Remove", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_Generic_ICollection_T__Add(
        &mut self,
        value: T,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Collections.Generic.ICollection<T>.Add", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_Generic_ICollection_T__get_IsReadOnly(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.Collections.Generic.ICollection<T>.get_IsReadOnly", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_Generic_IEnumerable_T__GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IEnumerator_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<T>,
        > = __cordl_object
            .invoke("System.Collections.Generic.IEnumerable<T>.GetEnumerator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_ICollection_CopyTo(
        &mut self,
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("System.Collections.ICollection.CopyTo", (array, index))?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_ICollection_get_IsSynchronized(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("System.Collections.ICollection.get_IsSynchronized", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_ICollection_get_SyncRoot(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("System.Collections.ICollection.get_SyncRoot", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
    pub fn ValidateNewNode(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateNewNode", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn ValidateNode(
        &mut self,
        node: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ValidateNode", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SerializationInfo_StreamingContext1(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (info, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Count", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_First(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<T>,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<T>,
        > = __cordl_object.invoke("get_First", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Last(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<T>,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedListNode_1<T>,
        > = __cordl_object.invoke("get_Last", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Collections+Generic+LinkedList_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Collections::Generic::LinkedList_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Collections+Generic+LinkedList_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::ICollection_1<T>>
for crate::System::Collections::Generic::LinkedList_1<T> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::ICollection_1<T> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+Generic+LinkedList_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::ICollection_1<T>>
for crate::System::Collections::Generic::LinkedList_1<T> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::Generic::ICollection_1<T> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+Generic+LinkedList_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IEnumerable_1<T>>
for crate::System::Collections::Generic::LinkedList_1<T> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::IEnumerable_1<T> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+Generic+LinkedList_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IEnumerable_1<T>>
for crate::System::Collections::Generic::LinkedList_1<T> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::Generic::IEnumerable_1<T> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+Generic+LinkedList_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IReadOnlyCollection_1<T>>
for crate::System::Collections::Generic::LinkedList_1<T> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::IReadOnlyCollection_1<T> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+Generic+LinkedList_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IReadOnlyCollection_1<T>>
for crate::System::Collections::Generic::LinkedList_1<T> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IReadOnlyCollection_1<T> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+Generic+LinkedList_1")]
impl<T: quest_hook::libil2cpp::Type> AsRef<crate::System::Collections::ICollection>
for crate::System::Collections::Generic::LinkedList_1<T> {
    fn as_ref(&self) -> &crate::System::Collections::ICollection {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+Generic+LinkedList_1")]
impl<T: quest_hook::libil2cpp::Type> AsMut<crate::System::Collections::ICollection>
for crate::System::Collections::Generic::LinkedList_1<T> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::ICollection {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+Generic+LinkedList_1")]
impl<T: quest_hook::libil2cpp::Type> AsRef<crate::System::Collections::IEnumerable>
for crate::System::Collections::Generic::LinkedList_1<T> {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+Generic+LinkedList_1")]
impl<T: quest_hook::libil2cpp::Type> AsMut<crate::System::Collections::IEnumerable>
for crate::System::Collections::Generic::LinkedList_1<T> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+Generic+LinkedList_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Runtime::Serialization::IDeserializationCallback>
for crate::System::Collections::Generic::LinkedList_1<T> {
    fn as_ref(
        &self,
    ) -> &crate::System::Runtime::Serialization::IDeserializationCallback {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+Generic+LinkedList_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Runtime::Serialization::IDeserializationCallback>
for crate::System::Collections::Generic::LinkedList_1<T> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::Serialization::IDeserializationCallback {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+Generic+LinkedList_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Runtime::Serialization::ISerializable>
for crate::System::Collections::Generic::LinkedList_1<T> {
    fn as_ref(&self) -> &crate::System::Runtime::Serialization::ISerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+Generic+LinkedList_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Runtime::Serialization::ISerializable>
for crate::System::Collections::Generic::LinkedList_1<T> {
    fn as_mut(&mut self) -> &mut crate::System::Runtime::Serialization::ISerializable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Collections+Generic+LinkedList_1+Enumerator")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct LinkedList_1_Enumerator<T: quest_hook::libil2cpp::Type> {
    pub _list: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::LinkedList_1<T>,
    >,
    pub _node: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::LinkedListNode_1<T>,
    >,
    pub _version: i32,
    pub _current: T,
    pub _index: i32,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "System+Collections+Generic+LinkedList_1+Enumerator")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::System::Collections::Generic::LinkedList_1_Enumerator < T > =>
    "System.Collections.Generic"."LinkedList`1/Enumerator<T>" < T >
);
#[cfg(feature = "System+Collections+Generic+LinkedList_1+Enumerator")]
unsafe impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::System::Collections::Generic::LinkedList_1_Enumerator<T> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "System+Collections+Generic+LinkedList_1+Enumerator")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::System::Collections::Generic::LinkedList_1_Enumerator<T> {
    pub fn Dispose(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
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
    pub fn System_Runtime_Serialization_IDeserializationCallback_OnDeserialization(
        &mut self,
        sender: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Runtime.Serialization.IDeserializationCallback.OnDeserialization",
            (sender),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn System_Runtime_Serialization_ISerializable_GetObjectData(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "System.Runtime.Serialization.ISerializable.GetObjectData",
            (info, context),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_LinkedList_1_0(
        &mut self,
        list: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::LinkedList_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (list),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_SerializationInfo_StreamingContext1(
        &mut self,
        info: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::Serialization::SerializationInfo,
        >,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            ".ctor",
            (info, context),
        )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Current(&mut self) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = quest_hook::libil2cpp::ValueTypeExt::invoke(
            self,
            "get_Current",
            (),
        )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Collections+Generic+LinkedList_1+Enumerator")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IEnumerator_1<T>>
for crate::System::Collections::Generic::LinkedList_1_Enumerator<T> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::IEnumerator_1<T> {
        todo!()
    }
}
#[cfg(feature = "System+Collections+Generic+LinkedList_1+Enumerator")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IEnumerator_1<T>>
for crate::System::Collections::Generic::LinkedList_1_Enumerator<T> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::Generic::IEnumerator_1<T> {
        todo!()
    }
}
#[cfg(feature = "System+Collections+Generic+LinkedList_1+Enumerator")]
impl<T: quest_hook::libil2cpp::Type> AsRef<crate::System::Collections::IEnumerator>
for crate::System::Collections::Generic::LinkedList_1_Enumerator<T> {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerator {
        todo!()
    }
}
#[cfg(feature = "System+Collections+Generic+LinkedList_1+Enumerator")]
impl<T: quest_hook::libil2cpp::Type> AsMut<crate::System::Collections::IEnumerator>
for crate::System::Collections::Generic::LinkedList_1_Enumerator<T> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerator {
        todo!()
    }
}
#[cfg(feature = "System+Collections+Generic+LinkedList_1+Enumerator")]
impl<T: quest_hook::libil2cpp::Type> AsRef<crate::System::IDisposable>
for crate::System::Collections::Generic::LinkedList_1_Enumerator<T> {
    fn as_ref(&self) -> &crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "System+Collections+Generic+LinkedList_1+Enumerator")]
impl<T: quest_hook::libil2cpp::Type> AsMut<crate::System::IDisposable>
for crate::System::Collections::Generic::LinkedList_1_Enumerator<T> {
    fn as_mut(&mut self) -> &mut crate::System::IDisposable {
        todo!()
    }
}
#[cfg(feature = "System+Collections+Generic+LinkedList_1+Enumerator")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Runtime::Serialization::IDeserializationCallback>
for crate::System::Collections::Generic::LinkedList_1_Enumerator<T> {
    fn as_ref(
        &self,
    ) -> &crate::System::Runtime::Serialization::IDeserializationCallback {
        todo!()
    }
}
#[cfg(feature = "System+Collections+Generic+LinkedList_1+Enumerator")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Runtime::Serialization::IDeserializationCallback>
for crate::System::Collections::Generic::LinkedList_1_Enumerator<T> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Runtime::Serialization::IDeserializationCallback {
        todo!()
    }
}
#[cfg(feature = "System+Collections+Generic+LinkedList_1+Enumerator")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Runtime::Serialization::ISerializable>
for crate::System::Collections::Generic::LinkedList_1_Enumerator<T> {
    fn as_ref(&self) -> &crate::System::Runtime::Serialization::ISerializable {
        todo!()
    }
}
#[cfg(feature = "System+Collections+Generic+LinkedList_1+Enumerator")]
impl<
    T: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Runtime::Serialization::ISerializable>
for crate::System::Collections::Generic::LinkedList_1_Enumerator<T> {
    fn as_mut(&mut self) -> &mut crate::System::Runtime::Serialization::ISerializable {
        todo!()
    }
}
