#[cfg(feature = "Priority_Queue+IFixedSizePriorityQueue_2")]
#[repr(C)]
#[derive(Debug)]
pub struct IFixedSizePriorityQueue_2<
    TItem: quest_hook::libil2cpp::Type,
    TPriority: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_TItem: std::marker::PhantomData<TItem>,
    __cordl_phantom_TPriority: std::marker::PhantomData<TPriority>,
}
#[cfg(feature = "Priority_Queue+IFixedSizePriorityQueue_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Priority_Queue::IFixedSizePriorityQueue_2 <
    TItem, TPriority > => "Priority_Queue"."IFixedSizePriorityQueue`2" < TItem, TPriority
    >
);
#[cfg(feature = "Priority_Queue+IFixedSizePriorityQueue_2")]
impl<
    TItem: quest_hook::libil2cpp::Type,
    TPriority: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::Priority_Queue::IFixedSizePriorityQueue_2<TItem, TPriority> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Priority_Queue+IFixedSizePriorityQueue_2")]
impl<
    TItem: quest_hook::libil2cpp::Type,
    TPriority: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::Priority_Queue::IFixedSizePriorityQueue_2<TItem, TPriority> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Priority_Queue+IFixedSizePriorityQueue_2")]
impl<
    TItem: quest_hook::libil2cpp::Type,
    TPriority: quest_hook::libil2cpp::Type,
> crate::Priority_Queue::IFixedSizePriorityQueue_2<TItem, TPriority> {
    pub fn ResetNode(
        &mut self,
        node: TItem,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TPriority: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ResetNode", (node))?;
        Ok(__cordl_ret.into())
    }
    pub fn Resize(
        &mut self,
        maxNodes: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TPriority: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Resize", (maxNodes))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_MaxSize(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TPriority: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_MaxSize", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Priority_Queue+IFixedSizePriorityQueue_2")]
impl<
    TItem: quest_hook::libil2cpp::Type,
    TPriority: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::Priority_Queue::IFixedSizePriorityQueue_2<TItem, TPriority> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Priority_Queue+IFixedSizePriorityQueue_2")]
impl<
    TItem: quest_hook::libil2cpp::Type,
    TPriority: quest_hook::libil2cpp::Type,
> AsRef<crate::Priority_Queue::IPriorityQueue_2<TItem, TPriority>>
for crate::Priority_Queue::IFixedSizePriorityQueue_2<TItem, TPriority> {
    fn as_ref(&self) -> &crate::Priority_Queue::IPriorityQueue_2<TItem, TPriority> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Priority_Queue+IFixedSizePriorityQueue_2")]
impl<
    TItem: quest_hook::libil2cpp::Type,
    TPriority: quest_hook::libil2cpp::Type,
> AsMut<crate::Priority_Queue::IPriorityQueue_2<TItem, TPriority>>
for crate::Priority_Queue::IFixedSizePriorityQueue_2<TItem, TPriority> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::Priority_Queue::IPriorityQueue_2<TItem, TPriority> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Priority_Queue+IFixedSizePriorityQueue_2")]
impl<
    TItem: quest_hook::libil2cpp::Type,
    TPriority: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IEnumerable_1<TItem>>
for crate::Priority_Queue::IFixedSizePriorityQueue_2<TItem, TPriority> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::IEnumerable_1<TItem> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Priority_Queue+IFixedSizePriorityQueue_2")]
impl<
    TItem: quest_hook::libil2cpp::Type,
    TPriority: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IEnumerable_1<TItem>>
for crate::Priority_Queue::IFixedSizePriorityQueue_2<TItem, TPriority> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerable_1<TItem> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Priority_Queue+IFixedSizePriorityQueue_2")]
impl<
    TItem: quest_hook::libil2cpp::Type,
    TPriority: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::IEnumerable>
for crate::Priority_Queue::IFixedSizePriorityQueue_2<TItem, TPriority> {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Priority_Queue+IFixedSizePriorityQueue_2")]
impl<
    TItem: quest_hook::libil2cpp::Type,
    TPriority: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::IEnumerable>
for crate::Priority_Queue::IFixedSizePriorityQueue_2<TItem, TPriority> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
