#[cfg(feature = "Zenject+ZenjectBinding")]
#[repr(C)]
#[derive(Debug)]
pub struct ZenjectBinding {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _components: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::UnityEngine::Component,
    >,
    pub _identifier: *mut quest_hook::libil2cpp::Il2CppString,
    pub _useSceneContext: bool,
    pub _ifNotBound: bool,
    pub _context: *mut crate::Zenject::Context,
    pub _bindType: crate::Zenject::ZenjectBinding_BindTypes,
}
#[cfg(feature = "Zenject+ZenjectBinding")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::ZenjectBinding => "Zenject"
    ."ZenjectBinding"
);
#[cfg(feature = "Zenject+ZenjectBinding")]
impl std::ops::Deref for crate::Zenject::ZenjectBinding {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ZenjectBinding")]
impl std::ops::DerefMut for crate::Zenject::ZenjectBinding {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ZenjectBinding")]
impl crate::Zenject::ZenjectBinding {
    #[cfg(feature = "Zenject+ZenjectBinding+BindTypes")]
    pub type BindTypes = crate::Zenject::ZenjectBinding_BindTypes;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_BindType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Zenject::ZenjectBinding_BindTypes> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Zenject::ZenjectBinding_BindTypes = __cordl_object
            .invoke("get_BindType", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Components(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<*mut crate::UnityEngine::Component>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::UnityEngine::Component,
        > = __cordl_object.invoke("get_Components", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Context(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::Zenject::Context> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::Zenject::Context = __cordl_object
            .invoke("get_Context", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Identifier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppString> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppString = __cordl_object
            .invoke("get_Identifier", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_IfNotBound(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_IfNotBound", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_UseSceneContext(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_UseSceneContext", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_Context(
        &mut self,
        value: *mut crate::Zenject::Context,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Context", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Zenject+ZenjectBinding")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::ZenjectBinding {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "Zenject+ZenjectBinding+BindTypes")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZenjectBinding_BindTypes {
    AllInterfaces = 1i32,
    AllInterfacesAndSelf = 2i32,
    BaseType = 3i32,
    _cordl_Self = 0i32,
}
#[cfg(feature = "Zenject+ZenjectBinding+BindTypes")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Zenject::ZenjectBinding_BindTypes => "Zenject"
    ."ZenjectBinding/BindTypes"
);
