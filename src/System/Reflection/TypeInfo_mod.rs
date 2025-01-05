#[cfg(feature = "System+Reflection+TypeInfo")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeInfo {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::System::Type>,
}
#[cfg(feature = "System+Reflection+TypeInfo")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::System::Reflection::TypeInfo =>
    "System.Reflection"."TypeInfo"
);
#[cfg(feature = "System+Reflection+TypeInfo")]
impl std::ops::Deref for crate::System::Reflection::TypeInfo {
    type Target = quest_hook::libil2cpp::Gc<crate::System::Type>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+TypeInfo")]
impl std::ops::DerefMut for crate::System::Reflection::TypeInfo {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Reflection+TypeInfo")]
impl crate::System::Reflection::TypeInfo {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn System_Reflection_IReflectableType_GetTypeInfo(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::System::Reflection::TypeInfo>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Reflection::TypeInfo,
        > = __cordl_object.invoke("System.Reflection.IReflectableType.GetTypeInfo", ())?;
        Ok(__cordl_ret.into())
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
    pub fn get_ImplementedInterfaces(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Gc<crate::System::Type>>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::System::Type>,
        > = __cordl_object.invoke("get_ImplementedInterfaces", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Reflection+TypeInfo")]
impl quest_hook::libil2cpp::ObjectType for crate::System::Reflection::TypeInfo {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "System+Reflection+TypeInfo")]
impl AsRef<quest_hook::libil2cpp::Gc<crate::System::Reflection::IReflectableType>>
for crate::System::Reflection::TypeInfo {
    fn as_ref(
        &self,
    ) -> &quest_hook::libil2cpp::Gc<crate::System::Reflection::IReflectableType> {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "System+Reflection+TypeInfo")]
impl AsMut<quest_hook::libil2cpp::Gc<crate::System::Reflection::IReflectableType>>
for crate::System::Reflection::TypeInfo {
    fn as_mut(
        &mut self,
    ) -> &mut quest_hook::libil2cpp::Gc<crate::System::Reflection::IReflectableType> {
        unsafe { std::mem::transmute(self) }
    }
}
