#[cfg(feature = "UnityEngine+IExposedPropertyTable")]
#[repr(C)]
#[derive(Debug)]
pub struct IExposedPropertyTable {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "UnityEngine+IExposedPropertyTable")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::IExposedPropertyTable =>
    "UnityEngine"."IExposedPropertyTable"
);
#[cfg(feature = "UnityEngine+IExposedPropertyTable")]
impl std::ops::Deref for crate::UnityEngine::IExposedPropertyTable {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+IExposedPropertyTable")]
impl std::ops::DerefMut for crate::UnityEngine::IExposedPropertyTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+IExposedPropertyTable")]
impl crate::UnityEngine::IExposedPropertyTable {
    pub fn GetReferenceValue(
        &mut self,
        id: crate::UnityEngine::PropertyName,
        idValid: quest_hook::libil2cpp::ByRefMut<bool>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::UnityEngine::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::UnityEngine::Object = __cordl_object
            .invoke("GetReferenceValue", (id, idValid))?;
        Ok(__cordl_ret)
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "UnityEngine+IExposedPropertyTable")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::IExposedPropertyTable {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
