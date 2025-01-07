#[cfg(feature = "Microsoft+CodeAnalysis+EmbeddedAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct EmbeddedAttribute {
    __cordl_parent: crate::System::Attribute,
}
#[cfg(feature = "Microsoft+CodeAnalysis+EmbeddedAttribute")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Microsoft::CodeAnalysis::EmbeddedAttribute {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Microsoft.CodeAnalysis";
    const CLASS_NAME: &'static str = "EmbeddedAttribute";
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
#[cfg(feature = "Microsoft+CodeAnalysis+EmbeddedAttribute")]
impl std::ops::Deref for crate::Microsoft::CodeAnalysis::EmbeddedAttribute {
    type Target = crate::System::Attribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Microsoft+CodeAnalysis+EmbeddedAttribute")]
impl std::ops::DerefMut for crate::Microsoft::CodeAnalysis::EmbeddedAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Microsoft+CodeAnalysis+EmbeddedAttribute")]
impl crate::Microsoft::CodeAnalysis::EmbeddedAttribute {
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
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Microsoft+CodeAnalysis+EmbeddedAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::Microsoft::CodeAnalysis::EmbeddedAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
