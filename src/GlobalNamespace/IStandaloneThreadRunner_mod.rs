#[cfg(feature = "IStandaloneThreadRunner")]
#[repr(C)]
#[derive(Debug)]
pub struct IStandaloneThreadRunner {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "IStandaloneThreadRunner")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::IStandaloneThreadRunner => ""
    ."IStandaloneThreadRunner"
);
#[cfg(feature = "IStandaloneThreadRunner")]
impl std::ops::Deref for crate::GlobalNamespace::IStandaloneThreadRunner {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "IStandaloneThreadRunner")]
impl std::ops::DerefMut for crate::GlobalNamespace::IStandaloneThreadRunner {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "IStandaloneThreadRunner")]
impl crate::GlobalNamespace::IStandaloneThreadRunner {
    pub fn Run(
        &mut self,
        runnable: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::IStandaloneThreadRunnable,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Run", (runnable))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "IStandaloneThreadRunner")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::IStandaloneThreadRunner {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
