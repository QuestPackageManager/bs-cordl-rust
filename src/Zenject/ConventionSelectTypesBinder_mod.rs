#[cfg(feature = "Zenject+ConventionSelectTypesBinder")]
#[repr(C)]
#[derive(Debug)]
pub struct ConventionSelectTypesBinder {
    __cordl_parent: crate::System::Object,
    pub _bindInfo: *mut crate::Zenject::ConventionBindInfo,
}
#[cfg(feature = "Zenject+ConventionSelectTypesBinder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::ConventionSelectTypesBinder =>
    "Zenject"."ConventionSelectTypesBinder"
);
#[cfg(feature = "Zenject+ConventionSelectTypesBinder")]
impl std::ops::Deref for crate::Zenject::ConventionSelectTypesBinder {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ConventionSelectTypesBinder")]
impl std::ops::DerefMut for crate::Zenject::ConventionSelectTypesBinder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ConventionSelectTypesBinder")]
impl crate::Zenject::ConventionSelectTypesBinder {
    #[cfg(feature = "Zenject+ConventionSelectTypesBinder+__c")]
    pub type __c = crate::Zenject::ConventionSelectTypesBinder___c;
    pub fn AllClasses(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::ConventionFilterTypesBinder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConventionFilterTypesBinder = __cordl_object
            .invoke("AllClasses", ())?;
        Ok(__cordl_ret)
    }
    pub fn AllNonAbstractClasses(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::ConventionFilterTypesBinder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConventionFilterTypesBinder = __cordl_object
            .invoke("AllNonAbstractClasses", ())?;
        Ok(__cordl_ret)
    }
    pub fn AllAbstractClasses(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::ConventionFilterTypesBinder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConventionFilterTypesBinder = __cordl_object
            .invoke("AllAbstractClasses", ())?;
        Ok(__cordl_ret)
    }
    pub fn AllTypes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::ConventionFilterTypesBinder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConventionFilterTypesBinder = __cordl_object
            .invoke("AllTypes", ())?;
        Ok(__cordl_ret)
    }
    pub fn AllInterfaces(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::ConventionFilterTypesBinder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConventionFilterTypesBinder = __cordl_object
            .invoke("AllInterfaces", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        bindInfo: *mut crate::Zenject::ConventionBindInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bindInfo))?;
        Ok(__cordl_ret)
    }
    pub fn CreateNextBinder(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut crate::Zenject::ConventionFilterTypesBinder,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConventionFilterTypesBinder = __cordl_object
            .invoke("CreateNextBinder", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        bindInfo: *mut crate::Zenject::ConventionBindInfo,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bindInfo))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Zenject+ConventionSelectTypesBinder")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::ConventionSelectTypesBinder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
