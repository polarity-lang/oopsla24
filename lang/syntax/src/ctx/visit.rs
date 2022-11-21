use crate::ast::*;
use crate::ctx::*;

pub trait VisitCtxExt<P: Phase> {
    fn ctx_visit_telescope<'a, I, F1, F2>(&mut self, params: I, f_acc: F1, f_inner: F2)
    where
        P: 'a,
        I: IntoIterator<Item = &'a Param<P>>,
        F1: Fn(&mut Self, &'a Param<P>),
        F2: FnOnce(&mut Self);

    fn ctx_visit_telescope_inst<'a, I, F1, F2>(&mut self, params: I, f_acc: F1, f_inner: F2)
    where
        P: 'a,
        I: IntoIterator<Item = &'a ParamInst<P>>,
        F1: Fn(&mut Self, &'a ParamInst<P>),
        F2: FnOnce(&mut Self);
}

impl<P: Phase, C: HasContext> VisitCtxExt<P> for C
where
    for<'a> &'a Param<P>: AsElement<<<C as HasContext>::Ctx as Context>::ElemIn>,
    for<'a> &'a ParamInst<P>: AsElement<<<C as HasContext>::Ctx as Context>::ElemIn>,
{
    fn ctx_visit_telescope<'a, I, F1, F2>(&mut self, params: I, f_acc: F1, f_inner: F2)
    where
        P: 'a,
        I: IntoIterator<Item = &'a Param<P>>,
        F1: Fn(&mut Self, &'a Param<P>),
        F2: FnOnce(&mut Self),
    {
        self.bind_fold(
            params.into_iter(),
            Vec::new(),
            |this, mut params_out, param| {
                f_acc(this, param);
                params_out.push(());
                params_out
            },
            |this, _params| f_inner(this),
        )
    }

    fn ctx_visit_telescope_inst<'a, I, F1, F2>(&mut self, params: I, f_acc: F1, f_inner: F2)
    where
        P: 'a,
        I: IntoIterator<Item = &'a ParamInst<P>>,
        F1: Fn(&mut Self, &'a ParamInst<P>),
        F2: FnOnce(&mut Self),
    {
        self.bind_fold(
            params.into_iter(),
            Vec::new(),
            |this, mut params_out, param| {
                f_acc(this, param);
                params_out.push(());
                params_out
            },
            |this, _params| f_inner(this),
        )
    }
}