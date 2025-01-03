#[cfg(feature = "Newtonsoft+Json+Utilities+FSharpUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct FSharpUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _ofSeq: *mut crate::System::Reflection::MethodInfo,
    pub _mapType: *mut crate::System::Type,
    pub _FSharpCoreAssembly_k__BackingField: *mut crate::System::Reflection::Assembly,
    pub _IsUnion_k__BackingField: *mut crate::Newtonsoft::Json::Utilities::MethodCall_2<
        *mut quest_hook::libil2cpp::Il2CppObject,
        *mut quest_hook::libil2cpp::Il2CppObject,
    >,
    pub _GetUnionCases_k__BackingField: *mut crate::Newtonsoft::Json::Utilities::MethodCall_2<
        *mut quest_hook::libil2cpp::Il2CppObject,
        *mut quest_hook::libil2cpp::Il2CppObject,
    >,
    pub _PreComputeUnionTagReader_k__BackingField: *mut crate::Newtonsoft::Json::Utilities::MethodCall_2<
        *mut quest_hook::libil2cpp::Il2CppObject,
        *mut quest_hook::libil2cpp::Il2CppObject,
    >,
    pub _PreComputeUnionReader_k__BackingField: *mut crate::Newtonsoft::Json::Utilities::MethodCall_2<
        *mut quest_hook::libil2cpp::Il2CppObject,
        *mut quest_hook::libil2cpp::Il2CppObject,
    >,
    pub _PreComputeUnionConstructor_k__BackingField: *mut crate::Newtonsoft::Json::Utilities::MethodCall_2<
        *mut quest_hook::libil2cpp::Il2CppObject,
        *mut quest_hook::libil2cpp::Il2CppObject,
    >,
    pub _GetUnionCaseInfoDeclaringType_k__BackingField: *mut crate::System::Func_2<
        *mut quest_hook::libil2cpp::Il2CppObject,
        *mut quest_hook::libil2cpp::Il2CppObject,
    >,
    pub _GetUnionCaseInfoName_k__BackingField: *mut crate::System::Func_2<
        *mut quest_hook::libil2cpp::Il2CppObject,
        *mut quest_hook::libil2cpp::Il2CppObject,
    >,
    pub _GetUnionCaseInfoTag_k__BackingField: *mut crate::System::Func_2<
        *mut quest_hook::libil2cpp::Il2CppObject,
        *mut quest_hook::libil2cpp::Il2CppObject,
    >,
    pub _GetUnionCaseInfoFields_k__BackingField: *mut crate::Newtonsoft::Json::Utilities::MethodCall_2<
        *mut quest_hook::libil2cpp::Il2CppObject,
        *mut quest_hook::libil2cpp::Il2CppObject,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+FSharpUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::FSharpUtils =>
    "Newtonsoft.Json.Utilities"."FSharpUtils"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+FSharpUtils")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::FSharpUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+FSharpUtils")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::FSharpUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+FSharpUtils")]
