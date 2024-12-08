#[cfg(feature = "Zenject+ConventionAssemblySelectionBinder")]
#[repr(C)]
#[derive(Debug)]
pub struct ConventionAssemblySelectionBinder {
    __cordl_parent: crate::System::Object,
    pub _BindInfo_k__BackingField: *mut crate::Zenject::ConventionBindInfo,
}
#[cfg(feature = "Zenject+ConventionAssemblySelectionBinder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::ConventionAssemblySelectionBinder =>
    "Zenject"."ConventionAssemblySelectionBinder"
);
#[cfg(feature = "Zenject+ConventionAssemblySelectionBinder")]
impl std::ops::Deref for crate::Zenject::ConventionAssemblySelectionBinder {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ConventionAssemblySelectionBinder")]
impl std::ops::DerefMut for crate::Zenject::ConventionAssemblySelectionBinder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ConventionAssemblySelectionBinder")]
impl crate::Zenject::ConventionAssemblySelectionBinder {
    #[cfg(feature = "Zenject+ConventionAssemblySelectionBinder+__c")]
    pub type __c = crate::Zenject::ConventionAssemblySelectionBinder___c;
    #[cfg(feature = "Zenject+ConventionAssemblySelectionBinder+__c__DisplayClass12_0")]
    pub type __c__DisplayClass12_0 = crate::Zenject::ConventionAssemblySelectionBinder___c__DisplayClass12_0;
    pub fn _ctor(
        &mut self,
        bindInfo: *mut crate::Zenject::ConventionBindInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bindInfo))?;
        Ok(__cordl_ret)
    }
    pub fn get_BindInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::ConventionBindInfo> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConventionBindInfo = __cordl_object
            .invoke("get_BindInfo", ())?;
        Ok(__cordl_ret)
    }
    pub fn FromAssembliesContaining_Il2CppArray0(
        &mut self,
        types: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FromAssembliesContaining", (types))?;
        Ok(__cordl_ret)
    }
    pub fn FromAssembliesContaining_IEnumerable_1_1(
        &mut self,
        types: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Type,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FromAssembliesContaining", (types))?;
        Ok(__cordl_ret)
    }
    pub fn FromAssembly(
        &mut self,
        assembly: *mut crate::System::Reflection::Assembly,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FromAssembly", (assembly))?;
        Ok(__cordl_ret)
    }
    pub fn FromAssembliesWhere(
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
            .invoke("FromAssembliesWhere", (predicate))?;
        Ok(__cordl_ret)
    }
    pub fn FromAssemblies_Il2CppArray0(
        &mut self,
        assemblies: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::System::Reflection::Assembly,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FromAssemblies", (assemblies))?;
        Ok(__cordl_ret)
    }
    pub fn FromAssemblies_IEnumerable_1_1(
        &mut self,
        assemblies: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Reflection::Assembly,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FromAssemblies", (assemblies))?;
        Ok(__cordl_ret)
    }
    pub fn FromAssemblyContaining<T>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FromAssemblyContaining", ())?;
        Ok(__cordl_ret)
    }
    pub fn FromAllAssemblies(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FromAllAssemblies", ())?;
        Ok(__cordl_ret)
    }
    pub fn FromThisAssembly(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FromThisAssembly", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_BindInfo(
        &mut self,
        value: *mut crate::Zenject::ConventionBindInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_BindInfo", (value))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        bindInfo: *mut crate::Zenject::ConventionBindInfo,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bindInfo))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Zenject+ConventionAssemblySelectionBinder")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::ConventionAssemblySelectionBinder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
