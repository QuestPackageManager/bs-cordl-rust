#[cfg(feature = "Zenject+InjectAttributeBase")]
#[repr(C)]
#[derive(Debug)]
pub struct InjectAttributeBase {
    __cordl_parent: crate::Zenject::Internal::PreserveAttribute,
    pub _Optional_k__BackingField: bool,
    pub _Id_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppObject,
    >,
    pub _Source_k__BackingField: crate::Zenject::InjectSources,
}
#[cfg(feature = "Zenject+InjectAttributeBase")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::InjectAttributeBase => "Zenject"
    ."InjectAttributeBase"
);
#[cfg(feature = "Zenject+InjectAttributeBase")]
impl std::ops::Deref for crate::Zenject::InjectAttributeBase {
    type Target = crate::Zenject::Internal::PreserveAttribute;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+InjectAttributeBase")]
impl std::ops::DerefMut for crate::Zenject::InjectAttributeBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+InjectAttributeBase")]
impl crate::Zenject::InjectAttributeBase {
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
    pub fn get_Id(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_Id", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Optional(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_Optional", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Source(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Zenject::InjectSources> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Zenject::InjectSources = __cordl_object
            .invoke("get_Source", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Id(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Id", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Optional(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Optional", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Source(
        &mut self,
        value: crate::Zenject::InjectSources,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Source", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+InjectAttributeBase")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::InjectAttributeBase {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
