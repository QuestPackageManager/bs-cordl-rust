#[cfg(feature = "HEU_ExampleInstanceCustomAttribute")]
#[repr(C)]
#[derive(Debug)]
pub struct HEU_ExampleInstanceCustomAttribute {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
}
#[cfg(feature = "HEU_ExampleInstanceCustomAttribute")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::HEU_ExampleInstanceCustomAttribute => ""
    ."HEU_ExampleInstanceCustomAttribute"
);
#[cfg(feature = "HEU_ExampleInstanceCustomAttribute")]
impl std::ops::Deref for crate::GlobalNamespace::HEU_ExampleInstanceCustomAttribute {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "HEU_ExampleInstanceCustomAttribute")]
impl std::ops::DerefMut for crate::GlobalNamespace::HEU_ExampleInstanceCustomAttribute {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "HEU_ExampleInstanceCustomAttribute")]
impl crate::GlobalNamespace::HEU_ExampleInstanceCustomAttribute {
    pub fn InstancerCallback(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("InstancerCallback", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn LogArray<T>(
        name: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
        arr: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppArray<T>>,
        tupleSize: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void>
    where
        T: quest_hook::libil2cpp::Type + quest_hook::libil2cpp::Argument
            + quest_hook::libil2cpp::Returned,
    {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LogArray", (name, arr, tupleSize))?;
        Ok(__cordl_ret.into())
    }
    pub fn LogAttr(
        outAttr: quest_hook::libil2cpp::Gc<
            crate::HoudiniEngineUnity::HEU_OutputAttribute,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("LogAttr", (outAttr))?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
}
#[cfg(feature = "HEU_ExampleInstanceCustomAttribute")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::HEU_ExampleInstanceCustomAttribute {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
