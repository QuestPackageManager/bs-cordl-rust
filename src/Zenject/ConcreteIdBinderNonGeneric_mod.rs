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
        bindContainer: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
        bindStatement: quest_hook::libil2cpp::Gc<crate::Zenject::BindStatement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bindContainer, bindInfo, bindStatement))?;
        Ok(__cordl_object.into())
    }
    pub fn WithId(
        &mut self,
        identifier: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::Zenject::ConcreteBinderNonGeneric>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Zenject::ConcreteBinderNonGeneric,
        > = __cordl_object.invoke("WithId", (identifier))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        bindContainer: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        bindInfo: quest_hook::libil2cpp::Gc<crate::Zenject::BindInfo>,
        bindStatement: quest_hook::libil2cpp::Gc<crate::Zenject::BindStatement>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bindContainer, bindInfo, bindStatement))?;
        Ok(__cordl_ret.into())
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
