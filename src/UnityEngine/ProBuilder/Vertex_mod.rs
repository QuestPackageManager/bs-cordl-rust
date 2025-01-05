#[cfg(feature = "UnityEngine+ProBuilder+Vertex")]
#[repr(C)]
#[derive(Debug)]
pub struct Vertex {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
    pub m_Position: crate::UnityEngine::Vector3,
    pub m_Color: crate::UnityEngine::Color,
    pub m_Normal: crate::UnityEngine::Vector3,
    pub m_Tangent: crate::UnityEngine::Vector4,
    pub m_UV0: crate::UnityEngine::Vector2,
    pub m_UV2: crate::UnityEngine::Vector2,
    pub m_UV3: crate::UnityEngine::Vector4,
    pub m_UV4: crate::UnityEngine::Vector4,
    pub m_Attributes: crate::UnityEngine::ProBuilder::MeshArrays,
}
#[cfg(feature = "UnityEngine+ProBuilder+Vertex")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::UnityEngine::ProBuilder::Vertex =>
    "UnityEngine.ProBuilder"."Vertex"
);
#[cfg(feature = "UnityEngine+ProBuilder+Vertex")]
impl std::ops::Deref for crate::UnityEngine::ProBuilder::Vertex {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Vertex")]
impl std::ops::DerefMut for crate::UnityEngine::ProBuilder::Vertex {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Vertex")]
impl crate::UnityEngine::ProBuilder::Vertex {
    pub fn Add_Vertex0(
        a: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
        b: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Vertex,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Add", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn Add_Vertex1(
        &mut self,
        b: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Add", (b))?;
        Ok(__cordl_ret.into())
    }
    pub fn Average(
        vertices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
            >,
        >,
        indexes: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<i32>,
        >,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Vertex,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Average", (vertices, indexes))?;
        Ok(__cordl_ret.into())
    }
    pub fn Divide_Vertex_f32_0(
        a: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Vertex,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Divide", (a, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Divide_f32_1(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Divide", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Il2CppObject0(
        &mut self,
        obj: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppObject>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (obj))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Vertex1(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (other))?;
        Ok(__cordl_ret.into())
    }
    pub fn Equals_Vertex_MeshArrays2(
        &mut self,
        other: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
        mask: crate::UnityEngine::ProBuilder::MeshArrays,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("Equals", (other, mask))?;
        Ok(__cordl_ret.into())
    }
    pub fn GetArrays_IList_1_ByRefMut_ByRefMut_ByRefMut_ByRefMut_ByRefMut_ByRefMut_ByRefMut_ByRefMut0(
        vertices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
            >,
        >,
        position: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
            >,
        >,
        color: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
            >,
        >,
        uv0: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
            >,
        >,
        normal: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
            >,
        >,
        tangent: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
            >,
        >,
        uv2: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
            >,
        >,
        uv3: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector4>,
            >,
        >,
        uv4: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector4>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetArrays",
                (vertices, position, color, uv0, normal, tangent, uv2, uv3, uv4),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetArrays_MeshArrays1(
        vertices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
            >,
        >,
        position: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
            >,
        >,
        color: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Color>,
            >,
        >,
        uv0: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
            >,
        >,
        normal: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector3>,
            >,
        >,
        tangent: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector4>,
            >,
        >,
        uv2: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                quest_hook::libil2cpp::Il2CppArray<crate::UnityEngine::Vector2>,
            >,
        >,
        uv3: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector4>,
            >,
        >,
        uv4: quest_hook::libil2cpp::ByRefMut<
            quest_hook::libil2cpp::Gc<
                crate::System::Collections::Generic::List_1<crate::UnityEngine::Vector4>,
            >,
        >,
        attributes: crate::UnityEngine::ProBuilder::MeshArrays,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke(
                "GetArrays",
                (
                    vertices,
                    position,
                    color,
                    uv0,
                    normal,
                    tangent,
                    uv2,
                    uv3,
                    uv4,
                    attributes,
                ),
            )?;
        Ok(__cordl_ret.into())
    }
    pub fn GetHashCode(&mut self) -> quest_hook::libil2cpp::Result<i32> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: i32 = __cordl_object.invoke("GetHashCode", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn HasArrays(
        &mut self,
        attribute: crate::UnityEngine::ProBuilder::MeshArrays,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("HasArrays", (attribute))?;
        Ok(__cordl_ret.into())
    }
    pub fn Mix(
        x: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
        y: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
        weight: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Vertex,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Mix", (x, y, weight))?;
        Ok(__cordl_ret.into())
    }
    pub fn Multiply_Vertex_f32_0(
        a: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Vertex,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("Multiply", (a, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn Multiply_f32_1(
        &mut self,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Multiply", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn New_0() -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", ())?;
        Ok(__cordl_object.into())
    }
    pub fn New_Vertex1(
        vertex: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Gc<Self>> {
        let __cordl_object: &mut Self = <Self as quest_hook::libil2cpp::Type>::class()
            .instantiate();
        quest_hook::libil2cpp::ObjectType::as_object_mut(__cordl_object)
            .invoke_void(".ctor", (vertex))?;
        Ok(__cordl_object.into())
    }
    pub fn Normalize(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Normalize", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn SetMesh(
        mesh: quest_hook::libil2cpp::Gc<crate::UnityEngine::Mesh>,
        vertices: quest_hook::libil2cpp::Gc<
            crate::System::Collections::Generic::IList_1<
                quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
            >,
        >,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_ret: quest_hook::libil2cpp::Void = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("SetMesh", (mesh, vertices))?;
        Ok(__cordl_ret.into())
    }
    pub fn Subtract_Vertex0(
        a: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
        b: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Vertex,
        > = <Self as quest_hook::libil2cpp::Type>::class().invoke("Subtract", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn Subtract_Vertex1(
        &mut self,
        b: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("Subtract", (b))?;
        Ok(__cordl_ret.into())
    }
    pub fn ToString(
        &mut self,
        args: quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<quest_hook::libil2cpp::Il2CppString>,
    > {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            quest_hook::libil2cpp::Il2CppString,
        > = __cordl_object.invoke("ToString", (args))?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn _ctor_Vertex1(
        &mut self,
        vertex: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke(".ctor", (vertex))?;
        Ok(__cordl_ret.into())
    }
    pub fn get_attributes(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::ProBuilder::MeshArrays> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::ProBuilder::MeshArrays = __cordl_object
            .invoke("get_attributes", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_color(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Color> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Color = __cordl_object
            .invoke("get_color", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasColor(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasColor", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasNormal(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasNormal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasPosition(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasPosition", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasTangent(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasTangent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasUV0(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasUV0", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasUV2(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasUV2", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasUV3(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasUV3", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_hasUV4(&mut self) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: bool = __cordl_object.invoke("get_hasUV4", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_normal(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_normal", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_position(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector3> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector3 = __cordl_object
            .invoke("get_position", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_tangent(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector4 = __cordl_object
            .invoke("get_tangent", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_uv0(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_uv0", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_uv2(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector2> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector2 = __cordl_object
            .invoke("get_uv2", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_uv3(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector4 = __cordl_object
            .invoke("get_uv3", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn get_uv4(
        &mut self,
    ) -> quest_hook::libil2cpp::Result<crate::UnityEngine::Vector4> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: crate::UnityEngine::Vector4 = __cordl_object
            .invoke("get_uv4", ())?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Addition(
        a: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
        b: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Vertex,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Addition", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Division(
        a: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Vertex,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Division", (a, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Equality(
        a: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
        b: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Equality", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Inequality(
        a: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
        b: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
    ) -> quest_hook::libil2cpp::Result<bool> {
        let __cordl_ret: bool = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Inequality", (a, b))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Multiply(
        a: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
        value: f32,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Vertex,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Multiply", (a, value))?;
        Ok(__cordl_ret.into())
    }
    pub fn op_Subtraction(
        a: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
        b: quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
    ) -> quest_hook::libil2cpp::Result<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
    > {
        let __cordl_ret: quest_hook::libil2cpp::Gc<
            crate::UnityEngine::ProBuilder::Vertex,
        > = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("op_Subtraction", (a, b))?;
        Ok(__cordl_ret.into())
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
        Ok(__cordl_ret.into())
    }
    pub fn set_hasColor(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_hasColor", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_hasNormal(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_hasNormal", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_hasPosition(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_hasPosition", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_hasTangent(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_hasTangent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_hasUV0(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_hasUV0", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_hasUV2(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_hasUV2", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_hasUV3(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_hasUV3", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_hasUV4(
        &mut self,
        value: bool,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_hasUV4", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_normal(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_normal", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_position(
        &mut self,
        value: crate::UnityEngine::Vector3,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_position", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_tangent(
        &mut self,
        value: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_tangent", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_uv0(
        &mut self,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_uv0", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_uv2(
        &mut self,
        value: crate::UnityEngine::Vector2,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_uv2", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_uv3(
        &mut self,
        value: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_uv3", (value))?;
        Ok(__cordl_ret.into())
    }
    pub fn set_uv4(
        &mut self,
        value: crate::UnityEngine::Vector4,
    ) -> quest_hook::libil2cpp::Result<quest_hook::libil2cpp::Void> {
        let __cordl_object: &mut quest_hook::libil2cpp::Il2CppObject = quest_hook::libil2cpp::ObjectType::as_object_mut(
            self,
        );
        let __cordl_ret: quest_hook::libil2cpp::Void = __cordl_object
            .invoke("set_uv4", (value))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Vertex")]
impl quest_hook::libil2cpp::ObjectType for crate::UnityEngine::ProBuilder::Vertex {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Vertex")]
impl AsRef<
    crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
    >,
> for crate::UnityEngine::ProBuilder::Vertex {
    fn as_ref(
        &self,
    ) -> &crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
#[cfg(feature = "UnityEngine+ProBuilder+Vertex")]
impl AsMut<
    crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
    >,
> for crate::UnityEngine::ProBuilder::Vertex {
    fn as_mut(
        &mut self,
    ) -> &mut crate::System::IEquatable_1<
        quest_hook::libil2cpp::Gc<crate::UnityEngine::ProBuilder::Vertex>,
    > {
        unsafe { std::mem::transmute(self) }
    }
}
