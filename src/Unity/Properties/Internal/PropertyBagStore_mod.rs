#[cfg(feature = "Unity+Properties+Internal+PropertyBagStore")]
#[repr(C)]
#[derive(Debug)]
pub struct PropertyBagStore {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Properties+Internal+PropertyBagStore")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Unity::Properties::Internal::PropertyBagStore {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Properties.Internal";
    const CLASS_NAME: &'static str = "PropertyBagStore";
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
#[cfg(feature = "Unity+Properties+Internal+PropertyBagStore")]
impl std::ops::Deref for crate::Unity::Properties::Internal::PropertyBagStore {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+PropertyBagStore")]
impl std::ops::DerefMut for crate::Unity::Properties::Internal::PropertyBagStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+Internal+PropertyBagStore")]
impl crate::Unity::Properties::Internal::PropertyBagStore {
    #[cfg(feature = "Unity+Properties+Internal+PropertyBagStore+TypedStore_1")]
    pub type TypedStore_1<TContainer: quest_hook::libil2cpp::Type> = crate::Unity::Properties::Internal::PropertyBagStore_TypedStore_1<
        TContainer,
    >;
    pub fn AddPropertyBag<TContainer>(
        propertyBag: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::IPropertyBag_1<TContainer>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Unity::Properties::IPropertyBag_1<TContainer>,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("AddPropertyBag")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "AddPropertyBag", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (propertyBag))
        };
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyBag_0<TContainer>() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Unity::Properties::IPropertyBag_1<TContainer>>,
    >
    where
        TContainer: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::Unity::Properties::IPropertyBag_1<TContainer>,
                >,
                0usize,
            >("GetPropertyBag")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetPropertyBag", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::IPropertyBag_1<TContainer>,
        > = unsafe { method.invoke_unchecked((), ()) };
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyBag_Type1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Unity::Properties::IPropertyBag>,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                quest_hook::libil2cpp::Gc<crate::Unity::Properties::IPropertyBag>,
                1usize,
            >("GetPropertyBag")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "GetPropertyBag", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Unity::Properties::IPropertyBag,
        > = unsafe { method.invoke_unchecked((), (_cordl_type)) };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Properties+Internal+PropertyBagStore")]
impl quest_hook::libil2cpp::ObjectType
for crate::Unity::Properties::Internal::PropertyBagStore {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+Internal+PropertyBagStore+TypedStore_1")]
#[repr(C)]
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PropertyBagStore_TypedStore_1<TContainer: quest_hook::libil2cpp::Type> {
    __cordl_phantom_TContainer: std::marker::PhantomData<TContainer>,
}
#[cfg(feature = "Unity+Properties+Internal+PropertyBagStore+TypedStore_1")]
unsafe impl<TContainer: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Type
for crate::Unity::Properties::Internal::PropertyBagStore_TypedStore_1<TContainer> {
    type Held<'a> = Self;
    type HeldRaw = Self;
    const NAMESPACE: &'static str = "Unity.Properties.Internal";
    const CLASS_NAME: &'static str = "PropertyBagStore/TypedStore`1";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "Unity.Properties.Internal",
                        "PropertyBagStore/TypedStore`1",
                    )
                    .unwrap()
                    .make_generic::<(TContainer)>()
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
#[cfg(feature = "Unity+Properties+Internal+PropertyBagStore+TypedStore_1")]
unsafe impl<TContainer: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Argument
for crate::Unity::Properties::Internal::PropertyBagStore_TypedStore_1<TContainer> {
    type Type = Self;
    fn matches(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_value_argument(ty)
    }
    fn invokable(&mut self) -> *mut ::std::ffi::c_void {
        self as *mut Self as *mut ::std::ffi::c_void
    }
}
#[cfg(feature = "Unity+Properties+Internal+PropertyBagStore+TypedStore_1")]
unsafe impl<TContainer: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Parameter
for crate::Unity::Properties::Internal::PropertyBagStore_TypedStore_1<TContainer> {
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
#[cfg(feature = "Unity+Properties+Internal+PropertyBagStore+TypedStore_1")]
unsafe impl<TContainer: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Returned
for crate::Unity::Properties::Internal::PropertyBagStore_TypedStore_1<TContainer> {
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
#[cfg(feature = "Unity+Properties+Internal+PropertyBagStore+TypedStore_1")]
unsafe impl<TContainer: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::Return
for crate::Unity::Properties::Internal::PropertyBagStore_TypedStore_1<TContainer> {
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
#[cfg(feature = "Unity+Properties+Internal+PropertyBagStore+TypedStore_1")]
unsafe impl<TContainer: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ThisArgument
for crate::Unity::Properties::Internal::PropertyBagStore_TypedStore_1<TContainer> {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "Unity+Properties+Internal+PropertyBagStore+TypedStore_1")]
impl<
    TContainer: quest_hook::libil2cpp::Type,
> crate::Unity::Properties::Internal::PropertyBagStore_TypedStore_1<TContainer> {}
