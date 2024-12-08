#[cfg(feature = "EnvironmentKeywords")]
#[repr(C)]
#[derive(Debug)]
pub struct EnvironmentKeywords {
    __cordl_parent: crate::System::Object,
    pub _environmentKeywords: *mut crate::System::Collections::Generic::IReadOnlyList_1<
        *mut crate::System::String,
    >,
    pub _environmentKeywordsSet: *mut crate::System::Collections::Generic::HashSet_1<
        *mut crate::System::String,
    >,
}
#[cfg(feature = "EnvironmentKeywords")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for EnvironmentKeywords => ""."EnvironmentKeywords"
);
#[cfg(feature = "EnvironmentKeywords")]
impl std::ops::Deref for EnvironmentKeywords {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentKeywords")]
impl std::ops::DerefMut for EnvironmentKeywords {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "EnvironmentKeywords")]
impl EnvironmentKeywords {
    pub fn get_environmentKeywords(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::System::String,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::System::String,
        > = __cordl_object.invoke("get_environmentKeywords", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        environmentKeywords: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (environmentKeywords))?;
        Ok(__cordl_ret)
    }
    pub fn HasKeyword(
        &mut self,
        keyword: *mut crate::System::String,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasKeyword", (keyword))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        environmentKeywords: *mut crate::System::Collections::Generic::IReadOnlyList_1<
            *mut crate::System::String,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (environmentKeywords))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "EnvironmentKeywords")]
impl quest_hook::libil2cpp::ObjectType for EnvironmentKeywords {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
