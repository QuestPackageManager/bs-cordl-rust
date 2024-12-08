#[cfg(feature = "Newtonsoft+Json+Utilities+FSharpUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct FSharpUtils {
    __cordl_parent: crate::System::Object,
    pub _ofSeq: *mut crate::System::Reflection::MethodInfo,
    pub _mapType: *mut crate::System::Type,
    pub _FSharpCoreAssembly_k__BackingField: *mut crate::System::Reflection::Assembly,
    pub _IsUnion_k__BackingField: *mut crate::Newtonsoft::Json::Utilities::MethodCall_2<
        *mut crate::System::Object,
        *mut crate::System::Object,
    >,
    pub _GetUnionCases_k__BackingField: *mut crate::Newtonsoft::Json::Utilities::MethodCall_2<
        *mut crate::System::Object,
        *mut crate::System::Object,
    >,
    pub _PreComputeUnionTagReader_k__BackingField: *mut crate::Newtonsoft::Json::Utilities::MethodCall_2<
        *mut crate::System::Object,
        *mut crate::System::Object,
    >,
    pub _PreComputeUnionReader_k__BackingField: *mut crate::Newtonsoft::Json::Utilities::MethodCall_2<
        *mut crate::System::Object,
        *mut crate::System::Object,
    >,
    pub _PreComputeUnionConstructor_k__BackingField: *mut crate::Newtonsoft::Json::Utilities::MethodCall_2<
        *mut crate::System::Object,
        *mut crate::System::Object,
    >,
    pub _GetUnionCaseInfoDeclaringType_k__BackingField: *mut crate::System::Func_2<
        *mut crate::System::Object,
        *mut crate::System::Object,
    >,
    pub _GetUnionCaseInfoName_k__BackingField: *mut crate::System::Func_2<
        *mut crate::System::Object,
        *mut crate::System::Object,
    >,
    pub _GetUnionCaseInfoTag_k__BackingField: *mut crate::System::Func_2<
        *mut crate::System::Object,
        *mut crate::System::Object,
    >,
    pub _GetUnionCaseInfoFields_k__BackingField: *mut crate::Newtonsoft::Json::Utilities::MethodCall_2<
        *mut crate::System::Object,
        *mut crate::System::Object,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+FSharpUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::FSharpUtils =>
    "Newtonsoft.Json.Utilities"."FSharpUtils"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+FSharpUtils")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::FSharpUtils {
    type Target = crate::System::Object;
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
        *mut crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
            *mut crate::System::Object,
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
        let __cordl_ret: *mut crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
            *mut crate::System::Object,
        > = __cordl_object.invoke("BuildMapCreator", ())?;
        Ok(__cordl_ret)
    }
    pub fn CreateMap(
        &mut self,
        keyType: *mut crate::System::Type,
        valueType: *mut crate::System::Type,
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
        > = __cordl_object.invoke("CreateMap", (keyType, valueType))?;
        Ok(__cordl_ret)
    }
    pub fn CreateSeq(
        &mut self,
        t: *mut crate::System::Type,
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
        > = __cordl_object.invoke("CreateSeq", (t))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        fsharpCoreAssembly: *mut crate::System::Reflection::Assembly,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (fsharpCoreAssembly))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        fsharpCoreAssembly: *mut crate::System::Reflection::Assembly,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (fsharpCoreAssembly))?;
        Ok(__cordl_ret)
    }
    pub fn get_FSharpCoreAssembly(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Reflection::Assembly> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Reflection::Assembly = __cordl_object
            .invoke("get_FSharpCoreAssembly", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_GetUnionCaseInfoDeclaringType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Func_2<
            *mut crate::System::Object,
            *mut crate::System::Object,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Func_2<
            *mut crate::System::Object,
            *mut crate::System::Object,
        > = __cordl_object.invoke("get_GetUnionCaseInfoDeclaringType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_GetUnionCaseInfoFields(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Utilities::MethodCall_2<
            *mut crate::System::Object,
            *mut crate::System::Object,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Utilities::MethodCall_2<
            *mut crate::System::Object,
            *mut crate::System::Object,
        > = __cordl_object.invoke("get_GetUnionCaseInfoFields", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_GetUnionCaseInfoName(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Func_2<
            *mut crate::System::Object,
            *mut crate::System::Object,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Func_2<
            *mut crate::System::Object,
            *mut crate::System::Object,
        > = __cordl_object.invoke("get_GetUnionCaseInfoName", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_GetUnionCaseInfoTag(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Func_2<
            *mut crate::System::Object,
            *mut crate::System::Object,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Func_2<
            *mut crate::System::Object,
            *mut crate::System::Object,
        > = __cordl_object.invoke("get_GetUnionCaseInfoTag", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_GetUnionCases(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Utilities::MethodCall_2<
            *mut crate::System::Object,
            *mut crate::System::Object,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Utilities::MethodCall_2<
            *mut crate::System::Object,
            *mut crate::System::Object,
        > = __cordl_object.invoke("get_GetUnionCases", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IsUnion(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Utilities::MethodCall_2<
            *mut crate::System::Object,
            *mut crate::System::Object,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Utilities::MethodCall_2<
            *mut crate::System::Object,
            *mut crate::System::Object,
        > = __cordl_object.invoke("get_IsUnion", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PreComputeUnionConstructor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Utilities::MethodCall_2<
            *mut crate::System::Object,
            *mut crate::System::Object,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Utilities::MethodCall_2<
            *mut crate::System::Object,
            *mut crate::System::Object,
        > = __cordl_object.invoke("get_PreComputeUnionConstructor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PreComputeUnionReader(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Utilities::MethodCall_2<
            *mut crate::System::Object,
            *mut crate::System::Object,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Utilities::MethodCall_2<
            *mut crate::System::Object,
            *mut crate::System::Object,
        > = __cordl_object.invoke("get_PreComputeUnionReader", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_PreComputeUnionTagReader(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Newtonsoft::Json::Utilities::MethodCall_2<
            *mut crate::System::Object,
            *mut crate::System::Object,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Newtonsoft::Json::Utilities::MethodCall_2<
            *mut crate::System::Object,
            *mut crate::System::Object,
        > = __cordl_object.invoke("get_PreComputeUnionTagReader", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_FSharpCoreAssembly(
        &mut self,
        value: *mut crate::System::Reflection::Assembly,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_FSharpCoreAssembly", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_GetUnionCaseInfoDeclaringType(
        &mut self,
        value: *mut crate::System::Func_2<
            *mut crate::System::Object,
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_GetUnionCaseInfoDeclaringType", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_GetUnionCaseInfoFields(
        &mut self,
        value: *mut crate::Newtonsoft::Json::Utilities::MethodCall_2<
            *mut crate::System::Object,
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_GetUnionCaseInfoFields", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_GetUnionCaseInfoName(
        &mut self,
        value: *mut crate::System::Func_2<
            *mut crate::System::Object,
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_GetUnionCaseInfoName", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_GetUnionCaseInfoTag(
        &mut self,
        value: *mut crate::System::Func_2<
            *mut crate::System::Object,
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_GetUnionCaseInfoTag", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_GetUnionCases(
        &mut self,
        value: *mut crate::Newtonsoft::Json::Utilities::MethodCall_2<
            *mut crate::System::Object,
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_GetUnionCases", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_IsUnion(
        &mut self,
        value: *mut crate::Newtonsoft::Json::Utilities::MethodCall_2<
            *mut crate::System::Object,
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_IsUnion", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_PreComputeUnionConstructor(
        &mut self,
        value: *mut crate::Newtonsoft::Json::Utilities::MethodCall_2<
            *mut crate::System::Object,
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_PreComputeUnionConstructor", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_PreComputeUnionReader(
        &mut self,
        value: *mut crate::Newtonsoft::Json::Utilities::MethodCall_2<
            *mut crate::System::Object,
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_PreComputeUnionReader", (value))?;
        Ok(__cordl_ret)
    }
    pub fn set_PreComputeUnionTagReader(
        &mut self,
        value: *mut crate::Newtonsoft::Json::Utilities::MethodCall_2<
            *mut crate::System::Object,
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_PreComputeUnionTagReader", (value))?;
        Ok(__cordl_ret)
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
