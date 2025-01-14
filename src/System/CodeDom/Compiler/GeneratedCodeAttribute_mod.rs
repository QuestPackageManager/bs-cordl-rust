#[cfg(feature = "System+CodeDom+Compiler+GeneratedCodeAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct GeneratedCodeAttribute {
    __cordl_parent: crate::System::Attribute,
    pub tool: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub version: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
}
#[cfg(feature = "System+CodeDom+Compiler+GeneratedCodeAttribute")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::CodeDom::Compiler::GeneratedCodeAttribute {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.CodeDom.Compiler";
    const CLASS_NAME: &'static str = "GeneratedCodeAttribute";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (tool, version))
        };
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
