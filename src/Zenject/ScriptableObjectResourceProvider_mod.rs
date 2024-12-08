#[cfg(feature = "Zenject+ScriptableObjectResourceProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct ScriptableObjectResourceProvider {
    __cordl_parent: crate::System::Object,
    pub _container: *mut crate::Zenject::DiContainer,
    pub _resourceType: *mut crate::System::Type,
    pub _resourcePath: *mut crate::System::String,
    pub _extraArguments: *mut crate::System::Collections::Generic::List_1<
        crate::Zenject::TypeValuePair,
    >,
    pub _createNew: bool,
    pub _concreteIdentifier: *mut crate::System::Object,
    pub _instantiateCallback: *mut crate::System::Action_2<
        *mut crate::Zenject::InjectContext,
        *mut crate::System::Object,
    >,
}
#[cfg(feature = "Zenject+ScriptableObjectResourceProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::ScriptableObjectResourceProvider =>
    "Zenject"."ScriptableObjectResourceProvider"
);
#[cfg(feature = "Zenject+ScriptableObjectResourceProvider")]
impl std::ops::Deref for crate::Zenject::ScriptableObjectResourceProvider {
    type Target = crate::System::Object;
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
    #[cfg(feature = "Zenject+ScriptableObjectResourceProvider+__c__DisplayClass13_0")]
    pub type __c__DisplayClass13_0 = crate::Zenject::ScriptableObjectResourceProvider___c__DisplayClass13_0;
    pub fn _ctor(
        &mut self,
        resourcePath: *mut crate::System::String,
        resourceType: *mut crate::System::Type,
        container: *mut crate::Zenject::DiContainer,
        extraArguments: *mut crate::System::Collections::Generic::IEnumerable_1<
            crate::Zenject::TypeValuePair,
        >,
        createNew: bool,
        concreteIdentifier: *mut crate::System::Object,
        instantiateCallback: *mut crate::System::Action_2<
            *mut crate::Zenject::InjectContext,
            *mut crate::System::Object,
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
        Ok(__cordl_ret)
    }
    pub fn GetInstanceType(
        &mut self,
        context: *mut crate::Zenject::InjectContext,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("GetInstanceType", (context))?;
        Ok(__cordl_ret)
    }
    pub fn GetAllInstancesWithInjectSplit(
        &mut self,
        context: *mut crate::Zenject::InjectContext,
        args: *mut crate::System::Collections::Generic::List_1<
            crate::Zenject::TypeValuePair,
        >,
        injectAction: quest_hook::libil2cpp::ByRefMut<*mut crate::System::Action>,
        buffer: *mut crate::System::Collections::Generic::List_1<
            *mut crate::System::Object,
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
        Ok(__cordl_ret)
    }
    pub fn get_IsCached(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IsCached", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TypeVariesBasedOnMemberType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object
            .invoke("get_TypeVariesBasedOnMemberType", ())?;
        Ok(__cordl_ret)
    }
    pub fn New(
        resourcePath: *mut crate::System::String,
        resourceType: *mut crate::System::Type,
        container: *mut crate::Zenject::DiContainer,
        extraArguments: *mut crate::System::Collections::Generic::IEnumerable_1<
            crate::Zenject::TypeValuePair,
        >,
        createNew: bool,
        concreteIdentifier: *mut crate::System::Object,
        instantiateCallback: *mut crate::System::Action_2<
            *mut crate::Zenject::InjectContext,
            *mut crate::System::Object,
        >,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
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
        Ok(__cordl_object)
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
