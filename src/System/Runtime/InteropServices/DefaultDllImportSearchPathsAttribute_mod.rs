#[cfg(feature = "System+Runtime+InteropServices+DefaultDllImportSearchPathsAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct DefaultDllImportSearchPathsAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _paths: crate::System::Runtime::InteropServices::DllImportSearchPath,
}
#[cfg(feature = "System+Runtime+InteropServices+DefaultDllImportSearchPathsAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::InteropServices::DefaultDllImportSearchPathsAttribute =>
    "System.Runtime.InteropServices"."DefaultDllImportSearchPathsAttribute"
);
#[cfg(feature = "System+Runtime+InteropServices+DefaultDllImportSearchPathsAttribute")]
impl std::ops::Deref
for crate::System::Runtime::InteropServices::DefaultDllImportSearchPathsAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+DefaultDllImportSearchPathsAttribute")]
impl std::ops::DerefMut
for crate::System::Runtime::InteropServices::DefaultDllImportSearchPathsAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+InteropServices+DefaultDllImportSearchPathsAttribute")]
impl crate::System::Runtime::InteropServices::DefaultDllImportSearchPathsAttribute {
    pub fn New(
        paths: crate::System::Runtime::InteropServices::DllImportSearchPath,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (paths))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        paths: crate::System::Runtime::InteropServices::DllImportSearchPath,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (paths))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+InteropServices+DefaultDllImportSearchPathsAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::InteropServices::DefaultDllImportSearchPathsAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
