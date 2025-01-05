#[cfg(feature = "Newtonsoft+Json+Utilities+DynamicUtils")]
#[repr(C)]
#[derive(Debug)]
pub struct DynamicUtils {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+DynamicUtils")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::DynamicUtils =>
    "Newtonsoft.Json.Utilities"."DynamicUtils"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+DynamicUtils")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::DynamicUtils {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+DynamicUtils")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::DynamicUtils {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+DynamicUtils")]
impl crate::Newtonsoft::Json::Utilities::DynamicUtils {
    #[cfg(feature = "Newtonsoft+Json+Utilities+DynamicUtils+BinderWrapper")]
    pub type BinderWrapper = crate::Newtonsoft::Json::Utilities::DynamicUtils_BinderWrapper;
    pub fn GetDynamicMemberNames(
        dynamicProvider: quest_hook::libil2cpp::Gc<
            crate::System::Dynamic::IDynamicMetaObjectProvider,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetDynamicMemberNames", (dynamicProvider))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+DynamicUtils")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::DynamicUtils {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+DynamicUtils+BinderWrapper")]
#[repr(C)]
#[derive(Debug)]
pub struct DynamicUtils_BinderWrapper {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+DynamicUtils+BinderWrapper")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Newtonsoft::Json::Utilities::DynamicUtils_BinderWrapper =>
    "Newtonsoft.Json.Utilities"."DynamicUtils/BinderWrapper"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+DynamicUtils+BinderWrapper")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::DynamicUtils_BinderWrapper {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+DynamicUtils+BinderWrapper")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Utilities::DynamicUtils_BinderWrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+DynamicUtils+BinderWrapper")]
impl crate::Newtonsoft::Json::Utilities::DynamicUtils_BinderWrapper {
    pub const BinderTypeName: &'static str = "Microsoft.CSharp.RuntimeBinder.Binder, Microsoft.CSharp, Version=4.0.0.0, Culture=neutral, PublicKeyToken=b03f5f7f11d50a3a";
    pub const CSharpArgumentInfoFlagsTypeName: &'static str = "Microsoft.CSharp.RuntimeBinder.CSharpArgumentInfoFlags, Microsoft.CSharp, Version=4.0.0.0, Culture=neutral, PublicKeyToken=b03f5f7f11d50a3a";
    pub const CSharpArgumentInfoTypeName: &'static str = "Microsoft.CSharp.RuntimeBinder.CSharpArgumentInfo, Microsoft.CSharp, Version=4.0.0.0, Culture=neutral, PublicKeyToken=b03f5f7f11d50a3a";
    pub const CSharpAssemblyName: &'static str = "Microsoft.CSharp, Version=4.0.0.0, Culture=neutral, PublicKeyToken=b03f5f7f11d50a3a";
    pub const CSharpBinderFlagsTypeName: &'static str = "Microsoft.CSharp.RuntimeBinder.CSharpBinderFlags, Microsoft.CSharp, Version=4.0.0.0, Culture=neutral, PublicKeyToken=b03f5f7f11d50a3a";
    pub fn CreateMemberCalls() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateMemberCalls", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CreateSharpArgumentInfoArray(
        values: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<i32>>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("CreateSharpArgumentInfoArray", (values))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetMember(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        context: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::CallSiteBinder,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::CallSiteBinder,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetMember", (name, context))?;
        Ok(__cordl_ret.into())
    }
    pub fn Init() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Init", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMember(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        context: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::CallSiteBinder,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Runtime::CompilerServices::CallSiteBinder,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetMember", (name, context))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+DynamicUtils+BinderWrapper")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::DynamicUtils_BinderWrapper {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
