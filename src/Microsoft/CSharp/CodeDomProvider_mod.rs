#[cfg(feature = "Microsoft+CSharp+CodeDomProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct CodeDomProvider {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "Microsoft+CSharp+CodeDomProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Microsoft::CSharp::CodeDomProvider =>
    "Microsoft.CSharp"."CodeDomProvider"
);
#[cfg(feature = "Microsoft+CSharp+CodeDomProvider")]
impl std::ops::Deref for crate::Microsoft::CSharp::CodeDomProvider {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Microsoft+CSharp+CodeDomProvider")]
impl std::ops::DerefMut for crate::Microsoft::CSharp::CodeDomProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Microsoft+CSharp+CodeDomProvider")]
impl crate::Microsoft::CSharp::CodeDomProvider {
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Microsoft+CSharp+CodeDomProvider")]
impl quest_hook::libil2cpp::ObjectType for crate::Microsoft::CSharp::CodeDomProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
