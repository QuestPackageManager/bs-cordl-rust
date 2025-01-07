#[cfg(feature = "Org+BouncyCastle+Tsp+GenTimeAccuracy")]
#[repr(C)]
#[derive(Debug)]
pub struct GenTimeAccuracy {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub accuracy: quest_hook::libil2cpp::Gc<
        crate::Org::BouncyCastle::Asn1::Tsp::Accuracy,
    >,
}
#[cfg(feature = "Org+BouncyCastle+Tsp+GenTimeAccuracy")]
unsafe impl quest_hook::libil2cpp::Type
for crate::Org::BouncyCastle::Tsp::GenTimeAccuracy {
    type Held<'a> = ::std::option::Option<&'a mut Self>;
    type HeldRaw = *mut Self;
    const NAMESPACE: &'static str = "Org.BouncyCastle.Tsp";
    const CLASS_NAME: &'static str = "GenTimeAccuracy";
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
#[cfg(feature = "Org+BouncyCastle+Tsp+GenTimeAccuracy")]
impl std::ops::Deref for crate::Org::BouncyCastle::Tsp::GenTimeAccuracy {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Tsp+GenTimeAccuracy")]
impl std::ops::DerefMut for crate::Org::BouncyCastle::Tsp::GenTimeAccuracy {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Org+BouncyCastle+Tsp+GenTimeAccuracy")]
impl crate::Org::BouncyCastle::Tsp::GenTimeAccuracy {
    pub fn GetTimeComponent(
        &mut self,
        _cordl_time: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::DerInteger,
        >,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetTimeComponent", (_cordl_time))?;
        Ok(__cordl_ret.into())
    }
    pub fn New(
        accuracy: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Tsp::Accuracy,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (accuracy))?;
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
        accuracy: quest_hook::libil2cpp::Gc<
            crate::Org::BouncyCastle::Asn1::Tsp::Accuracy,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (accuracy))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Micros(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Micros", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Millis(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Millis", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_Seconds(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Seconds", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Org+BouncyCastle+Tsp+GenTimeAccuracy")]
impl quest_hook::libil2cpp::ObjectType
for crate::Org::BouncyCastle::Tsp::GenTimeAccuracy {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
