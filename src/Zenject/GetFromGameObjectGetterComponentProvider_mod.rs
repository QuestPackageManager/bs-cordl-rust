#[cfg(feature = "Zenject+GetFromGameObjectGetterComponentProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct GetFromGameObjectGetterComponentProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _gameObjectGetter: *mut crate::System::Func_2<
        *mut crate::Zenject::InjectContext,
        *mut crate::UnityEngine::GameObject,
    >,
    pub _componentType: *mut crate::System::Type,
    pub _matchSingle: bool,
}
#[cfg(feature = "Zenject+GetFromGameObjectGetterComponentProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::Zenject::GetFromGameObjectGetterComponentProvider => "Zenject"
    ."GetFromGameObjectGetterComponentProvider"
);
#[cfg(feature = "Zenject+GetFromGameObjectGetterComponentProvider")]
impl std::ops::Deref for crate::Zenject::GetFromGameObjectGetterComponentProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+GetFromGameObjectGetterComponentProvider")]
impl std::ops::DerefMut for crate::Zenject::GetFromGameObjectGetterComponentProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+GetFromGameObjectGetterComponentProvider")]
impl crate::Zenject::GetFromGameObjectGetterComponentProvider {
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
        componentType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        gameObjectGetter: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                *mut crate::Zenject::InjectContext,
                *mut crate::UnityEngine::GameObject,
            >,
        >,
        matchSingle: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (componentType, gameObjectGetter, matchSingle))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        componentType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        gameObjectGetter: quest_hook::libil2cpp::Gc<
            crate::System::Func_2<
                *mut crate::Zenject::InjectContext,
                *mut crate::UnityEngine::GameObject,
            >,
        >,
        matchSingle: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (componentType, gameObjectGetter, matchSingle))?;
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
#[cfg(feature = "Zenject+GetFromGameObjectGetterComponentProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::GetFromGameObjectGetterComponentProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
