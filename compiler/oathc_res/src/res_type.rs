use std::fmt::Debug;

pub trait ResType: Sized + Debug {
    type Src: Debug;

    fn buf(context: &ResContext) -> &QueryBuffer<Self>;

    fn drop_src(src: Self::Src, context: &ResContext);

    fn eval(ast: &Self::Src, namespace: &impl Namespace, context: &ResContext) -> Self;
}
