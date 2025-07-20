#[cfg(feature = "System+Diagnostics+CodeAnalysis+ExcludeFromCodeCoverageAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct ExcludeFromCodeCoverageAttribute {
    __cordl_parent: crate::System::Attribute,
}
#[cfg(feature = "System+Diagnostics+CodeAnalysis+ExcludeFromCodeCoverageAttribute")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Diagnostics::CodeAnalysis::ExcludeFromCodeCoverageAttribute {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Diagnostics.CodeAnalysis";
    const CLASS_NAME: &'static str = "ExcludeFromCodeCoverageAttribute";
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
#[cfg(feature = "System+Diagnostics+CodeAnalysis+ExcludeFromCodeCoverageAttribute")]
impl std::ops::Deref
for crate::System::Diagnostics::CodeAnalysis::ExcludeFromCodeCoverageAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+CodeAnalysis+ExcludeFromCodeCoverageAttribute")]
impl std::ops::DerefMut
for crate::System::Diagnostics::CodeAnalysis::ExcludeFromCodeCoverageAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Diagnostics+CodeAnalysis+ExcludeFromCodeCoverageAttribute")]
impl crate::System::Diagnostics::CodeAnalysis::ExcludeFromCodeCoverageAttribute {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::System::Diagnostics::CodeAnalysis::ExcludeFromCodeCoverageAttribute as quest_hook::libil2cpp::Type>::class()
            .find_method::<(), quest_hook::libil2cpp::Void, 0usize>(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate
                    ::System::Diagnostics::CodeAnalysis::ExcludeFromCodeCoverageAttribute
                    as quest_hook::libil2cpp::Type > ::class(), ".ctor", 0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, ())?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Diagnostics+CodeAnalysis+ExcludeFromCodeCoverageAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Diagnostics::CodeAnalysis::ExcludeFromCodeCoverageAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
