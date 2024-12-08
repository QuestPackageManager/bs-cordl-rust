#[cfg(feature = "Newtonsoft+Json+Utilities+BidirectionalDictionary_2")]
#[repr(C)]
#[derive(Debug)]
pub struct BidirectionalDictionary_2<
    TFirst: quest_hook::libil2cpp::Type,
    TSecond: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: crate::System::Object,
    pub _firstToSecond: *mut crate::System::Collections::Generic::IDictionary_2<
        TFirst,
        TSecond,
    >,
    pub _secondToFirst: *mut crate::System::Collections::Generic::IDictionary_2<
        TSecond,
        TFirst,
    >,
    pub _duplicateFirstErrorMessage: *mut crate::System::String,
    pub _duplicateSecondErrorMessage: *mut crate::System::String,
    __cordl_phantom_TFirst: std::marker::PhantomData<TFirst>,
    __cordl_phantom_TSecond: std::marker::PhantomData<TSecond>,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+BidirectionalDictionary_2")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Utilities::BidirectionalDictionary_2 < TFirst, TSecond > =>
    "Newtonsoft.Json.Utilities"."BidirectionalDictionary`2" < TFirst, TSecond >
);
#[cfg(feature = "Newtonsoft+Json+Utilities+BidirectionalDictionary_2")]
impl<
    TFirst: quest_hook::libil2cpp::Type,
    TSecond: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::Newtonsoft::Json::Utilities::BidirectionalDictionary_2<TFirst, TSecond> {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+BidirectionalDictionary_2")]
impl<
    TFirst: quest_hook::libil2cpp::Type,
    TSecond: quest_hook::libil2cpp::Type,
> std::ops::DerefMut
for crate::Newtonsoft::Json::Utilities::BidirectionalDictionary_2<TFirst, TSecond> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+BidirectionalDictionary_2")]
impl<
    TFirst: quest_hook::libil2cpp::Type,
    TSecond: quest_hook::libil2cpp::Type,
> crate::Newtonsoft::Json::Utilities::BidirectionalDictionary_2<TFirst, TSecond> {
    pub fn TryGetByFirst(
        &mut self,
        first: TFirst,
        second: quest_hook::libil2cpp::ByRefMut<TSecond>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TFirst: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TSecond: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("TryGetByFirst", (first, second))?;
        Ok(__cordl_ret)
    }
    pub fn Set(
        &mut self,
        first: TFirst,
        second: TSecond,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TFirst: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TSecond: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Set", (first, second))?;
        Ok(__cordl_ret)
    }
    pub fn TryGetBySecond(
        &mut self,
        second: TSecond,
        first: quest_hook::libil2cpp::ByRefMut<TFirst>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        TFirst: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TSecond: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("TryGetBySecond", (second, first))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TFirst: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TSecond: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IEqualityComparer_1_IEqualityComparer_1_1(
        &mut self,
        firstEqualityComparer: *mut crate::System::Collections::Generic::IEqualityComparer_1<
            TFirst,
        >,
        secondEqualityComparer: *mut crate::System::Collections::Generic::IEqualityComparer_1<
            TSecond,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TFirst: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TSecond: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (firstEqualityComparer, secondEqualityComparer))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_IEqualityComparer_1_IEqualityComparer_1_String_String2(
        &mut self,
        firstEqualityComparer: *mut crate::System::Collections::Generic::IEqualityComparer_1<
            TFirst,
        >,
        secondEqualityComparer: *mut crate::System::Collections::Generic::IEqualityComparer_1<
            TSecond,
        >,
        duplicateFirstErrorMessage: *mut crate::System::String,
        duplicateSecondErrorMessage: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        TFirst: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TSecond: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    firstEqualityComparer,
                    secondEqualityComparer,
                    duplicateFirstErrorMessage,
                    duplicateSecondErrorMessage,
                ),
            )?;
        Ok(__cordl_ret)
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn New_IEqualityComparer_1_IEqualityComparer_1_1(
        firstEqualityComparer: *mut crate::System::Collections::Generic::IEqualityComparer_1<
            TFirst,
        >,
        secondEqualityComparer: *mut crate::System::Collections::Generic::IEqualityComparer_1<
            TSecond,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (firstEqualityComparer, secondEqualityComparer))?;
        Ok(__cordl_object)
    }
    pub fn New_IEqualityComparer_1_IEqualityComparer_1_String_String2(
        firstEqualityComparer: *mut crate::System::Collections::Generic::IEqualityComparer_1<
            TFirst,
        >,
        secondEqualityComparer: *mut crate::System::Collections::Generic::IEqualityComparer_1<
            TSecond,
        >,
        duplicateFirstErrorMessage: *mut crate::System::String,
        duplicateSecondErrorMessage: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    firstEqualityComparer,
                    secondEqualityComparer,
                    duplicateFirstErrorMessage,
                    duplicateSecondErrorMessage,
                ),
            )?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+BidirectionalDictionary_2")]
impl<
    TFirst: quest_hook::libil2cpp::Type,
    TSecond: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::BidirectionalDictionary_2<TFirst, TSecond> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
