#[cfg(feature = "System+CodeDom+Compiler+GeneratedCodeAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct GeneratedCodeAttribute {
    __cordl_parent: crate::System::Attribute,
    pub tool: *mut quest_hook::libil2cpp::Il2CppString,
    pub version: *mut quest_hook::libil2cpp::Il2CppString,
}
#[cfg(feature = "System+CodeDom+Compiler+GeneratedCodeAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::CodeDom::Compiler::GeneratedCodeAttribute => "System.CodeDom.Compiler"
    ."GeneratedCodeAttribute"
);
#[cfg(feature = "System+CodeDom+Compiler+GeneratedCodeAttribute")]
impl std::ops::Deref for crate::System::CodeDom::Compiler::GeneratedCodeAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+CodeDom+Compiler+GeneratedCodeAttribute")]
impl std::ops::DerefMut for crate::System::CodeDom::Compiler::GeneratedCodeAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+CodeDom+Compiler+GeneratedCodeAttribute")]
impl crate::System::CodeDom::Compiler::GeneratedCodeAttribute {
    pub fn New(
        tool: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        version: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (tool, version))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        tool: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        version: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (tool, version))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+CodeDom+Compiler+GeneratedCodeAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::CodeDom::Compiler::GeneratedCodeAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
