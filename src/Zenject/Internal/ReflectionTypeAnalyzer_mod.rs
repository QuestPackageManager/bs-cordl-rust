#[cfg(feature = "Zenject+Internal+ReflectionTypeAnalyzer")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectionTypeAnalyzer {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeAnalyzer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::Internal::ReflectionTypeAnalyzer =>
    "Zenject.Internal"."ReflectionTypeAnalyzer"
);
#[cfg(feature = "Zenject+Internal+ReflectionTypeAnalyzer")]
impl std::ops::Deref for crate::Zenject::Internal::ReflectionTypeAnalyzer {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeAnalyzer")]
impl std::ops::DerefMut for crate::Zenject::Internal::ReflectionTypeAnalyzer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeAnalyzer")]
impl crate::Zenject::Internal::ReflectionTypeAnalyzer {
    pub fn AddCustomInjectAttribute_0<T>() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddCustomInjectAttribute", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn AddCustomInjectAttribute_Gc1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AddCustomInjectAttribute", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateInjectableInfoForParam(
        parentType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        paramInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::ParameterInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::Internal::ReflectionTypeInfo_InjectParameterInfo,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::Internal::ReflectionTypeInfo_InjectParameterInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateInjectableInfoForParam", (parentType, paramInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetConstructorInfo(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Zenject::Internal::ReflectionTypeInfo_InjectConstructorInfo,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::Internal::ReflectionTypeInfo_InjectConstructorInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetConstructorInfo", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFieldInfos(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Zenject::Internal::ReflectionTypeInfo_InjectFieldInfo,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Zenject::Internal::ReflectionTypeInfo_InjectFieldInfo,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFieldInfos", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInjectableInfoForMember(
        parentType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        memInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetInjectableInfoForMember", (parentType, memInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMethodInfos(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Zenject::Internal::ReflectionTypeInfo_InjectMethodInfo,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Zenject::Internal::ReflectionTypeInfo_InjectMethodInfo,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMethodInfos", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetPropertyInfos(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Zenject::Internal::ReflectionTypeInfo_InjectPropertyInfo,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<
                crate::Zenject::Internal::ReflectionTypeInfo_InjectPropertyInfo,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetPropertyInfos", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetReflectionInfo(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::Internal::ReflectionTypeInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::Internal::ReflectionTypeInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetReflectionInfo", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetInjectConstructor(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::ConstructorInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ConstructorInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetInjectConstructor", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeAnalyzer")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::Internal::ReflectionTypeAnalyzer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
