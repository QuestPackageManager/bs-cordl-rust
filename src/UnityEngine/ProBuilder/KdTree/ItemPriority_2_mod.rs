#[cfg(feature = "UnityEngine+ProBuilder+KdTree+ItemPriority_2")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ItemPriority_2<
    TItem: quest_hook::libil2cpp::Type,
    TPriority: quest_hook::libil2cpp::Type,
> {
    pub Item: TItem,
    pub Priority: TPriority,
    __cordl_phantom_TItem: std::marker::PhantomData<TItem>,
    __cordl_phantom_TPriority: std::marker::PhantomData<TPriority>,
}
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+ItemPriority_2")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::KdTree::ItemPriority_2
    < TItem, TPriority > => "UnityEngine.ProBuilder.KdTree"
    ."ItemPriority`2<TItem,TPriority>" < TItem, TPriority >
);
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+ItemPriority_2")]
unsafe impl<
    TItem: quest_hook::libil2cpp::Type,
    TPriority: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ThisArgument
for crate::UnityEngine::ProBuilder::KdTree::ItemPriority_2<TItem, TPriority> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+ItemPriority_2")]
impl<
    TItem: quest_hook::libil2cpp::Type,
    TPriority: quest_hook::libil2cpp::Type,
> crate::UnityEngine::ProBuilder::KdTree::ItemPriority_2<TItem, TPriority> {}
