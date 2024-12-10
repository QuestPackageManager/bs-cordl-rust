#[cfg(feature = "MaterialPropertyValuesSetter")]
#[repr(C)]
#[derive(Debug)]
pub struct MaterialPropertyValuesSetter {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _materialPropertyBlockController: *mut crate::GlobalNamespace::MaterialPropertyBlockController,
    pub _floats: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::MaterialPropertyValuesSetter_PropertyNameFloatValuePair,
    >,
    pub _vectors: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::MaterialPropertyValuesSetter_PropertyNameVectorValuePair,
    >,
    pub _colors: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::MaterialPropertyValuesSetter_PropertyNameColorValuePair,
    >,
    pub _ints: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::MaterialPropertyValuesSetter_PropertyNameIntValuePair,
    >,
}
#[cfg(feature = "MaterialPropertyValuesSetter")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::MaterialPropertyValuesSetter =>
    ""."MaterialPropertyValuesSetter"
);
#[cfg(feature = "MaterialPropertyValuesSetter")]
impl std::ops::Deref for crate::GlobalNamespace::MaterialPropertyValuesSetter {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MaterialPropertyValuesSetter")]
impl std::ops::DerefMut for crate::GlobalNamespace::MaterialPropertyValuesSetter {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MaterialPropertyValuesSetter")]
impl crate::GlobalNamespace::MaterialPropertyValuesSetter {
    #[cfg(feature = "MaterialPropertyValuesSetter+PropertyNameColorValuePair")]
    pub type PropertyNameColorValuePair = crate::GlobalNamespace::MaterialPropertyValuesSetter_PropertyNameColorValuePair;
    #[cfg(feature = "MaterialPropertyValuesSetter+PropertyNameFloatValuePair")]
    pub type PropertyNameFloatValuePair = crate::GlobalNamespace::MaterialPropertyValuesSetter_PropertyNameFloatValuePair;
    #[cfg(feature = "MaterialPropertyValuesSetter+PropertyNameIntValuePair")]
    pub type PropertyNameIntValuePair = crate::GlobalNamespace::MaterialPropertyValuesSetter_PropertyNameIntValuePair;
    #[cfg(feature = "MaterialPropertyValuesSetter+PropertyNameVectorValuePair")]
    pub type PropertyNameVectorValuePair = crate::GlobalNamespace::MaterialPropertyValuesSetter_PropertyNameVectorValuePair;
    #[cfg(feature = "MaterialPropertyValuesSetter+PropertyValuePairBase")]
    pub type PropertyValuePairBase = crate::GlobalNamespace::MaterialPropertyValuesSetter_PropertyValuePairBase;
    pub fn ApplyParams(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("ApplyParams", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
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
    pub fn RefreshPropertyIds(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshPropertyIds", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
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
}
#[cfg(feature = "MaterialPropertyValuesSetter")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MaterialPropertyValuesSetter {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MaterialPropertyValuesSetter+PropertyNameColorValuePair")]
#[repr(C)]
#[derive(Debug)]
pub struct MaterialPropertyValuesSetter_PropertyNameColorValuePair {
    __cordl_parent: crate::GlobalNamespace::MaterialPropertyValuesSetter_PropertyValuePairBase,
    pub color: crate::UnityEngine::Color,
}
#[cfg(feature = "MaterialPropertyValuesSetter+PropertyNameColorValuePair")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MaterialPropertyValuesSetter_PropertyNameColorValuePair => ""
    ."MaterialPropertyValuesSetter/PropertyNameColorValuePair"
);
#[cfg(feature = "MaterialPropertyValuesSetter+PropertyNameColorValuePair")]
impl std::ops::Deref
for crate::GlobalNamespace::MaterialPropertyValuesSetter_PropertyNameColorValuePair {
    type Target = crate::GlobalNamespace::MaterialPropertyValuesSetter_PropertyValuePairBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MaterialPropertyValuesSetter+PropertyNameColorValuePair")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MaterialPropertyValuesSetter_PropertyNameColorValuePair {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MaterialPropertyValuesSetter+PropertyNameColorValuePair")]
impl crate::GlobalNamespace::MaterialPropertyValuesSetter_PropertyNameColorValuePair {
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
#[cfg(feature = "MaterialPropertyValuesSetter+PropertyNameColorValuePair")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MaterialPropertyValuesSetter_PropertyNameColorValuePair {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MaterialPropertyValuesSetter+PropertyNameFloatValuePair")]
#[repr(C)]
#[derive(Debug)]
pub struct MaterialPropertyValuesSetter_PropertyNameFloatValuePair {
    __cordl_parent: crate::GlobalNamespace::MaterialPropertyValuesSetter_PropertyValuePairBase,
    pub value: f32,
}
#[cfg(feature = "MaterialPropertyValuesSetter+PropertyNameFloatValuePair")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MaterialPropertyValuesSetter_PropertyNameFloatValuePair => ""
    ."MaterialPropertyValuesSetter/PropertyNameFloatValuePair"
);
#[cfg(feature = "MaterialPropertyValuesSetter+PropertyNameFloatValuePair")]
impl std::ops::Deref
for crate::GlobalNamespace::MaterialPropertyValuesSetter_PropertyNameFloatValuePair {
    type Target = crate::GlobalNamespace::MaterialPropertyValuesSetter_PropertyValuePairBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MaterialPropertyValuesSetter+PropertyNameFloatValuePair")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MaterialPropertyValuesSetter_PropertyNameFloatValuePair {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MaterialPropertyValuesSetter+PropertyNameFloatValuePair")]
impl crate::GlobalNamespace::MaterialPropertyValuesSetter_PropertyNameFloatValuePair {
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
#[cfg(feature = "MaterialPropertyValuesSetter+PropertyNameFloatValuePair")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MaterialPropertyValuesSetter_PropertyNameFloatValuePair {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MaterialPropertyValuesSetter+PropertyNameIntValuePair")]
#[repr(C)]
#[derive(Debug)]
pub struct MaterialPropertyValuesSetter_PropertyNameIntValuePair {
    __cordl_parent: crate::GlobalNamespace::MaterialPropertyValuesSetter_PropertyValuePairBase,
    pub value: i32,
}
#[cfg(feature = "MaterialPropertyValuesSetter+PropertyNameIntValuePair")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MaterialPropertyValuesSetter_PropertyNameIntValuePair => ""
    ."MaterialPropertyValuesSetter/PropertyNameIntValuePair"
);
#[cfg(feature = "MaterialPropertyValuesSetter+PropertyNameIntValuePair")]
impl std::ops::Deref
for crate::GlobalNamespace::MaterialPropertyValuesSetter_PropertyNameIntValuePair {
    type Target = crate::GlobalNamespace::MaterialPropertyValuesSetter_PropertyValuePairBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MaterialPropertyValuesSetter+PropertyNameIntValuePair")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MaterialPropertyValuesSetter_PropertyNameIntValuePair {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MaterialPropertyValuesSetter+PropertyNameIntValuePair")]
impl crate::GlobalNamespace::MaterialPropertyValuesSetter_PropertyNameIntValuePair {
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
#[cfg(feature = "MaterialPropertyValuesSetter+PropertyNameIntValuePair")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MaterialPropertyValuesSetter_PropertyNameIntValuePair {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MaterialPropertyValuesSetter+PropertyNameVectorValuePair")]
#[repr(C)]
#[derive(Debug)]
pub struct MaterialPropertyValuesSetter_PropertyNameVectorValuePair {
    __cordl_parent: crate::GlobalNamespace::MaterialPropertyValuesSetter_PropertyValuePairBase,
    pub vector: crate::UnityEngine::Vector4,
}
#[cfg(feature = "MaterialPropertyValuesSetter+PropertyNameVectorValuePair")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MaterialPropertyValuesSetter_PropertyNameVectorValuePair => ""
    ."MaterialPropertyValuesSetter/PropertyNameVectorValuePair"
);
#[cfg(feature = "MaterialPropertyValuesSetter+PropertyNameVectorValuePair")]
impl std::ops::Deref
for crate::GlobalNamespace::MaterialPropertyValuesSetter_PropertyNameVectorValuePair {
    type Target = crate::GlobalNamespace::MaterialPropertyValuesSetter_PropertyValuePairBase;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MaterialPropertyValuesSetter+PropertyNameVectorValuePair")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MaterialPropertyValuesSetter_PropertyNameVectorValuePair {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MaterialPropertyValuesSetter+PropertyNameVectorValuePair")]
