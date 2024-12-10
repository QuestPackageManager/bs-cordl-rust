#[cfg(feature = "IJumpOffsetYProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct IJumpOffsetYProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IJumpOffsetYProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IJumpOffsetYProvider => ""
    ."IJumpOffsetYProvider"
);
#[cfg(feature = "IJumpOffsetYProvider")]
impl std::ops::Deref for crate::GlobalNamespace::IJumpOffsetYProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IJumpOffsetYProvider")]
impl std::ops::DerefMut for crate::GlobalNamespace::IJumpOffsetYProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IJumpOffsetYProvider")]
impl crate::GlobalNamespace::IJumpOffsetYProvider {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_jumpOffsetY(&mut self) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f32 = __cordl_object.invoke("get_jumpOffsetY", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "IJumpOffsetYProvider")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::IJumpOffsetYProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
