#[cfg(feature = "BloomPrePassLight")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomPrePassLight {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _lightType: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BloomPrePassLightTypeSO,
    >,
    pub _registeredWithLightType: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BloomPrePassLightTypeSO,
    >,
    pub _isRegistered: bool,
    pub _isBeingDestroyed: bool,
}
#[cfg(feature = "BloomPrePassLight")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BloomPrePassLight => ""
    ."BloomPrePassLight"
);
#[cfg(feature = "BloomPrePassLight")]
impl std::ops::Deref for crate::GlobalNamespace::BloomPrePassLight {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassLight")]
impl std::ops::DerefMut for crate::GlobalNamespace::BloomPrePassLight {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassLight")]
impl crate::GlobalNamespace::BloomPrePassLight {
    pub const kColorOffset: i32 = 24i32;
    pub const kColorSize: i32 = 16i32;
    pub const kFloatSize: i32 = 4i32;
    pub const kUvOffset: i32 = 40i32;
    pub const kUvSize: i32 = 12i32;
    pub const kVertexDataSize: i32 = 52i32;
    pub const kVertexOffset: i32 = 0i32;
    pub const kVertexSize: i32 = 12i32;
    pub const kViewPosOffset: i32 = 12i32;
    pub const kViewPosSize: i32 = 12i32;
    #[cfg(feature = "BloomPrePassLight+LightsDataItem")]
    pub type LightsDataItem = crate::GlobalNamespace::BloomPrePassLight_LightsDataItem;
    #[cfg(feature = "BloomPrePassLight+QuadData")]
    pub type QuadData = crate::GlobalNamespace::BloomPrePassLight_QuadData;
    #[cfg(feature = "BloomPrePassLight+VertexData")]
    pub type VertexData = crate::GlobalNamespace::BloomPrePassLight_VertexData;
    pub fn DidRegisterLight(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("DidRegisterLight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn FillMeshData(
        &mut self,
        lightNum: quest_hook::libil2cpp::ByRefMut<i32>,
        lightQuads: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppArray<
                crate::GlobalNamespace::BloomPrePassLight_QuadData,
            >,
        >,
        viewMatrix: crate::UnityEngine::Matrix4x4,
        projectionMatrix: crate::UnityEngine::Matrix4x4,
        lineWidth: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(
                "FillMeshData",
                (lightNum, lightQuads, viewMatrix, projectionMatrix, lineWidth),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn OnDestroy(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("OnDestroy", ())?;
        Ok(__cordl_ret.into())
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
    pub fn Refresh(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Refresh", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn RegisterLight(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RegisterLight", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn UnregisterLight(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UnregisterLight", ())?;
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
    pub fn get_bloomLightsDict() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BloomPrePassLightTypeSO,
                >,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::HashSet_1<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BloomPrePassLight,
                        >,
                    >,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::Dictionary_2<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BloomPrePassLightTypeSO,
                >,
                quest_hook::libil2cpp::Gc<
                    crate::System::Collections::Generic::HashSet_1<
                        quest_hook::libil2cpp::Gc<
                            crate::GlobalNamespace::BloomPrePassLight,
                        >,
                    >,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_bloomLightsDict", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_lightsDataItems() -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BloomPrePassLight_LightsDataItem,
                >,
            >,
        >,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::List_1<
                quest_hook::libil2cpp::Gc<
                    crate::GlobalNamespace::BloomPrePassLight_LightsDataItem,
                >,
            >,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("get_lightsDataItems", ())?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BloomPrePassLight")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BloomPrePassLight {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BloomPrePassLight+LightsDataItem")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomPrePassLight_LightsDataItem {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub lightType: quest_hook::libil2cpp::Gc<
        crate::GlobalNamespace::BloomPrePassLightTypeSO,
    >,
    pub lights: quest_hook::libil2cpp::Gc<
        crate::System::Collections::Generic::HashSet_1<
            quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BloomPrePassLight>,
        >,
    >,
}
#[cfg(feature = "BloomPrePassLight+LightsDataItem")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BloomPrePassLight_LightsDataItem => ""
    ."BloomPrePassLight/LightsDataItem"
);
#[cfg(feature = "BloomPrePassLight+LightsDataItem")]
impl std::ops::Deref for crate::GlobalNamespace::BloomPrePassLight_LightsDataItem {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassLight+LightsDataItem")]
impl std::ops::DerefMut for crate::GlobalNamespace::BloomPrePassLight_LightsDataItem {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassLight+LightsDataItem")]
impl crate::GlobalNamespace::BloomPrePassLight_LightsDataItem {
    pub fn New(
        lightType: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BloomPrePassLightTypeSO,
        >,
        lights: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BloomPrePassLight>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (lightType, lights))?;
        Ok(__cordl_object.into())
    }
    pub fn _ctor(
        &mut self,
        lightType: quest_hook::libil2cpp::Gc<
            crate::GlobalNamespace::BloomPrePassLightTypeSO,
        >,
        lights: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::HashSet_1<
                quest_hook::libil2cpp::Gc<crate::GlobalNamespace::BloomPrePassLight>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (lightType, lights))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "BloomPrePassLight+LightsDataItem")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BloomPrePassLight_LightsDataItem {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BloomPrePassLight+QuadData")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct BloomPrePassLight_QuadData {
    padding: [u8; 208usize],
}
#[cfg(feature = "BloomPrePassLight+QuadData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BloomPrePassLight_QuadData =>
    ""."BloomPrePassLight/QuadData"
);
#[cfg(feature = "BloomPrePassLight+QuadData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::BloomPrePassLight_QuadData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BloomPrePassLight+QuadData")]
impl crate::GlobalNamespace::BloomPrePassLight_QuadData {}
#[cfg(feature = "BloomPrePassLight+VertexData")]
#[repr(C)]
#[derive(Debug, Clone, Default)]
pub struct BloomPrePassLight_VertexData {
    padding: [u8; 52usize],
}
#[cfg(feature = "BloomPrePassLight+VertexData")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BloomPrePassLight_VertexData =>
    ""."BloomPrePassLight/VertexData"
);
#[cfg(feature = "BloomPrePassLight+VertexData")]
unsafe impl quest_hook::libil2cpp::ThisArgument
for crate::GlobalNamespace::BloomPrePassLight_VertexData {
    type Type = Self;
    fn matches(method: &quest_hook::libil2cpp::MethodInfo) -> bool {
        <Self as quest_hook::libil2cpp::Type>::matches_this_argument(method)
    }
    fn invokable(&mut self) -> *mut std::ffi::c_void {
        unsafe { quest_hook::libil2cpp::value_box(self) as *mut std::ffi::c_void }
    }
}
#[cfg(feature = "BloomPrePassLight+VertexData")]
impl crate::GlobalNamespace::BloomPrePassLight_VertexData {}
