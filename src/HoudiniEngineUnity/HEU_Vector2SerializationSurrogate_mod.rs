#[cfg(feature = "HoudiniEngineUnity+HEU_Vector2SerializationSurrogate")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_Vector2SerializationSurrogate {
    __cordl_parent: crate::System::Object,
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Vector2SerializationSurrogate")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::HoudiniEngineUnity::HEU_Vector2SerializationSurrogate => "HoudiniEngineUnity"
    ."HEU_Vector2SerializationSurrogate"
);
#[cfg(feature = "HoudiniEngineUnity+HEU_Vector2SerializationSurrogate")]
impl std::ops::Deref for crate::HoudiniEngineUnity::HEU_Vector2SerializationSurrogate {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Vector2SerializationSurrogate")]
impl std::ops::DerefMut
for crate::HoudiniEngineUnity::HEU_Vector2SerializationSurrogate {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Vector2SerializationSurrogate")]
impl crate::HoudiniEngineUnity::HEU_Vector2SerializationSurrogate {
    pub fn System_Runtime_Serialization_ISerializationSurrogate_GetObjectData(
        &mut self,
        obj: *mut crate::System::Object,
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
        obj: *mut crate::System::Object,
        info: *mut crate::System::Runtime::Serialization::SerializationInfo,
        context: crate::System::Runtime::Serialization::StreamingContext,
        selector: *mut crate::System::Runtime::Serialization::ISurrogateSelector,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Object> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Object = __cordl_object
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
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "HoudiniEngineUnity+HEU_Vector2SerializationSurrogate")]
impl quest_hook::libil2cpp::ObjectType
for crate::HoudiniEngineUnity::HEU_Vector2SerializationSurrogate {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
