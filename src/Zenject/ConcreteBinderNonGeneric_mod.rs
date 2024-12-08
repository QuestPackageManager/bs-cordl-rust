#[cfg(feature = "Zenject+ConcreteBinderNonGeneric")]
#[repr(C)]
#[derive(Debug)]
pub struct ConcreteBinderNonGeneric {
    __cordl_parent: crate::Zenject::FromBinderNonGeneric,
}
#[cfg(feature = "Zenject+ConcreteBinderNonGeneric")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::ConcreteBinderNonGeneric => "Zenject"
    ."ConcreteBinderNonGeneric"
);
#[cfg(feature = "Zenject+ConcreteBinderNonGeneric")]
impl std::ops::Deref for crate::Zenject::ConcreteBinderNonGeneric {
    type Target = crate::Zenject::FromBinderNonGeneric;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ConcreteBinderNonGeneric")]
impl std::ops::DerefMut for crate::Zenject::ConcreteBinderNonGeneric {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ConcreteBinderNonGeneric")]
impl crate::Zenject::ConcreteBinderNonGeneric {
    pub fn ToSelf(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::FromBinderNonGeneric> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FromBinderNonGeneric = __cordl_object
            .invoke("ToSelf", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ToSelf_b__1_0(
        &mut self,
        container: *mut crate::Zenject::DiContainer,
        _cordl_type: *mut crate::System::Type,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::IProvider> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::IProvider = __cordl_object
            .invoke("<ToSelf>b__1_0", (container, _cordl_type))?;
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
    pub fn To_0<TConcrete>(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::FromBinderNonGeneric>
    where
        TConcrete: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FromBinderNonGeneric = __cordl_object
            .invoke("To", ())?;
        Ok(__cordl_ret)
    }
    pub fn To_Il2CppArray1(
        &mut self,
        concreteTypes: *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::FromBinderNonGeneric> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FromBinderNonGeneric = __cordl_object
            .invoke("To", (concreteTypes))?;
        Ok(__cordl_ret)
    }
    pub fn To_IEnumerable_1_2(
        &mut self,
        concreteTypes: *mut crate::System::Collections::Generic::IEnumerable_1<
            *mut crate::System::Type,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::FromBinderNonGeneric> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FromBinderNonGeneric = __cordl_object
            .invoke("To", (concreteTypes))?;
        Ok(__cordl_ret)
    }
    pub fn To_Action_1_3(
        &mut self,
        generator: *mut crate::System::Action_1<
            *mut crate::Zenject::ConventionSelectTypesBinder,
        >,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::FromBinderNonGeneric> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::FromBinderNonGeneric = __cordl_object
            .invoke("To", (generator))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        bindContainer: *mut crate::Zenject::DiContainer,
        bindInfo: *mut crate::Zenject::BindInfo,
        bindStatement: *mut crate::Zenject::BindStatement,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (bindContainer, bindInfo, bindStatement))?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "Zenject+ConcreteBinderNonGeneric")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::ConcreteBinderNonGeneric {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
