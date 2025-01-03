#[cfg(feature = "Zenject+CopyNonLazyBinder")]
#[repr(C)]
#[derive(Debug)]
pub struct CopyNonLazyBinder {
    __cordl_parent: crate::Zenject::NonLazyBinder,
    pub _secondaryBindInfos: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<*mut crate::Zenject::BindInfo>,
    >,
}
#[cfg(feature = "Zenject+CopyNonLazyBinder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::CopyNonLazyBinder => "Zenject"
    ."CopyNonLazyBinder"
);
#[cfg(feature = "Zenject+CopyNonLazyBinder")]
impl std::ops::Deref for crate::Zenject::CopyNonLazyBinder {
    type Target = crate::Zenject::NonLazyBinder;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+CopyNonLazyBinder")]
impl std::ops::DerefMut for crate::Zenject::CopyNonLazyBinder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+CopyNonLazyBinder")]
impl crate::Zenject::CopyNonLazyBinder {
    pub fn AddSecondaryCopyBindInfo(
        &mut self,
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("AddSecondaryCopyBindInfo", (bindInfo))?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyIntoAllSubContainers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::NonLazyBinder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::NonLazyBinder> = __cordl_object
            .invoke("CopyIntoAllSubContainers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn CopyIntoDirectSubContainers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::NonLazyBinder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::NonLazyBinder> = __cordl_object
            .invoke("CopyIntoDirectSubContainers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveIntoAllSubContainers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::NonLazyBinder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::NonLazyBinder> = __cordl_object
            .invoke("MoveIntoAllSubContainers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn MoveIntoDirectSubContainers(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::NonLazyBinder>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::Zenject::NonLazyBinder> = __cordl_object
            .invoke("MoveIntoDirectSubContainers", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bindInfo))?;
        Ok(__cordl_object.into())
    }
    pub fn SetInheritanceMethod(
        &mut self,
        method: crate::Zenject::BindingInheritanceMethods,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetInheritanceMethod", (method))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bindInfo))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+CopyNonLazyBinder")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::CopyNonLazyBinder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
