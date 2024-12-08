#[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionDelegateFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct ReflectionDelegateFactory {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionDelegateFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Utilities::ReflectionDelegateFactory =>
    "Newtonsoft.Json.Utilities"."ReflectionDelegateFactory"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionDelegateFactory")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::ReflectionDelegateFactory {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionDelegateFactory")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Utilities::ReflectionDelegateFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionDelegateFactory")]
impl crate::Newtonsoft::Json::Utilities::ReflectionDelegateFactory {
    pub fn CreateDefaultConstructor<T>(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Func_1<T>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Func_1<T> = __cordl_object
            .invoke("CreateDefaultConstructor", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn CreateGet_FieldInfo2<T>(
        &mut self,
        fieldInfo: *mut crate::System::Reflection::FieldInfo,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Func_2<T, *mut crate::System::Object>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Func_2<T, *mut crate::System::Object> = __cordl_object
            .invoke("CreateGet", (fieldInfo))?;
        Ok(__cordl_ret)
    }
    pub fn CreateGet_MemberInfo0<T>(
        &mut self,
        memberInfo: *mut crate::System::Reflection::MemberInfo,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Func_2<T, *mut crate::System::Object>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Func_2<T, *mut crate::System::Object> = __cordl_object
            .invoke("CreateGet", (memberInfo))?;
        Ok(__cordl_ret)
    }
    pub fn CreateGet_PropertyInfo1<T>(
        &mut self,
        propertyInfo: *mut crate::System::Reflection::PropertyInfo,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Func_2<T, *mut crate::System::Object>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Func_2<T, *mut crate::System::Object> = __cordl_object
            .invoke("CreateGet", (propertyInfo))?;
        Ok(__cordl_ret)
    }
    pub fn CreateMethodCall<T>(
        &mut self,
        method: *mut crate::System::Reflection::MethodBase,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Utilities::MethodCall_2<
            T,
            *mut crate::System::Object,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Utilities::MethodCall_2<
            T,
            *mut crate::System::Object,
        > = __cordl_object.invoke("CreateMethodCall", (method))?;
        Ok(__cordl_ret)
    }
    pub fn CreateParameterizedConstructor(
        &mut self,
        method: *mut crate::System::Reflection::MethodBase,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
            *mut crate::System::Object,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
            *mut crate::System::Object,
        > = __cordl_object.invoke("CreateParameterizedConstructor", (method))?;
        Ok(__cordl_ret)
    }
    pub fn CreateSet_FieldInfo1<T>(
        &mut self,
        fieldInfo: *mut crate::System::Reflection::FieldInfo,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Action_2<T, *mut crate::System::Object>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Action_2<T, *mut crate::System::Object> = __cordl_object
            .invoke("CreateSet", (fieldInfo))?;
        Ok(__cordl_ret)
    }
    pub fn CreateSet_MemberInfo0<T>(
        &mut self,
        memberInfo: *mut crate::System::Reflection::MemberInfo,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Action_2<T, *mut crate::System::Object>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Action_2<T, *mut crate::System::Object> = __cordl_object
            .invoke("CreateSet", (memberInfo))?;
        Ok(__cordl_ret)
    }
    pub fn CreateSet_PropertyInfo2<T>(
        &mut self,
        propertyInfo: *mut crate::System::Reflection::PropertyInfo,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Action_2<T, *mut crate::System::Object>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Action_2<T, *mut crate::System::Object> = __cordl_object
            .invoke("CreateSet", (propertyInfo))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+ReflectionDelegateFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::ReflectionDelegateFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
