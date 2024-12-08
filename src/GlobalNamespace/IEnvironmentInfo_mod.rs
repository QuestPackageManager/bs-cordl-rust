#[cfg(feature = "IEnvironmentInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct IEnvironmentInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IEnvironmentInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for IEnvironmentInfo => ""."IEnvironmentInfo"
);
#[cfg(feature = "IEnvironmentInfo")]
impl std::ops::Deref for IEnvironmentInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IEnvironmentInfo")]
impl std::ops::DerefMut for IEnvironmentInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IEnvironmentInfo")]
impl IEnvironmentInfo {
    pub fn get_defaultLightshowAsset(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::TextAsset> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::TextAsset = __cordl_object
            .invoke("get_defaultLightshowAsset", ())?;
        Ok(__cordl_ret)
    }
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
    pub fn get_environmentLightGroups(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut IEnvironmentLightGroups> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut IEnvironmentLightGroups = __cordl_object
            .invoke("get_environmentLightGroups", ())?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IEnvironmentInfo")]
impl quest_hook::libil2cpp::ObjectType for IEnvironmentInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
