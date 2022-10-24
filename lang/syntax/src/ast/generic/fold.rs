use std::marker::PhantomData;
use std::rc::Rc;

use data::HashMap;

use crate::common::*;
use crate::de_bruijn::*;

use super::def::*;

#[rustfmt::skip]
pub trait Folder<P: Phase, O: Out> {
    fn fold_prg(&mut self, decls: O::Decls, exp: Option<O::Exp>) -> O::Prg;
    fn fold_decls(&mut self, map: HashMap<Ident, O::Decl>, order: Vec<Ident>) -> O::Decls;
    fn fold_decl_data(&mut self, data: O::Data) -> O::Decl;
    fn fold_decl_codata(&mut self, codata: O::Codata) -> O::Decl;
    fn fold_decl_ctor(&mut self, ctor: O::Ctor) -> O::Decl;
    fn fold_decl_dtor(&mut self, dtor: O::Dtor) -> O::Decl;
    fn fold_decl_def(&mut self, def: O::Def) -> O::Decl;
    fn fold_decl_codef(&mut self, codef: O::Codef) -> O::Decl;
    fn fold_data(&mut self, info: O::Info, name: Ident, typ: O::TypAbs, ctors: Vec<Ident>, impl_block: Option<O::Impl>) -> O::Data;
    fn fold_codata(&mut self, info: O::Info, name: Ident, typ: O::TypAbs, dtors: Vec<Ident>, impl_block: Option<O::Impl>) -> O::Codata;
    fn fold_impl(&mut self, info: O::Info, name: Ident, defs: Vec<Ident>) -> O::Impl;
    fn fold_typ_abs(&mut self, params: O::Telescope) -> O::TypAbs;
    fn fold_ctor(&mut self, info: O::Info, name: Ident, params: O::Telescope, typ: O::TypApp) -> O::Ctor;
    fn fold_dtor(&mut self, info: O::Info, name: Ident, params: O::Telescope, on_typ: O::TypApp, in_typ: O::Exp) -> O::Dtor;
    fn fold_def(&mut self, info: O::Info, name: Ident, params: O::Telescope, on_typ: O::TypApp, in_typ: O::Exp, body: O::Match) -> O::Def;
    fn fold_codef(&mut self, info: O::Info, name: Ident, params: O::Telescope, typ: O::TypApp, body: O::Comatch) -> O::Codef;
    fn fold_match(&mut self, info: O::Info, cases: Vec<O::Case>) -> O::Match;
    fn fold_comatch(&mut self, info: O::Info, cases: Vec<O::Cocase>) -> O::Comatch;
    fn fold_case(&mut self, info: O::Info, name: Ident, args: O::Telescope, body: Option<O::Exp>) -> O::Case;
    fn fold_cocase(&mut self, info: O::Info, name: Ident, args: O::Telescope, body: Option<O::Exp>) -> O::Cocase;
    fn fold_typ_app(&mut self, info: O::TypeInfo, name: Ident, args: Vec<O::Exp>) -> O::TypApp;
    fn fold_exp_var(&mut self, info: O::TypeInfo, name: P::VarName, idx: O::Idx) -> O::Exp;
    fn fold_exp_typ_ctor(&mut self, info: O::TypeInfo, name: Ident, args: Vec<O::Exp>) -> O::Exp;
    fn fold_exp_ctor(&mut self, info: O::TypeInfo, name: Ident, args: Vec<O::Exp>) -> O::Exp;
    fn fold_exp_dtor(&mut self, info: O::TypeInfo, exp: O::Exp, name: Ident, args: Vec<O::Exp>) -> O::Exp;
    fn fold_exp_anno(&mut self, info: O::TypeInfo, exp: O::Exp, typ: O::Exp) -> O::Exp;
    fn fold_exp_type(&mut self, info: O::TypeInfo) -> O::Exp;
    fn fold_telescope<X, I, F1, F2>(&mut self, params: I, f_acc: F1, f_inner: F2) -> X
    where
        I: IntoIterator<Item=Param<P>>,
        F1: Fn(&mut Self, Param<P>) -> O::Param,
        F2: FnOnce(&mut Self, O::Telescope) -> X
    ;
    fn fold_param(&mut self, name: Ident, typ: O::Exp) -> O::Param;
    fn fold_info(&mut self, info: P::Info) -> O::Info;
    fn fold_type_info(&mut self, info: P::TypeInfo) -> O::TypeInfo;
    fn fold_idx(&mut self, idx: Idx) -> O::Idx;
}

