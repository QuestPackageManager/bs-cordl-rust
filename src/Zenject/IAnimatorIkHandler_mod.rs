#[cfg(feature = "Zenject+IAnimatorIkHandler")]
#[repr(C)]
#[derive(Debug)]
pub struct IAnimatorIkHandler {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Zenject+IAnimatorIkHandler")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::IAnimatorIkHandler => "Zenject"
    ."IAnimatorIkHandler"
);
#[cfg(feature = "Zenject+IAnimatorIkHandler")]
impl std::ops::Deref for crate::Zenject::IAnimatorIkHandler {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+IAnimatorIkHandler")]
impl std::ops::DerefMut for crate::Zenject::IAnimatorIkHandler {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+IAnimatorIkHandler")]
impl crate::Zenject::IAnimatorIkHandler {
    pub fn OnAnimatorIk(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnAnimatorIk", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
}
#[cfg(feature = "Zenject+IAnimatorIkHandler")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::IAnimatorIkHandler {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
