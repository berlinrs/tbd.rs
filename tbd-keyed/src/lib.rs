#![feature(specialization)]

extern crate tbd_core;

use tbd_core::model_wrappers::Wrapper;
use tbd_core::types::ModelLifeCycle;

use std::ops::{Deref, DerefMut};

pub trait Key: Clone {}

impl Key for u64 {}
impl Key for i64 {}

pub struct Keyed<PK: Key, W: Wrapper> where PK: Key,
                                            W: Wrapper + ModelLifeCycle {
    pub pk: Option<PK>,
    m: W
}

impl<PK, W> Keyed<PK, W> where PK: Key,
                               W: Wrapper<Returning=W> + ModelLifeCycle {
    pub fn new(w: W) -> Keyed<PK, W> {
        Keyed { pk: None, m: w }
    }
} 

impl<PK, W> Wrapper for Keyed<PK, W> where PK: Key,
                                           W: Wrapper<Returning=W> + ModelLifeCycle {
    type Wrapping = W::Wrapping;
    type Returning = Self;

    fn wrap(m: W::Wrapping) -> Self {
        let wrapped = W::wrap(m);
        Keyed { pk: None, m: wrapped }
    }
}

impl<PK, W> Deref for Keyed<PK, W> where PK: Key,
                                             W: Wrapper + ModelLifeCycle {
    type Target = W;

    fn deref(&self) -> &W {
        &self.m
    }
}

impl<PK, W> DerefMut for Keyed<PK, W> where PK: Key,
                                             W: Wrapper + ModelLifeCycle {    
    fn deref_mut(&mut self) -> &mut W {
        &mut self.m
    }
}

impl<PK, W> ModelLifeCycle for Keyed<PK, W> where PK: Key,
                                                  W: Wrapper<Returning=W> + ModelLifeCycle<PrimaryKey=PK> {
    type PrimaryKey = PK;

    fn created(&mut self, pk: PK) {
        self.pk = Some(pk.clone());
        self.m.created(pk);
    }
}