#[cfg(feature = "IDspTimeProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct IDspTimeProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IDspTimeProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IDspTimeProvider => ""
    ."IDspTimeProvider"
);
#[cfg(feature = "IDspTimeProvider")]
impl std::ops::Deref for crate::GlobalNamespace::IDspTimeProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IDspTimeProvider")]
impl std::ops::DerefMut for crate::GlobalNamespace::IDspTimeProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IDspTimeProvider")]
impl crate::GlobalNamespace::IDspTimeProvider {
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_dspTime(&mut self) -> quest_hook::libil2cpp::Result<f64> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: f64 = __cordl_object.invoke("get_dspTime", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "IDspTimeProvider")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::IDspTimeProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
