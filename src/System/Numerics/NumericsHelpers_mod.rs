#[cfg(feature = "System+Numerics+NumericsHelpers")]
#[repr(C)]
#[derive(Debug)]
pub struct NumericsHelpers {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "System+Numerics+NumericsHelpers")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Numerics::NumericsHelpers =>
    "System.Numerics"."NumericsHelpers"
);
#[cfg(feature = "System+Numerics+NumericsHelpers")]
impl std::ops::Deref for crate::System::Numerics::NumericsHelpers {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Numerics+NumericsHelpers")]
impl std::ops::DerefMut for crate::System::Numerics::NumericsHelpers {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Numerics+NumericsHelpers")]
impl crate::System::Numerics::NumericsHelpers {}
#[cfg(feature = "System+Numerics+NumericsHelpers")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Numerics::NumericsHelpers {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}