#[cfg(feature = "Zenject+IBindingFinalizer")]
#[repr(C)]
#[derive(Debug)]
pub struct IBindingFinalizer {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Zenject+IBindingFinalizer")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::IBindingFinalizer => "Zenject"
    ."IBindingFinalizer"
);
#[cfg(feature = "Zenject+IBindingFinalizer")]
impl std::ops::Deref for crate::Zenject::IBindingFinalizer {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+IBindingFinalizer")]
impl std::ops::DerefMut for crate::Zenject::IBindingFinalizer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+IBindingFinalizer")]
impl crate::Zenject::IBindingFinalizer {
    pub fn FinalizeBinding(
        &mut self,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("FinalizeBinding", (container))?;
        Ok(__cordl_ret.into())
    }
    pub fn from_object_mut(
        object_param: *mut quest_hook::libil2cpp::Il2CppObject,
    ) -> *mut Self {
        unsafe { (object_param as *mut Self) }
    }
    pub fn get_BindingInheritanceMethod(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Zenject::BindingInheritanceMethods> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Zenject::BindingInheritanceMethods = __cordl_object
            .invoke("get_BindingInheritanceMethod", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+IBindingFinalizer")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::IBindingFinalizer {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