pub trait Fold<P: Phase, O: Out> {
    type Out;

    fn fold<F>(self, f: &mut F) -> Self::Out
    where
        F: Folder<P, O>;
}

pub trait Out {
    type Prg;
    type Decls;
    type Decl;
    type Data;
    type Codata;
    type Impl;
    type TypAbs;
    type Ctor;
    type Dtor;
    type Def;
    type Codef;
    type Match;
    type Comatch;
    type Case;
    type Cocase;
    type TypApp;
    type Exp;
    type Telescope;
    type Param;
    type Info;
    type TypeInfo;
    type Idx;
}

pub struct Id<P: Phase> {
    phantom: PhantomData<P>,
}

impl<P: Phase> Out for Id<P> {
    type Prg = Prg<P>;
    type Decls = Decls<P>;
    type Decl = Decl<P>;
    type Data = Data<P>;
    type Codata = Codata<P>;
    type Impl = Impl<P>;
    type TypAbs = TypAbs<P>;
    type Ctor = Ctor<P>;
    type Dtor = Dtor<P>;
    type Def = Def<P>;
    type Codef = Codef<P>;
    type Match = Match<P>;
    type Comatch = Comatch<P>;
    type Case = Case<P>;
    type Cocase = Cocase<P>;
    type TypApp = Rc<TypApp<P>>;
    type Exp = Rc<Exp<P>>;
    type Telescope = Telescope<P>;
    type Param = Param<P>;
    type Info = P::Info;
    type TypeInfo = P::TypeInfo;
    type Idx = Idx;
}

pub struct Const<T> {
    phantom: PhantomData<T>,
}

impl<T> Out for Const<T> {
    type Prg = T;
    type Decls = T;
    type Decl = T;
    type Data = T;
    type Codata = T;
    type Impl = T;
    type TypAbs = T;
    type Ctor = T;
    type Dtor = T;
    type Def = T;
    type Codef = T;
    type Match = T;
    type Comatch = T;
    type Case = T;
    type Cocase = T;
    type TypApp = T;
    type Exp = T;
    type Telescope = T;
    type Param = T;
    type Info = T;
    type TypeInfo = T;
    type Idx = T;
}

impl<P: Phase, O: Out, T: Fold<P, O> + Clone> Fold<P, O> for Rc<T> {
    type Out = T::Out;

    fn fold<F>(self, f: &mut F) -> Self::Out
    where
        F: Folder<P, O>,
    {
        let x = (*self).clone();
        T::fold(x, f)
    }
}

impl<P: Phase, O: Out, T: Fold<P, O>> Fold<P, O> for Option<T> {
    type Out = Option<T::Out>;

    fn fold<F>(self, f: &mut F) -> Self::Out
    where
        F: Folder<P, O>,
    {
        self.map(|inner| inner.fold(f))
    }
}

impl<P: Phase, O: Out, T: Fold<P, O>> Fold<P, O> for Vec<T> {
    type Out = Vec<T::Out>;

    fn fold<F>(self, f: &mut F) -> Self::Out
    where
        F: Folder<P, O>,
    {
        self.into_iter().map(|inner| inner.fold(f)).collect()
    }
}

impl<P: Phase, O: Out> Fold<P, O> for Prg<P> {
    type Out = O::Prg;

    fn fold<F>(self, f: &mut F) -> Self::Out
    where
        F: Folder<P, O>,
    {
        let Prg { decls, exp } = self;
        let decls = decls.fold(f);
        let exp = exp.fold(f);
        f.fold_prg(decls, exp)
    }
}

impl<P: Phase, O: Out> Fold<P, O> for Decls<P> {
    type Out = O::Decls;

    fn fold<F>(self, f: &mut F) -> Self::Out
    where
        F: Folder<P, O>,
    {
        let Decls { map, order } = self;
        let map = map.into_iter().map(|(name, decl)| (name, decl.fold(f))).collect();
        f.fold_decls(map, order)
    }
}

impl<P: Phase, O: Out> Fold<P, O> for Decl<P> {
    type Out = O::Decl;

