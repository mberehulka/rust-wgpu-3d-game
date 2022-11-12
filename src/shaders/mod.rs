pub mod basic_anim;
pub mod basic;

pub enum Material {
    BasicAnim(basic_anim::Material),
    Basic(basic::Material)
}