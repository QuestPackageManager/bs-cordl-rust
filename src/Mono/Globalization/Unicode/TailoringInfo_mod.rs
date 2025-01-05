#[cfg(feature = "Mono+Globalization+Unicode+TailoringInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct TailoringInfo {
    __cordl_parent: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    pub LCID: i32,
    pub TailoringIndex: i32,
    pub TailoringCount: i32,
    pub FrenchSort: bool,
}
#[cfg(feature = "Mono+Globalization+Unicode+TailoringInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Mono::Globalization::Unicode::TailoringInfo =>
    "Mono.Globalization.Unicode"."TailoringInfo"
);
#[cfg(feature = "Mono+Globalization+Unicode+TailoringInfo")]
impl std::ops::Deref for crate::Mono::Globalization::Unicode::TailoringInfo {
    type Target = quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+TailoringInfo")]
impl std::ops::DerefMut for crate::Mono::Globalization::Unicode::TailoringInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+TailoringInfo")]
impl crate::Mono::Globalization::Unicode::TailoringInfo {
    pub fn New(
        lcid: i32,
        tailoringIndex: i32,
        tailoringCount: i32,
        frenchSort: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (lcid, tailoringIndex, tailoringCount, frenchSort))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        lcid: i32,
        tailoringIndex: i32,
        tailoringCount: i32,
        frenchSort: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (lcid, tailoringIndex, tailoringCount, frenchSort))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Mono+Globalization+Unicode+TailoringInfo")]
impl quest_hook::libil2cpp::ObjectType
for crate::Mono::Globalization::Unicode::TailoringInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
