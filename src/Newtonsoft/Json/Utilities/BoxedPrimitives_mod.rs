#[cfg(feature = "Newtonsoft+Json+Utilities+BoxedPrimitives")]
#[repr(C)]
#[derive(Debug)]
pub struct BoxedPrimitives {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Newtonsoft+Json+Utilities+BoxedPrimitives")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Newtonsoft::Json::Utilities::BoxedPrimitives =>
    "Newtonsoft.Json.Utilities"."BoxedPrimitives"
);
#[cfg(feature = "Newtonsoft+Json+Utilities+BoxedPrimitives")]
impl std::ops::Deref for crate::Newtonsoft::Json::Utilities::BoxedPrimitives {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+BoxedPrimitives")]
impl std::ops::DerefMut for crate::Newtonsoft::Json::Utilities::BoxedPrimitives {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+BoxedPrimitives")]
impl crate::Newtonsoft::Json::Utilities::BoxedPrimitives {
    pub fn Get_Decimal3(
        value: crate::System::Decimal,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Get", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Get__cordl_bool0(
        value: bool,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Get", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Get_f64_4(
        value: f64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Get", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Get_i32_1(
        value: i32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Get", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Get_i64_2(
        value: i64,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppObject,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Get", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Newtonsoft+Json+Utilities+BoxedPrimitives")]
impl quest_hook::libil2cpp::ObjectType
for crate::Newtonsoft::Json::Utilities::BoxedPrimitives {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
