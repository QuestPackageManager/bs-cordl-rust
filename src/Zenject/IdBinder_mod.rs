#[cfg(feature = "Zenject+IdBinder")]
#[repr(C)]
#[derive(Debug)]
pub struct IdBinder {
    __cordl_parent: crate::System::Object,
    pub _bindInfo: *mut crate::Zenject::BindInfo,
}
#[cfg(feature = "Zenject+IdBinder")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::IdBinder => "Zenject"."IdBinder"
);
#[cfg(feature = "Zenject+IdBinder")]
impl std::ops::Deref for crate::Zenject::IdBinder {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+IdBinder")]
impl std::ops::DerefMut for crate::Zenject::IdBinder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+IdBinder")]
impl crate::Zenject::IdBinder {
    pub fn New(
        bindInfo: *mut crate::Zenject::BindInfo,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bindInfo))?;
        Ok(__cordl_object)
    }
    pub fn WithId(
        &mut self,
        identifier: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("WithId", (identifier))?;
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
#[cfg(feature = "Zenject+IdBinder")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::IdBinder {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}