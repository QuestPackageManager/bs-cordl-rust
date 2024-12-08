#[cfg(feature = "Zenject+NonLazyBinder")]
#[repr(C)]
#[derive(Debug)]
pub struct NonLazyBinder {
    __cordl_parent: crate::Zenject::IfNotBoundBinder,
}
#[cfg(feature = "Zenject+NonLazyBinder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::NonLazyBinder => "Zenject"
    ."NonLazyBinder"
);
#[cfg(feature = "Zenject+NonLazyBinder")]
impl std::ops::Deref for crate::Zenject::NonLazyBinder {
    type Target = crate::Zenject::IfNotBoundBinder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+NonLazyBinder")]
impl std::ops::DerefMut for crate::Zenject::NonLazyBinder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+NonLazyBinder")]
impl crate::Zenject::NonLazyBinder {
    pub fn Lazy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::IfNotBoundBinder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::IfNotBoundBinder = __cordl_object
            .invoke("Lazy", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        bindInfo: *mut crate::Zenject::BindInfo,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bindInfo))?;
        Ok(__cordl_object)
    }
    pub fn NonLazy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::IfNotBoundBinder> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::IfNotBoundBinder = __cordl_object
            .invoke("NonLazy", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        bindInfo: *mut crate::Zenject::BindInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bindInfo))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+NonLazyBinder")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::NonLazyBinder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
