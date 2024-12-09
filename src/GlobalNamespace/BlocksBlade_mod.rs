#[cfg(feature = "BlocksBlade")]
#[repr(C)]
#[derive(Debug)]
pub struct BlocksBlade {
    __cordl_parent: crate::UnityEngine::MonoBehaviour,
    pub _elementMesh: *mut crate::UnityEngine::Mesh,
    pub _material: *mut crate::UnityEngine::Material,
    pub _numberOfElements: i32,
    pub _radius: f32,
    pub _length: f32,
    pub _minVelocity: f32,
    pub _maxVelocity: f32,
    pub _elementWidth: f32,
    pub _minElementLength: f32,
    pub _maxElementLength: f32,
    pub _color_k__BackingField: crate::UnityEngine::Color,
    pub _elements: *mut crate::System::Collections::Generic::List_1<
        *mut crate::GlobalNamespace::BlocksBlade_Element,
    >,
    pub _positions: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
    pub _sizes: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
    pub _colors: *mut quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
    pub _matrices: *mut quest_hook::libil2cpp::Il2CppArray<
        crate::UnityEngine::Matrix4x4,
    >,
    pub _materialPropertyBlock: *mut crate::UnityEngine::MaterialPropertyBlock,
    pub _layer: i32,
}
#[cfg(feature = "BlocksBlade")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BlocksBlade => ""."BlocksBlade"
);
#[cfg(feature = "BlocksBlade")]
impl std::ops::Deref for crate::GlobalNamespace::BlocksBlade {
    type Target = crate::UnityEngine::MonoBehaviour;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BlocksBlade")]
impl std::ops::DerefMut for crate::GlobalNamespace::BlocksBlade {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BlocksBlade")]
impl crate::GlobalNamespace::BlocksBlade {
    #[cfg(feature = "BlocksBlade+Element")]
    pub type Element = crate::GlobalNamespace::BlocksBlade_Element;
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn RandomPointOnCircle(
        &mut self,
        radius: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("RandomPointOnCircle", (radius))?;
        Ok(__cordl_ret)
    }
    pub fn SetUpElement(
        &mut self,
        element: *mut crate::GlobalNamespace::BlocksBlade_Element,
        velocity: f32,
        color: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("SetUpElement", (element, velocity, color))?;
        Ok(__cordl_ret)
    }
    pub fn Start(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Start", ())?;
        Ok(__cordl_ret)
    }
    pub fn Update(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Update", ())?;
        Ok(__cordl_ret)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
    pub fn get_color(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_color", ())?;
        Ok(__cordl_ret)
    }
    pub fn set_color(
        &mut self,
        value: crate::UnityEngine::Color,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_color", (value))?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BlocksBlade")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BlocksBlade {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BlocksBlade+Element")]
#[repr(C)]
#[derive(Debug)]
pub struct BlocksBlade_Element {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub idx: i32,
    pub velocity: f32,
}
#[cfg(feature = "BlocksBlade+Element")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::GlobalNamespace::BlocksBlade_Element => ""
    ."BlocksBlade/Element"
);
#[cfg(feature = "BlocksBlade+Element")]
impl std::ops::Deref for crate::GlobalNamespace::BlocksBlade_Element {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BlocksBlade+Element")]
impl std::ops::DerefMut for crate::GlobalNamespace::BlocksBlade_Element {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BlocksBlade+Element")]
impl crate::GlobalNamespace::BlocksBlade_Element {
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn _ctor(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BlocksBlade+Element")]
impl quest_hook::libil2cpp::ObjectType for crate::GlobalNamespace::BlocksBlade_Element {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
