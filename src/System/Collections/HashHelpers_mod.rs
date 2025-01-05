#[cfg(feature = "System+Collections+HashHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct HashHelpers {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Collections+HashHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Collections::HashHelpers =>
    "System.Collections"."HashHelpers"
);
#[cfg(feature = "System+Collections+HashHelpers")]
impl std::ops::Deref for crate::System::Collections::HashHelpers {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+HashHelpers")]
impl std::ops::DerefMut for crate::System::Collections::HashHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Collections+HashHelpers")]
impl crate::System::Collections::HashHelpers {
    pub fn ExpandPrime(oldSize: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExpandPrime", (oldSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPrime(min: i32) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPrime", (min))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPrime(candidate: i32) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPrime", (candidate))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SerializationInfoTable() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::ConditionalWeakTable_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Serialization::SerializationInfo,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::ConditionalWeakTable_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                quest_hook::libil2cpp::Gc<
                    crate::System::Runtime::Serialization::SerializationInfo,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_SerializationInfoTable", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Collections+HashHelpers")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Collections::HashHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