    fn fold<F>(self, f: &mut F) -> Self::Out
    where
        F: Folder<P, O>,
    {
        match self {
            Decl::Data(inner) => {
                let inner = inner.fold(f);
                f.fold_decl_data(inner)
            }
            Decl::Codata(inner) => {
                let inner = inner.fold(f);
                f.fold_decl_codata(inner)
            }
            Decl::Ctor(inner) => {
                let inner = inner.fold(f);
                f.fold_decl_ctor(inner)
            }
            Decl::Dtor(inner) => {
                let inner = inner.fold(f);
                f.fold_decl_dtor(inner)
            }
            Decl::Def(inner) => {
                let inner = inner.fold(f);
                f.fold_decl_def(inner)
            }
            Decl::Codef(inner) => {
                let inner = inner.fold(f);
                f.fold_decl_codef(inner)
            }
        }
    }
}

impl<P: Phase, O: Out> Fold<P, O> for Data<P> {
    type Out = O::Data;

    fn fold<F>(self, f: &mut F) -> Self::Out
    where
        F: Folder<P, O>,
    {
        let Data { info, name, typ, ctors, impl_block } = self;
        let typ = typ.fold(f);
        let impl_block = impl_block.fold(f);
        let info = f.fold_info(info);
        f.fold_data(info, name, typ, ctors, impl_block)
    }
}

impl<P: Phase, O: Out> Fold<P, O> for Codata<P> {
    type Out = O::Codata;

    fn fold<F>(self, f: &mut F) -> Self::Out
    where
        F: Folder<P, O>,
    {
        let Codata { info, name, typ, dtors, impl_block } = self;
        let typ = typ.fold(f);
        let impl_block = impl_block.fold(f);
        let info = f.fold_info(info);
        f.fold_codata(info, name, typ, dtors, impl_block)
    }
}

impl<P: Phase, O: Out> Fold<P, O> for Impl<P> {
    type Out = O::Impl;

    fn fold<F>(self, f: &mut F) -> Self::Out
    where
        F: Folder<P, O>,
    {
        let Impl { info, name, defs } = self;
        let info = f.fold_info(info);
        f.fold_impl(info, name, defs)
    }
}

impl<P: Phase, O: Out> Fold<P, O> for TypAbs<P> {
    type Out = O::TypAbs;

    fn fold<F>(self, f: &mut F) -> Self::Out
    where
        F: Folder<P, O>,
    {
        let TypAbs { params } = self;
        let Telescope { params } = params;
        let params = f.fold_telescope(params, |f, param| param.fold(f), |_, params| params);
        f.fold_typ_abs(params)
    }
}

impl<P: Phase, O: Out> Fold<P, O> for Ctor<P> {
    type Out = O::Ctor;

    fn fold<F>(self, f: &mut F) -> Self::Out
    where
        F: Folder<P, O>,
    {
        let Ctor { info, name, params, typ } = self;
        let Telescope { params } = params;
        let (params, typ) =
            f.fold_telescope(params, |f, param| param.fold(f), |f, params| (params, typ.fold(f)));
        let info = f.fold_info(info);
        f.fold_ctor(info, name, params, typ)
    }
}

impl<P: Phase, O: Out> Fold<P, O> for Dtor<P> {
    type Out = O::Dtor;

    fn fold<F>(self, f: &mut F) -> Self::Out
    where
        F: Folder<P, O>,
    {
        let Dtor { info, name, params, on_typ, in_typ } = self;
        let Telescope { params } = params;
        let (params, on_typ, in_typ) = f.fold_telescope(
            params,
            |f, param| param.fold(f),
            |f, params| (params, on_typ.fold(f), in_typ.fold(f)),
        );
        let info = f.fold_info(info);
        f.fold_dtor(info, name, params, on_typ, in_typ)
    }
}

impl<P: Phase, O: Out> Fold<P, O> for Def<P> {
    type Out = O::Def;

    fn fold<F>(self, f: &mut F) -> Self::Out
    where
        F: Folder<P, O>,
    {
        let Def { info, name, params, on_typ, in_typ, body } = self;
        let Telescope { params } = params;
        let (params, on_typ, in_typ, body) = f.fold_telescope(
            params,
            |f, param| param.fold(f),
            |f, params| (params, on_typ.fold(f), in_typ.fold(f), body.fold(f)),
        );
        let info = f.fold_info(info);
        f.fold_def(info, name, params, on_typ, in_typ, body)
    }
}

impl<P: Phase, O: Out> Fold<P, O> for Codef<P> {
    type Out = O::Codef;

