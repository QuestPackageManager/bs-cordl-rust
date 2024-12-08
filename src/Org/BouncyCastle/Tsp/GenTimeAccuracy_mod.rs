#[cfg(feature = "Org+BouncyCastle+Tsp+GenTimeAccuracy")]
#[repr(C)]
#[derive(Debug)]
pub struct GenTimeAccuracy {
    __cordl_parent: crate::System::Object,
    pub accuracy: *mut crate::Org::BouncyCastle::Asn1::Tsp::Accuracy,
}
#[cfg(feature = "Org+BouncyCastle+Tsp+GenTimeAccuracy")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Org::BouncyCastle::Tsp::GenTimeAccuracy =>
    "Org.BouncyCastle.Tsp"."GenTimeAccuracy"
);
#[cfg(feature = "Org+BouncyCastle+Tsp+GenTimeAccuracy")]
impl std::ops::Deref for crate::Org::BouncyCastle::Tsp::GenTimeAccuracy {
    type Target = crate::System::Object;
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
    pub fn _ctor(
        &mut self,
        accuracy: *mut crate::Org::BouncyCastle::Asn1::Tsp::Accuracy,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (accuracy))?;
        Ok(__cordl_ret)
    }
    pub fn get_Seconds(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Seconds", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Millis(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Millis", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_Micros(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_Micros", ())?;
        Ok(__cordl_ret)
    }
    pub fn ToString(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<*mut crate::System::String> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut crate::System::String = __cordl_object
            .invoke("ToString", ())?;
        Ok(__cordl_ret)
    }
    pub fn GetTimeComponent(
        &mut self,
        _cordl_time: *mut crate::Org::BouncyCastle::Asn1::DerInteger,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetTimeComponent", (_cordl_time))?;
        Ok(__cordl_ret)
    }
    pub fn New(
        accuracy: *mut crate::Org::BouncyCastle::Asn1::Tsp::Accuracy,
    ) -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (accuracy))?;
        Ok(__cordl_object)
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
