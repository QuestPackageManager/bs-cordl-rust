#[cfg(feature = "Newtonsoft+Json+Utilities+BidirectionalDictionary_2")]
#[repr(C)]
#[derive(Debug)]
pub struct BidirectionalDictionary_2<
    TFirst: quest_hook::libil2cpp::Type,
    TSecond: quest_hook::libil2cpp::Type,
> {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _firstToSecond: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IDictionary_2<TFirst, TSecond>,
    >,
    pub _secondToFirst: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::IDictionary_2<TSecond, TFirst>,
    >,
    pub _duplicateFirstErrorMessage: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    pub _duplicateSecondErrorMessage: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppString,
    >,
    __cordl_phantom_TFirst: std::marker::PhantomData<TFirst>,
    __cordl_phantom_TSecond: std::marker::PhantomData<TSecond>,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+BidirectionalDictionary_2")]
unsafe impl<
    TFirst: quest_hook::libil2cpp::Type,
    TSecond: quest_hook::libil2cpp::Type,
> quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::Utilities::BidirectionalDictionary_2<TFirst, TSecond> {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Utilities";
    const CLASS_NAME: &'static str = "BidirectionalDictionary`2";
    fn class() -> &'static quest_hook::libil2cpp::Il2CppClass {
        static CLASS: ::std::sync::OnceLock<
            &'static quest_hook::libil2cpp::Il2CppClass,
        > = ::std::sync::OnceLock::new();
        CLASS
            .get_or_init(|| {
                quest_hook::libil2cpp::Il2CppClass::find(
                        "Newtonsoft.Json.Utilities",
                        "BidirectionalDictionary`2",
                    )
                    .unwrap()
                    .make_generic::<(TFirst, TSecond)>()
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
#[cfg(feature = "Newtonsoft+Json+Utilities+BidirectionalDictionary_2")]
impl<
    TFirst: quest_hook::libil2cpp::Type,
    TSecond: quest_hook::libil2cpp::Type,
> std::ops::Deref
for crate::Newtonsoft::Json::Utilities::BidirectionalDictionary_2<TFirst, TSecond> {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TFirst: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TSecond: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_IEqualityComparer_1_IEqualityComparer_1_1(
        firstEqualityComparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEqualityComparer_1<TFirst>,
        >,
        secondEqualityComparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEqualityComparer_1<TSecond>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TFirst: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TSecond: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (firstEqualityComparer, secondEqualityComparer))?;
        Ok(__cordl_object.into())
    }
    pub fn New_IEqualityComparer_1_IEqualityComparer_1_Il2CppString_Il2CppString2(
        firstEqualityComparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEqualityComparer_1<TFirst>,
        >,
        secondEqualityComparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEqualityComparer_1<TSecond>,
        >,
        duplicateFirstErrorMessage: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        duplicateSecondErrorMessage: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        TFirst: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
        TSecond: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
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
        Ok(__cordl_object.into())
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
        Ok(__cordl_ret.into())
    }
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IEqualityComparer_1_IEqualityComparer_1_1(
        &mut self,
        firstEqualityComparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEqualityComparer_1<TFirst>,
        >,
        secondEqualityComparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEqualityComparer_1<TSecond>,
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
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_IEqualityComparer_1_IEqualityComparer_1_Il2CppString_Il2CppString2(
        &mut self,
        firstEqualityComparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEqualityComparer_1<TFirst>,
        >,
        secondEqualityComparer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEqualityComparer_1<TSecond>,
        >,
        duplicateFirstErrorMessage: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
        duplicateSecondErrorMessage: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
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
            .invoke(
                ".ctor",
                (
                    firstEqualityComparer,
                    secondEqualityComparer,
                    duplicateFirstErrorMessage,
                    duplicateSecondErrorMessage,
                ),
            )?;
        Ok(__cordl_ret.into())
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
