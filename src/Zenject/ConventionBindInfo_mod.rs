#[cfg(feature = "Zenject+ConventionBindInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct ConventionBindInfo {
    __cordl_parent: crate::System::Object,
    pub _typeFilters: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Func_2<*mut crate::System::Type, bool>,
    >,
    pub _assemblyFilters: *mut crate::System::Collections::Generic::List_1<
        *mut crate::System::Func_2<*mut crate::System::Reflection::Assembly, bool>,
    >,
}
#[cfg(feature = "Zenject+ConventionBindInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::ConventionBindInfo => "Zenject"
    ."ConventionBindInfo"
);
#[cfg(feature = "Zenject+ConventionBindInfo")]
impl std::ops::Deref for crate::Zenject::ConventionBindInfo {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ConventionBindInfo")]
impl std::ops::DerefMut for crate::Zenject::ConventionBindInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ConventionBindInfo")]
impl crate::Zenject::ConventionBindInfo {
    #[cfg(feature = "Zenject+ConventionBindInfo+__c__DisplayClass7_0")]
    pub type __c__DisplayClass7_0 = crate::Zenject::ConventionBindInfo___c__DisplayClass7_0;
    #[cfg(feature = "Zenject+ConventionBindInfo+__c__DisplayClass6_0")]
    pub type __c__DisplayClass6_0 = crate::Zenject::ConventionBindInfo___c__DisplayClass6_0;
    pub fn ResolveTypes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::List_1<*mut crate::System::Type>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Type,
        > = __cordl_object.invoke("ResolveTypes", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddAssemblyFilter(
        &mut self,
        predicate: *mut crate::System::Func_2<
            *mut crate::System::Reflection::Assembly,
            bool,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddAssemblyFilter", (predicate))?;
        Ok(__cordl_ret)
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
    pub fn ShouldIncludeType(
        &mut self,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ShouldIncludeType", (_cordl_type))?;
        Ok(__cordl_ret)
    }
    pub fn GetTypes(
        &mut self,
        assembly: *mut crate::System::Reflection::Assembly,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Type,
        > = __cordl_object.invoke("GetTypes", (assembly))?;
        Ok(__cordl_ret)
    }
    pub fn GetAllAssemblies(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Reflection::Assembly,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Reflection::Assembly,
        > = __cordl_object.invoke("GetAllAssemblies", ())?;
        Ok(__cordl_ret)
    }
    pub fn AddTypeFilter(
        &mut self,
        predicate: *mut crate::System::Func_2<*mut crate::System::Type, bool>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddTypeFilter", (predicate))?;
        Ok(__cordl_ret)
    }
    pub fn _ResolveTypes_b__9_0(
        &mut self,
        assembly: *mut crate::System::Reflection::Assembly,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IEnumerable_1<*mut crate::System::Type>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Type,
        > = __cordl_object.invoke("<ResolveTypes>b__9_0", (assembly))?;
        Ok(__cordl_ret)
    }
    pub fn ShouldIncludeAssembly(
        &mut self,
        assembly: *mut crate::System::Reflection::Assembly,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("ShouldIncludeAssembly", (assembly))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Zenject+ConventionBindInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::ConventionBindInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
