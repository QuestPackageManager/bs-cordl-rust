#[cfg(feature = "System+Collections+Generic+IDictionaryDebugView_2")]
#[repr(C)]
#[derive(Debug)]
pub struct IDictionaryDebugView_2<
    K: quest_hook::libil2cpp::Type,
    V: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    __cordl_phantom_K: std::marker::PhantomData<K>,
    __cordl_phantom_V: std::marker::PhantomData<V>,
}
#[cfg(feature = "System+Collections+Generic+IDictionaryDebugView_2")]
unsafe impl<
    K: quest_hook::libil2cpp::Type,
    V: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::System::Collections::Generic::IDictionaryDebugView_2<K, V> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Collections.Generic";
    const CLASS_NAME: &'static str = "IDictionaryDebugView`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "System.Collections.Generic",
                        "IDictionaryDebugView`2",
                    )
                    .unwrap()
                    .make_generic::<(K, V)>()
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
#[cfg(feature = "System+Collections+Generic+IDictionaryDebugView_2")]
impl<K: quest_hook::libil2cpp::Type, V: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::System::Collections::Generic::IDictionaryDebugView_2<K, V> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Generic+IDictionaryDebugView_2")]
impl<K: quest_hook::libil2cpp::Type, V: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::System::Collections::Generic::IDictionaryDebugView_2<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+Generic+IDictionaryDebugView_2")]
impl<
    K: quest_hook::libil2cpp::Type,
    V: quest_hook::libil2cpp::Type,
> crate::System::Collections::Generic::IDictionaryDebugView_2<K, V> {}
#[cfg(feature = "System+Collections+Generic+IDictionaryDebugView_2")]
impl<
    K: quest_hook::libil2cpp::Type,
    V: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::System::Collections::Generic::IDictionaryDebugView_2<K, V> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
