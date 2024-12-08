#[cfg(feature = "System+Runtime+CompilerServices+InternalsVisibleToAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct InternalsVisibleToAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _assemblyName: *mut crate::System::String,
    pub _allInternalsVisible: bool,
}
#[cfg(feature = "System+Runtime+CompilerServices+InternalsVisibleToAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::CompilerServices::InternalsVisibleToAttribute =>
    "System.Runtime.CompilerServices"."InternalsVisibleToAttribute"
);
#[cfg(feature = "System+Runtime+CompilerServices+InternalsVisibleToAttribute")]
impl std::ops::Deref
for crate::System::Runtime::CompilerServices::InternalsVisibleToAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+InternalsVisibleToAttribute")]
impl std::ops::DerefMut
for crate::System::Runtime::CompilerServices::InternalsVisibleToAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+InternalsVisibleToAttribute")]
impl crate::System::Runtime::CompilerServices::InternalsVisibleToAttribute {
    pub fn set_AllInternalsVisible(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AllInternalsVisible", (value))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        assemblyName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (assemblyName))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        assemblyName: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (assemblyName))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+InternalsVisibleToAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::CompilerServices::InternalsVisibleToAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
