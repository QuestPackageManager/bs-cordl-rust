#[cfg(feature = "ModestTree+Util+ValuePair")]
#[repr(C)]
#[derive(Debug)]
pub struct ValuePair {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "ModestTree+Util+ValuePair")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::ModestTree::Util::ValuePair => "ModestTree.Util"
    ."ValuePair"
);
#[cfg(feature = "ModestTree+Util+ValuePair")]
impl std::ops::Deref for crate::ModestTree::Util::ValuePair {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ModestTree+Util+ValuePair")]
impl std::ops::DerefMut for crate::ModestTree::Util::ValuePair {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ModestTree+Util+ValuePair")]
impl crate::ModestTree::Util::ValuePair {
    pub fn New_T1_T2_0<T1, T2>(
        first: T1,
        second: T2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<T1, T2>>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<T1, T2> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("New", (first, second))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_T3_1<T1, T2, T3>(
        first: T1,
        second: T2,
        third: T3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<T1, T2, T3>>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<T1, T2, T3> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("New", (first, second, third))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_T3_T4_2<T1, T2, T3, T4>(
        first: T1,
        second: T2,
        third: T3,
        fourth: T4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<T1, T2, T3, T4>>
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
        let __cordl_ret: quest_hook::libil2cpp::Gc<T1, T2, T3, T4> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("New", (first, second, third, fourth))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ModestTree+Util+ValuePair")]
impl quest_hook::libil2cpp::ObjectType for crate::ModestTree::Util::ValuePair {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
