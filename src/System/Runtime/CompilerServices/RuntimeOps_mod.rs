#[cfg(feature = "System+Runtime+CompilerServices+RuntimeOps")]
#[repr(C)]
#[derive(Debug)]
pub struct RuntimeOps {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "System+Runtime+CompilerServices+RuntimeOps")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Runtime::CompilerServices::RuntimeOps =>
    "System.Runtime.CompilerServices"."RuntimeOps"
);
#[cfg(feature = "System+Runtime+CompilerServices+RuntimeOps")]
impl std::ops::Deref for crate::System::Runtime::CompilerServices::RuntimeOps {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+RuntimeOps")]
impl std::ops::DerefMut for crate::System::Runtime::CompilerServices::RuntimeOps {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+RuntimeOps")]
impl crate::System::Runtime::CompilerServices::RuntimeOps {
    pub fn ExpandoCheckVersion(
        expando: quest_hook::libil2cpp::Gc<crate::System::Dynamic::ExpandoObject>,
        version: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExpandoCheckVersion", (expando, version))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExpandoPromoteClass(
        expando: quest_hook::libil2cpp::Gc<crate::System::Dynamic::ExpandoObject>,
        oldClass: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        newClass: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("ExpandoPromoteClass", (expando, oldClass, newClass))?;
        Ok(__cordl_ret.into())
    }
    pub fn ExpandoTryDeleteValue(
        expando: quest_hook::libil2cpp::Gc<crate::System::Dynamic::ExpandoObject>,
        indexClass: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        index: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ignoreCase: bool,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ExpandoTryDeleteValue",
                (expando, indexClass, index, name, ignoreCase),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ExpandoTryGetValue(
        expando: quest_hook::libil2cpp::Gc<crate::System::Dynamic::ExpandoObject>,
        indexClass: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        index: i32,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ignoreCase: bool,
        value: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ExpandoTryGetValue",
                (expando, indexClass, index, name, ignoreCase, value),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn ExpandoTrySetValue(
        expando: quest_hook::libil2cpp::Gc<crate::System::Dynamic::ExpandoObject>,
        indexClass: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        index: i32,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        ignoreCase: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "ExpandoTrySetValue",
                (expando, indexClass, index, value, name, ignoreCase),
            )?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+RuntimeOps")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::CompilerServices::RuntimeOps {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
