#[cfg(feature = "Unity+Mathematics+noise")]
#[repr(C)]
#[derive(Debug)]
pub struct noise {
    __cordl_parent: quest_hook::libil2cpp::Il2CppObject,
}
#[cfg(feature = "Unity+Mathematics+noise")]
quest_hook::libil2cpp::unsafe_impl_reference_type!(
    in quest_hook::libil2cpp for crate ::Unity::Mathematics::noise => "Unity.Mathematics"
    ."noise"
);
#[cfg(feature = "Unity+Mathematics+noise")]
impl std::ops::Deref for crate::Unity::Mathematics::noise {
    type Target = quest_hook::libil2cpp::Il2CppObject;
    fn deref(&self) -> &Self::Target {
        unsafe { &self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Mathematics+noise")]
impl std::ops::DerefMut for crate::Unity::Mathematics::noise {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut self.__cordl_parent }
    }
}
#[cfg(feature = "Unity+Mathematics+noise")]
impl crate::Unity::Mathematics::noise {
    pub fn cellular2x2(
        P: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cellular2x2", (P))?;
        Ok(__cordl_ret.into())
    }
    pub fn cellular2x2x2(
        P: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cellular2x2x2", (P))?;
        Ok(__cordl_ret.into())
    }
    pub fn cellular_float2_0(
        P: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cellular", (P))?;
        Ok(__cordl_ret.into())
    }
    pub fn cellular_float3_1(
        P: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cellular", (P))?;
        Ok(__cordl_ret.into())
    }
    pub fn cnoise_float2_0(
        P: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cnoise", (P))?;
        Ok(__cordl_ret.into())
    }
    pub fn cnoise_float3_1(
        P: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cnoise", (P))?;
        Ok(__cordl_ret.into())
    }
    pub fn cnoise_float4_2(
        P: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("cnoise", (P))?;
        Ok(__cordl_ret.into())
    }
    pub fn fade_float2_0(
        t: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("fade", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn fade_float3_1(
        t: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("fade", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn fade_float4_2(
        t: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("fade", (t))?;
        Ok(__cordl_ret.into())
    }
    pub fn grad4(
        j: f32,
        ip: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("grad4", (j, ip))?;
        Ok(__cordl_ret.into())
    }
    pub fn mod289_f32_0(x: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mod289", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn mod289_float2_1(
        x: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mod289", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn mod289_float3_2(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mod289", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn mod289_float4_3(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mod289", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn mod7_float3_0(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mod7", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn mod7_float4_1(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("mod7", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn permute_f32_0(x: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("permute", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn permute_float3_1(
        x: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("permute", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn permute_float4_2(
        x: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("permute", (x))?;
        Ok(__cordl_ret.into())
    }
    pub fn pnoise_float2_float2_0(
        P: crate::Unity::Mathematics::float2,
        rep: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("pnoise", (P, rep))?;
        Ok(__cordl_ret.into())
    }
    pub fn pnoise_float3_float3_1(
        P: crate::Unity::Mathematics::float3,
        rep: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("pnoise", (P, rep))?;
        Ok(__cordl_ret.into())
    }
    pub fn pnoise_float4_float4_2(
        P: crate::Unity::Mathematics::float4,
        rep: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("pnoise", (P, rep))?;
        Ok(__cordl_ret.into())
    }
    pub fn psrdnoise_f32_0(
        pos: crate::Unity::Mathematics::float2,
        per: crate::Unity::Mathematics::float2,
        rot: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("psrdnoise", (pos, per, rot))?;
        Ok(__cordl_ret.into())
    }
    pub fn psrdnoise_float2_float2_1(
        pos: crate::Unity::Mathematics::float2,
        per: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("psrdnoise", (pos, per))?;
        Ok(__cordl_ret.into())
    }
    pub fn psrnoise_f32_0(
        pos: crate::Unity::Mathematics::float2,
        per: crate::Unity::Mathematics::float2,
        rot: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("psrnoise", (pos, per, rot))?;
        Ok(__cordl_ret.into())
    }
    pub fn psrnoise_float2_float2_1(
        pos: crate::Unity::Mathematics::float2,
        per: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("psrnoise", (pos, per))?;
        Ok(__cordl_ret.into())
    }
    pub fn rgrad2(
        p: crate::Unity::Mathematics::float2,
        rot: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float2> {
        let __cordl_ret: crate::Unity::Mathematics::float2 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("rgrad2", (p, rot))?;
        Ok(__cordl_ret.into())
    }
    pub fn snoise_float2_0(
        v: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("snoise", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn snoise_float3_1(
        v: crate::Unity::Mathematics::float3,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("snoise", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn snoise_float3_ByRefMut2(
        v: crate::Unity::Mathematics::float3,
        gradient: quest_hook::libil2cpp::ByRefMut<crate::Unity::Mathematics::float3>,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("snoise", (v, gradient))?;
        Ok(__cordl_ret.into())
    }
    pub fn snoise_float4_3(
        v: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("snoise", (v))?;
        Ok(__cordl_ret.into())
    }
    pub fn srdnoise_f32_0(
        pos: crate::Unity::Mathematics::float2,
        rot: f32,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("srdnoise", (pos, rot))?;
        Ok(__cordl_ret.into())
    }
    pub fn srdnoise_float2_1(
        pos: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float3> {
        let __cordl_ret: crate::Unity::Mathematics::float3 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("srdnoise", (pos))?;
        Ok(__cordl_ret.into())
    }
    pub fn srnoise_f32_0(
        pos: crate::Unity::Mathematics::float2,
        rot: f32,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("srnoise", (pos, rot))?;
        Ok(__cordl_ret.into())
    }
    pub fn srnoise_float2_1(
        pos: crate::Unity::Mathematics::float2,
    ) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("srnoise", (pos))?;
        Ok(__cordl_ret.into())
    }
    pub fn taylorInvSqrt_f32_0(r: f32) -> quest_hook::libil2cpp::Result<f32> {
        let __cordl_ret: f32 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("taylorInvSqrt", (r))?;
        Ok(__cordl_ret.into())
    }
    pub fn taylorInvSqrt_float4_1(
        r: crate::Unity::Mathematics::float4,
    ) -> quest_hook::libil2cpp::Result<crate::Unity::Mathematics::float4> {
        let __cordl_ret: crate::Unity::Mathematics::float4 = <Self as quest_hook::libil2cpp::Type>::class()
            .invoke("taylorInvSqrt", (r))?;
        Ok(__cordl_ret.into())
    }
}
#[cfg(feature = "Unity+Mathematics+noise")]
impl quest_hook::libil2cpp::ObjectType for crate::Unity::Mathematics::noise {
    fn as_object(&self) -> &quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object(&self.__cordl_parent)
    }
    fn as_object_mut(&mut self) -> &mut quest_hook::libil2cpp::Il2CppObject {
        quest_hook::libil2cpp::ObjectType::as_object_mut(&mut self.__cordl_parent)
    }
}
