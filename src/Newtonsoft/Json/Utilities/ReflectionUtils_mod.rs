#[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectionUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::ReflectionUtils =>
    "Newtonsoft.Json.Utilities"."ReflectionUtils"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionUtils")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::ReflectionUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionUtils")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::ReflectionUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionUtils")]
impl crate::Newtonsoft::Json::Utilities::ReflectionUtils {
    #[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionUtils+__c")]
    pub type __c = crate::Newtonsoft::Json::Utilities::ReflectionUtils___c;
    #[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionUtils+__c__DisplayClass31_0")]
    pub type __c__DisplayClass31_0 = crate::Newtonsoft::Json::Utilities::ReflectionUtils___c__DisplayClass31_0;
    #[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionUtils+__c__DisplayClass44_0")]
    pub type __c__DisplayClass44_0 = crate::Newtonsoft::Json::Utilities::ReflectionUtils___c__DisplayClass44_0;
    #[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionUtils+__c__DisplayClass44_1")]
    pub type __c__DisplayClass44_1 = crate::Newtonsoft::Json::Utilities::ReflectionUtils___c__DisplayClass44_1;
    #[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionUtils+__c__DisplayClass45_0")]
    pub type __c__DisplayClass45_0 = crate::Newtonsoft::Json::Utilities::ReflectionUtils___c__DisplayClass45_0;
    pub fn CanReadMemberValue(
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        nonPublic: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CanReadMemberValue", (member, nonPublic))?;
        Ok(__cordl_ret.into())
    }
    pub fn CanSetMemberValue(
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        nonPublic: bool,
        canSetReadOnly: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CanSetMemberValue", (member, nonPublic, canSetReadOnly))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureNotByRefType(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnsureNotByRefType", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureNotNullableType(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnsureNotNullableType", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssemblyDelimiterIndex(
        fullyQualifiedTypeName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<crate::System::Nullable_1<i32>> {
        let __cordl_ret: crate::System::Nullable_1<i32> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAssemblyDelimiterIndex", (fullyQualifiedTypeName))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAttribute_Il2CppObject0<T>(
        attributeProvider: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAttribute", (attributeProvider))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAttribute__cordl_bool1<T>(
        attributeProvider: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAttribute", (attributeProvider, inherit))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAttributes_Type__cordl_bool1(
        attributeProvider: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
        attributeType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Attribute>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Attribute>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAttributes", (attributeProvider, attributeType, inherit))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAttributes__cordl_bool0<T>(
        attributeProvider: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
        inherit: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAttributes", (attributeProvider, inherit))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetBaseDefinition(
        propertyInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetBaseDefinition", (propertyInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetChildPrivateFields(
        initialFields: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::System::Reflection::MemberInfo,
            >,
        >,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        bindingAttr: crate::System::Reflection::BindingFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetChildPrivateFields", (initialFields, _cordl_type, bindingAttr))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetChildPrivateProperties(
        initialProperties: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                *mut crate::System::Reflection::PropertyInfo,
            >,
        >,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        bindingAttr: crate::System::Reflection::BindingFlags,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetChildPrivateProperties",
                (initialProperties, _cordl_type, bindingAttr),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCollectionItemType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCollectionItemType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultConstructor_Type0(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::ConstructorInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ConstructorInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDefaultConstructor", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultConstructor__cordl_bool1(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
        nonPublic: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::ConstructorInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ConstructorInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDefaultConstructor", (t, nonPublic))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDefaultValue(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDefaultValue", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDictionaryKeyValueTypes(
        dictionaryType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        keyType: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Type>,
        valueType: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDictionaryKeyValueTypes", (dictionaryType, keyType, valueType))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFields(
        targetType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        bindingAttr: crate::System::Reflection::BindingFlags,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::System::Reflection::FieldInfo,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::System::Reflection::FieldInfo,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFields", (targetType, bindingAttr))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFieldsAndProperties(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        bindingAttr: crate::System::Reflection::BindingFlags,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::System::Reflection::MemberInfo,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::System::Reflection::MemberInfo,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFieldsAndProperties", (_cordl_type, bindingAttr))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetFullyQualifiedTypeName(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
        binder: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::ISerializationBinder,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetFullyQualifiedTypeName", (t, binder))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMemberInfoFromType(
        targetType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        memberInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MemberInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMemberInfoFromType", (targetType, memberInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMemberUnderlyingType(
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMemberUnderlyingType", (member))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMemberValue(
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMemberValue", (member, target))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetObjectType(
        v: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetObjectType", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetProperties(
        targetType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        bindingAttr: crate::System::Reflection::BindingFlags,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::System::Reflection::PropertyInfo,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::System::Reflection::PropertyInfo,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetProperties", (targetType, bindingAttr))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetTypeName(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
        assemblyFormat: crate::Newtonsoft::Json::TypeNameAssemblyFormatHandling,
        binder: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::ISerializationBinder,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetTypeName", (t, assemblyFormat, binder))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasDefaultConstructor(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
        nonPublic: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasDefaultConstructor", (t, nonPublic))?;
        Ok(__cordl_ret.into())
    }
    pub fn ImplementsGenericDefinition_ByRefMut1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        genericInterfaceDefinition: quest_hook::libil2cpp::Gc<crate::System::Type>,
        implementingType: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ImplementsGenericDefinition",
                (_cordl_type, genericInterfaceDefinition, implementingType),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ImplementsGenericDefinition_Type_Type0(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        genericInterfaceDefinition: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ImplementsGenericDefinition",
                (_cordl_type, genericInterfaceDefinition),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InheritsGenericDefinitionInternal(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        genericClassDefinition: quest_hook::libil2cpp::Gc<crate::System::Type>,
        implementingType: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "InheritsGenericDefinitionInternal",
                (_cordl_type, genericClassDefinition, implementingType),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InheritsGenericDefinition_ByRefMut1(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        genericClassDefinition: quest_hook::libil2cpp::Gc<crate::System::Type>,
        implementingType: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "InheritsGenericDefinition",
                (_cordl_type, genericClassDefinition, implementingType),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn InheritsGenericDefinition_Type_Type0(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        genericClassDefinition: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("InheritsGenericDefinition", (_cordl_type, genericClassDefinition))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsByRefLikeType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsByRefLikeType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsGenericDefinition(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        genericInterfaceDefinition: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsGenericDefinition", (_cordl_type, genericInterfaceDefinition))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsIndexedProperty(
        property: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsIndexedProperty", (property))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsMethodOverridden(
        currentType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        methodDeclaringType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        method: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsMethodOverridden", (currentType, methodDeclaringType, method))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNullable(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsNullable", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsNullableType(
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsNullableType", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsOverridenGenericMember(
        memberInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        bindingAttr: crate::System::Reflection::BindingFlags,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsOverridenGenericMember", (memberInfo, bindingAttr))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPublic(
        property: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPublic", (property))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsVirtual(
        propertyInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsVirtual", (propertyInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveAssemblyDetails(
        fullyQualifiedTypeName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveAssemblyDetails", (fullyQualifiedTypeName))?;
        Ok(__cordl_ret.into())
    }
    pub fn RemoveFlag(
        bindingAttr: crate::System::Reflection::BindingFlags,
        flag: crate::System::Reflection::BindingFlags,
    ) -> quest_hook::libil2cpp::Result<crate::System::Reflection::BindingFlags> {
        let __cordl_ret: crate::System::Reflection::BindingFlags = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("RemoveFlag", (bindingAttr, flag))?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMemberValue(
        member: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        target: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetMemberValue", (member, target, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn SplitFullyQualifiedTypeName(
        fullyQualifiedTypeName: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        >,
    ) -> quest_hook::libil2cpp::Result<
        crate::Newtonsoft::Json::Utilities::StructMultiKey_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut quest_hook::libil2cpp::Il2CppString,
        >,
    > {
        let __cordl_ret: crate::Newtonsoft::Json::Utilities::StructMultiKey_2<
            *mut quest_hook::libil2cpp::Il2CppString,
            *mut quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SplitFullyQualifiedTypeName", (fullyQualifiedTypeName))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::ReflectionUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
