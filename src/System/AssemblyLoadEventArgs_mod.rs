#[cfg(feature = "System+AssemblyLoadEventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct AssemblyLoadEventArgs {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::EventArgs>,
    pub _LoadedAssembly_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::System::Reflection::Assembly,
    >,
}
#[cfg(feature = "System+AssemblyLoadEventArgs")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::AssemblyLoadEventArgs => "System"
    ."AssemblyLoadEventArgs"
);
#[cfg(feature = "System+AssemblyLoadEventArgs")]
impl std::ops::Deref for crate::System::AssemblyLoadEventArgs {
    type Target = quest_hook::libil2cpp::Gc<crate::System::EventArgs>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+AssemblyLoadEventArgs")]
impl std::ops::DerefMut for crate::System::AssemblyLoadEventArgs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+AssemblyLoadEventArgs")]
impl crate::System::AssemblyLoadEventArgs {
    pub fn New(
        loadedAssembly: quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (loadedAssembly))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        loadedAssembly: quest_hook::libil2cpp::Gc<crate::System::Reflection::Assembly>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (loadedAssembly))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+AssemblyLoadEventArgs")]
impl quest_hook::libil2cpp::ObjectType for crate::System::AssemblyLoadEventArgs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
