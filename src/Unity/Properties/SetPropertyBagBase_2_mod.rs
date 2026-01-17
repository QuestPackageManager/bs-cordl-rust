#[cfg(feature = "cordl_class_Unity+Properties+SetPropertyBagBase_2")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct SetPropertyBagBase_2<
    TSet: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::Unity::Properties::PropertyBag_1<TSet>,
    pub m_Property: quest_hook::libil2cpp::Gc<
        crate::Unity::Properties::SetPropertyBagBase_2_SetElementProperty<TSet, TElement>,
    >,
    __cordl_phantom_TSet: std::marker::PhantomData<TSet>,
    __cordl_phantom_TElement: std::marker::PhantomData<TElement>,
}
#[cfg(feature = "cordl_class_Unity+Properties+SetPropertyBagBase_2")]
unsafe impl<TSet: quest_hook::libil2cpp::Type, TElement: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::Type for crate::Unity::Properties::SetPropertyBagBase_2<TSet, TElement>
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Properties";
    const CLASS_NAME: &'static str = "SetPropertyBagBase`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find("Unity.Properties", "SetPropertyBagBase`2")
                .unwrap()
                .make_generic::<(TSet, TElement)>()
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
#[cfg(feature = "Unity+Properties+SetPropertyBagBase_2")]
impl<TSet: quest_hook::libil2cpp::Type, TElement: quest_hook::libil2cpp::Type> std::ops::Deref
    for crate::Unity::Properties::SetPropertyBagBase_2<TSet, TElement>
{
    type Target = crate::Unity::Properties::PropertyBag_1<TSet>;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+SetPropertyBagBase_2")]
impl<TSet: quest_hook::libil2cpp::Type, TElement: quest_hook::libil2cpp::Type> std::ops::DerefMut
    for crate::Unity::Properties::SetPropertyBagBase_2<TSet, TElement>
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+SetPropertyBagBase_2")]
impl<TSet: quest_hook::libil2cpp::Type, TElement: quest_hook::libil2cpp::Type>
    crate::Unity::Properties::SetPropertyBagBase_2<TSet, TElement>
{
    #[cfg(feature = "Unity+Properties+SetPropertyBagBase_2+SetElementProperty")]
    pub type SetElementProperty =
        crate::Unity::Properties::SetPropertyBagBase_2_SetElementProperty<TSet, TElement>;
    pub fn GetPropertiesEnumerable(
        &mut self,
        container: TSet,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::Unity::Properties::IProperty_1<TSet>>,
            >,
        >,
    >
    where
        TSet: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
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
                    .find_method::<(TSet), quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::IEnumerable_1<
                            quest_hook::libil2cpp::Gc<crate::Unity::Properties::IProperty_1<TSet>>,
                        >,
                    >, 1usize>("GetPropertiesEnumerable")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetPropertiesEnumerable",
                            1usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::Unity::Properties::IProperty_1<TSet>>,
            >,
        > = unsafe { cordl_method_info.invoke_unchecked(self, (container))? };
        Ok(__cordl_ret.into())
    }
    pub fn GetProperties_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Properties::PropertyCollection_1<TSet>>
    where
        TSet: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
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
                        crate::Unity::Properties::PropertyCollection_1<TSet>,
                        0usize,
                    >("GetProperties")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetProperties", 0usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Properties::PropertyCollection_1<TSet> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn GetProperties_ByRefMut1(
        &mut self,
        container: quest_hook::libil2cpp::ByRefMut<TSet>,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Properties::PropertyCollection_1<TSet>>
    where
        TSet: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
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
                        (quest_hook::libil2cpp::ByRefMut<TSet>),
                        crate::Unity::Properties::PropertyCollection_1<TSet>,
                        1usize,
                    >("GetProperties")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "GetProperties", 1usize
                        )
                    })
            });
        let __cordl_ret: crate::Unity::Properties::PropertyCollection_1<TSet> =
            unsafe { cordl_method_info.invoke_unchecked(self, (container))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TSet: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
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
    pub fn TryGetProperty(
        &mut self,
        container: quest_hook::libil2cpp::ByRefMut<TSet>,
        key: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        property: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::Unity::Properties::IProperty_1<TSet>>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TSet: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
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
                    .find_method::<(
                        quest_hook::libil2cpp::ByRefMut<TSet>,
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        quest_hook::libil2cpp::ByRefMut<
                            quest_hook::libil2cpp::Gc<crate::Unity::Properties::IProperty_1<TSet>>,
                        >,
                    ), bool, 3usize>("TryGetProperty")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "TryGetProperty",
                            3usize
                        )
                    })
            });
        let __cordl_ret: bool =
            unsafe { cordl_method_info.invoke_unchecked(self, (container, key, property))? };
        Ok(__cordl_ret.into())
    }
    pub fn Unity_Properties_ICollectionPropertyBagAccept_TSet__Accept(
        &mut self,
        visitor: quest_hook::libil2cpp::Gc<crate::Unity::Properties::ICollectionPropertyBagVisitor>,
        container: quest_hook::libil2cpp::ByRefMut<TSet>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSet: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
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
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<
                            crate::Unity::Properties::ICollectionPropertyBagVisitor,
                        >,
                        quest_hook::libil2cpp::ByRefMut<TSet>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "Unity.Properties.ICollectionPropertyBagAccept<TSet>.Accept",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Unity.Properties.ICollectionPropertyBagAccept<TSet>.Accept",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (visitor, container))? };
        Ok(__cordl_ret.into())
    }
    pub fn Unity_Properties_ISetPropertyBagAccept_TSet__Accept(
        &mut self,
        visitor: quest_hook::libil2cpp::Gc<crate::Unity::Properties::ISetPropertyBagVisitor>,
        container: quest_hook::libil2cpp::ByRefMut<TSet>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSet: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
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
                    .find_method::<(
                        quest_hook::libil2cpp::Gc<crate::Unity::Properties::ISetPropertyBagVisitor>,
                        quest_hook::libil2cpp::ByRefMut<TSet>,
                    ), quest_hook::libil2cpp::Void, 2usize>(
                        "Unity.Properties.ISetPropertyBagAccept<TSet>.Accept",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "Unity.Properties.ISetPropertyBagAccept<TSet>.Accept",
                            2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (visitor, container))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSet: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
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
#[cfg(feature = "cordl_class_Unity+Properties+SetPropertyBagBase_2")]
impl<TSet: quest_hook::libil2cpp::Type, TElement: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::ObjectType
    for crate::Unity::Properties::SetPropertyBagBase_2<TSet, TElement>
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+SetPropertyBagBase_2")]
impl<TSet: quest_hook::libil2cpp::Type, TElement: quest_hook::libil2cpp::Type>
    AsRef<crate::Unity::Properties::ICollectionPropertyBagAccept_1<TSet>>
    for crate::Unity::Properties::SetPropertyBagBase_2<TSet, TElement>
{
    fn as_ref(&self) -> &crate::Unity::Properties::ICollectionPropertyBagAccept_1<TSet> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+SetPropertyBagBase_2")]
impl<TSet: quest_hook::libil2cpp::Type, TElement: quest_hook::libil2cpp::Type>
    AsMut<crate::Unity::Properties::ICollectionPropertyBagAccept_1<TSet>>
    for crate::Unity::Properties::SetPropertyBagBase_2<TSet, TElement>
{
    fn as_mut(&mut self) -> &mut crate::Unity::Properties::ICollectionPropertyBagAccept_1<TSet> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+SetPropertyBagBase_2")]
impl<TSet: quest_hook::libil2cpp::Type, TElement: quest_hook::libil2cpp::Type>
    AsRef<crate::Unity::Properties::ICollectionPropertyBag_2<TSet, TElement>>
    for crate::Unity::Properties::SetPropertyBagBase_2<TSet, TElement>
{
    fn as_ref(&self) -> &crate::Unity::Properties::ICollectionPropertyBag_2<TSet, TElement> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+SetPropertyBagBase_2")]
impl<TSet: quest_hook::libil2cpp::Type, TElement: quest_hook::libil2cpp::Type>
    AsMut<crate::Unity::Properties::ICollectionPropertyBag_2<TSet, TElement>>
    for crate::Unity::Properties::SetPropertyBagBase_2<TSet, TElement>
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::Unity::Properties::ICollectionPropertyBag_2<TSet, TElement> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+SetPropertyBagBase_2")]
impl<TSet: quest_hook::libil2cpp::Type, TElement: quest_hook::libil2cpp::Type>
    AsRef<
        crate::Unity::Properties::IKeyedProperties_2<
            TSet,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    > for crate::Unity::Properties::SetPropertyBagBase_2<TSet, TElement>
{
    fn as_ref(
        &self,
    ) -> &crate::Unity::Properties::IKeyedProperties_2<
        TSet,
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+SetPropertyBagBase_2")]
impl<TSet: quest_hook::libil2cpp::Type, TElement: quest_hook::libil2cpp::Type>
    AsMut<
        crate::Unity::Properties::IKeyedProperties_2<
            TSet,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    > for crate::Unity::Properties::SetPropertyBagBase_2<TSet, TElement>
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::Unity::Properties::IKeyedProperties_2<
        TSet,
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+SetPropertyBagBase_2")]
impl<TSet: quest_hook::libil2cpp::Type, TElement: quest_hook::libil2cpp::Type>
    AsRef<crate::Unity::Properties::IPropertyBag>
    for crate::Unity::Properties::SetPropertyBagBase_2<TSet, TElement>
{
    fn as_ref(&self) -> &crate::Unity::Properties::IPropertyBag {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+SetPropertyBagBase_2")]
impl<TSet: quest_hook::libil2cpp::Type, TElement: quest_hook::libil2cpp::Type>
    AsMut<crate::Unity::Properties::IPropertyBag>
    for crate::Unity::Properties::SetPropertyBagBase_2<TSet, TElement>
{
    fn as_mut(&mut self) -> &mut crate::Unity::Properties::IPropertyBag {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+SetPropertyBagBase_2")]
impl<TSet: quest_hook::libil2cpp::Type, TElement: quest_hook::libil2cpp::Type>
    AsRef<crate::Unity::Properties::IPropertyBag_1<TSet>>
    for crate::Unity::Properties::SetPropertyBagBase_2<TSet, TElement>
{
    fn as_ref(&self) -> &crate::Unity::Properties::IPropertyBag_1<TSet> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+SetPropertyBagBase_2")]
impl<TSet: quest_hook::libil2cpp::Type, TElement: quest_hook::libil2cpp::Type>
    AsMut<crate::Unity::Properties::IPropertyBag_1<TSet>>
    for crate::Unity::Properties::SetPropertyBagBase_2<TSet, TElement>
{
    fn as_mut(&mut self) -> &mut crate::Unity::Properties::IPropertyBag_1<TSet> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+SetPropertyBagBase_2")]
impl<TSet: quest_hook::libil2cpp::Type, TElement: quest_hook::libil2cpp::Type>
    AsRef<crate::Unity::Properties::ISetPropertyBagAccept_1<TSet>>
    for crate::Unity::Properties::SetPropertyBagBase_2<TSet, TElement>
{
    fn as_ref(&self) -> &crate::Unity::Properties::ISetPropertyBagAccept_1<TSet> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+SetPropertyBagBase_2")]
impl<TSet: quest_hook::libil2cpp::Type, TElement: quest_hook::libil2cpp::Type>
    AsMut<crate::Unity::Properties::ISetPropertyBagAccept_1<TSet>>
    for crate::Unity::Properties::SetPropertyBagBase_2<TSet, TElement>
{
    fn as_mut(&mut self) -> &mut crate::Unity::Properties::ISetPropertyBagAccept_1<TSet> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+SetPropertyBagBase_2")]
impl<TSet: quest_hook::libil2cpp::Type, TElement: quest_hook::libil2cpp::Type>
    AsRef<crate::Unity::Properties::ISetPropertyBag_2<TSet, TElement>>
    for crate::Unity::Properties::SetPropertyBagBase_2<TSet, TElement>
{
    fn as_ref(&self) -> &crate::Unity::Properties::ISetPropertyBag_2<TSet, TElement> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+SetPropertyBagBase_2")]
impl<TSet: quest_hook::libil2cpp::Type, TElement: quest_hook::libil2cpp::Type>
    AsMut<crate::Unity::Properties::ISetPropertyBag_2<TSet, TElement>>
    for crate::Unity::Properties::SetPropertyBagBase_2<TSet, TElement>
{
    fn as_mut(&mut self) -> &mut crate::Unity::Properties::ISetPropertyBag_2<TSet, TElement> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "cordl_class_Unity+Properties+SetPropertyBagBase_2+SetElementProperty")]
#[repr(C)]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
pub struct SetPropertyBagBase_2_SetElementProperty<
    TSet: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::Unity::Properties::Property_2<TSet, TElement>,
    pub m_Value: TElement,
    __cordl_phantom_TSet: std::marker::PhantomData<TSet>,
    __cordl_phantom_TElement: std::marker::PhantomData<TElement>,
}
#[cfg(feature = "cordl_class_Unity+Properties+SetPropertyBagBase_2+SetElementProperty")]
unsafe impl<TSet: quest_hook::libil2cpp::Type, TElement: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::Type
    for crate::Unity::Properties::SetPropertyBagBase_2_SetElementProperty<TSet, TElement>
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Properties";
    const CLASS_NAME: &'static str = "SetPropertyBagBase`2/SetElementProperty";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find(
                "Unity.Properties",
                "SetPropertyBagBase`2/SetElementProperty",
            )
            .unwrap()
            .make_generic::<(TSet, TElement)>()
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
#[cfg(feature = "Unity+Properties+SetPropertyBagBase_2+SetElementProperty")]
impl<TSet: quest_hook::libil2cpp::Type, TElement: quest_hook::libil2cpp::Type> std::ops::Deref
    for crate::Unity::Properties::SetPropertyBagBase_2_SetElementProperty<TSet, TElement>
{
    type Target = crate::Unity::Properties::Property_2<TSet, TElement>;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+SetPropertyBagBase_2+SetElementProperty")]
impl<TSet: quest_hook::libil2cpp::Type, TElement: quest_hook::libil2cpp::Type> std::ops::DerefMut
    for crate::Unity::Properties::SetPropertyBagBase_2_SetElementProperty<TSet, TElement>
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+SetPropertyBagBase_2+SetElementProperty")]
impl<TSet: quest_hook::libil2cpp::Type, TElement: quest_hook::libil2cpp::Type>
    crate::Unity::Properties::SetPropertyBagBase_2_SetElementProperty<TSet, TElement>
{
    pub fn GetValue(
        &mut self,
        container: quest_hook::libil2cpp::ByRefMut<TSet>,
    ) -> quest_hook::libil2cpp::Result<TElement>
    where
        TSet: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
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
                    .find_method::<(quest_hook::libil2cpp::ByRefMut<TSet>), TElement, 1usize>(
                        "GetValue",
                    )
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "GetValue",
                            1usize
                        )
                    })
            });
        let __cordl_ret: TElement =
            unsafe { cordl_method_info.invoke_unchecked(self, (container))? };
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TSet: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
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
    pub fn SetValue(
        &mut self,
        container: quest_hook::libil2cpp::ByRefMut<TSet>,
        value: TElement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSet: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
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
                        (quest_hook::libil2cpp::ByRefMut<TSet>, TElement),
                        quest_hook::libil2cpp::Void,
                        2usize,
                    >("SetValue")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "SetValue", 2usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Void =
            unsafe { cordl_method_info.invoke_unchecked(self, (container, value))? };
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TSet: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
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
    pub fn get_IsReadOnly(&mut self) -> quest_hook::libil2cpp::Result<bool>
    where
        TSet: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
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
                    .find_method::<(), bool, 0usize>("get_IsReadOnly")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            <Self as quest_hook::libil2cpp::Type>::class(),
                            "get_IsReadOnly",
                            0usize
                        )
                    })
            });
        let __cordl_ret: bool = unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>>
    where
        TSet: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
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
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                        0usize,
                    >("get_Name")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_Name", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn get_ObjectKey(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>>
    where
        TSet: quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
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
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                        0usize,
                    >("get_ObjectKey")
                    .unwrap_or_else(|e| {
                        panic!(
                            "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                            < Self as quest_hook::libil2cpp::Type > ::class(),
                            "get_ObjectKey", 0usize
                        )
                    })
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject> =
            unsafe { cordl_method_info.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "cordl_class_Unity+Properties+SetPropertyBagBase_2+SetElementProperty")]
impl<TSet: quest_hook::libil2cpp::Type, TElement: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::ObjectType
    for crate::Unity::Properties::SetPropertyBagBase_2_SetElementProperty<TSet, TElement>
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+SetPropertyBagBase_2+SetElementProperty")]
impl<TSet: quest_hook::libil2cpp::Type, TElement: quest_hook::libil2cpp::Type>
    AsRef<crate::Unity::Properties::ISetElementProperty>
    for crate::Unity::Properties::SetPropertyBagBase_2_SetElementProperty<TSet, TElement>
{
    fn as_ref(&self) -> &crate::Unity::Properties::ISetElementProperty {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+SetPropertyBagBase_2+SetElementProperty")]
impl<TSet: quest_hook::libil2cpp::Type, TElement: quest_hook::libil2cpp::Type>
    AsMut<crate::Unity::Properties::ISetElementProperty>
    for crate::Unity::Properties::SetPropertyBagBase_2_SetElementProperty<TSet, TElement>
{
    fn as_mut(&mut self) -> &mut crate::Unity::Properties::ISetElementProperty {
        unsafe { std::mem::transmute(self) }
    }
}
