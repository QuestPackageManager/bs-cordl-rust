#[cfg(feature = "Zenject+FactoryFromBinderUntyped")]
#[repr(C)]
#[derive(Debug)]
pub struct FactoryFromBinderUntyped {
    __cordl_parent: crate::Zenject::FactoryFromBinderBase,
}
#[cfg(feature = "Zenject+FactoryFromBinderUntyped")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::FactoryFromBinderUntyped => "Zenject"
    ."FactoryFromBinderUntyped"
);
#[cfg(feature = "Zenject+FactoryFromBinderUntyped")]
impl std::ops::Deref for crate::Zenject::FactoryFromBinderUntyped {
    type Target = crate::Zenject::FactoryFromBinderBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactoryFromBinderUntyped")]
impl std::ops::DerefMut for crate::Zenject::FactoryFromBinderUntyped {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+FactoryFromBinderUntyped")]
impl crate::Zenject::FactoryFromBinderUntyped {
    pub fn New(
        bindContainer: *mut crate::Zenject::DiContainer,
        contractType: *mut crate::System::Type,
        bindInfo: *mut crate::Zenject::BindInfo,
        factoryBindInfo: *mut crate::Zenject::FactoryBindInfo,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (bindContainer, contractType, bindInfo, factoryBindInfo),
            )?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        bindContainer: *mut crate::Zenject::DiContainer,
        contractType: *mut crate::System::Type,
        bindInfo: *mut crate::Zenject::BindInfo,
        factoryBindInfo: *mut crate::Zenject::FactoryBindInfo,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (bindContainer, contractType, bindInfo, factoryBindInfo))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+FactoryFromBinderUntyped")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::FactoryFromBinderUntyped {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
