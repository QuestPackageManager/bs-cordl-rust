#[cfg(feature = "System+Runtime+Remoting+WellKnownServiceTypeEntry")]
#[repr(C)]
#[derive(Debug)]
pub struct WellKnownServiceTypeEntry {
    __cordl_parent: crate::System::Runtime::Remoting::TypeEntry,
    pub obj_type: quest_hook::libil2cpp::Gc<crate::System::Type>,
    pub obj_uri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    pub obj_mode: crate::System::Runtime::Remoting::WellKnownObjectMode,
}
#[cfg(feature = "System+Runtime+Remoting+WellKnownServiceTypeEntry")]
unsafe impl quest_hook::libil2cpp::Type
for crate::System::Runtime::Remoting::WellKnownServiceTypeEntry {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "System.Runtime.Remoting";
    const CLASS_NAME: &'static str = "WellKnownServiceTypeEntry";
    fn matches_reference_argument(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        ty.class().is_assignable_from(<Self as quest_hook::libil2cpp::Type>::class())
    }
    fn matches_value_argument(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
    fn matches_reference_parameter(ty: &quest_hook::libil2cpp::Il2CppType) -> bool {
        <Self as quest_hook::libil2cpp::Type>::class().is_assignable_from(ty.class())
    }
    fn matches_value_parameter(_: &quest_hook::libil2cpp::Il2CppType) -> bool {
        false
    }
}
#[cfg(feature = "System+Runtime+Remoting+WellKnownServiceTypeEntry")]
impl std::ops::Deref for crate::System::Runtime::Remoting::WellKnownServiceTypeEntry {
    type Target = crate::System::Runtime::Remoting::TypeEntry;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+WellKnownServiceTypeEntry")]
impl std::ops::DerefMut for crate::System::Runtime::Remoting::WellKnownServiceTypeEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "System+Runtime+Remoting+WellKnownServiceTypeEntry")]
impl crate::System::Runtime::Remoting::WellKnownServiceTypeEntry {
    pub fn New(
        typeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        assemblyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        objectUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mode: crate::System::Runtime::Remoting::WellKnownObjectMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (typeName, assemblyName, objectUri, mode))?;
        Ok(__cordl_object.into())
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor(
        &mut self,
        typeName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        assemblyName: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        objectUri: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        mode: crate::System::Runtime::Remoting::WellKnownObjectMode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (typeName, assemblyName, objectUri, mode))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Mode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::System::Runtime::Remoting::WellKnownObjectMode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::System::Runtime::Remoting::WellKnownObjectMode = __cordl_object
            .invoke("get_Mode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ObjectType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<crate::System::Type>> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<crate::System::Type> = __cordl_object
            .invoke("get_ObjectType", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_ObjectUri(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("get_ObjectUri", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "System+Runtime+Remoting+WellKnownServiceTypeEntry")]
impl quest_hook::libil2cpp::ObjectType
for crate::System::Runtime::Remoting::WellKnownServiceTypeEntry {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
