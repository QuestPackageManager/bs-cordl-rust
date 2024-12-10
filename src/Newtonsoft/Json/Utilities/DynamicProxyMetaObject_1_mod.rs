#[cfg(feature = "Newtonsoft+Json+Utilities+DynamicProxyMetaObject_1")]
#[repr(C)]
#[derive(Debug)]
pub struct DynamicProxyMetaObject_1<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::Dynamic::DynamicMetaObject,
    pub _proxy: *mut crate::Newtonsoft::Json::Utilities::DynamicProxy_1<T>,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+DynamicProxyMetaObject_1")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Utilities::DynamicProxyMetaObject_1 < T > =>
    "Newtonsoft.Json.Utilities"."DynamicProxyMetaObject`1" < T >
);
#[cfg(feature = "Newtonsoft+Json+Utilities+DynamicProxyMetaObject_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Newtonsoft::Json::Utilities::DynamicProxyMetaObject_1<T> {
    type Target = crate::System::Dynamic::DynamicMetaObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+DynamicProxyMetaObject_1")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Newtonsoft::Json::Utilities::DynamicProxyMetaObject_1<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+DynamicProxyMetaObject_1")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::Newtonsoft::Json::Utilities::DynamicProxyMetaObject_1<T> {
    #[cfg(feature = "Newtonsoft+Json+Utilities+DynamicProxyMetaObject_1+Fallback")]
    pub type Fallback = crate::Newtonsoft::Json::Utilities::DynamicProxyMetaObject_1_Fallback<
        T,
    >;
    #[cfg(
        feature = "Newtonsoft+Json+Utilities+DynamicProxyMetaObject_1+GetBinderAdapter"
    )]
    pub type GetBinderAdapter = crate::Newtonsoft::Json::Utilities::DynamicProxyMetaObject_1_GetBinderAdapter<
        T,
    >;
    #[cfg(feature = "Newtonsoft+Json+Utilities+DynamicProxyMetaObject_1+__c")]
    pub type __c = crate::Newtonsoft::Json::Utilities::DynamicProxyMetaObject_1___c<T>;
    #[cfg(
        feature = "Newtonsoft+Json+Utilities+DynamicProxyMetaObject_1+__c__DisplayClass10_0"
    )]
    pub type __c__DisplayClass10_0 = crate::Newtonsoft::Json::Utilities::DynamicProxyMetaObject_1___c__DisplayClass10_0<
        T,
    >;
    #[cfg(
        feature = "Newtonsoft+Json+Utilities+DynamicProxyMetaObject_1+__c__DisplayClass11_0"
    )]
    pub type __c__DisplayClass11_0 = crate::Newtonsoft::Json::Utilities::DynamicProxyMetaObject_1___c__DisplayClass11_0<
        T,
    >;
    #[cfg(
        feature = "Newtonsoft+Json+Utilities+DynamicProxyMetaObject_1+__c__DisplayClass12_0"
    )]
    pub type __c__DisplayClass12_0 = crate::Newtonsoft::Json::Utilities::DynamicProxyMetaObject_1___c__DisplayClass12_0<
        T,
    >;
    #[cfg(
        feature = "Newtonsoft+Json+Utilities+DynamicProxyMetaObject_1+__c__DisplayClass13_0"
    )]
    pub type __c__DisplayClass13_0 = crate::Newtonsoft::Json::Utilities::DynamicProxyMetaObject_1___c__DisplayClass13_0<
        T,
    >;
    #[cfg(
        feature = "Newtonsoft+Json+Utilities+DynamicProxyMetaObject_1+__c__DisplayClass14_0"
    )]
    pub type __c__DisplayClass14_0 = crate::Newtonsoft::Json::Utilities::DynamicProxyMetaObject_1___c__DisplayClass14_0<
        T,
    >;
    #[cfg(
        feature = "Newtonsoft+Json+Utilities+DynamicProxyMetaObject_1+__c__DisplayClass3_0"
    )]
    pub type __c__DisplayClass3_0 = crate::Newtonsoft::Json::Utilities::DynamicProxyMetaObject_1___c__DisplayClass3_0<
        T,
    >;
    #[cfg(
        feature = "Newtonsoft+Json+Utilities+DynamicProxyMetaObject_1+__c__DisplayClass4_0"
    )]
    pub type __c__DisplayClass4_0 = crate::Newtonsoft::Json::Utilities::DynamicProxyMetaObject_1___c__DisplayClass4_0<
        T,
    >;
    #[cfg(
        feature = "Newtonsoft+Json+Utilities+DynamicProxyMetaObject_1+__c__DisplayClass5_0"
    )]
    pub type __c__DisplayClass5_0 = crate::Newtonsoft::Json::Utilities::DynamicProxyMetaObject_1___c__DisplayClass5_0<
        T,
    >;
    #[cfg(
        feature = "Newtonsoft+Json+Utilities+DynamicProxyMetaObject_1+__c__DisplayClass6_0"
    )]
    pub type __c__DisplayClass6_0 = crate::Newtonsoft::Json::Utilities::DynamicProxyMetaObject_1___c__DisplayClass6_0<
        T,
    >;
    #[cfg(
        feature = "Newtonsoft+Json+Utilities+DynamicProxyMetaObject_1+__c__DisplayClass7_0"
    )]
    pub type __c__DisplayClass7_0 = crate::Newtonsoft::Json::Utilities::DynamicProxyMetaObject_1___c__DisplayClass7_0<
        T,
    >;
    #[cfg(
        feature = "Newtonsoft+Json+Utilities+DynamicProxyMetaObject_1+__c__DisplayClass8_0"
    )]
    pub type __c__DisplayClass8_0 = crate::Newtonsoft::Json::Utilities::DynamicProxyMetaObject_1___c__DisplayClass8_0<
        T,
    >;
    #[cfg(
        feature = "Newtonsoft+Json+Utilities+DynamicProxyMetaObject_1+__c__DisplayClass9_0"
    )]
    pub type __c__DisplayClass9_0 = crate::Newtonsoft::Json::Utilities::DynamicProxyMetaObject_1___c__DisplayClass9_0<
        T,
    >;
    pub fn BindBinaryOperation(
        &mut self,
        binder: quest_hook::libil2cpp::Gc<crate::System::Dynamic::BinaryOperationBinder>,
        arg: quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        > = __cordl_object.invoke("BindBinaryOperation", (binder, arg))?;
        Ok(__cordl_ret.into())
    }
    pub fn BindConvert(
        &mut self,
        binder: quest_hook::libil2cpp::Gc<crate::System::Dynamic::ConvertBinder>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        > = __cordl_object.invoke("BindConvert", (binder))?;
        Ok(__cordl_ret.into())
    }
    pub fn BindCreateInstance(
        &mut self,
        binder: quest_hook::libil2cpp::Gc<crate::System::Dynamic::CreateInstanceBinder>,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Dynamic::DynamicMetaObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        > = __cordl_object.invoke("BindCreateInstance", (binder, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn BindDeleteIndex(
        &mut self,
        binder: quest_hook::libil2cpp::Gc<crate::System::Dynamic::DeleteIndexBinder>,
        indexes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Dynamic::DynamicMetaObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        > = __cordl_object.invoke("BindDeleteIndex", (binder, indexes))?;
        Ok(__cordl_ret.into())
    }
    pub fn BindDeleteMember(
        &mut self,
        binder: quest_hook::libil2cpp::Gc<crate::System::Dynamic::DeleteMemberBinder>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        > = __cordl_object.invoke("BindDeleteMember", (binder))?;
        Ok(__cordl_ret.into())
    }
    pub fn BindGetIndex(
        &mut self,
        binder: quest_hook::libil2cpp::Gc<crate::System::Dynamic::GetIndexBinder>,
        indexes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Dynamic::DynamicMetaObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        > = __cordl_object.invoke("BindGetIndex", (binder, indexes))?;
        Ok(__cordl_ret.into())
    }
    pub fn BindGetMember(
        &mut self,
        binder: quest_hook::libil2cpp::Gc<crate::System::Dynamic::GetMemberBinder>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        > = __cordl_object.invoke("BindGetMember", (binder))?;
        Ok(__cordl_ret.into())
    }
    pub fn BindInvoke(
        &mut self,
        binder: quest_hook::libil2cpp::Gc<crate::System::Dynamic::InvokeBinder>,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Dynamic::DynamicMetaObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        > = __cordl_object.invoke("BindInvoke", (binder, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn BindInvokeMember(
        &mut self,
        binder: quest_hook::libil2cpp::Gc<crate::System::Dynamic::InvokeMemberBinder>,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Dynamic::DynamicMetaObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        > = __cordl_object.invoke("BindInvokeMember", (binder, args))?;
        Ok(__cordl_ret.into())
    }
    pub fn BindSetIndex(
        &mut self,
        binder: quest_hook::libil2cpp::Gc<crate::System::Dynamic::SetIndexBinder>,
        indexes: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Dynamic::DynamicMetaObject,
            >,
        >,
        value: quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        > = __cordl_object.invoke("BindSetIndex", (binder, indexes, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BindSetMember(
        &mut self,
        binder: quest_hook::libil2cpp::Gc<crate::System::Dynamic::SetMemberBinder>,
        value: quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        > = __cordl_object.invoke("BindSetMember", (binder, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn BindUnaryOperation(
        &mut self,
        binder: quest_hook::libil2cpp::Gc<crate::System::Dynamic::UnaryOperationBinder>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        > = __cordl_object.invoke("BindUnaryOperation", (binder))?;
        Ok(__cordl_ret.into())
    }
    pub fn BuildCallMethodWithResult(
        &mut self,
        methodName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        binder: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObjectBinder,
        >,
        args: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::System::Linq::Expressions::Expression,
            >,
        >,
        fallbackResult: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        >,
        fallbackInvoke: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::DynamicProxyMetaObject_1_Fallback<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        > = __cordl_object
            .invoke(
                "BuildCallMethodWithResult",
                (methodName, binder, args, fallbackResult, fallbackInvoke),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn CallMethodNoResult(
        &mut self,
        methodName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        binder: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObjectBinder,
        >,
        args: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                *mut crate::System::Linq::Expressions::Expression,
            >,
        >,
        fallback: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::DynamicProxyMetaObject_1_Fallback<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        > = __cordl_object
            .invoke("CallMethodNoResult", (methodName, binder, args, fallback))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallMethodReturnLast(
        &mut self,
        methodName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        binder: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObjectBinder,
        >,
        args: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::System::Linq::Expressions::Expression,
            >,
        >,
        fallback: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::DynamicProxyMetaObject_1_Fallback<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        > = __cordl_object
            .invoke("CallMethodReturnLast", (methodName, binder, args, fallback))?;
        Ok(__cordl_ret.into())
    }
    pub fn CallMethodWithResult(
        &mut self,
        methodName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        binder: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObjectBinder,
        >,
        args: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut crate::System::Linq::Expressions::Expression,
            >,
        >,
        fallback: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::DynamicProxyMetaObject_1_Fallback<T>,
        >,
        fallbackInvoke: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::DynamicProxyMetaObject_1_Fallback<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        > = __cordl_object
            .invoke(
                "CallMethodWithResult",
                (methodName, binder, args, fallback, fallbackInvoke),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetDynamicMemberNames(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        >,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                *mut quest_hook::libil2cpp::Il2CppString,
            >,
        > = __cordl_object.invoke("GetDynamicMemberNames", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetRestrictions(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::BindingRestrictions>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::BindingRestrictions,
        > = __cordl_object.invoke("GetRestrictions", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn IsOverridden(
        &mut self,
        method: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<bool>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("IsOverridden", (method))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        value: T,
        proxy: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::DynamicProxy_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (expression, value, proxy))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        expression: quest_hook::libil2cpp::Gc<
            crate::System::Linq::Expressions::Expression,
        >,
        value: T,
        proxy: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::DynamicProxy_1<T>,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (expression, value, proxy))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+DynamicProxyMetaObject_1")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::DynamicProxyMetaObject_1<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+DynamicProxyMetaObject_1+Fallback")]
#[repr(C)]
#[derive(Debug)]
pub struct DynamicProxyMetaObject_1_Fallback<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::MulticastDelegate,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+DynamicProxyMetaObject_1+Fallback")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Utilities::DynamicProxyMetaObject_1_Fallback < T > =>
    "Newtonsoft.Json.Utilities"."DynamicProxyMetaObject`1/Fallback" < T >
);
#[cfg(feature = "Newtonsoft+Json+Utilities+DynamicProxyMetaObject_1+Fallback")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Newtonsoft::Json::Utilities::DynamicProxyMetaObject_1_Fallback<T> {
    type Target = crate::System::MulticastDelegate;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+DynamicProxyMetaObject_1+Fallback")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Newtonsoft::Json::Utilities::DynamicProxyMetaObject_1_Fallback<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+DynamicProxyMetaObject_1+Fallback")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::Newtonsoft::Json::Utilities::DynamicProxyMetaObject_1_Fallback<T> {
    pub fn BeginInvoke(
        &mut self,
        errorSuggestion: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        >,
        callback: quest_hook::libil2cpp::Gc<crate::System::AsyncCallback>,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult> = __cordl_object
            .invoke("BeginInvoke", (errorSuggestion, callback, object))?;
        Ok(__cordl_ret.into())
    }
    pub fn EndInvoke(
        &mut self,
        result: quest_hook::libil2cpp::Gc<crate::System::IAsyncResult>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        > = __cordl_object.invoke("EndInvoke", (result))?;
        Ok(__cordl_ret.into())
    }
    pub fn Invoke(
        &mut self,
        errorSuggestion: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        > = __cordl_object.invoke("Invoke", (errorSuggestion))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (object, method))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        object: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        method: crate::System::IntPtr,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (object, method))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+DynamicProxyMetaObject_1+Fallback")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::DynamicProxyMetaObject_1_Fallback<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+DynamicProxyMetaObject_1+GetBinderAdapter")]
#[repr(C)]
#[derive(Debug)]
pub struct DynamicProxyMetaObject_1_GetBinderAdapter<T: quest_hook::libil2cpp::Type> {
    __cordl_parent: crate::System::Dynamic::GetMemberBinder,
    __cordl_phantom_T: std::marker::PhantomData<T>,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+DynamicProxyMetaObject_1+GetBinderAdapter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Utilities::DynamicProxyMetaObject_1_GetBinderAdapter < T > =>
    "Newtonsoft.Json.Utilities"."DynamicProxyMetaObject`1/GetBinderAdapter" < T >
);
#[cfg(feature = "Newtonsoft+Json+Utilities+DynamicProxyMetaObject_1+GetBinderAdapter")]
impl<T: quest_hook::libil2cpp::Type> std::ops::Deref
for crate::Newtonsoft::Json::Utilities::DynamicProxyMetaObject_1_GetBinderAdapter<T> {
    type Target = crate::System::Dynamic::GetMemberBinder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+DynamicProxyMetaObject_1+GetBinderAdapter")]
impl<T: quest_hook::libil2cpp::Type> std::ops::DerefMut
for crate::Newtonsoft::Json::Utilities::DynamicProxyMetaObject_1_GetBinderAdapter<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+DynamicProxyMetaObject_1+GetBinderAdapter")]
impl<
    T: quest_hook::libil2cpp::Type,
> crate::Newtonsoft::Json::Utilities::DynamicProxyMetaObject_1_GetBinderAdapter<T> {
    pub fn FallbackGetMember(
        &mut self,
        target: quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
        errorSuggestion: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Dynamic::DynamicMetaObject>,
    >
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::DynamicMetaObject,
        > = __cordl_object.invoke("FallbackGetMember", (target, errorSuggestion))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        binder: quest_hook::libil2cpp::Gc<crate::System::Dynamic::InvokeMemberBinder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (binder))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        binder: quest_hook::libil2cpp::Gc<crate::System::Dynamic::InvokeMemberBinder>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Type
            + quest_hook::libil2cpp::Argument + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (binder))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+DynamicProxyMetaObject_1+GetBinderAdapter")]
impl<T: quest_hook::libil2cpp::Type> quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::DynamicProxyMetaObject_1_GetBinderAdapter<T> {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
