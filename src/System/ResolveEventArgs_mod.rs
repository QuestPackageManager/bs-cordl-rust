#[cfg(feature = "System+ResolveEventArgs")]
#[repr(C)]
#[derive(Debug)]
pub struct ResolveEventArgs {
    __cordl_parent: crate::System::EventArgs,
    pub _Name_k__BackingField: *mut crate::System::String,
    pub _RequestingAssembly_k__BackingField: *mut crate::System::Reflection::Assembly,
}
#[cfg(feature = "System+ResolveEventArgs")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::ResolveEventArgs => "System"
    ."ResolveEventArgs"
);
#[cfg(feature = "System+ResolveEventArgs")]
impl std::ops::Deref for crate::System::ResolveEventArgs {
    type Target = crate::System::EventArgs;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+ResolveEventArgs")]
impl std::ops::DerefMut for crate::System::ResolveEventArgs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+ResolveEventArgs")]
impl crate::System::ResolveEventArgs {
    pub fn New_Assembly1(
        name: *mut crate::System::String,
        requestingAssembly: *mut crate::System::Reflection::Assembly,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name, requestingAssembly))?;
        Ok(__cordl_object)
    }
    pub fn New_String0(
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (name))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_Assembly1(
        &mut self,
        name: *mut crate::System::String,
        requestingAssembly: *mut crate::System::Reflection::Assembly,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name, requestingAssembly))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_String0(
        &mut self,
        name: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (name))?;
        Ok(__cordl_ret)
    }
    pub fn get_Name(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("get_Name", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+ResolveEventArgs")]
impl quest_hook::libil2cpp::ObjectType for crate::System::ResolveEventArgs {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
