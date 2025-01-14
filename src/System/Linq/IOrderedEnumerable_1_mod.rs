#[cfg(feature = "System+Linq+IOrderedEnumerable_1")]
#[repr(C)]
#[derive(Debug)]
pub struct IOrderedEnumerable_1<TElement: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_TElement: std::marker::PhantomData<TElement>,
}
#[cfg(feature = "System+Linq+IOrderedEnumerable_1")]
unsafe impl<TElement: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::System::Linq::IOrderedEnumerable_1<TElement> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Linq";
    const CLASS_NAME: &'static str = "IOrderedEnumerable`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "System.Linq",
                        "IOrderedEnumerable`1",
                    )
                    .unwrap()
                    .make_generic::<(TElement)>()
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
#[cfg(feature = "System+Linq+IOrderedEnumerable_1")]
impl<TElement: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Linq::IOrderedEnumerable_1<TElement> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+IOrderedEnumerable_1")]
impl<TElement: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Linq::IOrderedEnumerable_1<TElement> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Linq+IOrderedEnumerable_1")]
impl<
    TElement: quest_hook::libil2cpp::Type,
> crate::System::Linq::IOrderedEnumerable_1<TElement> {
    pub fn CreateOrderedEnumerable<TKey>(
        &mut self,
        keySelector: quest_hook::libil2cpp::Gc<crate::System::Func_2<TElement, TKey>>,
        comparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IComparer_1<TKey>,
        >,
        descending: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Linq::IOrderedEnumerable_1<TElement>>,
    >
    where
        TElement: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<crate::System::Func_2<TElement, TKey>>,
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IComparer_1<TKey>,
                    >,
                    bool,
                ),
                quest_hook::libil2cpp::Gc<
                    crate::System::Linq::IOrderedEnumerable_1<TElement>,
                >,
                3usize,
            >("CreateOrderedEnumerable")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "CreateOrderedEnumerable", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Linq::IOrderedEnumerable_1<TElement>,
        > = unsafe {
            method.invoke_unchecked(self, (keySelector, comparer, descending))
        };
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "System+Linq+IOrderedEnumerable_1")]
impl<TElement: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::System::Linq::IOrderedEnumerable_1<TElement> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Linq+IOrderedEnumerable_1")]
impl<
    TElement: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::Generic::IEnumerable_1<TElement>>
for crate::System::Linq::IOrderedEnumerable_1<TElement> {
    fn as_ref(&self) -> &crate::System::Collections::Generic::IEnumerable_1<TElement> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+IOrderedEnumerable_1")]
impl<
    TElement: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::Generic::IEnumerable_1<TElement>>
for crate::System::Linq::IOrderedEnumerable_1<TElement> {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::Collections::Generic::IEnumerable_1<TElement> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+IOrderedEnumerable_1")]
impl<
    TElement: quest_hook::libil2cpp::Type,
> AsRef<crate::System::Collections::IEnumerable>
for crate::System::Linq::IOrderedEnumerable_1<TElement> {
    fn as_ref(&self) -> &crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Linq+IOrderedEnumerable_1")]
impl<
    TElement: quest_hook::libil2cpp::Type,
> AsMut<crate::System::Collections::IEnumerable>
for crate::System::Linq::IOrderedEnumerable_1<TElement> {
    fn as_mut(&mut self) -> &mut crate::System::Collections::IEnumerable {
        unsafe { std::mem::transmute(self) }
    }
}
