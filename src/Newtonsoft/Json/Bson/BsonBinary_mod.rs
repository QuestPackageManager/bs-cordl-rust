#[cfg(feature = "Newtonsoft+Json+Bson+BsonBinary")]
#[repr(C)]
#[derive(Debug)]
pub struct BsonBinary {
    __cordl_parent: crate::Newtonsoft::Json::Bson::BsonValue,
    pub _BinaryType_k__BackingField: crate::Newtonsoft::Json::Bson::BsonBinaryType,
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonBinary")]
unsafe impl quest_hook::libil2cpp::Type for crate::Newtonsoft::Json::Bson::BsonBinary {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Bson";
    const CLASS_NAME: &'static str = "BsonBinary";
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
#[cfg(feature = "Newtonsoft+Json+Bson+BsonBinary")]
impl std::ops::Deref for crate::Newtonsoft::Json::Bson::BsonBinary {
    type Target = crate::Newtonsoft::Json::Bson::BsonValue;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonBinary")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Bson::BsonBinary {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonBinary")]
impl crate::Newtonsoft::Json::Bson::BsonBinary {
    pub fn New(
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        binaryType: crate::Newtonsoft::Json::Bson::BsonBinaryType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (value, binaryType))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        value: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
        binaryType: crate::Newtonsoft::Json::Bson::BsonBinaryType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (
                    quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<u8>>,
                    crate::Newtonsoft::Json::Bson::BsonBinaryType,
                ),
                quest_hook::libil2cpp::Void,
                2usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, ".ctor", 2usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value, binaryType))
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_BinaryType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::Newtonsoft::Json::Bson::BsonBinaryType> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                crate::Newtonsoft::Json::Bson::BsonBinaryType,
                0usize,
            >("get_BinaryType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "get_BinaryType", 0usize
                )
            });
        let __cordl_ret: crate::Newtonsoft::Json::Bson::BsonBinaryType = unsafe {
            method.invoke_unchecked(self, ())
        };
        Ok(__cordl_ret.into())
    }
    pub fn set_BinaryType(
        &mut self,
        value: crate::Newtonsoft::Json::Bson::BsonBinaryType,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <Self as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (crate::Newtonsoft::Json::Bson::BsonBinaryType),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_BinaryType")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}",
                    self, "set_BinaryType", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Bson+BsonBinary")]
impl quest_hook::libil2cpp::ObjectType for crate::Newtonsoft::Json::Bson::BsonBinary {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
