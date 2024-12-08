#[cfg(feature = "SliderMeshConstructorCrossedStrips")]
#[repr(C)]
#[derive(Debug)]
pub struct SliderMeshConstructorCrossedStrips {
    __cordl_parent: SliderMeshConstructor,
    pub _triangleMap: *mut quest_hook::libil2cpp::Il2CppArray<i32>,
}
#[cfg(feature = "SliderMeshConstructorCrossedStrips")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for SliderMeshConstructorCrossedStrips => ""
    ."SliderMeshConstructorCrossedStrips"
);
#[cfg(feature = "SliderMeshConstructorCrossedStrips")]
impl std::ops::Deref for SliderMeshConstructorCrossedStrips {
    type Target = SliderMeshConstructor;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "SliderMeshConstructorCrossedStrips")]
impl std::ops::DerefMut for SliderMeshConstructorCrossedStrips {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "SliderMeshConstructorCrossedStrips")]
impl SliderMeshConstructorCrossedStrips {
    pub fn CreateSliderMeshInternal(
        &mut self,
        path: *mut VertexPath,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("CreateSliderMeshInternal", (path))?;
        Ok(__cordl_ret)
    }
    pub fn GetTrianglesCount(
        &mut self,
        path: *mut VertexPath,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetTrianglesCount", (path))?;
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
    pub fn GetVertexCount(
        &mut self,
        path: *mut VertexPath,
    ) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetVertexCount", (path))?;
        Ok(__cordl_ret)
    }
    pub fn New() -> quest_hook::libil2cpp::Result<&'static mut Self> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object)
    }
}
#[cfg(feature = "SliderMeshConstructorCrossedStrips")]
impl quest_hook::libil2cpp::ObjectType for SliderMeshConstructorCrossedStrips {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
