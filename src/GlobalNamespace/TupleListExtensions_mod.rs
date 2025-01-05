#[cfg(feature = "TupleListExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct TupleListExtensions {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "TupleListExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::TupleListExtensions => ""
    ."TupleListExtensions"
);
#[cfg(feature = "TupleListExtensions")]
impl std::ops::Deref for crate::GlobalNamespace::TupleListExtensions {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "TupleListExtensions")]
impl std::ops::DerefMut for crate::GlobalNamespace::TupleListExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "TupleListExtensions")]
impl crate::GlobalNamespace::TupleListExtensions {
    pub fn Add_Gc_T1_T2_0<T1, T2>(
        list: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Gc<T1, T2>>,
        item1: T1,
        item2: T2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Add", (list, item1, item2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Add_T3_1<T1, T2, T3>(
        list: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Gc<T1, T2, T3>>,
        item1: T1,
        item2: T2,
        item3: T3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Add", (list, item1, item2, item3))?;
        Ok(__cordl_ret.into())
    }
    pub fn Add_T3_T4_2<T1, T2, T3, T4>(
        list: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Gc<T1, T2, T3, T4>>,
        item1: T1,
        item2: T2,
        item3: T3,
        item4: T4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Add", (list, item1, item2, item3, item4))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "TupleListExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::TupleListExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
