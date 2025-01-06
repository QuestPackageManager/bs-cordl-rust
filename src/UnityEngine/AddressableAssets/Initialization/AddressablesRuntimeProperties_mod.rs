#[cfg(
    feature = "UnityEngine+AddressableAssets+Initialization+AddressablesRuntimeProperties"
)]
#[repr(C)]
#[derive(Debug)]
pub struct AddressablesRuntimeProperties {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+Initialization+AddressablesRuntimeProperties"
)]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::UnityEngine::AddressableAssets::Initialization::AddressablesRuntimeProperties =>
    "UnityEngine.AddressableAssets.Initialization"."AddressablesRuntimeProperties"
);
#[cfg(
    feature = "UnityEngine+AddressableAssets+Initialization+AddressablesRuntimeProperties"
)]
impl std::ops::Deref
for crate::UnityEngine::AddressableAssets::Initialization::AddressablesRuntimeProperties {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+Initialization+AddressablesRuntimeProperties"
)]
impl std::ops::DerefMut
for crate::UnityEngine::AddressableAssets::Initialization::AddressablesRuntimeProperties {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+Initialization+AddressablesRuntimeProperties"
)]
impl crate::UnityEngine::AddressableAssets::Initialization::AddressablesRuntimeProperties {
    pub fn ClearCachedPropertyValues() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Void,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ClearCachedPropertyValues", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn EvaluateProperty(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EvaluateProperty", (name))?;
        Ok(__cordl_ret.into())
    }
    pub fn EvaluateString_Il2CppString0(
        input: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("EvaluateString", (input))?;
        Ok(__cordl_ret.into())
    }
    pub fn EvaluateString__cordl_char__cordl_char_Func_2_1(
        inputString: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        startDelimiter: char,
        endDelimiter: char,
        varFunc: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "EvaluateString",
                (inputString, startDelimiter, endDelimiter, varFunc),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetAssemblies() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("GetAssemblies", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn GetCachedValueCount() -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_ret: i32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("GetCachedValueCount", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetPropertyValue(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        val: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetPropertyValue", (name, val))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(
    feature = "UnityEngine+AddressableAssets+Initialization+AddressablesRuntimeProperties"
)]
impl quest_hook::libil2cpp::ObjectType
for crate::UnityEngine::AddressableAssets::Initialization::AddressablesRuntimeProperties {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
