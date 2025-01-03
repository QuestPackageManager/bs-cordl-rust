#[cfg(feature = "ModestTree+TypeExtensions")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeExtensions {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "ModestTree+TypeExtensions")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::ModestTree::TypeExtensions => "ModestTree"
    ."TypeExtensions"
);
#[cfg(feature = "ModestTree+TypeExtensions")]
impl std::ops::Deref for crate::ModestTree::TypeExtensions {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "ModestTree+TypeExtensions")]
impl std::ops::DerefMut for crate::ModestTree::TypeExtensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "ModestTree+TypeExtensions")]
impl crate::ModestTree::TypeExtensions {
    #[cfg(feature = "ModestTree+TypeExtensions+_GetParentTypes_d__28")]
    pub type _GetParentTypes_d__28 = crate::ModestTree::TypeExtensions__GetParentTypes_d__28;
    #[cfg(feature = "ModestTree+TypeExtensions+__c__DisplayClass35_0")]
    pub type __c__DisplayClass35_0 = crate::ModestTree::TypeExtensions___c__DisplayClass35_0;
    #[cfg(feature = "ModestTree+TypeExtensions+__c__DisplayClass35_1")]
    pub type __c__DisplayClass35_1 = crate::ModestTree::TypeExtensions___c__DisplayClass35_1;
    #[cfg(feature = "ModestTree+TypeExtensions+__c__DisplayClass39_0")]
    pub type __c__DisplayClass39_0 = crate::ModestTree::TypeExtensions___c__DisplayClass39_0;
    #[cfg(feature = "ModestTree+TypeExtensions+__c__DisplayClass39_1")]
    pub type __c__DisplayClass39_1 = crate::ModestTree::TypeExtensions___c__DisplayClass39_1;
    pub fn AllAttributes_MemberInfo0<T>(
        provider: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IEnumerable_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AllAttributes", (provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn AllAttributes_MemberInfo_Il2CppArray1(
        provider: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        attributeTypes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::System::Attribute,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::System::Attribute,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AllAttributes", (provider, attributeTypes))?;
        Ok(__cordl_ret.into())
    }
    pub fn AllAttributes_ParameterInfo2<T>(
        provider: quest_hook::libil2cpp::Gc<crate::System::Reflection::ParameterInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Collections::Generic::IEnumerable_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<T>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AllAttributes", (provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn AllAttributes_ParameterInfo_Il2CppArray3(
        provider: quest_hook::libil2cpp::Gc<crate::System::Reflection::ParameterInfo>,
        attributeTypes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::System::Attribute,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::System::Attribute,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("AllAttributes", (provider, attributeTypes))?;
        Ok(__cordl_ret.into())
    }
    pub fn BaseType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("BaseType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn Constructors(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Reflection::ConstructorInfo,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Reflection::ConstructorInfo,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Constructors", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn ContainsGenericParameters(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ContainsGenericParameters", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeclaredInstanceFields(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Reflection::FieldInfo>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Reflection::FieldInfo>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeclaredInstanceFields", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeclaredInstanceMethods(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Reflection::MethodInfo,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Reflection::MethodInfo,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeclaredInstanceMethods", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn DeclaredInstanceProperties(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Reflection::PropertyInfo,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Reflection::PropertyInfo,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DeclaredInstanceProperties", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn DerivesFromOrEqual_Type0<T>(
        a: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DerivesFromOrEqual", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn DerivesFromOrEqual_Type1(
        a: quest_hook::libil2cpp::Gc<crate::System::Type>,
        b: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DerivesFromOrEqual", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn DerivesFrom_Type0<T>(
        a: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DerivesFrom", (a))?;
        Ok(__cordl_ret.into())
    }
    pub fn DerivesFrom_Type1(
        a: quest_hook::libil2cpp::Gc<crate::System::Type>,
        b: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("DerivesFrom", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn GenericArguments(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GenericArguments", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAttribute<T>(
        provider: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetAttribute", (provider))?;
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
    pub fn GetParentTypes(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<*mut crate::System::Type>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<*mut crate::System::Type>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetParentTypes", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasAttribute_MemberInfo1<T>(
        provider: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasAttribute", (provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasAttribute_MemberInfo_Il2CppArray0(
        provider: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
        attributeTypes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasAttribute", (provider, attributeTypes))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasAttribute_ParameterInfo3<T>(
        provider: quest_hook::libil2cpp::Gc<crate::System::Reflection::ParameterInfo>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasAttribute", (provider))?;
        Ok(__cordl_ret.into())
    }
    pub fn HasAttribute_ParameterInfo_Il2CppArray2(
        provider: quest_hook::libil2cpp::Gc<crate::System::Reflection::ParameterInfo>,
        attributeTypes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("HasAttribute", (provider, attributeTypes))?;
        Ok(__cordl_ret.into())
    }
    pub fn Interfaces(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Interfaces", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsAbstract(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsAbstract", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsAssignableToGenericType(
        givenType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        genericType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsAssignableToGenericType", (givenType, genericType))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsClosedGenericType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsClosedGenericType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsEnum(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsEnum", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsGenericType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsGenericType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsGenericTypeDefinition(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsGenericTypeDefinition", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsInterface(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsInterface", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsOpenGenericType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsOpenGenericType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsPrimitive(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsPrimitive", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsSealed(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsSealed", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn IsValueType(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("IsValueType", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn Method(
        del: quest_hook::libil2cpp::Gc<crate::System::Delegate>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Method", (del))?;
        Ok(__cordl_ret.into())
    }
    pub fn TryGetAttribute<T>(
        provider: quest_hook::libil2cpp::Gc<crate::System::Reflection::MemberInfo>,
    ) -> quest_hook::libil2cpp::Result<T>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: T = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("TryGetAttribute", (provider))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "ModestTree+TypeExtensions")]
impl quest_hook::libil2cpp::ObjectType for crate::ModestTree::TypeExtensions {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
