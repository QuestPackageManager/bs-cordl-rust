#[cfg(feature = "cordl_class_Unity+Properties+ICollectionPropertyBag_2")]
#[cfg_attr(feature = "derive_Debug", derive(Debug))]
#[repr(C)]
pub struct ICollectionPropertyBag_2<
    TCollection: quest_hook::libil2cpp::Type,
    TElement: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_TCollection: std::marker::PhantomData<TCollection>,
    __cordl_phantom_TElement: std::marker::PhantomData<TElement>,
}
#[cfg(feature = "cordl_class_Unity+Properties+ICollectionPropertyBag_2")]
unsafe impl<TCollection: quest_hook::libil2cpp::Type, TElement: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::Type
    for crate::Unity::Properties::ICollectionPropertyBag_2<TCollection, TElement>
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Properties";
    const CLASS_NAME: &'static str = "ICollectionPropertyBag`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find("Unity.Properties", "ICollectionPropertyBag`2")
                .unwrap()
                .make_generic::<(TCollection, TElement)>()
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
#[cfg(feature = "Unity+Properties+ICollectionPropertyBag_2")]
impl<TCollection: quest_hook::libil2cpp::Type, TElement: quest_hook::libil2cpp::Type>
    std::ops::Deref for crate::Unity::Properties::ICollectionPropertyBag_2<TCollection, TElement>
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+ICollectionPropertyBag_2")]
impl<TCollection: quest_hook::libil2cpp::Type, TElement: quest_hook::libil2cpp::Type>
    std::ops::DerefMut
    for crate::Unity::Properties::ICollectionPropertyBag_2<TCollection, TElement>
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+ICollectionPropertyBag_2")]
impl<TCollection: quest_hook::libil2cpp::Type, TElement: quest_hook::libil2cpp::Type>
    crate::Unity::Properties::ICollectionPropertyBag_2<TCollection, TElement>
{
    pub fn from_object_mut(object_param: *mut quest_hook::libil2cpp::Il2CppObject) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "cordl_class_Unity+Properties+ICollectionPropertyBag_2")]
impl<TCollection: quest_hook::libil2cpp::Type, TElement: quest_hook::libil2cpp::Type>
    quest_hook::libil2cpp::ObjectType
    for crate::Unity::Properties::ICollectionPropertyBag_2<TCollection, TElement>
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+ICollectionPropertyBag_2")]
impl<TCollection: quest_hook::libil2cpp::Type, TElement: quest_hook::libil2cpp::Type>
    AsRef<crate::Unity::Properties::ICollectionPropertyBagAccept_1<TCollection>>
    for crate::Unity::Properties::ICollectionPropertyBag_2<TCollection, TElement>
{
    fn as_ref(&self) -> &crate::Unity::Properties::ICollectionPropertyBagAccept_1<TCollection> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+ICollectionPropertyBag_2")]
impl<TCollection: quest_hook::libil2cpp::Type, TElement: quest_hook::libil2cpp::Type>
    AsMut<crate::Unity::Properties::ICollectionPropertyBagAccept_1<TCollection>>
    for crate::Unity::Properties::ICollectionPropertyBag_2<TCollection, TElement>
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::Unity::Properties::ICollectionPropertyBagAccept_1<TCollection> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+ICollectionPropertyBag_2")]
impl<TCollection: quest_hook::libil2cpp::Type, TElement: quest_hook::libil2cpp::Type>
    AsRef<crate::Unity::Properties::IPropertyBag>
    for crate::Unity::Properties::ICollectionPropertyBag_2<TCollection, TElement>
{
    fn as_ref(&self) -> &crate::Unity::Properties::IPropertyBag {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+ICollectionPropertyBag_2")]
impl<TCollection: quest_hook::libil2cpp::Type, TElement: quest_hook::libil2cpp::Type>
    AsMut<crate::Unity::Properties::IPropertyBag>
    for crate::Unity::Properties::ICollectionPropertyBag_2<TCollection, TElement>
{
    fn as_mut(&mut self) -> &mut crate::Unity::Properties::IPropertyBag {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+ICollectionPropertyBag_2")]
impl<TCollection: quest_hook::libil2cpp::Type, TElement: quest_hook::libil2cpp::Type>
    AsRef<crate::Unity::Properties::IPropertyBag_1<TCollection>>
    for crate::Unity::Properties::ICollectionPropertyBag_2<TCollection, TElement>
{
    fn as_ref(&self) -> &crate::Unity::Properties::IPropertyBag_1<TCollection> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+ICollectionPropertyBag_2")]
impl<TCollection: quest_hook::libil2cpp::Type, TElement: quest_hook::libil2cpp::Type>
    AsMut<crate::Unity::Properties::IPropertyBag_1<TCollection>>
    for crate::Unity::Properties::ICollectionPropertyBag_2<TCollection, TElement>
{
    fn as_mut(&mut self) -> &mut crate::Unity::Properties::IPropertyBag_1<TCollection> {
        unsafe { std::mem::transmute(self) }
    }
}