impl crate::GlobalNamespace::MaterialPropertyValuesSetter_PropertyNameVectorValuePair {
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
#[cfg(feature = "MaterialPropertyValuesSetter+PropertyNameVectorValuePair")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MaterialPropertyValuesSetter_PropertyNameVectorValuePair {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "MaterialPropertyValuesSetter+PropertyValuePairBase")]
#[repr(C)]
#[derive(Debug)]
pub struct MaterialPropertyValuesSetter_PropertyValuePairBase {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub _propertyName: *mut quest_hook::libil2cpp::Il2CppString,
    pub _propertyId_k__BackingField: i32,
}
#[cfg(feature = "MaterialPropertyValuesSetter+PropertyValuePairBase")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::MaterialPropertyValuesSetter_PropertyValuePairBase => ""
    ."MaterialPropertyValuesSetter/PropertyValuePairBase"
);
#[cfg(feature = "MaterialPropertyValuesSetter+PropertyValuePairBase")]
impl std::ops::Deref
for crate::GlobalNamespace::MaterialPropertyValuesSetter_PropertyValuePairBase {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "MaterialPropertyValuesSetter+PropertyValuePairBase")]
impl std::ops::DerefMut
for crate::GlobalNamespace::MaterialPropertyValuesSetter_PropertyValuePairBase {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "MaterialPropertyValuesSetter+PropertyValuePairBase")]
impl crate::GlobalNamespace::MaterialPropertyValuesSetter_PropertyValuePairBase {
    pub fn New() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn RefreshPropertyId(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("RefreshPropertyId", ())?;
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
    pub fn get_propertyId(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("get_propertyId", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn set_propertyId(
        &mut self,
        value: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_propertyId", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "MaterialPropertyValuesSetter+PropertyValuePairBase")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::MaterialPropertyValuesSetter_PropertyValuePairBase {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