    fn fold<F>(self, f: &mut F) -> Self::Out
    where
        F: Folder<P, O>,
    {
        let Codef { info, name, params, typ, body } = self;
        let Telescope { params } = params;
        let (params, typ, body) = f.fold_telescope(
            params,
            |f, param| param.fold(f),
            |f, params| (params, typ.fold(f), body.fold(f)),
        );
        let info = f.fold_info(info);
        f.fold_codef(info, name, params, typ, body)
    }
}

impl<P: Phase, O: Out> Fold<P, O> for Match<P> {
    type Out = O::Match;

    fn fold<F>(self, f: &mut F) -> Self::Out
    where
        F: Folder<P, O>,
    {
        let Match { info, cases } = self;
        let cases = cases.fold(f);
        let info = f.fold_info(info);
        f.fold_match(info, cases)
    }
}

impl<P: Phase, O: Out> Fold<P, O> for Comatch<P> {
    type Out = O::Comatch;

    fn fold<F>(self, f: &mut F) -> Self::Out
    where
        F: Folder<P, O>,
    {
        let Comatch { info, cases } = self;
        let cases = cases.fold(f);
        let info = f.fold_info(info);
        f.fold_comatch(info, cases)
    }
}

impl<P: Phase, O: Out> Fold<P, O> for Case<P> {
    type Out = O::Case;

    fn fold<F>(self, f: &mut F) -> Self::Out
    where
        F: Folder<P, O>,
    {
        let Case { info, name, args, body } = self;
        let Telescope { params } = args;
        let (args, body) =
            f.fold_telescope(params, |f, arg| arg.fold(f), |f, args| (args, body.fold(f)));
        let info = f.fold_info(info);
        f.fold_case(info, name, args, body)
    }
}

impl<P: Phase, O: Out> Fold<P, O> for Cocase<P> {
    type Out = O::Cocase;

    fn fold<F>(self, f: &mut F) -> Self::Out
    where
        F: Folder<P, O>,
    {
        let Cocase { info, name, args, body } = self;
        let Telescope { params } = args;
        let (args, body) =
            f.fold_telescope(params, |f, arg| arg.fold(f), |f, args| (args, body.fold(f)));
        let info = f.fold_info(info);
        f.fold_cocase(info, name, args, body)
    }
}

impl<P: Phase, O: Out> Fold<P, O> for TypApp<P> {
    type Out = O::TypApp;

    fn fold<F>(self, f: &mut F) -> Self::Out
    where
        F: Folder<P, O>,
    {
        let TypApp { info, name, args } = self;
        let args = args.fold(f);
        let info = f.fold_type_info(info);
        f.fold_typ_app(info, name, args)
    }
}

impl<P: Phase, O: Out> Fold<P, O> for Exp<P> {
    type Out = O::Exp;

    fn fold<F>(self, f: &mut F) -> Self::Out
    where
        F: Folder<P, O>,
    {
        match self {
            Exp::Var { info, name, idx } => {
                let info = f.fold_type_info(info);
                let idx = f.fold_idx(idx);
                f.fold_exp_var(info, name, idx)
            }
            Exp::TypCtor { info, name, args } => {
                let args = args.fold(f);
                let info = f.fold_type_info(info);
                f.fold_exp_typ_ctor(info, name, args)
            }
            Exp::Ctor { info, name, args } => {
                let args = args.fold(f);
                let info = f.fold_type_info(info);
                f.fold_exp_ctor(info, name, args)
            }
            Exp::Dtor { info, exp, name, args } => {
                let exp = exp.fold(f);
                let args = args.fold(f);
                let info = f.fold_type_info(info);
                f.fold_exp_dtor(info, exp, name, args)
            }
            Exp::Anno { info, exp, typ } => {
                let exp = exp.fold(f);
                let typ = typ.fold(f);
                let info = f.fold_type_info(info);
                f.fold_exp_anno(info, exp, typ)
            }
            Exp::Type { info } => {
                let info = f.fold_type_info(info);
                f.fold_exp_type(info)
            }
        }
    }
}

impl<P: Phase, O: Out> Fold<P, O> for Param<P> {
    type Out = O::Param;

    fn fold<F>(self, f: &mut F) -> Self::Out
    where
        F: Folder<P, O>,
    {
        let Param { name, typ } = self;
        let typ = typ.fold(f);
        f.fold_param(name, typ)
    }
}