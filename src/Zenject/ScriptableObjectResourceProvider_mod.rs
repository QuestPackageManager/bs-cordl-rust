#[cfg(feature = "Zenject+ScriptableObjectResourceProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct ScriptableObjectResourceProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
    pub _resourceType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub _resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub _extraArguments: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::List_1<crate::Zenject::TypeValuePair>,
    >,
    pub _createNew: bool,
    pub _concreteIdentifier: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppObject,
    >,
    pub _instantiateCallback: quest_hook::libil2cpp::Gc<
        crate::System::Action_2<
            *mut crate::Zenject::InjectContext,
            *mut quest_hook::libil2cpp::Il2CppObject,
        >,
    >,
}
#[cfg(feature = "Zenject+ScriptableObjectResourceProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::ScriptableObjectResourceProvider =>
    "Zenject"."ScriptableObjectResourceProvider"
);
#[cfg(feature = "Zenject+ScriptableObjectResourceProvider")]
impl std::ops::Deref for crate::Zenject::ScriptableObjectResourceProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ScriptableObjectResourceProvider")]
impl std::ops::DerefMut for crate::Zenject::ScriptableObjectResourceProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ScriptableObjectResourceProvider")]
impl crate::Zenject::ScriptableObjectResourceProvider {
    pub fn GetAllInstancesWithInjectSplit(
        &mut self,
        context: quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
        args: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::Zenject::TypeValuePair>,
        >,
        injectAction: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Action>,
        buffer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "GetAllInstancesWithInjectSplit",
                (context, args, injectAction, buffer),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetInstanceType(
        &mut self,
        context: quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("GetInstanceType", (context))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        resourceType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        extraArguments: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::Zenject::TypeValuePair,
            >,
        >,
        createNew: bool,
        concreteIdentifier: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
        instantiateCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::Zenject::InjectContext,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(
                ".ctor",
                (
                    resourcePath,
                    resourceType,
                    container,
                    extraArguments,
                    createNew,
                    concreteIdentifier,
                    instantiateCallback,
                ),
            )?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        resourcePath: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        resourceType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        container: quest_hook::libil2cpp::Gc<crate::Zenject::DiContainer>,
        extraArguments: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IEnumerable_1<
                crate::Zenject::TypeValuePair,
            >,
        >,
        createNew: bool,
        concreteIdentifier: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        >,
        instantiateCallback: quest_hook::libil2cpp::Gc<
            crate::System::Action_2<
                *mut crate::Zenject::InjectContext,
                *mut quest_hook::libil2cpp::Il2CppObject,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                ".ctor",
                (
                    resourcePath,
                    resourceType,
                    container,
                    extraArguments,
                    createNew,
                    concreteIdentifier,
                    instantiateCallback,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn get_IsCached(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsCached", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_TypeVariesBasedOnMemberType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_TypeVariesBasedOnMemberType", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+ScriptableObjectResourceProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::ScriptableObjectResourceProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+ScriptableObjectResourceProvider")]
impl AsRef<crate::Zenject::IProvider>
for crate::Zenject::ScriptableObjectResourceProvider {
    fn as_ref(&self) -> &crate::Zenject::IProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+ScriptableObjectResourceProvider")]
impl AsMut<crate::Zenject::IProvider>
for crate::Zenject::ScriptableObjectResourceProvider {
    fn as_mut(&mut self) -> &mut crate::Zenject::IProvider {
        unsafe { std::mem::transmute(self) }
    }
}
