#[cfg(feature = "Mono+Globalization+Unicode+TailoringInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct TailoringInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub LCID: i32,
    pub TailoringIndex: i32,
    pub TailoringCount: i32,
    pub FrenchSort: bool,
}
#[cfg(feature = "Mono+Globalization+Unicode+TailoringInfo")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Mono::Globalization::Unicode::TailoringInfo {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Mono.Globalization.Unicode";
    const CLASS_NAME: &'static str = "TailoringInfo";
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
#[cfg(feature = "Mono+Globalization+Unicode+TailoringInfo")]
impl std::ops::Deref for crate::Mono::Globalization::Unicode::TailoringInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
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
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Mono::Globalization::Unicode::TailoringInfo as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (i32, i32, i32, bool),
                quest_hook::libil2cpp::Void,
                4usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Mono::Globalization::Unicode::TailoringInfo as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 4usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method
                .invoke_unchecked(
                    self,
                    (lcid, tailoringIndex, tailoringCount, frenchSort),
                )?
        };
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
