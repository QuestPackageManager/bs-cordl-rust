#[cfg(feature = "Zenject+InstantiateOnPrefabComponentProvider")]
#[repr(C)]
#[derive(Debug)]
pub struct InstantiateOnPrefabComponentProvider {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _prefabInstantiator: quest_hook::libil2cpp::Gc<
        crate::Zenject::IPrefabInstantiator,
    >,
    pub _componentType: quest_hook::libil2cpp::Gc<crate::System::Type>,
}
#[cfg(feature = "Zenject+InstantiateOnPrefabComponentProvider")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::InstantiateOnPrefabComponentProvider =>
    "Zenject"."InstantiateOnPrefabComponentProvider"
);
#[cfg(feature = "Zenject+InstantiateOnPrefabComponentProvider")]
impl std::ops::Deref for crate::Zenject::InstantiateOnPrefabComponentProvider {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+InstantiateOnPrefabComponentProvider")]
impl std::ops::DerefMut for crate::Zenject::InstantiateOnPrefabComponentProvider {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+InstantiateOnPrefabComponentProvider")]
impl crate::Zenject::InstantiateOnPrefabComponentProvider {
    pub fn GetAllInstancesWithInjectSplit(
        &mut self,
        context: quest_hook::libil2cpp::Gc<crate::Zenject::InjectContext>,
        args: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<crate::Zenject::TypeValuePair>,
        >,
        injectAction: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<crate::System::Action>,
        >,
        buffer: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
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
        prefabInstantiator: quest_hook::libil2cpp::Gc<
            crate::Zenject::IPrefabInstantiator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (componentType, prefabInstantiator))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        componentType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        prefabInstantiator: quest_hook::libil2cpp::Gc<
            crate::Zenject::IPrefabInstantiator,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (componentType, prefabInstantiator))?;
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
#[cfg(feature = "Zenject+InstantiateOnPrefabComponentProvider")]
impl quest_hook::libil2cpp::ObjectType
for crate::Zenject::InstantiateOnPrefabComponentProvider {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+InstantiateOnPrefabComponentProvider")]
impl AsRef<crate::Zenject::IProvider>
for crate::Zenject::InstantiateOnPrefabComponentProvider {
    fn as_ref(&self) -> &crate::Zenject::IProvider {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "Zenject+InstantiateOnPrefabComponentProvider")]
impl AsMut<crate::Zenject::IProvider>
for crate::Zenject::InstantiateOnPrefabComponentProvider {
    fn as_mut(&mut self) -> &mut crate::Zenject::IProvider {
        unsafe { std::mem::transmute(self) }
    }
}
