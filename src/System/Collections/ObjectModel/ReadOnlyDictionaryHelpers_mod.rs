#[cfg(feature = "System+Collections+ObjectModel+ReadOnlyDictionaryHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct ReadOnlyDictionaryHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Collections+ObjectModel+ReadOnlyDictionaryHelpers")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Collections::ObjectModel::ReadOnlyDictionaryHelpers {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Collections.ObjectModel";
    const CLASS_NAME: &'static str = "ReadOnlyDictionaryHelpers";
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
#[cfg(feature = "System+Collections+ObjectModel+ReadOnlyDictionaryHelpers")]
impl std::ops::Deref
for crate::System::Collections::ObjectModel::ReadOnlyDictionaryHelpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+ObjectModel+ReadOnlyDictionaryHelpers")]
impl std::ops::DerefMut
for crate::System::Collections::ObjectModel::ReadOnlyDictionaryHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+ObjectModel+ReadOnlyDictionaryHelpers")]
impl crate::System::Collections::ObjectModel::ReadOnlyDictionaryHelpers {
    pub fn CopyToNonGenericICollectionHelper<T>(
        collection: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::ICollection_1<T>,
        >,
        array: quest_hook::libil2cpp::Gc<crate::System::Array>,
        index: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Collections::ObjectModel::ReadOnlyDictionaryHelpers as quest_hook::libil2cpp::Type>::class()
            .find_static_method::<
                (
                    quest_hook::libil2cpp::Gc<
                        crate::System::Collections::Generic::ICollection_1<T>,
                    >,
                    quest_hook::libil2cpp::Gc<crate::System::Array>,
                    i32,
                ),
                quest_hook::libil2cpp::Void,
                3usize,
            >("CopyToNonGenericICollectionHelper")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::System::Collections::ObjectModel::ReadOnlyDictionaryHelpers
                    as quest_hook::libil2cpp::Type > ::class(),
                    "CopyToNonGenericICollectionHelper", 3usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked((), (collection, array, index))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Collections+ObjectModel+ReadOnlyDictionaryHelpers")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Collections::ObjectModel::ReadOnlyDictionaryHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
