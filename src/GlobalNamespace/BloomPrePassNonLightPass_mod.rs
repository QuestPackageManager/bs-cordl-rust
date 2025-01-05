#[cfg(feature = "BloomPrePassNonLightPass")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomPrePassNonLightPass {
    __cordl_parent: quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>,
    pub _executionTimeType: crate::GlobalNamespace::BloomPrePassNonLightPass_ExecutionTimeType,
    pub _registeredExecutionTimeType: crate::GlobalNamespace::BloomPrePassNonLightPass_ExecutionTimeType,
}
#[cfg(feature = "BloomPrePassNonLightPass")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BloomPrePassNonLightPass => ""
    ."BloomPrePassNonLightPass"
);
#[cfg(feature = "BloomPrePassNonLightPass")]
impl std::ops::Deref for crate::GlobalNamespace::BloomPrePassNonLightPass {
    type Target = quest_hook::libil2cpp::Gc<crate::UnityEngine::MonoBehaviour>;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassNonLightPass")]
impl std::ops::DerefMut for crate::GlobalNamespace::BloomPrePassNonLightPass {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassNonLightPass")]
impl crate::GlobalNamespace::BloomPrePassNonLightPass {
    #[cfg(feature = "BloomPrePassNonLightPass+ExecutionTimeType")]
    pub type ExecutionTimeType = crate::GlobalNamespace::BloomPrePassNonLightPass_ExecutionTimeType;
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDisable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDisable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnEnable(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnEnable", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn OnValidate(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnValidate", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Register(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Register", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Render(
        &mut self,
        dest: quest_hook::libil2cpp::Gc<crate::UnityEngine::RenderTexture>,
        viewMatrix: crate::UnityEngine::Matrix4x4,
        projectionMatrix: crate::UnityEngine::Matrix4x4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Render", (dest, viewMatrix, projectionMatrix))?;
        Ok(__cordl_ret.into())
    }
    pub fn Unregister(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Unregister", ())?;
        Ok(__cordl_ret.into())
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
    pub fn get_bloomPrePassAfterBlurList() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BloomPrePassNonLightPass>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BloomPrePassNonLightPass>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_bloomPrePassAfterBlurList", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_bloomPrePassBeforeBlurList() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BloomPrePassNonLightPass>,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BloomPrePassNonLightPass>,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_bloomPrePassBeforeBlurList", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_executionTimeType(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        crate::GlobalNamespace::BloomPrePassNonLightPass_ExecutionTimeType,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::GlobalNamespace::BloomPrePassNonLightPass_ExecutionTimeType = __cordl_object
            .invoke("get_executionTimeType", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BloomPrePassNonLightPass")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BloomPrePassNonLightPass {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BloomPrePassNonLightPass+ExecutionTimeType")]
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BloomPrePassNonLightPass_ExecutionTimeType {
    #[default]
    AfterBlur = 2i32,
    BeforeBlur = 1i32,
    None = 0i32,
}
#[cfg(feature = "BloomPrePassNonLightPass+ExecutionTimeType")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BloomPrePassNonLightPass_ExecutionTimeType => ""
    ."BloomPrePassNonLightPass/ExecutionTimeType"
);