impl crate::Newtonsoft::Json::Utilities::FSharpUtils {
    pub const FSharpListTypeName: &'static str = "FSharpList`1";
    pub const FSharpMapTypeName: &'static str = "FSharpMap`2";
    pub const FSharpSetTypeName: &'static str = "FSharpSet`1";
    #[cfg(feature = "Newtonsoft+Json+Utilities+FSharpUtils+__c__55_2")]
    pub type __c__55_2<
        TKey: quest_hook::libil2cpp::Type,
        TValue: quest_hook::libil2cpp::Type,
    > = crate::Newtonsoft::Json::Utilities::FSharpUtils___c__55_2<TKey, TValue>;
    #[cfg(feature = "Newtonsoft+Json+Utilities+FSharpUtils+__c__DisplayClass52_0")]
    pub type __c__DisplayClass52_0 = crate::Newtonsoft::Json::Utilities::FSharpUtils___c__DisplayClass52_0;
    #[cfg(feature = "Newtonsoft+Json+Utilities+FSharpUtils+__c__DisplayClass55_0_2")]
    pub type __c__DisplayClass55_0_2<
        TKey: quest_hook::libil2cpp::Type,
        TValue: quest_hook::libil2cpp::Type,
    > = crate::Newtonsoft::Json::Utilities::FSharpUtils___c__DisplayClass55_0_2<
        TKey,
        TValue,
    >;
    pub fn BuildMapCreator<TKey, TValue>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    >
    where
        TKey: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
        TValue: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        > = __cordl_object.invoke("BuildMapCreator", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateFSharpFuncCall(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        methodName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::MethodCall_2<
                *mut quest_hook::libil2cpp::Il2CppObject,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::MethodCall_2<
                *mut quest_hook::libil2cpp::Il2CppObject,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateFSharpFuncCall", (_cordl_type, methodName))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateMap(
        &mut self,
        keyType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        valueType: quest_hook::libil2cpp::Gc<crate::System::Type>,
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
        > = __cordl_object.invoke("CreateMap", (keyType, valueType))?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateSeq(
        &mut self,
        t: quest_hook::libil2cpp::Gc<crate::System::Type>,
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
        > = __cordl_object.invoke("CreateSeq", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn EnsureInitialized(
        fsharpCoreAssembly: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::Assembly,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EnsureInitialized", (fsharpCoreAssembly))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMethodWithNonPublicFallback(
        _cordl_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
        methodName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        bindingFlags: crate::System::Reflection::BindingFlags,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::MethodInfo>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::MethodInfo,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetMethodWithNonPublicFallback",
                (_cordl_type, methodName, bindingFlags),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        fsharpCoreAssembly: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::Assembly,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (fsharpCoreAssembly))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        fsharpCoreAssembly: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::Assembly,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (fsharpCoreAssembly))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_FSharpCoreAssembly(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::Assembly,
        > = __cordl_object.invoke("get_FSharpCoreAssembly", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_GetUnionCaseInfoDeclaringType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                *mut quest_hook::libil2cpp::Il2CppObject,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                *mut quest_hook::libil2cpp::Il2CppObject,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        > = __cordl_object.invoke("get_GetUnionCaseInfoDeclaringType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_GetUnionCaseInfoFields(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::MethodCall_2<
                *mut quest_hook::libil2cpp::Il2CppObject,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::MethodCall_2<
                *mut quest_hook::libil2cpp::Il2CppObject,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        > = __cordl_object.invoke("get_GetUnionCaseInfoFields", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_GetUnionCaseInfoName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                *mut quest_hook::libil2cpp::Il2CppObject,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                *mut quest_hook::libil2cpp::Il2CppObject,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        > = __cordl_object.invoke("get_GetUnionCaseInfoName", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_GetUnionCaseInfoTag(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                *mut quest_hook::libil2cpp::Il2CppObject,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                *mut quest_hook::libil2cpp::Il2CppObject,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        > = __cordl_object.invoke("get_GetUnionCaseInfoTag", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_GetUnionCases(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::MethodCall_2<
                *mut quest_hook::libil2cpp::Il2CppObject,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::MethodCall_2<
                *mut quest_hook::libil2cpp::Il2CppObject,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        > = __cordl_object.invoke("get_GetUnionCases", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Instance() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Newtonsoft::Json::Utilities::FSharpUtils>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::FSharpUtils,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("get_Instance", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsUnion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::MethodCall_2<
                *mut quest_hook::libil2cpp::Il2CppObject,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::MethodCall_2<
                *mut quest_hook::libil2cpp::Il2CppObject,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        > = __cordl_object.invoke("get_IsUnion", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PreComputeUnionConstructor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::MethodCall_2<
                *mut quest_hook::libil2cpp::Il2CppObject,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::MethodCall_2<
                *mut quest_hook::libil2cpp::Il2CppObject,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        > = __cordl_object.invoke("get_PreComputeUnionConstructor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PreComputeUnionReader(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::MethodCall_2<
                *mut quest_hook::libil2cpp::Il2CppObject,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::MethodCall_2<
                *mut quest_hook::libil2cpp::Il2CppObject,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        > = __cordl_object.invoke("get_PreComputeUnionReader", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_PreComputeUnionTagReader(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::MethodCall_2<
                *mut quest_hook::libil2cpp::Il2CppObject,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::MethodCall_2<
                *mut quest_hook::libil2cpp::Il2CppObject,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        > = __cordl_object.invoke("get_PreComputeUnionTagReader", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_FSharpCoreAssembly(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_FSharpCoreAssembly", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_GetUnionCaseInfoDeclaringType(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                *mut quest_hook::libil2cpp::Il2CppObject,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_GetUnionCaseInfoDeclaringType", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_GetUnionCaseInfoFields(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::MethodCall_2<
                *mut quest_hook::libil2cpp::Il2CppObject,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_GetUnionCaseInfoFields", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_GetUnionCaseInfoName(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                *mut quest_hook::libil2cpp::Il2CppObject,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_GetUnionCaseInfoName", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_GetUnionCaseInfoTag(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                *mut quest_hook::libil2cpp::Il2CppObject,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_GetUnionCaseInfoTag", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_GetUnionCases(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::MethodCall_2<
                *mut quest_hook::libil2cpp::Il2CppObject,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_GetUnionCases", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_IsUnion(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::MethodCall_2<
                *mut quest_hook::libil2cpp::Il2CppObject,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsUnion", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_PreComputeUnionConstructor(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::MethodCall_2<
                *mut quest_hook::libil2cpp::Il2CppObject,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_PreComputeUnionConstructor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_PreComputeUnionReader(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::MethodCall_2<
                *mut quest_hook::libil2cpp::Il2CppObject,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_PreComputeUnionReader", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_PreComputeUnionTagReader(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Utilities::MethodCall_2<
                *mut quest_hook::libil2cpp::Il2CppObject,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_PreComputeUnionTagReader", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+FSharpUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::FSharpUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
