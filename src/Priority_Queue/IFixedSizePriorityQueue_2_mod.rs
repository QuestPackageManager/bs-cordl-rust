#[cfg(feature = "cordl_class_Priority_Queue+IFixedSizePriorityQueue_2")]
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
#[cfg(feature = "cordl_class_Priority_Queue+IFixedSizePriorityQueue_2")]
unsafe impl<
    TItem: quest_hook::libil2cpp::Type,
    TPriority: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::Priority_Queue::IFixedSizePriorityQueue_2<TItem, TPriority> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Priority_Queue";
    const CLASS_NAME: &'static str = "IFixedSizePriorityQueue`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "Priority_Queue",
                        "IFixedSizePriorityQueue`2",
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
#[cfg(feature = "Priority_Queue+IFixedSizePriorityQueue_2")]
impl<
    TItem: quest_hook::libil2cpp::Type,
    TPriority: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::Priority_Queue::IFixedSizePriorityQueue_2<TItem, TPriority> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Priority_Queue+IFixedSizePriorityQueue_2")]
impl<
    TItem: quest_hook::libil2cpp::Type,
    TPriority: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::Priority_Queue::IFixedSizePriorityQueue_2<TItem, TPriority> {
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (TItem),
                        quest_hook::libil2cpp::Void,
                        1usize,
                    >("ResetNode")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "ResetNode", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (node))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(i32), quest_hook::libil2cpp::Void, 1usize>("Resize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(), "Resize",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            cordl_method_info.invoke_unchecked(self, (maxNodes))?
        };
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
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> = std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo = METHOD
            .get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), i32, 0usize>("get_MaxSize")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_MaxSize", 0usize
                        )
                    })
            });
        let __cordl_ret: i32 = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Priority_Queue+IFixedSizePriorityQueue_2")]
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
