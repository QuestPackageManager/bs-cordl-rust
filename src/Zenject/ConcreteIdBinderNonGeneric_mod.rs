#[cfg(feature = "Zenject+ConcreteIdBinderNonGeneric")]
#[repr(C)]
#[derive(Debug)]
pub struct ConcreteIdBinderNonGeneric {
    __cordl_parent: crate::Zenject::ConcreteBinderNonGeneric,
}
#[cfg(feature = "Zenject+ConcreteIdBinderNonGeneric")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::ConcreteIdBinderNonGeneric => "Zenject"
    ."ConcreteIdBinderNonGeneric"
);
#[cfg(feature = "Zenject+ConcreteIdBinderNonGeneric")]
impl std::ops::Deref for crate::Zenject::ConcreteIdBinderNonGeneric {
    type Target = crate::Zenject::ConcreteBinderNonGeneric;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ConcreteIdBinderNonGeneric")]
impl std::ops::DerefMut for crate::Zenject::ConcreteIdBinderNonGeneric {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ConcreteIdBinderNonGeneric")]
impl crate::Zenject::ConcreteIdBinderNonGeneric {
    pub fn New(
        bindContainer: *mut crate::Zenject::DiContainer,
        bindInfo: *mut crate::Zenject::BindInfo,
        bindStatement: *mut crate::Zenject::BindStatement,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bindContainer, bindInfo, bindStatement))?;
        Ok(__cordl_object)
    }
    pub fn WithId(
        &mut self,
        identifier: *mut crate::System::Object,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::ConcreteBinderNonGeneric> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::ConcreteBinderNonGeneric = __cordl_object
            .invoke("WithId", (identifier))?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
        bindContainer: *mut crate::Zenject::DiContainer,
        bindInfo: *mut crate::Zenject::BindInfo,
        bindStatement: *mut crate::Zenject::BindStatement,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bindContainer, bindInfo, bindStatement))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+ConcreteIdBinderNonGeneric")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::ConcreteIdBinderNonGeneric {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
