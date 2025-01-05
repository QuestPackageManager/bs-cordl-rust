#[cfg(feature = "System+Diagnostics+CodeAnalysis+NotNullWhenAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct NotNullWhenAttribute {
    __cordl_parent: crate::System::Attribute,
    pub _ReturnValue_k__BackingField: bool,
}
#[cfg(feature = "System+Diagnostics+CodeAnalysis+NotNullWhenAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Diagnostics::CodeAnalysis::NotNullWhenAttribute =>
    "System.Diagnostics.CodeAnalysis"."NotNullWhenAttribute"
);
#[cfg(feature = "System+Diagnostics+CodeAnalysis+NotNullWhenAttribute")]
impl std::ops::Deref for crate::System::Diagnostics::CodeAnalysis::NotNullWhenAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+CodeAnalysis+NotNullWhenAttribute")]
impl std::ops::DerefMut
for crate::System::Diagnostics::CodeAnalysis::NotNullWhenAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+CodeAnalysis+NotNullWhenAttribute")]
impl crate::System::Diagnostics::CodeAnalysis::NotNullWhenAttribute {
    pub fn New(
        returnValue: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (returnValue))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        returnValue: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (returnValue))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ReturnValue(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_ReturnValue", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Diagnostics+CodeAnalysis+NotNullWhenAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Diagnostics::CodeAnalysis::NotNullWhenAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
