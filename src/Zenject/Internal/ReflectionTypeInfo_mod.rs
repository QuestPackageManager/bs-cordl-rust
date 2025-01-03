#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectionTypeInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub Type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub BaseType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub InjectProperties: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::Zenject::Internal::ReflectionTypeInfo_InjectPropertyInfo,
        >,
    >,
    pub InjectFields: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::Zenject::Internal::ReflectionTypeInfo_InjectFieldInfo,
        >,
    >,
    pub InjectConstructor: quest_hook::libil2cpp::Gc<
        crate::Zenject::Internal::ReflectionTypeInfo_InjectConstructorInfo,
    >,
    pub InjectMethods: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::Zenject::Internal::ReflectionTypeInfo_InjectMethodInfo,
        >,
    >,
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::Internal::ReflectionTypeInfo =>
    "Zenject.Internal"."ReflectionTypeInfo"
);
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo")]
impl std::ops::Deref for crate::Zenject::Internal::ReflectionTypeInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo")]
impl std::ops::DerefMut for crate::Zenject::Internal::ReflectionTypeInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo")]
impl crate::Zenject::Internal::ReflectionTypeInfo {
    #[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectConstructorInfo")]
    pub type InjectConstructorInfo = crate::Zenject::Internal::ReflectionTypeInfo_InjectConstructorInfo;
    #[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectFieldInfo")]
    pub type InjectFieldInfo = crate::Zenject::Internal::ReflectionTypeInfo_InjectFieldInfo;
    #[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectMethodInfo")]
    pub type InjectMethodInfo = crate::Zenject::Internal::ReflectionTypeInfo_InjectMethodInfo;
    #[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectParameterInfo")]
    pub type InjectParameterInfo = crate::Zenject::Internal::ReflectionTypeInfo_InjectParameterInfo;
    #[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectPropertyInfo")]
    pub type InjectPropertyInfo = crate::Zenject::Internal::ReflectionTypeInfo_InjectPropertyInfo;
    pub fn New(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        baseType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        injectConstructor: quest_hook::libil2cpp::Gc<
            crate::Zenject::Internal::ReflectionTypeInfo_InjectConstructorInfo,
        >,
        injectMethods: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::Zenject::Internal::ReflectionTypeInfo_InjectMethodInfo,
            >,
        >,
        injectFields: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::Zenject::Internal::ReflectionTypeInfo_InjectFieldInfo,
            >,
        >,
        injectProperties: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::Zenject::Internal::ReflectionTypeInfo_InjectPropertyInfo,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    _cordl_type,
                    baseType,
                    injectConstructor,
                    injectMethods,
                    injectFields,
                    injectProperties,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        baseType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        injectConstructor: quest_hook::libil2cpp::Gc<
            crate::Zenject::Internal::ReflectionTypeInfo_InjectConstructorInfo,
        >,
        injectMethods: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::Zenject::Internal::ReflectionTypeInfo_InjectMethodInfo,
            >,
        >,
        injectFields: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::Zenject::Internal::ReflectionTypeInfo_InjectFieldInfo,
            >,
        >,
        injectProperties: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::Zenject::Internal::ReflectionTypeInfo_InjectPropertyInfo,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    _cordl_type,
                    baseType,
                    injectConstructor,
                    injectMethods,
                    injectFields,
                    injectProperties,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::Internal::ReflectionTypeInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectConstructorInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectionTypeInfo_InjectConstructorInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub ConstructorInfo: quest_hook::libil2cpp::Gc<
        crate::System::Reflection::ConstructorInfo,
    >,
    pub Parameters: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::Zenject::Internal::ReflectionTypeInfo_InjectParameterInfo,
        >,
    >,
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectConstructorInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Zenject::Internal::ReflectionTypeInfo_InjectConstructorInfo => "Zenject.Internal"
    ."ReflectionTypeInfo/InjectConstructorInfo"
);
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectConstructorInfo")]
impl std::ops::Deref
for crate::Zenject::Internal::ReflectionTypeInfo_InjectConstructorInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectConstructorInfo")]
impl std::ops::DerefMut
for crate::Zenject::Internal::ReflectionTypeInfo_InjectConstructorInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectConstructorInfo")]
impl crate::Zenject::Internal::ReflectionTypeInfo_InjectConstructorInfo {
    pub fn New(
        constructorInfo: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ConstructorInfo,
        >,
        parameters: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::Zenject::Internal::ReflectionTypeInfo_InjectParameterInfo,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (constructorInfo, parameters))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        constructorInfo: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ConstructorInfo,
        >,
        parameters: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::Zenject::Internal::ReflectionTypeInfo_InjectParameterInfo,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (constructorInfo, parameters))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectConstructorInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::Internal::ReflectionTypeInfo_InjectConstructorInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectFieldInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectionTypeInfo_InjectFieldInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub FieldInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::FieldInfo>,
    pub InjectableInfo: quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectFieldInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Zenject::Internal::ReflectionTypeInfo_InjectFieldInfo => "Zenject.Internal"
    ."ReflectionTypeInfo/InjectFieldInfo"
);
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectFieldInfo")]
impl std::ops::Deref for crate::Zenject::Internal::ReflectionTypeInfo_InjectFieldInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectFieldInfo")]
impl std::ops::DerefMut
for crate::Zenject::Internal::ReflectionTypeInfo_InjectFieldInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectFieldInfo")]
impl crate::Zenject::Internal::ReflectionTypeInfo_InjectFieldInfo {
    pub fn New(
        fieldInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::FieldInfo>,
        injectableInfo: quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (fieldInfo, injectableInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        fieldInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::FieldInfo>,
        injectableInfo: quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (fieldInfo, injectableInfo))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectFieldInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::Internal::ReflectionTypeInfo_InjectFieldInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectMethodInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectionTypeInfo_InjectMethodInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub MethodInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    pub Parameters: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<
            *mut crate::Zenject::Internal::ReflectionTypeInfo_InjectParameterInfo,
        >,
    >,
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectMethodInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Zenject::Internal::ReflectionTypeInfo_InjectMethodInfo => "Zenject.Internal"
    ."ReflectionTypeInfo/InjectMethodInfo"
);
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectMethodInfo")]
impl std::ops::Deref for crate::Zenject::Internal::ReflectionTypeInfo_InjectMethodInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectMethodInfo")]
impl std::ops::DerefMut
for crate::Zenject::Internal::ReflectionTypeInfo_InjectMethodInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectMethodInfo")]
impl crate::Zenject::Internal::ReflectionTypeInfo_InjectMethodInfo {
    pub fn New(
        methodInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        parameters: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::Zenject::Internal::ReflectionTypeInfo_InjectParameterInfo,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (methodInfo, parameters))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        methodInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
        parameters: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut crate::Zenject::Internal::ReflectionTypeInfo_InjectParameterInfo,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (methodInfo, parameters))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectMethodInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::Internal::ReflectionTypeInfo_InjectMethodInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectParameterInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectionTypeInfo_InjectParameterInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub ParameterInfo: quest_hook::libil2cpp::Gc<
        crate::System::Reflection::ParameterInfo,
    >,
    pub InjectableInfo: quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectParameterInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Zenject::Internal::ReflectionTypeInfo_InjectParameterInfo => "Zenject.Internal"
    ."ReflectionTypeInfo/InjectParameterInfo"
);
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectParameterInfo")]
impl std::ops::Deref
for crate::Zenject::Internal::ReflectionTypeInfo_InjectParameterInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectParameterInfo")]
impl std::ops::DerefMut
for crate::Zenject::Internal::ReflectionTypeInfo_InjectParameterInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectParameterInfo")]
impl crate::Zenject::Internal::ReflectionTypeInfo_InjectParameterInfo {
    pub fn New(
        parameterInfo: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ParameterInfo,
        >,
        injectableInfo: quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (parameterInfo, injectableInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        parameterInfo: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::ParameterInfo,
        >,
        injectableInfo: quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (parameterInfo, injectableInfo))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectParameterInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::Internal::ReflectionTypeInfo_InjectParameterInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectPropertyInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectionTypeInfo_InjectPropertyInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub PropertyInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
    pub InjectableInfo: quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectPropertyInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Zenject::Internal::ReflectionTypeInfo_InjectPropertyInfo => "Zenject.Internal"
    ."ReflectionTypeInfo/InjectPropertyInfo"
);
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectPropertyInfo")]
impl std::ops::Deref
for crate::Zenject::Internal::ReflectionTypeInfo_InjectPropertyInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectPropertyInfo")]
impl std::ops::DerefMut
for crate::Zenject::Internal::ReflectionTypeInfo_InjectPropertyInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectPropertyInfo")]
impl crate::Zenject::Internal::ReflectionTypeInfo_InjectPropertyInfo {
    pub fn New(
        propertyInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
        injectableInfo: quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (propertyInfo, injectableInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        propertyInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
        injectableInfo: quest_hook::libil2cpp::Gc<crate::Zenject::InjectableInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (propertyInfo, injectableInfo))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+Internal+ReflectionTypeInfo+InjectPropertyInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::Internal::ReflectionTypeInfo_InjectPropertyInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
