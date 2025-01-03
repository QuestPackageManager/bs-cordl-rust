#[cfg(feature = "Newtonsoft+Json+Utilities+LateBoundReflectionDelegateFactory")]
#[repr(C)]
#[derive(Debug)]
pub struct LateBoundReflectionDelegateFactory {
    __cordl_parent: crate::Newtonsoft::Json::Utilities::ReflectionDelegateFactory,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+LateBoundReflectionDelegateFactory")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Utilities::LateBoundReflectionDelegateFactory =>
    "Newtonsoft.Json.Utilities"."LateBoundReflectionDelegateFactory"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+LateBoundReflectionDelegateFactory")]
impl std::ops::Deref
for crate::Newtonsoft::Json::Utilities::LateBoundReflectionDelegateFactory {
    type Target = crate::Newtonsoft::Json::Utilities::ReflectionDelegateFactory;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+LateBoundReflectionDelegateFactory")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Utilities::LateBoundReflectionDelegateFactory {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+LateBoundReflectionDelegateFactory")]
impl crate::Newtonsoft::Json::Utilities::LateBoundReflectionDelegateFactory {
    #[cfg(
        feature = "Newtonsoft+Json+Utilities+LateBoundReflectionDelegateFactory+__c__DisplayClass3_0"
    )]
    pub type __c__DisplayClass3_0 = crate::Newtonsoft::Json::Utilities::LateBoundReflectionDelegateFactory___c__DisplayClass3_0;
    #[cfg(
        feature = "Newtonsoft+Json+Utilities+LateBoundReflectionDelegateFactory+__c__DisplayClass4_0_1"
    )]
    pub type __c__DisplayClass4_0_1<T: quest_hook::libil2cpp::Type> = crate::Newtonsoft::Json::Utilities::LateBoundReflectionDelegateFactory___c__DisplayClass4_0_1<
        T,
    >;
    #[cfg(
        feature = "Newtonsoft+Json+Utilities+LateBoundReflectionDelegateFactory+__c__DisplayClass5_0_1"
    )]
    pub type __c__DisplayClass5_0_1<T: quest_hook::libil2cpp::Type> = crate::Newtonsoft::Json::Utilities::LateBoundReflectionDelegateFactory___c__DisplayClass5_0_1<
        T,
    >;
    #[cfg(
        feature = "Newtonsoft+Json+Utilities+LateBoundReflectionDelegateFactory+__c__DisplayClass6_0_1"
    )]
    pub type __c__DisplayClass6_0_1<T: quest_hook::libil2cpp::Type> = crate::Newtonsoft::Json::Utilities::LateBoundReflectionDelegateFactory___c__DisplayClass6_0_1<
        T,
    >;
    #[cfg(
        feature = "Newtonsoft+Json+Utilities+LateBoundReflectionDelegateFactory+__c__DisplayClass7_0_1"
    )]
    pub type __c__DisplayClass7_0_1<T: quest_hook::libil2cpp::Type> = crate::Newtonsoft::Json::Utilities::LateBoundReflectionDelegateFactory___c__DisplayClass7_0_1<
        T,
    >;
    #[cfg(
        feature = "Newtonsoft+Json+Utilities+LateBoundReflectionDelegateFactory+__c__DisplayClass8_0_1"
    )]
    pub type __c__DisplayClass8_0_1<T: quest_hook::libil2cpp::Type> = crate::Newtonsoft::Json::Utilities::LateBoundReflectionDelegateFactory___c__DisplayClass8_0_1<
        T,
    >;
    #[cfg(
        feature = "Newtonsoft+Json+Utilities+LateBoundReflectionDelegateFactory+__c__DisplayClass9_0_1"
    )]
    pub type __c__DisplayClass9_0_1<T: quest_hook::libil2cpp::Type> = crate::Newtonsoft::Json::Utilities::LateBoundReflectionDelegateFactory___c__DisplayClass9_0_1<
        T,
    >;
    pub fn CreateDefaultConstructor<T>(
        &mut self,
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Func_1<T>>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Func_1<T>> = __cordl_object
            .invoke("CreateDefaultConstructor", (_cordl_type))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateGet_FieldInfo1<T>(
        &mut self,
        fieldInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::FieldInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Func_2<T, *mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<T, *mut quest_hook::libil2cpp::Il2CppObject>,
        > = __cordl_object.invoke("CreateGet", (fieldInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateGet_PropertyInfo0<T>(
        &mut self,
        propertyInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Func_2<T, *mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<T, *mut quest_hook::libil2cpp::Il2CppObject>,
        > = __cordl_object.invoke("CreateGet", (propertyInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateMethodCall<T>(
        &mut self,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::MethodCall_2<
                T,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::MethodCall_2<
                T,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        > = __cordl_object.invoke("CreateMethodCall", (method))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateParameterizedConstructor(
        &mut self,
        method: quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodBase>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        > = __cordl_object.invoke("CreateParameterizedConstructor", (method))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateSet_FieldInfo0<T>(
        &mut self,
        fieldInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::FieldInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Action_2<T, *mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<T, *mut quest_hook::libil2cpp::Il2CppObject>,
        > = __cordl_object.invoke("CreateSet", (fieldInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateSet_PropertyInfo1<T>(
        &mut self,
        propertyInfo: quest_hook::libil2cpp::Gc<crate::System::Reflection::PropertyInfo>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Action_2<T, *mut quest_hook::libil2cpp::Il2CppObject>,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<T, *mut quest_hook::libil2cpp::Il2CppObject>,
        > = __cordl_object.invoke("CreateSet", (propertyInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn get_Instance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::ReflectionDelegateFactory,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::ReflectionDelegateFactory,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_Instance", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+LateBoundReflectionDelegateFactory")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::LateBoundReflectionDelegateFactory {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
