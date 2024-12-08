#[cfg(feature = "System+Runtime+CompilerServices+CompilationRelaxationsAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct CompilationRelaxationsAttribute {
    __cordl_parent: crate::System::Attribute,
    pub m_relaxations: i32,
}
#[cfg(feature = "System+Runtime+CompilerServices+CompilationRelaxationsAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Runtime::CompilerServices::CompilationRelaxationsAttribute =>
    "System.Runtime.CompilerServices"."CompilationRelaxationsAttribute"
);
#[cfg(feature = "System+Runtime+CompilerServices+CompilationRelaxationsAttribute")]
impl std::ops::Deref
for crate::System::Runtime::CompilerServices::CompilationRelaxationsAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+CompilationRelaxationsAttribute")]
impl std::ops::DerefMut
for crate::System::Runtime::CompilerServices::CompilationRelaxationsAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+CompilationRelaxationsAttribute")]
impl crate::System::Runtime::CompilerServices::CompilationRelaxationsAttribute {
    pub fn New_CompilationRelaxations1(
        relaxations: crate::System::Runtime::CompilerServices::CompilationRelaxations,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (relaxations))?;
        Ok(__cordl_object)
    }
    pub fn New_i32_0(relaxations: i32) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (relaxations))?;
        Ok(__cordl_object)
    }
    pub fn _ctor_CompilationRelaxations1(
        &mut self,
        relaxations: crate::System::Runtime::CompilerServices::CompilationRelaxations,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (relaxations))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor_i32_0(
        &mut self,
        relaxations: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (relaxations))?;
        Ok(__cordl_ret)
    }
    pub fn get_CompilationRelaxations(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_CompilationRelaxations", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "System+Runtime+CompilerServices+CompilationRelaxationsAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::CompilerServices::CompilationRelaxationsAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
