#[cfg(feature = "cordl_class_Unity+Properties+IDictionaryPropertyBag_3")]
#[derive(Debug)]
#[repr(C)]
pub struct IDictionaryPropertyBag_3<
    TDictionary: quest_hook::libil2cpp::Type,
    TKey: quest_hook::libil2cpp::Type,
    TValue: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_TDictionary: std::marker::PhantomData<TDictionary>,
    __cordl_phantom_TKey: std::marker::PhantomData<TKey>,
    __cordl_phantom_TValue: std::marker::PhantomData<TValue>,
}
#[cfg(feature = "cordl_class_Unity+Properties+IDictionaryPropertyBag_3")]
unsafe impl<
        TDictionary: quest_hook::libil2cpp::Type,
        TKey: quest_hook::libil2cpp::Type,
        TValue: quest_hook::libil2cpp::Type,
    > quest_hook::libil2cpp::Type
    for crate::Unity::Properties::IDictionaryPropertyBag_3<TDictionary, TKey, TValue>
{
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Unity.Properties";
    const CLASS_NAME: &'static str = "IDictionaryPropertyBag`3";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<&'static quest_hook::libil2cpp::Il2CppClass> =
            ::std::sync::OnceLock::new();
        CLASS.get_or_init(|| {
            quest_hook::libil2cpp::Il2CppClass::find("Unity.Properties", "IDictionaryPropertyBag`3")
                .unwrap()
                .make_generic::<(TDictionary, TKey, TValue)>()
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
#[cfg(feature = "Unity+Properties+IDictionaryPropertyBag_3")]
impl<
        TDictionary: quest_hook::libil2cpp::Type,
        TKey: quest_hook::libil2cpp::Type,
        TValue: quest_hook::libil2cpp::Type,
    > std::ops::Deref
    for crate::Unity::Properties::IDictionaryPropertyBag_3<TDictionary, TKey, TValue>
{
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+IDictionaryPropertyBag_3")]
impl<
        TDictionary: quest_hook::libil2cpp::Type,
        TKey: quest_hook::libil2cpp::Type,
        TValue: quest_hook::libil2cpp::Type,
    > std::ops::DerefMut
    for crate::Unity::Properties::IDictionaryPropertyBag_3<TDictionary, TKey, TValue>
{
    fn deref_mut(&mut self) -> &mut <Self as std::ops::Deref>::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Properties+IDictionaryPropertyBag_3")]
impl<
        TDictionary: quest_hook::libil2cpp::Type,
        TKey: quest_hook::libil2cpp::Type,
        TValue: quest_hook::libil2cpp::Type,
    > crate::Unity::Properties::IDictionaryPropertyBag_3<TDictionary, TKey, TValue>
{
    pub fn from_object_mut(object_param: *mut quest_hook::libil2cpp::Il2CppObject) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "cordl_class_Unity+Properties+IDictionaryPropertyBag_3")]
impl<
        TDictionary: quest_hook::libil2cpp::Type,
        TKey: quest_hook::libil2cpp::Type,
        TValue: quest_hook::libil2cpp::Type,
    > quest_hook::libil2cpp::ObjectType
    for crate::Unity::Properties::IDictionaryPropertyBag_3<TDictionary, TKey, TValue>
{
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Unity+Properties+IDictionaryPropertyBag_3")]
impl<
        TDictionary: quest_hook::libil2cpp::Type,
        TKey: quest_hook::libil2cpp::Type,
        TValue: quest_hook::libil2cpp::Type,
    > AsRef<crate::Unity::Properties::ICollectionPropertyBagAccept_1<TDictionary>>
    for crate::Unity::Properties::IDictionaryPropertyBag_3<TDictionary, TKey, TValue>
{
    fn as_ref(&self) -> &crate::Unity::Properties::ICollectionPropertyBagAccept_1<TDictionary> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+IDictionaryPropertyBag_3")]
impl<
        TDictionary: quest_hook::libil2cpp::Type,
        TKey: quest_hook::libil2cpp::Type,
        TValue: quest_hook::libil2cpp::Type,
    > AsMut<crate::Unity::Properties::ICollectionPropertyBagAccept_1<TDictionary>>
    for crate::Unity::Properties::IDictionaryPropertyBag_3<TDictionary, TKey, TValue>
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::Unity::Properties::ICollectionPropertyBagAccept_1<TDictionary> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+IDictionaryPropertyBag_3")]
impl<
        TDictionary: quest_hook::libil2cpp::Type,
        TKey: quest_hook::libil2cpp::Type,
        TValue: quest_hook::libil2cpp::Type,
    >
    AsRef<
        crate::Unity::Properties::ICollectionPropertyBag_2<
            TDictionary,
            crate::System::Collections::Generic::KeyValuePair_2<TKey, TValue>,
        >,
    > for crate::Unity::Properties::IDictionaryPropertyBag_3<TDictionary, TKey, TValue>
{
    fn as_ref(
        &self,
    ) -> &crate::Unity::Properties::ICollectionPropertyBag_2<
        TDictionary,
        crate::System::Collections::Generic::KeyValuePair_2<TKey, TValue>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+IDictionaryPropertyBag_3")]
impl<
        TDictionary: quest_hook::libil2cpp::Type,
        TKey: quest_hook::libil2cpp::Type,
        TValue: quest_hook::libil2cpp::Type,
    >
    AsMut<
        crate::Unity::Properties::ICollectionPropertyBag_2<
            TDictionary,
            crate::System::Collections::Generic::KeyValuePair_2<TKey, TValue>,
        >,
    > for crate::Unity::Properties::IDictionaryPropertyBag_3<TDictionary, TKey, TValue>
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::Unity::Properties::ICollectionPropertyBag_2<
        TDictionary,
        crate::System::Collections::Generic::KeyValuePair_2<TKey, TValue>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+IDictionaryPropertyBag_3")]
impl<
        TDictionary: quest_hook::libil2cpp::Type,
        TKey: quest_hook::libil2cpp::Type,
        TValue: quest_hook::libil2cpp::Type,
    > AsRef<crate::Unity::Properties::IDictionaryPropertyBagAccept_1<TDictionary>>
    for crate::Unity::Properties::IDictionaryPropertyBag_3<TDictionary, TKey, TValue>
{
    fn as_ref(&self) -> &crate::Unity::Properties::IDictionaryPropertyBagAccept_1<TDictionary> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+IDictionaryPropertyBag_3")]
impl<
        TDictionary: quest_hook::libil2cpp::Type,
        TKey: quest_hook::libil2cpp::Type,
        TValue: quest_hook::libil2cpp::Type,
    > AsMut<crate::Unity::Properties::IDictionaryPropertyBagAccept_1<TDictionary>>
    for crate::Unity::Properties::IDictionaryPropertyBag_3<TDictionary, TKey, TValue>
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::Unity::Properties::IDictionaryPropertyBagAccept_1<TDictionary> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+IDictionaryPropertyBag_3")]
impl<
        TDictionary: quest_hook::libil2cpp::Type,
        TKey: quest_hook::libil2cpp::Type,
        TValue: quest_hook::libil2cpp::Type,
    >
    AsRef<
        crate::Unity::Properties::IKeyedProperties_2<
            TDictionary,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    > for crate::Unity::Properties::IDictionaryPropertyBag_3<TDictionary, TKey, TValue>
{
    fn as_ref(
        &self,
    ) -> &crate::Unity::Properties::IKeyedProperties_2<
        TDictionary,
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+IDictionaryPropertyBag_3")]
impl<
        TDictionary: quest_hook::libil2cpp::Type,
        TKey: quest_hook::libil2cpp::Type,
        TValue: quest_hook::libil2cpp::Type,
    >
    AsMut<
        crate::Unity::Properties::IKeyedProperties_2<
            TDictionary,
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    > for crate::Unity::Properties::IDictionaryPropertyBag_3<TDictionary, TKey, TValue>
{
    fn as_mut(
        &mut self,
    ) -> &mut crate::Unity::Properties::IKeyedProperties_2<
        TDictionary,
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+IDictionaryPropertyBag_3")]
impl<
        TDictionary: quest_hook::libil2cpp::Type,
        TKey: quest_hook::libil2cpp::Type,
        TValue: quest_hook::libil2cpp::Type,
    > AsRef<crate::Unity::Properties::IPropertyBag>
    for crate::Unity::Properties::IDictionaryPropertyBag_3<TDictionary, TKey, TValue>
{
    fn as_ref(&self) -> &crate::Unity::Properties::IPropertyBag {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+IDictionaryPropertyBag_3")]
impl<
        TDictionary: quest_hook::libil2cpp::Type,
        TKey: quest_hook::libil2cpp::Type,
        TValue: quest_hook::libil2cpp::Type,
    > AsMut<crate::Unity::Properties::IPropertyBag>
    for crate::Unity::Properties::IDictionaryPropertyBag_3<TDictionary, TKey, TValue>
{
    fn as_mut(&mut self) -> &mut crate::Unity::Properties::IPropertyBag {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+IDictionaryPropertyBag_3")]
impl<
        TDictionary: quest_hook::libil2cpp::Type,
        TKey: quest_hook::libil2cpp::Type,
        TValue: quest_hook::libil2cpp::Type,
    > AsRef<crate::Unity::Properties::IPropertyBag_1<TDictionary>>
    for crate::Unity::Properties::IDictionaryPropertyBag_3<TDictionary, TKey, TValue>
{
    fn as_ref(&self) -> &crate::Unity::Properties::IPropertyBag_1<TDictionary> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Unity+Properties+IDictionaryPropertyBag_3")]
impl<
        TDictionary: quest_hook::libil2cpp::Type,
        TKey: quest_hook::libil2cpp::Type,
        TValue: quest_hook::libil2cpp::Type,
    > AsMut<crate::Unity::Properties::IPropertyBag_1<TDictionary>>
    for crate::Unity::Properties::IDictionaryPropertyBag_3<TDictionary, TKey, TValue>
{
    fn as_mut(&mut self) -> &mut crate::Unity::Properties::IPropertyBag_1<TDictionary> {
        unsafe { std::mem::transmute(self) }
    }
}
