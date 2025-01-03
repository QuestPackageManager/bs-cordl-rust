#[cfg(feature = "Zenject+SignalBindingBindInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct SignalBindingBindInfo {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _Identifier_k__BackingField: quest_hook::libil2cpp::Gc<
        quest_hook::libil2cpp::Il2CppObject,
    >,
    pub _SignalType_k__BackingField: quest_hook::libil2cpp::Gc<crate::System::Type>,
}
#[cfg(feature = "Zenject+SignalBindingBindInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Zenject::SignalBindingBindInfo => "Zenject"
    ."SignalBindingBindInfo"
);
#[cfg(feature = "Zenject+SignalBindingBindInfo")]
impl std::ops::Deref for crate::Zenject::SignalBindingBindInfo {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SignalBindingBindInfo")]
impl std::ops::DerefMut for crate::Zenject::SignalBindingBindInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Zenject+SignalBindingBindInfo")]
impl crate::Zenject::SignalBindingBindInfo {
    pub fn New(
        signalType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (signalType))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        signalType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (signalType))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Identifier(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = __cordl_object.invoke("get_Identifier", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_SignalType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("get_SignalType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_Identifier(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_Identifier", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_SignalType(
        &mut self,
        value: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_SignalType", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Zenject+SignalBindingBindInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::Zenject::SignalBindingBindInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
