#[cfg(feature = "System+Text+RegularExpressions+ExclusiveReference")]
#[repr(C)]
#[derive(Debug)]
pub struct ExclusiveReference {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _ref: *mut crate::System::Text::RegularExpressions::RegexRunner,
    pub _obj: *mut crate::System::Text::RegularExpressions::RegexRunner,
    pub _locked: i32,
}
#[cfg(feature = "System+Text+RegularExpressions+ExclusiveReference")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::System::Text::RegularExpressions::ExclusiveReference =>
    "System.Text.RegularExpressions"."ExclusiveReference"
);
#[cfg(feature = "System+Text+RegularExpressions+ExclusiveReference")]
impl std::ops::Deref for crate::System::Text::RegularExpressions::ExclusiveReference {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+ExclusiveReference")]
impl std::ops::DerefMut for crate::System::Text::RegularExpressions::ExclusiveReference {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Text+RegularExpressions+ExclusiveReference")]
impl crate::System::Text::RegularExpressions::ExclusiveReference {
    pub fn Get(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Text::RegularExpressions::RegexRunner,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Text::RegularExpressions::RegexRunner = __cordl_object
            .invoke("Get", ())?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Release(
        &mut self,
        obj: *mut crate::System::Text::RegularExpressions::RegexRunner,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Release", (obj))?;
        Ok(__cordl_ret)
    }
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
}
#[cfg(feature = "System+Text+RegularExpressions+ExclusiveReference")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Text::RegularExpressions::ExclusiveReference {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
