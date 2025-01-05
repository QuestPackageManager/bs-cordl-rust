#[cfg(feature = "System+Tuple")]
#[repr(C)]
#[derive(Debug)]
pub struct Tuple {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "System+Tuple")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Tuple => "System"."Tuple"
);
#[cfg(feature = "System+Tuple")]
impl std::ops::Deref for crate::System::Tuple {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Tuple")]
impl std::ops::DerefMut for crate::System::Tuple {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Tuple")]
impl crate::System::Tuple {
    pub fn CombineHashCodes_i32_1(
        h1: i32,
        h2: i32,
        h3: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CombineHashCodes", (h1, h2, h3))?;
        Ok(__cordl_ret.into())
    }
    pub fn CombineHashCodes_i32_i32_0(
        h1: i32,
        h2: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CombineHashCodes", (h1, h2))?;
        Ok(__cordl_ret.into())
    }
    pub fn CombineHashCodes_i32_i32_2(
        h1: i32,
        h2: i32,
        h3: i32,
        h4: i32,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CombineHashCodes", (h1, h2, h3, h4))?;
        Ok(__cordl_ret.into())
    }
    pub fn Create_T1_T2_0<T1, T2>(
        item1: T1,
        item2: T2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<T1, T2>>
    where
        T1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        T2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<T1, T2> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Create", (item1, item2))?;
        Ok(__cordl_ret.into())
    }
    pub fn Create_T3_1<T1, T2, T3>(
        item1: T1,
        item2: T2,
        item3: T3,
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
            .invoke("Create", (item1, item2, item3))?;
        Ok(__cordl_ret.into())
    }
    pub fn Create_T3_T4_2<T1, T2, T3, T4>(
        item1: T1,
        item2: T2,
        item3: T3,
        item4: T4,
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
            .invoke("Create", (item1, item2, item3, item4))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Tuple")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Tuple {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
