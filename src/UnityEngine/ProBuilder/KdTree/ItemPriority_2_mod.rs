#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+KdTree+ItemPriority_2")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[cfg_attr(feature = "derive_Clone", derive(Clone))]
#[cfg_attr(feature = "derive_Default", derive(Default))]
#[cfg_attr(feature = "derive_PartialEq", derive(PartialEq))]
#[repr(C)]
pub struct ItemPriority_2<
    TItem: quest_hook::libil2cpp::Type,
    TPriority: quest_hook::libil2cpp::Type,
> {
    pub Item: TItem,
    pub Priority: TPriority,
    __cordl_phantom_TItem: std::marker::PhantomData<TItem>,
    __cordl_phantom_TPriority: std::marker::PhantomData<TPriority>,
}
#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+KdTree+ItemPriority_2")]
unsafe impl<TItem: quest_hook::libil2cpp::Type, TPriority: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::Type
    for crate::UnityEngine::ProBuilder::KdTree::ItemPriority_2<TItem, TPriority>
{
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "UnityEngine.ProBuilder.KdTree";
    const CLASS_NAME: &'static str = "ItemPriority`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find(
                "UnityEngine.ProBuilder.KdTree",
                "ItemPriority`2",
            )
            .unwrap()
            .make_generic::<(TItem, TPriority)>()
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
            && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.is_ref() && <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
}
#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+KdTree+ItemPriority_2")]
unsafe impl<TItem: quest_hook::libil2cpp::Type, TPriority: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::Argument
    for crate::UnityEngine::ProBuilder::KdTree::ItemPriority_2<TItem, TPriority>
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+KdTree+ItemPriority_2")]
unsafe impl<TItem: quest_hook::libil2cpp::Type, TPriority: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::Parameter
    for crate::UnityEngine::ProBuilder::KdTree::ItemPriority_2<TItem, TPriority>
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_parameter(ty)
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Parameter>::Actual) -> Self {
        actual
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Parameter>::Actual {
        self
    }
}
#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+KdTree+ItemPriority_2")]
unsafe impl<TItem: quest_hook::libil2cpp::Type, TPriority: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::Returned
    for crate::UnityEngine::ProBuilder::KdTree::ItemPriority_2<TItem, TPriority>
{
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_returned(ty)
    }
    fn from_object(object: Option<&mut quest_hook::libil2cpp::Il2CppObject>) -> Self {
        unsafe {
            quest_hook::libil2cpp::raw::unbox(quest_hook::libil2cpp::WrapRaw::raw(object.unwrap()))
        }
    }
}
#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+KdTree+ItemPriority_2")]
unsafe impl<TItem: quest_hook::libil2cpp::Type, TPriority: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::Return
    for crate::UnityEngine::ProBuilder::KdTree::ItemPriority_2<TItem, TPriority>
{
    type Actual = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_return(ty)
    }
    fn into_actual(self) -> <Self as quest_hook::libil2cpp::Return>::Actual {
        self
    }
    fn from_actual(actual: <Self as quest_hook::libil2cpp::Return>::Actual) -> Self {
        actual
    }
}
#[cfg(feature = "cordl_class_UnityEngine+ProBuilder+KdTree+ItemPriority_2")]
unsafe impl<TItem: quest_hook::libil2cpp::Type, TPriority: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::ThisArgument
    for crate::UnityEngine::ProBuilder::KdTree::ItemPriority_2<TItem, TPriority>
{
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+KdTree+ItemPriority_2")]
impl<TItem: quest_hook::libil2cpp::Type, TPriority: quest_hook::libil2cpp::Type>
    crate::UnityEngine::ProBuilder::KdTree::ItemPriority_2<TItem, TPriority>
{
}
