#[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_TestHelpers {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::HoudiniEngineUnity::HEU_TestHelpers =>
    "HoudiniEngineUnity"."HEU_TestHelpers"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_TestHelpers {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers")]
impl std::ops::DerefMut for crate::HoudiniEngineUnity::HEU_TestHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers")]
impl crate::HoudiniEngineUnity::HEU_TestHelpers {
    #[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers+RequireClass_1")]
    pub type RequireClass_1<T: quest_hook::libil2cpp::Type> = crate::HoudiniEngineUnity::HEU_TestHelpers_RequireClass_1<
        T,
    >;
    #[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers+RequireStruct_1")]
    pub type RequireStruct_1<T: quest_hook::libil2cpp::Type> = crate::HoudiniEngineUnity::HEU_TestHelpers_RequireStruct_1<
        T,
    >;
    pub fn AssertTrueLogEquivalent_Gc_Gc1(
        a: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        b: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        result: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AssertTrueLogEquivalent",
                (a, b, result, header, subject, optional1, optional2, optional3),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertTrueLogEquivalent_Gc_Gc10(
        a: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        b: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
        result: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AssertTrueLogEquivalent",
                (a, b, result, header, subject, optional1, optional2, optional3),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertTrueLogEquivalent_Gc_Gc11<T>(
        a: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<quest_hook::libil2cpp::Gc<T>>,
        >,
        b: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<quest_hook::libil2cpp::Gc<T>>,
        >,
        result: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AssertTrueLogEquivalent",
                (a, b, result, header, subject, optional1, optional2, optional3),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertTrueLogEquivalent_Gc_Gc12<T>(
        a: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<quest_hook::libil2cpp::Gc<T>>,
        >,
        b: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<quest_hook::libil2cpp::Gc<T>>,
        >,
        result: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AssertTrueLogEquivalent",
                (a, b, result, header, subject, optional1, optional2, optional3),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertTrueLogEquivalent_Gc_Gc2<T>(
        a: quest_hook::libil2cpp::Gc<T>,
        b: quest_hook::libil2cpp::Gc<T>,
        result: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AssertTrueLogEquivalent",
                (a, b, result, header, subject, optional1, optional2, optional3),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertTrueLogEquivalent_Gc_Gc3<T>(
        a: quest_hook::libil2cpp::Gc<T>,
        b: quest_hook::libil2cpp::Gc<T>,
        result: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AssertTrueLogEquivalent",
                (a, b, result, header, subject, optional1, optional2, optional3),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertTrueLogEquivalent_Gc_Gc4(
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        result: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AssertTrueLogEquivalent",
                (a, b, result, header, subject, optional1, optional2, optional3),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertTrueLogEquivalent_Gc_Gc7<T>(
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Gc<T>>,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Gc<T>>,
        result: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AssertTrueLogEquivalent",
                (a, b, result, header, subject, optional1, optional2, optional3),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertTrueLogEquivalent_Gc_Gc8<T>(
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Gc<T>>,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Gc<T>>,
        result: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AssertTrueLogEquivalent",
                (a, b, result, header, subject, optional1, optional2, optional3),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertTrueLogEquivalent_Gc_Gc9<T>(
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        result: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AssertTrueLogEquivalent",
                (a, b, result, header, subject, optional1, optional2, optional3),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertTrueLogEquivalent_Gc_Gc_Gc5<T>(
        a: quest_hook::libil2cpp::Gc<T>,
        b: quest_hook::libil2cpp::Gc<T>,
        result: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl__: quest_hook::libil2cpp::Gc<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AssertTrueLogEquivalent",
                (
                    a,
                    b,
                    result,
                    header,
                    subject,
                    optional1,
                    optional2,
                    optional3,
                    _cordl__,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertTrueLogEquivalent_Gc_Gc_Gc6<T>(
        a: quest_hook::libil2cpp::Gc<T>,
        b: quest_hook::libil2cpp::Gc<T>,
        result: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl__: quest_hook::libil2cpp::Gc<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AssertTrueLogEquivalent",
                (
                    a,
                    b,
                    result,
                    header,
                    subject,
                    optional1,
                    optional2,
                    optional3,
                    _cordl__,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn AssertTrueLogEquivalent_T_T_Gc0<T>(
        a: T,
        b: T,
        result: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        _cordl__: quest_hook::libil2cpp::Gc<T>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "AssertTrueLogEquivalent",
                (
                    a,
                    b,
                    result,
                    header,
                    subject,
                    optional1,
                    optional2,
                    optional3,
                    _cordl__,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn PrintTestLogAndSetResult(
        expression: bool,
        result: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional1: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional2: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        optional3: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "PrintTestLogAndSetResult",
                (expression, result, header, subject, optional1, optional2, optional3),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldBeTested_Gc_Gc1(
        a: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        b: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        bResult: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShouldBeTested", (a, b, bResult, header, subject))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldBeTested_Gc_Gc2<T>(
        a: quest_hook::libil2cpp::Gc<T>,
        b: quest_hook::libil2cpp::Gc<T>,
        bResult: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShouldBeTested", (a, b, bResult, header, subject))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldBeTested_Gc_Gc3<T>(
        a: quest_hook::libil2cpp::Gc<T>,
        b: quest_hook::libil2cpp::Gc<T>,
        bResult: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShouldBeTested", (a, b, bResult, header, subject))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldBeTested_Gc_Gc4<T>(
        a: quest_hook::libil2cpp::Gc<T>,
        b: quest_hook::libil2cpp::Gc<T>,
        bResult: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShouldBeTested", (a, b, bResult, header, subject))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldBeTested_Gc_Gc5<T>(
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        bResult: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShouldBeTested", (a, b, bResult, header, subject))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldBeTested_Gc_Gc6(
        a: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        b: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bResult: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShouldBeTested", (a, b, bResult, header, subject))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldBeTested_T_T0<T>(
        a: T,
        b: T,
        bResult: quest_hook::libil2cpp::ByRefMut<bool>,
        header: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        subject: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShouldBeTested", (a, b, bResult, header, subject))?;
        Ok(__cordl_ret.into())
    }
    pub fn TestOutputObjectEquivalence(
        a: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
        b: quest_hook::libil2cpp::Gc<crate::UnityEngine::GameObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TestOutputObjectEquivalence", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers")]
impl quest_hook::libil2cpp::ObjectType for crate::HoudiniEngineUnity::HEU_TestHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers+RequireClass_1")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_TestHelpers_RequireClass_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers+RequireClass_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_TestHelpers_RequireClass_1 < T > => "HoudiniEngineUnity"
    ."HEU_TestHelpers/RequireClass`1" < T >
);
#[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers+RequireClass_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::HoudiniEngineUnity::HEU_TestHelpers_RequireClass_1<T> {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers+RequireClass_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::HoudiniEngineUnity::HEU_TestHelpers_RequireClass_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers+RequireClass_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::HoudiniEngineUnity::HEU_TestHelpers_RequireClass_1<T> {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers+RequireClass_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_TestHelpers_RequireClass_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers+RequireStruct_1")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_TestHelpers_RequireStruct_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers+RequireStruct_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_TestHelpers_RequireStruct_1 < T > => "HoudiniEngineUnity"
    ."HEU_TestHelpers/RequireStruct`1" < T >
);
#[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers+RequireStruct_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::HoudiniEngineUnity::HEU_TestHelpers_RequireStruct_1<T> {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers+RequireStruct_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::HoudiniEngineUnity::HEU_TestHelpers_RequireStruct_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers+RequireStruct_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::HoudiniEngineUnity::HEU_TestHelpers_RequireStruct_1<T> {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(&mut self) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_TestHelpers+RequireStruct_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_TestHelpers_RequireStruct_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
