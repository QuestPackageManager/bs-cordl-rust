#[cfg(feature = "cordl_class_System+Linq+OrderedEnumerable_1")]
#[repr(C)]
#[derive(Debug)]
pub struct OrderedEnumerable_1<TElement: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub source:
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IEnumerable_1<TElement>>,
    __cordl_phantom_TElement: std::marker::PhantomData<TElement>,
}
#[cfg(feature = "cordl_class_System+Linq+OrderedEnumerable_1")]
unsafe impl<TElement: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
    for crate::System::Linq::OrderedEnumerable_1<TElement>
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq";
    const CLASS_NAME: &'static str = "OrderedEnumerable`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find("System.Linq", "OrderedEnumerable`1")
                .unwrap()
                .make_generic::<(TElement)>()
                .unwrap()
                .unwrap()
        })
    }
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class()
            .is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
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
#[cfg(feature = "System+Linq+OrderedEnumerable_1")]
impl<TElement: quest_hook::libil2cpp::Type> std::ops::Deref
    for crate::System::Linq::OrderedEnumerable_1<TElement>
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+OrderedEnumerable_1")]
impl<TElement: quest_hook::libil2cpp::Type> std::ops::DerefMut
    for crate::System::Linq::OrderedEnumerable_1<TElement>
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+OrderedEnumerable_1")]
impl<TElement: quest_hook::libil2cpp::Type> crate::System::Linq::OrderedEnumerable_1<TElement> {
    pub fn GetEnumerableSorter(
        &mut self,
        next: quest_hook::libil2cpp::Gc<crate::System::Linq::EnumerableSorter_1<TElement>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::EnumerableSorter_1<TElement>>,
    >
    where
        TElement: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (quest_hook::libil2cpp::Gc<
                            crate::System::Linq::EnumerableSorter_1<TElement>,
                        >),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Linq::EnumerableSorter_1<TElement>,
                        >,
                        1usize,
                    >("GetEnumerableSorter")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetEnumerableSorter", 1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::EnumerableSorter_1<TElement>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (next))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IEnumerator_1<TElement>>,
    >
    where
        TElement: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IEnumerator_1<TElement>,
                    >, 0usize>("GetEnumerator")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetEnumerator",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerator_1<TElement>,
        > = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TElement: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self =
            <Self as quest_hook::libil2cpp::Type>::class().instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn System_Collections_IEnumerable_GetEnumerator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator>,
    >
    where
        TElement: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Collections::IEnumerator,
                        >,
                        0usize,
                    >("System.Collections.IEnumerable.GetEnumerator")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "System.Collections.IEnumerable.GetEnumerator", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Collections::IEnumerator> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn System_Linq_IOrderedEnumerable_TElement__CreateOrderedEnumerable<TKey>(
        &mut self,
        keySelector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TElement, TKey>>,
        comparer: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IComparer_1<TKey>>,
        descending: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::IOrderedEnumerable_1<TElement>>,
    >
    where
        TElement: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<
                        (
                            quest_hook::libil2cpp::Gc<
                                crate::System::Func_2<TElement, TKey>,
                            >,
                            quest_hook::libil2cpp::Gc<
                                crate::System::Collections::Generic::IComparer_1<TKey>,
                            >,
                            bool,
                        ),
                        quest_hook::libil2cpp::Gc<
                            crate::System::Linq::IOrderedEnumerable_1<TElement>,
                        >,
                        3usize,
                    >("System.Linq.IOrderedEnumerable<TElement>.CreateOrderedEnumerable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "System.Linq.IOrderedEnumerable<TElement>.CreateOrderedEnumerable",
                            3usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::IOrderedEnumerable_1<TElement>,
        > = unsafe {
            cordl_method_info.invoke_unchecked(self, (keySelector, comparer, descending))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TElement: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static METHOD: std::sync::OnceLock<&'static quest_hook::libil2cpp::MethodInfo> =
            std::sync::OnceLock::new();
        let cordl_method_info: &'static quest_hook::libil2cpp::MethodInfo =
            METHOD.get_or_init(|| {
                <Self as quest_hook::libil2cpp::Type>::class()
                    .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            ".ctor",
                            0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_System+Linq+OrderedEnumerable_1")]
impl<TElement: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
    for crate::System::Linq::OrderedEnumerable_1<TElement>
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+OrderedEnumerable_1")]
impl<TElement: quest_hook::libil2cpp::Type>
    AsRef<crate::System::Collections::Generic::IEnumerable_1<TElement>>
    for crate::System::Linq::OrderedEnumerable_1<TElement>
{
    fn as_ref(&self) -> &crate::System::Collections::Generic::IEnumerable_1<TElement> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+OrderedEnumerable_1")]
impl<TElement: quest_hook::libil2cpp::Type>
    AsMut<crate::System::Collections::Generic::IEnumerable_1<TElement>>
    for crate::System::Linq::OrderedEnumerable_1<TElement>
{
    fn as_mut(&mut self) -> &mut crate::System::Collections::Generic::IEnumerable_1<TElement> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+OrderedEnumerable_1")]
impl<TElement: quest_hook::libil2cpp::Type> AsRef<crate::System::Collections::IEnumerable>
    for crate::System::Linq::OrderedEnumerable_1<TElement>
{
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+OrderedEnumerable_1")]
impl<TElement: quest_hook::libil2cpp::Type> AsMut<crate::System::Collections::IEnumerable>
    for crate::System::Linq::OrderedEnumerable_1<TElement>
{
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+OrderedEnumerable_1")]
impl<TElement: quest_hook::libil2cpp::Type>
    AsRef<crate::System::Linq::IOrderedEnumerable_1<TElement>>
    for crate::System::Linq::OrderedEnumerable_1<TElement>
{
    fn as_ref(&self) -> &crate::System::Linq::IOrderedEnumerable_1<TElement> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+OrderedEnumerable_1")]
impl<TElement: quest_hook::libil2cpp::Type>
    AsMut<crate::System::Linq::IOrderedEnumerable_1<TElement>>
    for crate::System::Linq::OrderedEnumerable_1<TElement>
{
    fn as_mut(&mut self) -> &mut crate::System::Linq::IOrderedEnumerable_1<TElement> {
        unsafe { std::mem::transmute(self) }
    }
}
