#[cfg(feature = "Zenject+ValidationMarker")]
#[repr(C)]
#[derive(Debug)]
pub struct ValidationMarker {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _InstantiateFailed_k__BackingField: bool,
    pub _MarkedType_k__BackingField: *mut crate::System::Type,
}
#[cfg(feature = "Zenject+ValidationMarker")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::ValidationMarker => "Zenject"
    ."ValidationMarker"
);
#[cfg(feature = "Zenject+ValidationMarker")]
impl std::ops::Deref for crate::Zenject::ValidationMarker {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ValidationMarker")]
impl std::ops::DerefMut for crate::Zenject::ValidationMarker {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+ValidationMarker")]
impl crate::Zenject::ValidationMarker {
    pub fn New_Type1(
        markedType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (markedType))?;
        Ok(__cordl_object.into())
    }
    pub fn New__cordl_bool0(
        markedType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        instantiateFailed: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (markedType, instantiateFailed))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor_Type1(
        &mut self,
        markedType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (markedType))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor__cordl_bool0(
        &mut self,
        markedType: quest_hook::libil2cpp::Gc<crate::System::Type>,
        instantiateFailed: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (markedType, instantiateFailed))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_InstantiateFailed(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_InstantiateFailed", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_MarkedType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("get_MarkedType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_InstantiateFailed(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_InstantiateFailed", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_MarkedType(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_MarkedType", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+ValidationMarker")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::ValidationMarker {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
