#[cfg(feature = "BloomPrePassBackgroundColorsGradient")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomPrePassBackgroundColorsGradient {
    __cordl_parent: crate::GlobalNamespace::BloomPrePassBackgroundTextureGradient,
    pub _elements: *mut quest_hook::libil2cpp::Il2CppArray<
        *mut crate::GlobalNamespace::BloomPrePassBackgroundColorsGradient_Element,
    >,
}
#[cfg(feature = "BloomPrePassBackgroundColorsGradient")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BloomPrePassBackgroundColorsGradient => ""
    ."BloomPrePassBackgroundColorsGradient"
);
#[cfg(feature = "BloomPrePassBackgroundColorsGradient")]
impl std::ops::Deref for crate::GlobalNamespace::BloomPrePassBackgroundColorsGradient {
    type Target = crate::GlobalNamespace::BloomPrePassBackgroundTextureGradient;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassBackgroundColorsGradient")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BloomPrePassBackgroundColorsGradient {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassBackgroundColorsGradient")]
impl crate::GlobalNamespace::BloomPrePassBackgroundColorsGradient {
    #[cfg(feature = "BloomPrePassBackgroundColorsGradient+Element")]
    pub type Element = crate::GlobalNamespace::BloomPrePassBackgroundColorsGradient_Element;
    pub fn EvaluateColor(
        &mut self,
        t: f32,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("EvaluateColor", (t))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<*mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
    pub fn UpdatePixels(
        &mut self,
        pixels: crate::Unity::Collections::NativeArray_1<crate::UnityEngine::Color32>,
        numberOfPixels: i32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("UpdatePixels", (pixels, numberOfPixels))?;
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
    pub fn get_elements(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<
        *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::BloomPrePassBackgroundColorsGradient_Element,
        >,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: *mut quest_hook::libil2cpp::Il2CppArray<
            *mut crate::GlobalNamespace::BloomPrePassBackgroundColorsGradient_Element,
        > = __cordl_object.invoke("get_elements", ())?;
        Ok(__cordl_ret)
    }
}
#[cfg(feature = "BloomPrePassBackgroundColorsGradient")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BloomPrePassBackgroundColorsGradient {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "BloomPrePassBackgroundColorsGradient+Element")]
#[repr(C)]
#[derive(Debug)]
pub struct BloomPrePassBackgroundColorsGradient_Element {
    __cordl_parent: crate::System::Object,
    pub color: crate::UnityEngine::Color,
    pub startT: f32,
    pub exp: f32,
}
#[cfg(feature = "BloomPrePassBackgroundColorsGradient+Element")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate
    ::GlobalNamespace::BloomPrePassBackgroundColorsGradient_Element => ""
    ."BloomPrePassBackgroundColorsGradient/Element"
);
#[cfg(feature = "BloomPrePassBackgroundColorsGradient+Element")]
impl std::ops::Deref
for crate::GlobalNamespace::BloomPrePassBackgroundColorsGradient_Element {
    type Target = crate::System::Object;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassBackgroundColorsGradient+Element")]
impl std::ops::DerefMut
for crate::GlobalNamespace::BloomPrePassBackgroundColorsGradient_Element {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "BloomPrePassBackgroundColorsGradient+Element")]
impl crate::GlobalNamespace::BloomPrePassBackgroundColorsGradient_Element {
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
#[cfg(feature = "BloomPrePassBackgroundColorsGradient+Element")]
impl quest_hook::libil2cpp::ObjectType
for crate::GlobalNamespace::BloomPrePassBackgroundColorsGradient_Element {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
