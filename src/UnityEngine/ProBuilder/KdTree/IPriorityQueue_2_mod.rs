#[cfg(feature = "UnityEngine+ProBuilder+KdTree+IPriorityQueue_2")]
#[repr(C)]
#[derive(Debug)]
pub struct IPriorityQueue_2<
    TItem: quest_hook::libil2cpp::Type,
    TPriority: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_TItem: std::marker::PhantomData<TItem>,
    __cordl_phantom_TPriority: std::marker::PhantomData<TPriority>,
}
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+IPriorityQueue_2")]
unsafe impl<
    TItem: quest_hook::libil2cpp::Type,
    TPriority: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::UnityEngine::ProBuilder::KdTree::IPriorityQueue_2<TItem, TPriority> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "UnityEngine.ProBuilder.KdTree";
    const CLASS_NAME: &'static str = "IPriorityQueue`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "UnityEngine.ProBuilder.KdTree",
                        "IPriorityQueue`2",
                    )
                    .unwrap()
                    .make_generic::<(TItem, TPriority)>()
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
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+IPriorityQueue_2")]
impl<
    TItem: quest_hook::libil2cpp::Type,
    TPriority: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::UnityEngine::ProBuilder::KdTree::IPriorityQueue_2<TItem, TPriority> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+IPriorityQueue_2")]
impl<
    TItem: quest_hook::libil2cpp::Type,
    TPriority: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::UnityEngine::ProBuilder::KdTree::IPriorityQueue_2<TItem, TPriority> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+IPriorityQueue_2")]
impl<
    TItem: quest_hook::libil2cpp::Type,
    TPriority: quest_hook::libil2cpp::Type,
> crate::UnityEngine::ProBuilder::KdTree::IPriorityQueue_2<TItem, TPriority> {
    pub fn Dequeue(&mut self) -> quest_hook::libil2cpp::Result<TItem>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TPriority: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), TItem, 0usize>("Dequeue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Dequeue", 0usize
                )
            });
        let __cordl_ret: TItem = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
    pub fn Enqueue(
        &mut self,
        item: TItem,
        priority: TPriority,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TPriority: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (TItem, TPriority),
                quest_hook::libil2cpp::Void,
                2usize,
            >("Enqueue")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "Enqueue", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (item, priority))
        };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_Count(&mut self) -> quest_hook::libil2cpp::Result<i32>
    where
        TItem: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TPriority: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), i32, 0usize>("get_Count")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_Count", 0usize
                )
            });
        let __cordl_ret: i32 = unsafe { method.invoke_unchecked(self, ()) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+IPriorityQueue_2")]
impl<
    TItem: quest_hook::libil2cpp::Type,
    TPriority: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::ProBuilder::KdTree::IPriorityQueue_2<TItem, TPriority> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
