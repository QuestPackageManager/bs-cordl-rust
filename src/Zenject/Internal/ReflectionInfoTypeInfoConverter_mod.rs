#[cfg(feature = "Zenject+Internal+ReflectionInfoTypeInfoConverter")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectionInfoTypeInfoConverter {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Zenject+Internal+ReflectionInfoTypeInfoConverter")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Zenject::Internal::ReflectionInfoTypeInfoConverter {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Zenject.Internal";
    const CLASS_NAME: &'static str = "ReflectionInfoTypeInfoConverter";
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
#[cfg(feature = "Zenject+Internal+ReflectionInfoTypeInfoConverter")]
impl std::ops::Deref for crate::Zenject::Internal::ReflectionInfoTypeInfoConverter {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionInfoTypeInfoConverter")]
impl std::ops::DerefMut for crate::Zenject::Internal::ReflectionInfoTypeInfoConverter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionInfoTypeInfoConverter")]
impl crate::Zenject::Internal::ReflectionInfoTypeInfoConverter {
    pub fn ConvertConstructor(
        injectConstructor: quest_hook::libil2cpp::Gc<
            crate::Zenject::Internal::ReflectionTypeInfo_InjectConstructorInfo,
        >,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo_InjectConstructorInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::InjectTypeInfo_InjectConstructorInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertConstructor", (injectConstructor, _cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertField(
        parentType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        injectField: quest_hook::libil2cpp::Gc<
            crate::Zenject::Internal::ReflectionTypeInfo_InjectFieldInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo_InjectMemberInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::InjectTypeInfo_InjectMemberInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertField", (parentType, injectField))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertMethod(
        injectMethod: quest_hook::libil2cpp::Gc<
            crate::Zenject::Internal::ReflectionTypeInfo_InjectMethodInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo_InjectMethodInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::InjectTypeInfo_InjectMethodInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertMethod", (injectMethod))?;
        Ok(__cordl_ret.into())
    }
    pub fn ConvertProperty(
        parentType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        injectProperty: quest_hook::libil2cpp::Gc<
            crate::Zenject::Internal::ReflectionTypeInfo_InjectPropertyInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::InjectTypeInfo_InjectMemberInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::InjectTypeInfo_InjectMemberInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ConvertProperty", (parentType, injectProperty))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAllFields(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
        flags: crate::System::Reflection::BindingFlags,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::FieldInfo>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::FieldInfo>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAllFields", (t, flags))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetOnlyPropertySetter(
        parentType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        propertyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ZenMemberSetterMethod>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ZenMemberSetterMethod,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetOnlyPropertySetter", (parentType, propertyName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetSetter(
        parentType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        memInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ZenMemberSetterMethod>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ZenMemberSetterMethod,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetSetter", (parentType, memInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryCreateActionForMethod(
        methodInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ZenInjectMethod>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::ZenInjectMethod> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryCreateActionForMethod", (methodInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryCreateFactoryMethod(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        reflectionInfo: quest_hook::libil2cpp::Gc<
            crate::Zenject::Internal::ReflectionTypeInfo_InjectConstructorInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ZenFactoryMethod>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::ZenFactoryMethod> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryCreateFactoryMethod", (_cordl_type, reflectionInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryCreateFactoryMethodCompiledLambdaExpression(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        constructor: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ConstructorInfo,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ZenFactoryMethod>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::ZenFactoryMethod> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "TryCreateFactoryMethodCompiledLambdaExpression",
                (_cordl_type, constructor),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetSetterAsCompiledExpression(
        parentType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        memInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ZenMemberSetterMethod>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ZenMemberSetterMethod,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetSetterAsCompiledExpression", (parentType, memInfo))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionInfoTypeInfoConverter")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::Internal::ReflectionInfoTypeInfoConverter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
