#[cfg(feature = "ShuffleExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct ShuffleExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ShuffleExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::ShuffleExtensions => ""
    ."ShuffleExtensions"
);
#[cfg(feature = "ShuffleExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::ShuffleExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ShuffleExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::ShuffleExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ShuffleExtensions")]
impl crate::GlobalNamespace::ShuffleExtensions {
    pub fn PickRandomElementsWithTombstone<T>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        >,
        limit: i32,
        count: i32,
        random: quest_hook::libil2cpp::Gc<crate::System::Random>,
        tombstone: T,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IEnumerable_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "PickRandomElementsWithTombstone",
                (source, limit, count, random, tombstone),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn Shuffle<T>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        >,
        random: quest_hook::libil2cpp::Gc<crate::System::Random>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IEnumerable_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Shuffle", (source, random))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShuffleInPlace<T>(
        list: quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IList_1<T>>,
        random: quest_hook::libil2cpp::Gc<crate::System::Random>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShuffleInPlace", (list, random))?;
        Ok(__cordl_ret.into())
    }
    pub fn TakeWithTombstone<T>(
        source: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        >,
        limit: i32,
        tombstone: T,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IEnumerable_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TakeWithTombstone", (source, limit, tombstone))?;
        Ok(__cordl_ret.into())
    }
    pub fn ZipSkipTombstone(
        collection1: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
        collection2: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<i32>,
        >,
        collection2Tombstone: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::System::ValueTuple_2<i32, i32>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::System::ValueTuple_2<i32, i32>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ZipSkipTombstone",
                (collection1, collection2, collection2Tombstone),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ShuffleExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::ShuffleExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
