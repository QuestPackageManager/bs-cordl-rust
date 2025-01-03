#[cfg(feature = "Zenject+TypeAnalyzer")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeAnalyzer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Zenject+TypeAnalyzer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::TypeAnalyzer => "Zenject"
    ."TypeAnalyzer"
);
#[cfg(feature = "Zenject+TypeAnalyzer")]
impl std::ops::Deref for crate::Zenject::TypeAnalyzer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+TypeAnalyzer")]
impl std::ops::DerefMut for crate::Zenject::TypeAnalyzer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+TypeAnalyzer")]
impl crate::Zenject::TypeAnalyzer {
    pub const ReflectionBakingFactoryMethodName: &'static str = "__zenCreate";
    pub const ReflectionBakingFieldSetterPrefix: &'static str = "__zenFieldSetter";
    pub const ReflectionBakingGetInjectInfoMethodName: &'static str = "__zenCreateInjectTypeInfo";
    pub const ReflectionBakingInjectMethodPrefix: &'static str = "__zenInjectMethod";
    pub const ReflectionBakingPropertySetterPrefix: &'static str = "__zenPropertySetter";
    pub fn CreateTypeInfoFromReflection(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateTypeInfoFromReflection", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInfoInternal(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInfoInternal", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInfo_0<T>() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInfo_Type1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInfo", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasInfo_0<T>() -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HasInfo_Type1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasInfo", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsStaticType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsStaticType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldAllowDuringValidationInternal(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShouldAllowDuringValidationInternal", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldAllowDuringValidation_0<T>() -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShouldAllowDuringValidation", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldAllowDuringValidation_Type1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShouldAllowDuringValidation", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn ShouldSkipTypeAnalysis(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ShouldSkipTypeAnalysis", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetInfo_0<T>() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetInfo", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetInfo_Type1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetInfo", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ReflectionBakingCoverageMode() -> quest_hook::libil2cpp::Result<
        crate::Zenject::ReflectionBakingCoverageModes,
    > {
        let __cordl_ret: crate::Zenject::ReflectionBakingCoverageModes = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_ReflectionBakingCoverageMode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_ReflectionBakingCoverageMode(
        value: crate::Zenject::ReflectionBakingCoverageModes,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("set_ReflectionBakingCoverageMode", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+TypeAnalyzer")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::TypeAnalyzer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
