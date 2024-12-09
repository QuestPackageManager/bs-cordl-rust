#[cfg(feature = "Newtonsoft+Json+Utilities+TypeInformation")]
#[repr(C)]
#[derive(Debug)]
pub struct TypeInformation {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _Type_k__BackingField: *mut crate::System::Type,
    pub _TypeCode_k__BackingField: crate::Newtonsoft::Json::Utilities::PrimitiveTypeCode,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+TypeInformation")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::TypeInformation =>
    "Newtonsoft.Json.Utilities"."TypeInformation"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+TypeInformation")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::TypeInformation {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+TypeInformation")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::TypeInformation {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+TypeInformation")]
impl crate::Newtonsoft::Json::Utilities::TypeInformation {
    pub fn New(
        _cordl_type: *mut crate::System::Type,
        typeCode: crate::Newtonsoft::Json::Utilities::PrimitiveTypeCode,
    ) -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (_cordl_type, typeCode))?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
        _cordl_type: *mut crate::System::Type,
        typeCode: crate::Newtonsoft::Json::Utilities::PrimitiveTypeCode,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (_cordl_type, typeCode))?;
        Ok(__cordl_ret)
    }
    pub fn get_Type(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::Type> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::Type = __cordl_object
            .invoke("get_Type", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_TypeCode(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::Newtonsoft::Json::Utilities::PrimitiveTypeCode,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::Newtonsoft::Json::Utilities::PrimitiveTypeCode = __cordl_object
            .invoke("get_TypeCode", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+TypeInformation")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::TypeInformation {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
