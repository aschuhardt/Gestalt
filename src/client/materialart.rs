use std::mem;
/*
pub trait MatArtStaticTyped {
    fn mat_art_type_static() -> String;
}*/
/*
impl<T> MatArtTypeLocal for T 
where T : MatArtTypeStatic {
    fn mat_art_type(&self) -> String {
        Self::mat_art_type_static()
    }
}*/

pub trait MaterialArt {
    fn get_render_type(&self) -> u32; /*In the files specifying renderers for individual MaterialArts, this will be 
    a string which we'll then run through a hashmap to get this u32.*/
    //fn get_data(&self) -> &[u8]; //Required for polymorphism shenanigans. Almost always involves the unsafe mem::transmute.
    //fn mat_art_type(&self) -> String;
}
#[derive(Clone, Debug)]
pub struct MatArtSimple { 
    pub texture_name : String,
    //pub cull_self : bool, //Do we cull the same material?
    //pub cull_others : bool, //Do we cull materials other than this one?
}
/*
impl MatArtStaticTyped for MatArtSimple {
    fn mat_art_type_static() -> String {String::from("MatArtSimple")}
}
*/
impl MaterialArt for MatArtSimple {
    fn get_render_type(&self) -> u32 { 1 }
    //fn mat_art_type() -> String {String::from("MatArtSimple")}
    /*fn get_data(&self) -> &[u8] {
        return unsafe { mem::transmute(self) };
    }*/
    //fn mat_art_type(&self) -> String { Self::mat_art_type_static() }
}
/*
pub fn mat_art_to<'a, T>(ma : &'a MaterialArt) -> Option<&'a T>
    where T : MaterialArt + MatArtStaticTyped  {
    if ma.mat_art_type() == T::mat_art_type_static() {
        unsafe { 
        let result : &T = mem::transmute(ma.get_data()); 
        return Some(result);
        }
    }
    else {
        return None;
    }
}*/