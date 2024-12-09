#[cfg(feature = "HoudiniEngineUnity+HEU_Vector3SerializationSurrogate")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_Vector3SerializationSurrogate {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Vector3SerializationSurrogate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_Vector3SerializationSurrogate => "HoudiniEngineUnity"
    ."HEU_Vector3SerializationSurrogate"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_Vector3SerializationSurrogate")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_Vector3SerializationSurrogate {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Vector3SerializationSurrogate")]
impl std::ops::DerefMut
for crate::HoudiniEngineUnity::HEU_Vector3SerializationSurrogate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Vector3SerializationSurrogate")]
impl crate::HoudiniEngineUnity::HEU_Vector3SerializationSurrogate {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn System_Runtime_Serialization_ISerializationSurrogate_GetObjectData(
        &mut self,
        obj: *mut quest_hook::libil2cpp::Il2CppObject,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "System.Runtime.Serialization.ISerializationSurrogate.GetObjectData",
                (obj, info, context),
            )?;
        Ok(__cordl_ret)
    }
    pub fn System_Runtime_Serialization_ISerializationSurrogate_SetObjectData(
        &mut self,
        obj: *mut quest_hook::libil2cpp::Il2CppObject,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
        selector: *mut crate::System::Runtime::Serialization::ISurrogateSelector,
    ) -> quest_hook::libil2cpp::Result<*mut quest_hook::libil2cpp::Il2CppObject> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppObject = __cordl_object
            .invoke(
                "System.Runtime.Serialization.ISerializationSurrogate.SetObjectData",
                (obj, info, context, selector),
            )?;
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
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Vector3SerializationSurrogate")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_Vector3SerializationSurrogate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
