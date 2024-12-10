#[cfg(feature = "System+Runtime+CompilerServices+InternalsVisibleToAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct InternalsVisibleToAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _assemblyName: *mut quest_hook::libil2cpp::Il2CppString,
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
    pub fn New(
        assemblyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (assemblyName))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        assemblyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (assemblyName))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_AllInternalsVisible(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_AllInternalsVisible", (value))?;
        Ok(__cordl_ret.into())
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
