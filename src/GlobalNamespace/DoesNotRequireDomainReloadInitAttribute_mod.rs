#[cfg(feature = "DoesNotRequireDomainReloadInitAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct DoesNotRequireDomainReloadInitAttribute {
    __cordl_parent: crate::UnityEngine::PropertyAttribute,
}
#[cfg(feature = "DoesNotRequireDomainReloadInitAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::DoesNotRequireDomainReloadInitAttribute => ""
    ."DoesNotRequireDomainReloadInitAttribute"
);
#[cfg(feature = "DoesNotRequireDomainReloadInitAttribute")]
impl std::ops::Deref
for crate::GlobalNamespace::DoesNotRequireDomainReloadInitAttribute {
    type Target = crate::UnityEngine::PropertyAttribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "DoesNotRequireDomainReloadInitAttribute")]
impl std::ops::DerefMut
for crate::GlobalNamespace::DoesNotRequireDomainReloadInitAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "DoesNotRequireDomainReloadInitAttribute")]
impl crate::GlobalNamespace::DoesNotRequireDomainReloadInitAttribute {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "DoesNotRequireDomainReloadInitAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::DoesNotRequireDomainReloadInitAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
