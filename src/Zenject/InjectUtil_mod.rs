#[cfg(feature = "Zenject+InjectUtil")]
#[repr(C)]
#[derive(Debug)]
pub struct InjectUtil {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Zenject+InjectUtil")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::InjectUtil => "Zenject"."InjectUtil"
);
#[cfg(feature = "Zenject+InjectUtil")]
impl std::ops::Deref for crate::Zenject::InjectUtil {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+InjectUtil")]
impl std::ops::DerefMut for crate::Zenject::InjectUtil {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+InjectUtil")]
impl crate::Zenject::InjectUtil {
    pub fn CreateArgList(
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::TypeValuePair>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::TypeValuePair> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateArgList", (args))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateArgListExplicit_T0<T>(
        param: T,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::TypeValuePair>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::TypeValuePair> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateArgListExplicit", (param))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateArgListExplicit_TParam1_TParam2_1<TParam1, TParam2>(
        param1: TParam1,
        param2: TParam2,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::TypeValuePair>,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::TypeValuePair> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateArgListExplicit", (param1, param2))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateArgListExplicit_TParam1_TParam2_TParam3_2<TParam1, TParam2, TParam3>(
        param1: TParam1,
        param2: TParam2,
        param3: TParam3,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::TypeValuePair>,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::TypeValuePair> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateArgListExplicit", (param1, param2, param3))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateArgListExplicit_TParam1_TParam2_TParam3_TParam4_3<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
    >(
        param1: TParam1,
        param2: TParam2,
        param3: TParam3,
        param4: TParam4,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::TypeValuePair>,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::TypeValuePair> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateArgListExplicit", (param1, param2, param3, param4))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateArgListExplicit_TParam1_TParam2_TParam3_TParam4_TParam5_4<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TParam5,
    >(
        param1: TParam1,
        param2: TParam2,
        param3: TParam3,
        param4: TParam4,
        param5: TParam5,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::TypeValuePair>,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam5: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::TypeValuePair> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateArgListExplicit", (param1, param2, param3, param4, param5))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateArgListExplicit_TParam1_TParam2_TParam3_TParam4_TParam5_TParam6_5<
        TParam1,
        TParam2,
        TParam3,
        TParam4,
        TParam5,
        TParam6,
    >(
        param1: TParam1,
        param2: TParam2,
        param3: TParam3,
        param4: TParam4,
        param5: TParam5,
        param6: TParam6,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::TypeValuePair>,
    >
    where
        TParam1: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam2: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam3: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam4: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam5: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TParam6: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::TypeValuePair> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "CreateArgListExplicit",
                (param1, param2, param3, param4, param5, param6),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateTypePair<T>(
        param: T,
    ) -> quest_hook::libil2cpp::Result<crate::Zenject::TypeValuePair>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: crate::Zenject::TypeValuePair = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateTypePair", (param))?;
        Ok(__cordl_ret.into())
    }
    pub fn PopValueWithType(
        extraArgMap: quest_hook::libil2cpp::Gc<crate::Zenject::TypeValuePair>,
        injectedFieldType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        value: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("PopValueWithType", (extraArgMap, injectedFieldType, value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+InjectUtil")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::InjectUtil {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
