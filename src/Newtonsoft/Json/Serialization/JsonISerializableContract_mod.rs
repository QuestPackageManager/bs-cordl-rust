#[cfg(feature = "Newtonsoft+Json+Serialization+JsonISerializableContract")]
#[repr(C)]
#[derive(Debug)]
pub struct JsonISerializableContract {
    __cordl_parent: crate::Newtonsoft::Json::Serialization::JsonContainerContract,
    pub _ISerializableCreator_k__BackingField: quest_hook::libil2cpp::Gc<
        crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
            quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
        >,
    >,
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonISerializableContract")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Newtonsoft::Json::Serialization::JsonISerializableContract {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Newtonsoft.Json.Serialization";
    const CLASS_NAME: &'static str = "JsonISerializableContract";
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
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonISerializableContract")]
impl std::ops::Deref
for crate::Newtonsoft::Json::Serialization::JsonISerializableContract {
    type Target = crate::Newtonsoft::Json::Serialization::JsonContainerContract;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonISerializableContract")]
impl std::ops::DerefMut
for crate::Newtonsoft::Json::Serialization::JsonISerializableContract {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonISerializableContract")]
impl crate::Newtonsoft::Json::Serialization::JsonISerializableContract {
    pub fn New(
        underlyingType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (underlyingType))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        underlyingType: quest_hook::libil2cpp::Gc<crate::System::Type>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Newtonsoft::Json::Serialization::JsonISerializableContract as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<crate::System::Type>),
                quest_hook::libil2cpp::Void,
                1usize,
            >(".ctor")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Newtonsoft::Json::Serialization::JsonISerializableContract as
                    quest_hook::libil2cpp::Type > ::class(), ".ctor", 1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (underlyingType))?
        };
        Ok(__cordl_ret.into())
    }
    pub fn get_ISerializableCreator(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    > {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Newtonsoft::Json::Serialization::JsonISerializableContract as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (),
                quest_hook::libil2cpp::Gc<
                    crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    >,
                >,
                0usize,
            >("get_ISerializableCreator")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Newtonsoft::Json::Serialization::JsonISerializableContract as
                    quest_hook::libil2cpp::Type > ::class(), "get_ISerializableCreator",
                    0usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        > = unsafe { method.invoke_unchecked(self, ())? };
        Ok(__cordl_ret.into())
    }
    pub fn set_ISerializableCreator(
        &mut self,
        value: quest_hook::libil2cpp::Gc<
            crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
                quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        static method: &'static quest_hook::libil2cpp::MethodInfo = <crate::Newtonsoft::Json::Serialization::JsonISerializableContract as quest_hook::libil2cpp::Type>::class()
            .find_method::<
                (quest_hook::libil2cpp::Gc<
                    crate::Newtonsoft::Json::Serialization::ObjectConstructor_1<
                        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
                    >,
                >),
                quest_hook::libil2cpp::Void,
                1usize,
            >("set_ISerializableCreator")
            .unwrap_or_else(|e| {
                panic!(
                    "no matching methods found for non-void {}.{}({}) Cause: {e:?}", <
                    crate ::Newtonsoft::Json::Serialization::JsonISerializableContract as
                    quest_hook::libil2cpp::Type > ::class(), "set_ISerializableCreator",
                    1usize
                )
            });
        let __cordl_ret: quest_hook::libil2cpp::Void = unsafe {
            method.invoke_unchecked(self, (value))?
        };
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Serialization+JsonISerializableContract")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Serialization::JsonISerializableContract {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
